use crate::bid_internal::*;
use crate::bid_types::{BidUint384, BidUint512};
use crate::bid128::*;
use crate::{BidUint64, BidUint128, BidUint192, BidUint256};

/// Rounds positive numbers with 2 <= q <= 18 to nearest only for 1 <= x <= 3.
///
/// Note:
///    In round128_2_18() positive numbers with 2 <= q <= 18 will be
///    rounded to nearest only for 1 <= x <= 3:
///     x = 1 or x = 2 when q = 17
///     x = 2 or x = 3 when q = 18
/// However, for generality and possible uses outside the frame of IEEE 754
/// this implementation works for 1 <= x <= q - 1
///
/// assume *ptr_is_midpoint_lt_even, *ptr_is_midpoint_gt_even,
/// *ptr_is_inexact_lt_midpoint, and *ptr_is_inexact_gt_midpoint are
/// initialized to 0 by the caller
///
/// round a number c with q decimal digits, 2 <= q <= 18
/// to q - x digits, 1 <= x <= 17
/// c = c + 1/2 * 10^x where the result c fits in 64 bits
/// (because the largest value is 999999999999999999 + 50000000000000000 =
/// 0x0e92596fd628ffff, which fits in 60 bits)
#[allow(clippy::too_many_arguments)]
pub fn bid_round64_2_18(
  q: i32,
  x: i32,
  mut c: BidUint64,
  ptr_c_star: &mut BidUint64,
  incr_exp: &mut i32,
  ptr_is_midpoint_lt_even: &mut bool,
  ptr_is_midpoint_gt_even: &mut bool,
  ptr_is_inexact_lt_midpoint: &mut bool,
  ptr_is_inexact_gt_midpoint: &mut bool,
) {
  let mut p128: BidUint128 = Default::default();
  let mut f_star: BidUint128 = Default::default();
  let mut c_star: BidUint64;
  let tmp64: BidUint64;
  let mut ind: i32;

  ind = x - 1; // 0 <= ind <= 16
  c = c.wrapping_add(bid_midpoint64![ind]);
  // kx ~= 10^(-x), kx = bid_Kx64[ind] * 2^(-Ex), 0 <= ind <= 16
  // P128 = (c + 1/2 * 10^x) * kx * 2^Ex = (c + 1/2 * 10^x) * Kx
  // the approximation kx of 10^(-x) was rounded up to 64 bits
  mul_64x64_to_128mach!(p128, c, bid_kx64![ind]);
  // calculate c* = floor (P128) and f*
  // Cstar = P128 >> Ex
  // fstar = low Ex bits of P128
  let shift = bid_ex64m64![ind] as i32; // in [3, 56]
  c_star = p128.w[1] >> shift;
  f_star.w[1] = p128.w[1] & bid_mask64![ind];
  f_star.w[0] = p128.w[0];
  // the top Ex bits of 10^(-x) are T* = bid_ten2mxtrunc64![ind], e.g.
  // if x=1, T*=bid_ten2mxtrunc64![0]=0xcccccccccccccccc
  // if (0 < f* < 10^(-x)) then the result is a midpoint
  //   if floor(c*) is even then c* = floor(c*) - logical right
  //       shift; c* has q - x decimal digits, correct by Prop. 1)
  //   else if floor(c*) is odd c* = floor(c*)-1 (logical right
  //       shift; c* has q - x decimal digits, correct by Pr. 1)
  // else
  //   c* = floor(c*) (logical right shift; c has q - x decimal digits,
  //       correct by Property 1)
  // in the caling function n = c* * 10^(e+x)

  // determine inexactness of the rounding of c*
  // if (0 < f* - 1/2 < 10^(-x)) then
  //   the result is exact
  // else // if (f* - 1/2 > T*) then
  //   the result is inexact
  if f_star.w[1] > bid_half64![ind] || (f_star.w[1] == bid_half64![ind] && f_star.w[0] > 0) {
    // f* > 1/2 and the result may be exact
    // Calculate f* - 1/2
    tmp64 = f_star.w[1] - bid_half64![ind];
    if tmp64 > 0 || f_star.w[0] > bid_ten2mxtrunc64![ind] {
      // f* - 1/2 > 10^(-x)
      *ptr_is_inexact_lt_midpoint = true;
    } // else the result is exact
  } else {
    // the result is inexact; f2* <= 1/2
    *ptr_is_inexact_gt_midpoint = true;
  }
  // check for midpoints (could do this before determining inexactness)
  if f_star.w[1] == 0 && f_star.w[0] <= bid_ten2mxtrunc64![ind] {
    // the result is a midpoint
    if c_star & 0x01 > 0 {
      // Cstar is odd; MP in [EVEN, ODD]
      // if floor(c*) is odd c = floor(c*) - 1; the result may be 0
      dec!(c_star); // Cstar is now even
      *ptr_is_midpoint_gt_even = true;
      *ptr_is_inexact_lt_midpoint = false;
      *ptr_is_inexact_gt_midpoint = false;
    } else {
      // else MP in [ODD, EVEN]
      *ptr_is_midpoint_lt_even = true;
      *ptr_is_inexact_lt_midpoint = false;
      *ptr_is_inexact_gt_midpoint = false;
    }
  }
  // check for rounding overflow, which occurs if Cstar = 10^(q-x)
  ind = q - x; // 1 <= ind <= q - 1
  if c_star == bid_ten2k64![ind] {
    // if  Cstar = 10^(q-x)
    c_star = bid_ten2k64![ind - 1]; // Cstar = 10^(q-x-1)
    *incr_exp = 1;
  } else {
    // 10^33 <= Cstar <= 10^34 - 1
    *incr_exp = 0;
  }
  *ptr_c_star = c_star;
}

/// Rounds positive numbers with 19 <= q <= 38 to nearest only for 1 <= x <= 23.
///
/// Note:
///    In bid_round128_19_38() positive numbers with 19 <= q <= 38 will be
///    rounded to nearest only for 1 <= x <= 23:
///     x = 3 or x = 4 when q = 19
///     x = 4 or x = 5 when q = 20
///     ...
///     x = 18 or x = 19 when q = 34
///     x = 1 or x = 2 or x = 19 or x = 20 when q = 35
///     x = 2 or x = 3 or x = 20 or x = 21 when q = 36
///     x = 3 or x = 4 or x = 21 or x = 22 when q = 37
///     x = 4 or x = 5 or x = 22 or x = 23 when q = 38
/// However, for generality and possible uses outside the frame of IEEE 754
/// this implementation works for 1 <= x <= q - 1
///
/// assume *ptr_is_midpoint_lt_even, *ptr_is_midpoint_gt_even,
/// *ptr_is_inexact_lt_midpoint, and *ptr_is_inexact_gt_midpoint are
/// initialized to 0 by the caller
///
/// round a number C with q decimal digits, 19 <= q <= 38
/// to q - x digits, 1 <= x <= 37
/// C = C + 1/2 * 10^x where the result C fits in 128 bits
/// (because the largest value is 99999999999999999999999999999999999999 +
/// 5000000000000000000000000000000000000 =
/// 0x4efe43b0c573e7e68a043d8fffffffff, which fits is 127 bits)
#[allow(clippy::too_many_arguments)]
pub fn bid_round128_19_38(
  q: i32,
  x: i32,
  mut c: BidUint128,
  ptr_c_star: &mut BidUint128,
  incr_exp: &mut i32,
  ptr_is_midpoint_lt_even: &mut bool,
  ptr_is_midpoint_gt_even: &mut bool,
  ptr_is_inexact_lt_midpoint: &mut bool,
  ptr_is_inexact_gt_midpoint: &mut bool,
) {
  let mut p256: BidUint256 = Default::default();
  let mut f_star: BidUint256 = Default::default();
  let mut c_star: BidUint128 = Default::default();
  let mut tmp64: BidUint64;
  let mut ind: i32;

  ind = x - 1; // 0 <= ind <= 36
  if ind <= 18 {
    // if 0 <= ind <= 18
    tmp64 = c.w[0];
    c.w[0] = c.w[0].wrapping_add(bid_midpoint64!(ind));
    if c.w[0] < tmp64 {
      inc!(c.w[1]);
    }
  } else {
    // if 19 <= ind <= 37
    tmp64 = c.w[0];
    c.w[0] = c.w[0].wrapping_add(bid_midpoint128!(ind - 19).w[0]);
    if c.w[0] < tmp64 {
      inc!(c.w[1]);
    }
    c.w[1] = c.w[1].wrapping_add(bid_midpoint128!(ind - 19).w[1]);
  }
  // kx ~= 10^(-x), kx = bid_kx128[ind] * 2^(-Ex), 0 <= ind <= 36
  // P256 = (C + 1/2 * 10^x) * kx * 2^Ex = (C + 1/2 * 10^x) * Kx
  // the approximation kx of 10^(-x) was rounded up to 128 bits
  mul_128x128_to_256!(p256, c, bid_kx128!(ind));
  // calculate C* = floor (P256) and f*
  // Cstar = P256 >> Ex
  // fstar = low Ex bits of P256
  let shift = bid_ex128m128!(ind) as i32; // in [2, 63] but have to consider two cases
  if ind <= 18 {
    // if 0 <= ind <= 18
    c_star.w[0] = (p256.w[2] >> shift) | (p256.w[3] << (64 - shift));
    c_star.w[1] = p256.w[3] >> shift;
    f_star.w[0] = p256.w[0];
    f_star.w[1] = p256.w[1];
    f_star.w[2] = p256.w[2] & bid_mask128!(ind);
    f_star.w[3] = 0;
  } else {
    // if 19 <= ind <= 37
    c_star.w[0] = p256.w[3] >> shift;
    c_star.w[1] = 0;
    f_star.w[0] = p256.w[0];
    f_star.w[1] = p256.w[1];
    f_star.w[2] = p256.w[2];
    f_star.w[3] = p256.w[3] & bid_mask128!(ind);
  }
  // the top Ex bits of 10^(-x) are T* = bid_ten2mxtrunc64![ind], e.g.
  // if x=1, T*=bid_ten2mxtrunc128[0]=0xcccccccccccccccccccccccccccccccc
  // if (0 < f* < 10^(-x)) then the result is a midpoint
  //   if floor(C*) is even then C* = floor(C*) - logical right
  //       shift; C* has q - x decimal digits, correct by Prop. 1)
  //   else if floor(C*) is odd C* = floor(C*)-1 (logical right
  //       shift; C* has q - x decimal digits, correct by Pr. 1)
  // else
  //   C* = floor(C*) (logical right shift; C has q - x decimal digits,
  //       correct by Property 1)
  // in the caling function n = C* * 10^(e+x)

  // determine inexactness of the rounding of C*
  // if (0 < f* - 1/2 < 10^(-x)) then
  //   the result is exact
  // else // if (f* - 1/2 > T*) then
  //   the result is inexact
  if ind <= 18 {
    // if 0 <= ind <= 18
    if f_star.w[2] > bid_half128!(ind) || (f_star.w[2] == bid_half128!(ind) && (f_star.w[1] > 0 || f_star.w[0] > 0)) {
      // f* > 1/2 and the result may be exact
      // Calculate f* - 1/2
      tmp64 = f_star.w[2] - bid_half128!(ind);
      if tmp64 > 0 || f_star.w[1] > bid_ten2mxtrunc128!(ind).w[1] || (f_star.w[1] == bid_ten2mxtrunc128!(ind).w[1] && f_star.w[0] > bid_ten2mxtrunc128!(ind).w[0]) {
        // f* - 1/2 > 10^(-x)
        *ptr_is_inexact_lt_midpoint = true;
      } // else the result is exact
    } else {
      // the result is inexact; f2* <= 1/2
      *ptr_is_inexact_gt_midpoint = true;
    }
  } else {
    // if 19 <= ind <= 37
    if f_star.w[3] > bid_half128!(ind) || (f_star.w[3] == bid_half128!(ind) && (f_star.w[2] > 0 || f_star.w[1] > 0 || f_star.w[0] > 0)) {
      // f* > 1/2 and the result may be exact
      // Calculate f* - 1/2
      tmp64 = f_star.w[3] - bid_half128!(ind);
      if tmp64 > 0 || f_star.w[2] > 0 || f_star.w[1] > bid_ten2mxtrunc128!(ind).w[1] || (f_star.w[1] == bid_ten2mxtrunc128!(ind).w[1] && f_star.w[0] > bid_ten2mxtrunc128!(ind).w[0]) {
        // f* - 1/2 > 10^(-x)
        *ptr_is_inexact_lt_midpoint = true;
      } // else the result is exact
    } else {
      // the result is inexact; f2* <= 1/2
      *ptr_is_inexact_gt_midpoint = true;
    }
  }
  // check for midpoints (could do this before determining inexactness)
  if f_star.w[3] == 0 && f_star.w[2] == 0 && (f_star.w[1] < bid_ten2mxtrunc128!(ind).w[1] || (f_star.w[1] == bid_ten2mxtrunc128!(ind).w[1] && f_star.w[0] <= bid_ten2mxtrunc128!(ind).w[0])) {
    // the result is a midpoint
    if c_star.w[0] & 0x01 > 0 {
      // Cstar is odd; MP in [EVEN, ODD]
      // if floor(C*) is odd C = floor(C*) - 1; the result may be 0
      dec!(c_star.w[0]); // Cstar is now even
      if c_star.w[0] == 0xffffffffffffffff {
        dec!(c_star.w[1]);
      }
      *ptr_is_midpoint_gt_even = true;
      *ptr_is_inexact_lt_midpoint = false;
      *ptr_is_inexact_gt_midpoint = false;
    } else {
      // else MP in [ODD, EVEN]
      *ptr_is_midpoint_lt_even = true;
      *ptr_is_inexact_lt_midpoint = false;
      *ptr_is_inexact_gt_midpoint = false;
    }
  }
  // check for rounding overflow, which occurs if Cstar = 10^(q-x)
  ind = q - x; // 1 <= ind <= q - 1
  if ind <= 19 {
    if c_star.w[1] == 0 && c_star.w[0] == bid_ten2k64!(ind) {
      // if  Cstar = 10^(q-x)
      c_star.w[0] = bid_ten2k64!(ind - 1); // Cstar = 10^(q-x-1)
      *incr_exp = 1;
    } else {
      *incr_exp = 0;
    }
  } else if ind == 20 {
    // if ind = 20
    if c_star.w[1] == bid_ten2k128!(0).w[1] && c_star.w[0] == bid_ten2k128!(0).w[0] {
      // if  Cstar = 10^(q-x)
      c_star.w[0] = bid_ten2k64!(19); // Cstar = 10^(q-x-1)
      c_star.w[1] = 0;
      *incr_exp = 1;
    } else {
      *incr_exp = 0;
    }
  } else {
    // if 21 <= ind <= 37
    if c_star.w[1] == bid_ten2k128!(ind - 20).w[1] && c_star.w[0] == bid_ten2k128!(ind - 20).w[0] {
      // if  Cstar = 10^(q-x)
      c_star.w[0] = bid_ten2k128!(ind - 21).w[0]; // Cstar = 10^(q-x-1)
      c_star.w[1] = bid_ten2k128!(ind - 21).w[1];
      *incr_exp = 1;
    } else {
      *incr_exp = 0;
    }
  }
  ptr_c_star.w[1] = c_star.w[1];
  ptr_c_star.w[0] = c_star.w[0];
}

/// Rounds positive numbers with 39 <= q <= 57 to nearest only for 5 <= x <= 42.
///
/// Note:
///    In bid_round192_39_57() positive numbers with 39 <= q <= 57 will be
///    rounded to nearest only for 5 <= x <= 42:
///     x = 23 or x = 24 or x = 5 or x = 6 when q = 39
///     x = 24 or x = 25 or x = 6 or x = 7 when q = 40
///     ...
///     x = 41 or x = 42 or x = 23 or x = 24 when q = 57
///
/// However, for generality and possible uses outside the frame of IEEE 754
/// this implementation works for 1 <= x <= q - 1
///
/// assume *ptr_is_midpoint_lt_even, *ptr_is_midpoint_gt_even,
/// *ptr_is_inexact_lt_midpoint, and *ptr_is_inexact_gt_midpoint are
/// initialized to 0 by the caller
///
/// round a number c with q decimal digits, 39 <= q <= 57
/// to q - x digits, 1 <= x <= 56
/// c = c + 1/2 * 10^x where the result c fits in 192 bits
/// (because the largest value is
/// 999999999999999999999999999999999999999999999999999999999 +
///  50000000000000000000000000000000000000000000000000000000 =
/// 0x2ad282f212a1da846afdaf18c034ff09da7fffffffffffff, which fits in 190 bits)
#[allow(clippy::too_many_arguments)]
pub fn bid_round192_39_57(
  q: i32,
  x: i32,
  mut c: BidUint192,
  ptr_c_star: &mut BidUint192,
  incr_exp: &mut i32,
  ptr_is_midpoint_lt_even: &mut bool,
  ptr_is_midpoint_gt_even: &mut bool,
  ptr_is_inexact_lt_midpoint: &mut bool,
  ptr_is_inexact_gt_midpoint: &mut bool,
) {
  let mut p384: BidUint384 = Default::default();
  let mut f_star: BidUint384 = Default::default();
  let mut c_star: BidUint192 = Default::default();
  let mut tmp64: BidUint64;
  let mut ind: i32;

  ind = x - 1; // 0 <= ind <= 55
  if ind <= 18 {
    // if 0 <= ind <= 18
    tmp64 = c.w[0];
    c.w[0] = c.w[0].wrapping_add(bid_midpoint64!(ind));
    if c.w[0] < tmp64 {
      inc!(c.w[1]);
      if c.w[1] == 0x0 {
        inc!(c.w[2]);
      }
    }
  } else if ind <= 37 {
    // if 19 <= ind <= 37
    tmp64 = c.w[0];
    c.w[0] = c.w[0].wrapping_add(bid_midpoint128!(ind - 19).w[0]);
    if c.w[0] < tmp64 {
      inc!(c.w[1]);
      if c.w[1] == 0x0 {
        inc!(c.w[2]);
      }
    }
    tmp64 = c.w[1];
    c.w[1] = c.w[1].wrapping_add(bid_midpoint128!(ind - 19).w[1]);
    if c.w[1] < tmp64 {
      inc!(c.w[2]);
    }
  } else {
    // if 38 <= ind <= 57 (actually ind <= 55)
    tmp64 = c.w[0];
    c.w[0] = c.w[0].wrapping_add(bid_midpoint192!(ind - 38).w[0]);
    if c.w[0] < tmp64 {
      inc!(c.w[1]);
      if c.w[1] == 0 {
        inc!(c.w[2]);
      }
    }
    tmp64 = c.w[1];
    c.w[1] = c.w[1].wrapping_add(bid_midpoint192!(ind - 38).w[1]);
    if c.w[1] < tmp64 {
      inc!(c.w[2]);
    }
    c.w[2] = c.w[2].wrapping_add(bid_midpoint192!(ind - 38).w[2]);
  }
  // kx ~= 10^(-x), kx = bid_kx192[ind] * 2^(-Ex), 0 <= ind <= 55
  // p384 = (c + 1/2 * 10^x) * kx * 2^Ex = (c + 1/2 * 10^x) * Kx
  // the approximation kx of 10^(-x) was rounded up to 192 bits
  mul_192x192_to_384!(p384, c, bid_kx192!(ind));
  // calculate c* = floor (p384) and f*
  // Cstar = p384 >> Ex
  // fstar = low Ex bits of p384
  let shift = bid_ex192m192!(ind) as i32; // in [1, 63] but have to consider three cases
  if ind <= 18 {
    // if 0 <= ind <= 18
    c_star.w[2] = p384.w[5] >> shift;
    c_star.w[1] = (p384.w[5] << (64 - shift)) | (p384.w[4] >> shift);
    c_star.w[0] = (p384.w[4] << (64 - shift)) | (p384.w[3] >> shift);
    f_star.w[5] = 0;
    f_star.w[4] = 0;
    f_star.w[3] = p384.w[3] & bid_mask192!(ind);
    f_star.w[2] = p384.w[2];
    f_star.w[1] = p384.w[1];
    f_star.w[0] = p384.w[0];
  } else if ind <= 37 {
    // if 19 <= ind <= 37
    c_star.w[2] = 0;
    c_star.w[1] = p384.w[5] >> shift;
    c_star.w[0] = (p384.w[5] << (64 - shift)) | (p384.w[4] >> shift);
    f_star.w[5] = 0;
    f_star.w[4] = p384.w[4] & bid_mask192!(ind);
    f_star.w[3] = p384.w[3];
    f_star.w[2] = p384.w[2];
    f_star.w[1] = p384.w[1];
    f_star.w[0] = p384.w[0];
  } else {
    // if 38 <= ind <= 57
    c_star.w[2] = 0;
    c_star.w[1] = 0;
    c_star.w[0] = p384.w[5] >> shift;
    f_star.w[5] = p384.w[5] & bid_mask192!(ind);
    f_star.w[4] = p384.w[4];
    f_star.w[3] = p384.w[3];
    f_star.w[2] = p384.w[2];
    f_star.w[1] = p384.w[1];
    f_star.w[0] = p384.w[0];
  }

  // the top Ex bits of 10^(-x) are T* = bid_ten2mxtrunc192!(ind], e.g. if x=1,
  // T*=bid_ten2mxtrunc192[0]=0xcccccccccccccccccccccccccccccccccccccccccccccccc
  // if (0 < f* < 10^(-x)) then the result is a midpoint
  //   if floor(c*) is even then c* = floor(c*) - logical right
  //       shift; c* has q - x decimal digits, correct by Prop. 1)
  //   else if floor(c*) is odd c* = floor(c*)-1 (logical right
  //       shift; c* has q - x decimal digits, correct by Pr. 1)
  // else
  //   c* = floor(c*) (logical right shift; c has q - x decimal digits,
  //       correct by Property 1)
  // in the caling function n = c* * 10^(e+x)

  // determine inexactness of the rounding of c*
  // if (0 < f* - 1/2 < 10^(-x)) then
  //   the result is exact
  // else // if (f* - 1/2 > T*) then
  //   the result is inexact
  if ind <= 18 {
    // if 0 <= ind <= 18
    if f_star.w[3] > bid_half192!(ind) || (f_star.w[3] == bid_half192!(ind) && (f_star.w[2] > 0 || f_star.w[1] > 0 || f_star.w[0] > 0)) {
      // f* > 1/2 and the result may be exact
      // Calculate f* - 1/2
      tmp64 = f_star.w[3] - bid_half192!(ind);
      if tmp64 > 0
        || f_star.w[2] > bid_ten2mxtrunc192!(ind).w[2]
        || (f_star.w[2] == bid_ten2mxtrunc192!(ind).w[2] && f_star.w[1] > bid_ten2mxtrunc192!(ind).w[1])
        || (f_star.w[2] == bid_ten2mxtrunc192!(ind).w[2] && f_star.w[1] == bid_ten2mxtrunc192!(ind).w[1] && f_star.w[0] > bid_ten2mxtrunc192!(ind).w[0])
      {
        // f* - 1/2 > 10^(-x)
        *ptr_is_inexact_lt_midpoint = true;
      } // else the result is exact
    } else {
      // the result is inexact; f2* <= 1/2
      *ptr_is_inexact_gt_midpoint = true;
    }
  } else if ind <= 37 {
    // if 19 <= ind <= 37
    if f_star.w[4] > bid_half192!(ind) || (f_star.w[4] == bid_half192!(ind) && (f_star.w[3] > 0 || f_star.w[2] > 0 || f_star.w[1] > 0 || f_star.w[0] > 0)) {
      // f* > 1/2 and the result may be exact
      // Calculate f* - 1/2
      tmp64 = f_star.w[4] - bid_half192!(ind);
      if tmp64 > 0
        || f_star.w[3] > 0
        || f_star.w[2] > bid_ten2mxtrunc192!(ind).w[2]
        || (f_star.w[2] == bid_ten2mxtrunc192!(ind).w[2] && f_star.w[1] > bid_ten2mxtrunc192!(ind).w[1])
        || (f_star.w[2] == bid_ten2mxtrunc192!(ind).w[2] && f_star.w[1] == bid_ten2mxtrunc192!(ind).w[1] && f_star.w[0] > bid_ten2mxtrunc192!(ind).w[0])
      {
        // f* - 1/2 > 10^(-x)
        *ptr_is_inexact_lt_midpoint = true;
      } // else the result is exact
    } else {
      // the result is inexact; f2* <= 1/2
      *ptr_is_inexact_gt_midpoint = true;
    }
  } else {
    // if 38 <= ind <= 55
    if f_star.w[5] > bid_half192!(ind) || (f_star.w[5] == bid_half192!(ind) && (f_star.w[4] > 0 || f_star.w[3] > 0 || f_star.w[2] > 0 || f_star.w[1] > 0 || f_star.w[0] > 0)) {
      // f* > 1/2 and the result may be exact
      // Calculate f* - 1/2
      tmp64 = f_star.w[5] - bid_half192!(ind);
      if tmp64 > 0
        || f_star.w[4] > 0
        || f_star.w[3] > 0
        || f_star.w[2] > bid_ten2mxtrunc192!(ind).w[2]
        || (f_star.w[2] == bid_ten2mxtrunc192!(ind).w[2] && f_star.w[1] > bid_ten2mxtrunc192!(ind).w[1])
        || (f_star.w[2] == bid_ten2mxtrunc192!(ind).w[2] && f_star.w[1] == bid_ten2mxtrunc192!(ind).w[1] && f_star.w[0] > bid_ten2mxtrunc192!(ind).w[0])
      {
        // f* - 1/2 > 10^(-x)
        *ptr_is_inexact_lt_midpoint = true;
      } // else the result is exact
    } else {
      // the result is inexact; f2* <= 1/2
      *ptr_is_inexact_gt_midpoint = true;
    }
  }
  // check for midpoints (could do this before determining inexactness)
  if f_star.w[5] == 0
    && f_star.w[4] == 0
    && f_star.w[3] == 0
    && (f_star.w[2] < bid_ten2mxtrunc192!(ind).w[2]
      || (f_star.w[2] == bid_ten2mxtrunc192!(ind).w[2] && f_star.w[1] < bid_ten2mxtrunc192!(ind).w[1])
      || (f_star.w[2] == bid_ten2mxtrunc192!(ind).w[2] && f_star.w[1] == bid_ten2mxtrunc192!(ind).w[1] && f_star.w[0] <= bid_ten2mxtrunc192!(ind).w[0]))
  {
    // the result is a midpoint
    if c_star.w[0] & 0x01 > 0 {
      // Cstar is odd; MP in [EVEN, ODD]
      // if floor(c*) is odd c = floor(c*) - 1; the result may be 0
      dec!(c_star.w[0]); // Cstar is now even
      if c_star.w[0] == 0xffffffffffffffff {
        dec!(c_star.w[1]);
        if c_star.w[1] == 0xffffffffffffffff {
          dec!(c_star.w[2]);
        }
      }
      *ptr_is_midpoint_gt_even = true;
      *ptr_is_inexact_lt_midpoint = false;
      *ptr_is_inexact_gt_midpoint = false;
    } else {
      // else MP in [ODD, EVEN]
      *ptr_is_midpoint_lt_even = true;
      *ptr_is_inexact_lt_midpoint = false;
      *ptr_is_inexact_gt_midpoint = false;
    }
  }
  // check for rounding overflow, which occurs if Cstar = 10^(q-x)
  ind = q - x; // 1 <= ind <= q - 1
  if ind <= 19 {
    if c_star.w[2] == 0 && c_star.w[1] == 0 && c_star.w[0] == bid_ten2k64!(ind) {
      // if  Cstar = 10^(q-x)
      c_star.w[0] = bid_ten2k64!(ind - 1); // Cstar = 10^(q-x-1)
      *incr_exp = 1;
    } else {
      *incr_exp = 0;
    }
  } else if ind == 20 {
    // if ind = 20
    if c_star.w[2] == 0 && c_star.w[1] == bid_ten2k128!(0).w[1] && c_star.w[0] == bid_ten2k128!(0).w[0] {
      // if  Cstar = 10^(q-x)
      c_star.w[0] = bid_ten2k64!(19); // Cstar = 10^(q-x-1)
      c_star.w[1] = 0;
      *incr_exp = 1;
    } else {
      *incr_exp = 0;
    }
  } else if ind <= 38 {
    // if 21 <= ind <= 38
    if c_star.w[2] == 0 && c_star.w[1] == bid_ten2k128!(ind - 20).w[1] && c_star.w[0] == bid_ten2k128!(ind - 20).w[0] {
      // if  Cstar = 10^(q-x)
      c_star.w[0] = bid_ten2k128!(ind - 21).w[0]; // Cstar = 10^(q-x-1)
      c_star.w[1] = bid_ten2k128!(ind - 21).w[1];
      *incr_exp = 1;
    } else {
      *incr_exp = 0;
    }
  } else if ind == 39 {
    if c_star.w[2] == bid_ten2k256!(0).w[2] && c_star.w[1] == bid_ten2k256!(0).w[1] && c_star.w[0] == bid_ten2k256!(0).w[0] {
      // if  Cstar = 10^(q-x)
      c_star.w[0] = bid_ten2k128!(18).w[0]; // Cstar = 10^(q-x-1)
      c_star.w[1] = bid_ten2k128!(18).w[1];
      c_star.w[2] = 0;
      *incr_exp = 1;
    } else {
      *incr_exp = 0;
    }
  } else {
    // if 40 <= ind <= 56
    if c_star.w[2] == bid_ten2k256!(ind - 39).w[2] && c_star.w[1] == bid_ten2k256!(ind - 39).w[1] && c_star.w[0] == bid_ten2k256!(ind - 39).w[0] {
      // if  Cstar = 10^(q-x)
      c_star.w[0] = bid_ten2k256!(ind - 40).w[0]; // Cstar = 10^(q-x-1)
      c_star.w[1] = bid_ten2k256!(ind - 40).w[1];
      c_star.w[2] = bid_ten2k256!(ind - 40).w[2];
      *incr_exp = 1;
    } else {
      *incr_exp = 0;
    }
  }
  ptr_c_star.w[2] = c_star.w[2];
  ptr_c_star.w[1] = c_star.w[1];
  ptr_c_star.w[0] = c_star.w[0];
}

#[allow(clippy::too_many_arguments)]
pub fn bid_round256_58_76(
  q: i32,
  x: i32,
  mut c: BidUint256,
  ptr_c_star: &mut BidUint256,
  incr_exp: &mut i32,
  ptr_is_midpoint_lt_even: &mut bool,
  ptr_is_midpoint_gt_even: &mut bool,
  ptr_is_inexact_lt_midpoint: &mut bool,
  ptr_is_inexact_gt_midpoint: &mut bool,
) {
  let mut p512: BidUint512 = Default::default();
  let mut f_star: BidUint512 = Default::default();
  let mut c_star: BidUint256 = Default::default();
  let mut tmp64: BidUint64;
  let mut ind: i32;

  // Note:
  //    In bid_round256_58_76() positive numbers with 58 <= q <= 76 will be
  //    rounded to nearest only for 24 <= x <= 61:
  //     x = 42 or x = 43 or x = 24 or x = 25 when q = 58
  //     x = 43 or x = 44 or x = 25 or x = 26 when q = 59
  //     ...
  //     x = 60 or x = 61 or x = 42 or x = 43 when q = 76
  // However, for generality and possible uses outside the frame of IEEE 754
  // this implementation works for 1 <= x <= q - 1

  // assume *ptr_is_midpoint_lt_even, *ptr_is_midpoint_gt_even,
  // *ptr_is_inexact_lt_midpoint, and *ptr_is_inexact_gt_midpoint are
  // initialized to 0 by the caller

  // round a number c with q decimal digits, 58 <= q <= 76
  // to q - x digits, 1 <= x <= 75
  // c = c + 1/2 * 10^x where the result c fits in 256 bits
  // (because the largest value is 9999999999999999999999999999999999999999
  //     999999999999999999999999999999999999 + 500000000000000000000000000
  //     000000000000000000000000000000000000000000000000 =
  //     0x1736ca15d27a56cae15cf0e7b403d1f2bd6ebb0a50dc83ffffffffffffffffff,
  // which fits in 253 bits)
  ind = x - 1; // 0 <= ind <= 74
  if ind <= 18 {
    // if 0 <= ind <= 18
    tmp64 = c.w[0];
    c.w[0] = c.w[0].wrapping_add(bid_midpoint64!(ind));
    if c.w[0] < tmp64 {
      inc!(c.w[1]);
      if c.w[1] == 0 {
        inc!(c.w[2]);
        if c.w[2] == 0 {
          inc!(c.w[3]);
        }
      }
    }
  } else if ind <= 37 {
    // if 19 <= ind <= 37
    tmp64 = c.w[0];
    c.w[0] = c.w[0].wrapping_add(bid_midpoint128!(ind - 19).w[0]);
    if c.w[0] < tmp64 {
      inc!(c.w[1]);
      if c.w[1] == 0 {
        inc!(c.w[2]);
        if c.w[2] == 0 {
          inc!(c.w[3]);
        }
      }
    }
    tmp64 = c.w[1];
    c.w[1] = c.w[1].wrapping_add(bid_midpoint128!(ind - 19).w[1]);
    if c.w[1] < tmp64 {
      inc!(c.w[2]);
      if c.w[2] == 0 {
        inc!(c.w[3]);
      }
    }
  } else if ind <= 57 {
    // if 38 <= ind <= 57
    tmp64 = c.w[0];
    c.w[0] = c.w[0].wrapping_add(bid_midpoint192!(ind - 38).w[0]);
    if c.w[0] < tmp64 {
      inc!(c.w[1]);
      if c.w[1] == 0 {
        inc!(c.w[2]);
        if c.w[2] == 0 {
          inc!(c.w[3]);
        }
      }
    }
    tmp64 = c.w[1];
    c.w[1] = c.w[1].wrapping_add(bid_midpoint192!(ind - 38).w[1]);
    if c.w[1] < tmp64 {
      inc!(c.w[2]);
      if c.w[2] == 0 {
        inc!(c.w[3]);
      }
    }
    tmp64 = c.w[2];
    c.w[2] = c.w[2].wrapping_add(bid_midpoint192!(ind - 38).w[2]);
    if c.w[2] < tmp64 {
      inc!(c.w[3]);
    }
  } else {
    // if 58 <= ind <= 76 (actually 58 <= ind <= 74)
    tmp64 = c.w[0];
    c.w[0] = c.w[0].wrapping_add(bid_midpoint256!(ind - 58).w[0]);
    if c.w[0] < tmp64 {
      inc!(c.w[1]);
      if c.w[1] == 0 {
        inc!(c.w[2]);
        if c.w[2] == 0 {
          inc!(c.w[3]);
        }
      }
    }
    tmp64 = c.w[1];
    c.w[1] = c.w[1].wrapping_add(bid_midpoint256!(ind - 58).w[1]);
    if c.w[1] < tmp64 {
      inc!(c.w[2]);
      if c.w[2] == 0x0 {
        inc!(c.w[3]);
      }
    }
    tmp64 = c.w[2];
    c.w[2] = c.w[2].wrapping_add(bid_midpoint256!(ind - 58).w[2]);
    if c.w[2] < tmp64 {
      inc!(c.w[3]);
    }
    c.w[3] = c.w[3].wrapping_add(bid_midpoint256!(ind - 58).w[3]);
  }
  // kx ~= 10^(-x), kx = bid_kx256[ind] * 2^(-Ex), 0 <= ind <= 74
  // P512 = (c + 1/2 * 10^x) * kx * 2^Ex = (c + 1/2 * 10^x) * Kx
  // the approximation kx of 10^(-x) was rounded up to 192 bits
  mul_256x256_to_512!(p512, c, bid_kx256!(ind));
  // calculate c* = floor (P512) and f*
  // Cstar = P512 >> Ex
  // fstar = low Ex bits of P512
  let shift = bid_ex256m256!(ind) as i32; // in [0, 63] but have to consider four cases
  if ind <= 18 {
    // if 0 <= ind <= 18
    c_star.w[3] = p512.w[7] >> shift;
    c_star.w[2] = (p512.w[7] << (64 - shift)) | (p512.w[6] >> shift);
    c_star.w[1] = (p512.w[6] << (64 - shift)) | (p512.w[5] >> shift);
    c_star.w[0] = (p512.w[5] << (64 - shift)) | (p512.w[4] >> shift);
    f_star.w[7] = 0;
    f_star.w[6] = 0;
    f_star.w[5] = 0;
    f_star.w[4] = p512.w[4] & bid_mask256!(ind);
    f_star.w[3] = p512.w[3];
    f_star.w[2] = p512.w[2];
    f_star.w[1] = p512.w[1];
    f_star.w[0] = p512.w[0];
  } else if ind <= 37 {
    // if 19 <= ind <= 37
    c_star.w[3] = 0;
    c_star.w[2] = p512.w[7] >> shift;
    c_star.w[1] = (p512.w[7] << (64 - shift)) | (p512.w[6] >> shift);
    c_star.w[0] = (p512.w[6] << (64 - shift)) | (p512.w[5] >> shift);
    f_star.w[7] = 0;
    f_star.w[6] = 0;
    f_star.w[5] = p512.w[5] & bid_mask256!(ind);
    f_star.w[4] = p512.w[4];
    f_star.w[3] = p512.w[3];
    f_star.w[2] = p512.w[2];
    f_star.w[1] = p512.w[1];
    f_star.w[0] = p512.w[0];
  } else if ind <= 56 {
    // if 38 <= ind <= 56
    c_star.w[3] = 0;
    c_star.w[2] = 0;
    c_star.w[1] = p512.w[7] >> shift;
    c_star.w[0] = (p512.w[7] << (64 - shift)) | (p512.w[6] >> shift);
    f_star.w[7] = 0;
    f_star.w[6] = p512.w[6] & bid_mask256!(ind);
    f_star.w[5] = p512.w[5];
    f_star.w[4] = p512.w[4];
    f_star.w[3] = p512.w[3];
    f_star.w[2] = p512.w[2];
    f_star.w[1] = p512.w[1];
    f_star.w[0] = p512.w[0];
  } else if ind == 57 {
    c_star.w[3] = 0;
    c_star.w[2] = 0;
    c_star.w[1] = 0;
    c_star.w[0] = p512.w[7];
    f_star.w[7] = 0;
    f_star.w[6] = p512.w[6];
    f_star.w[5] = p512.w[5];
    f_star.w[4] = p512.w[4];
    f_star.w[3] = p512.w[3];
    f_star.w[2] = p512.w[2];
    f_star.w[1] = p512.w[1];
    f_star.w[0] = p512.w[0];
  } else {
    // if 58 <= ind <= 74
    c_star.w[3] = 0;
    c_star.w[2] = 0;
    c_star.w[1] = 0;
    c_star.w[0] = p512.w[7] >> shift;
    f_star.w[7] = p512.w[7] & bid_mask256!(ind);
    f_star.w[6] = p512.w[6];
    f_star.w[5] = p512.w[5];
    f_star.w[4] = p512.w[4];
    f_star.w[3] = p512.w[3];
    f_star.w[2] = p512.w[2];
    f_star.w[1] = p512.w[1];
    f_star.w[0] = p512.w[0];
  }

  // the top Ex bits of 10^(-x) are T* = bid_ten2mxtrunc256!(ind], e.g. if x=1,
  // T*=bid_ten2mxtrunc256[0]=
  //     0xcccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc
  // if (0 < f* < 10^(-x)) then the result is a midpoint
  //   if floor(c*) is even then c* = floor(c*) - logical right
  //       shift; c* has q - x decimal digits, correct by Prop. 1)
  //   else if floor(c*) is odd c* = floor(c*)-1 (logical right
  //       shift; c* has q - x decimal digits, correct by Pr. 1)
  // else
  //   c* = floor(c*) (logical right shift; c has q - x decimal digits,
  //       correct by Property 1)
  // in the caling function n = c* * 10^(e+x)

  // determine inexactness of the rounding of c*
  // if (0 < f* - 1/2 < 10^(-x)) then
  //   the result is exact
  // else // if (f* - 1/2 > T*) then
  //   the result is inexact
  if ind <= 18 {
    // if 0 <= ind <= 18
    if f_star.w[4] > bid_half256!(ind) || (f_star.w[4] == bid_half256!(ind) && (f_star.w[3] > 0 || f_star.w[2] > 0 || f_star.w[1] > 0 || f_star.w[0] > 0)) {
      // f* > 1/2 and the result may be exact
      // Calculate f* - 1/2
      tmp64 = f_star.w[4] - bid_half256!(ind);
      if tmp64 > 0
        || f_star.w[3] > bid_ten2mxtrunc256!(ind).w[2]
        || (f_star.w[3] == bid_ten2mxtrunc256!(ind).w[3] && f_star.w[2] > bid_ten2mxtrunc256!(ind).w[2])
        || (f_star.w[3] == bid_ten2mxtrunc256!(ind).w[3] && f_star.w[2] == bid_ten2mxtrunc256!(ind).w[2] && f_star.w[1] > bid_ten2mxtrunc256!(ind).w[1])
        || (f_star.w[3] == bid_ten2mxtrunc256!(ind).w[3] && f_star.w[2] == bid_ten2mxtrunc256!(ind).w[2] && f_star.w[1] == bid_ten2mxtrunc256!(ind).w[1] && f_star.w[0] > bid_ten2mxtrunc256!(ind).w[0])
      {
        // f* - 1/2 > 10^(-x)
        *ptr_is_inexact_lt_midpoint = true;
      } // else the result is exact
    } else {
      // the result is inexact; f2* <= 1/2
      *ptr_is_inexact_gt_midpoint = true;
    }
  } else if ind <= 37 {
    // if 19 <= ind <= 37
    if f_star.w[5] > bid_half256!(ind) || (f_star.w[5] == bid_half256!(ind) && (f_star.w[4] > 0 || f_star.w[3] > 0 || f_star.w[2] > 0 || f_star.w[1] > 0 || f_star.w[0] > 0)) {
      // f* > 1/2 and the result may be exact
      // Calculate f* - 1/2
      tmp64 = f_star.w[5] - bid_half256!(ind);
      if tmp64 > 0
        || f_star.w[4] > 0
        || f_star.w[3] > bid_ten2mxtrunc256!(ind).w[3]
        || (f_star.w[3] == bid_ten2mxtrunc256!(ind).w[3] && f_star.w[2] > bid_ten2mxtrunc256!(ind).w[2])
        || (f_star.w[3] == bid_ten2mxtrunc256!(ind).w[3] && f_star.w[2] == bid_ten2mxtrunc256!(ind).w[2] && f_star.w[1] > bid_ten2mxtrunc256!(ind).w[1])
        || (f_star.w[3] == bid_ten2mxtrunc256!(ind).w[3] && f_star.w[2] == bid_ten2mxtrunc256!(ind).w[2] && f_star.w[1] == bid_ten2mxtrunc256!(ind).w[1] && f_star.w[0] > bid_ten2mxtrunc256!(ind).w[0])
      {
        // f* - 1/2 > 10^(-x)
        *ptr_is_inexact_lt_midpoint = true;
      } // else the result is exact
    } else {
      // the result is inexact; f2* <= 1/2
      *ptr_is_inexact_gt_midpoint = true;
    }
  } else if ind <= 57 {
    // if 38 <= ind <= 57
    if f_star.w[6] > bid_half256!(ind) || (f_star.w[6] == bid_half256!(ind) && (f_star.w[5] > 0 || f_star.w[4] > 0 || f_star.w[3] > 0 || f_star.w[2] > 0 || f_star.w[1] > 0 || f_star.w[0] > 0)) {
      // f* > 1/2 and the result may be exact
      // Calculate f* - 1/2
      tmp64 = f_star.w[6] - bid_half256!(ind);
      if tmp64 > 0
        || f_star.w[5] > 0
        || f_star.w[4] > 0
        || f_star.w[3] > bid_ten2mxtrunc256!(ind).w[3]
        || (f_star.w[3] == bid_ten2mxtrunc256!(ind).w[3] && f_star.w[2] > bid_ten2mxtrunc256!(ind).w[2])
        || (f_star.w[3] == bid_ten2mxtrunc256!(ind).w[3] && f_star.w[2] == bid_ten2mxtrunc256!(ind).w[2] && f_star.w[1] > bid_ten2mxtrunc256!(ind).w[1])
        || (f_star.w[3] == bid_ten2mxtrunc256!(ind).w[3] && f_star.w[2] == bid_ten2mxtrunc256!(ind).w[2] && f_star.w[1] == bid_ten2mxtrunc256!(ind).w[1] && f_star.w[0] > bid_ten2mxtrunc256!(ind).w[0])
      {
        // f* - 1/2 > 10^(-x)
        *ptr_is_inexact_lt_midpoint = true;
      } // else the result is exact
    } else {
      // the result is inexact; f2* <= 1/2
      *ptr_is_inexact_gt_midpoint = true;
    }
  } else {
    // if 58 <= ind <= 74
    if f_star.w[7] > bid_half256!(ind) || (f_star.w[7] == bid_half256!(ind) && (f_star.w[6] > 0 || f_star.w[5] > 0 || f_star.w[4] > 0 || f_star.w[3] > 0 || f_star.w[2] > 0 || f_star.w[1] > 0 || f_star.w[0] > 0)) {
      // f* > 1/2 and the result may be exact
      // Calculate f* - 1/2
      tmp64 = f_star.w[7] - bid_half256!(ind);
      if tmp64 > 0
        || f_star.w[6] > 0
        || f_star.w[5] > 0
        || f_star.w[4] > 0
        || f_star.w[3] > bid_ten2mxtrunc256!(ind).w[3]
        || (f_star.w[3] == bid_ten2mxtrunc256!(ind).w[3] && f_star.w[2] > bid_ten2mxtrunc256!(ind).w[2])
        || (f_star.w[3] == bid_ten2mxtrunc256!(ind).w[3] && f_star.w[2] == bid_ten2mxtrunc256!(ind).w[2] && f_star.w[1] > bid_ten2mxtrunc256!(ind).w[1])
        || (f_star.w[3] == bid_ten2mxtrunc256!(ind).w[3] && f_star.w[2] == bid_ten2mxtrunc256!(ind).w[2] && f_star.w[1] == bid_ten2mxtrunc256!(ind).w[1] && f_star.w[0] > bid_ten2mxtrunc256!(ind).w[0])
      {
        // f* - 1/2 > 10^(-x)
        *ptr_is_inexact_lt_midpoint = true;
      } // else the result is exact
    } else {
      // the result is inexact; f2* <= 1/2
      *ptr_is_inexact_gt_midpoint = true;
    }
  }
  // check for midpoints (could do this before determining inexactness)
  if f_star.w[7] == 0
    && f_star.w[6] == 0
    && f_star.w[5] == 0
    && f_star.w[4] == 0
    && (f_star.w[3] < bid_ten2mxtrunc256!(ind).w[3]
      || (f_star.w[3] == bid_ten2mxtrunc256!(ind).w[3] && f_star.w[2] < bid_ten2mxtrunc256!(ind).w[2])
      || (f_star.w[3] == bid_ten2mxtrunc256!(ind).w[3] && f_star.w[2] == bid_ten2mxtrunc256!(ind).w[2] && f_star.w[1] < bid_ten2mxtrunc256!(ind).w[1])
      || (f_star.w[3] == bid_ten2mxtrunc256!(ind).w[3] && f_star.w[2] == bid_ten2mxtrunc256!(ind).w[2] && f_star.w[1] == bid_ten2mxtrunc256!(ind).w[1] && f_star.w[0] <= bid_ten2mxtrunc256!(ind).w[0]))
  {
    // the result is a midpoint
    if c_star.w[0] & 0x01 > 0 {
      // Cstar is odd; MP in [EVEN, ODD]
      // if floor(c*) is odd c = floor(c*) - 1; the result may be 0
      dec!(c_star.w[0]); // Cstar is now even
      if c_star.w[0] == 0xffffffffffffffff {
        dec!(c_star.w[1]);
        if c_star.w[1] == 0xffffffffffffffff {
          dec!(c_star.w[2]);
          if c_star.w[2] == 0xffffffffffffffff {
            dec!(c_star.w[3]);
          }
        }
      }
      *ptr_is_midpoint_gt_even = true;
      *ptr_is_inexact_lt_midpoint = false;
      *ptr_is_inexact_gt_midpoint = false;
    } else {
      // else MP in [ODD, EVEN]
      *ptr_is_midpoint_lt_even = true;
      *ptr_is_inexact_lt_midpoint = false;
      *ptr_is_inexact_gt_midpoint = false;
    }
  }
  // check for rounding overflow, which occurs if Cstar = 10^(q-x)
  ind = q - x; // 1 <= ind <= q - 1
  if ind <= 19 {
    if c_star.w[3] == 0 && c_star.w[2] == 0 && c_star.w[1] == 0 && c_star.w[0] == bid_ten2k64!(ind) {
      // if  Cstar = 10^(q-x)
      c_star.w[0] = bid_ten2k64!(ind - 1); // Cstar = 10^(q-x-1)
      *incr_exp = 1;
    } else {
      *incr_exp = 0;
    }
  } else if ind == 20 {
    // if ind = 20
    if c_star.w[3] == 0 && c_star.w[2] == 0 && c_star.w[1] == bid_ten2k128!(0).w[1] && c_star.w[0] == bid_ten2k128!(0).w[0] {
      // if  Cstar = 10^(q-x)
      c_star.w[0] = bid_ten2k64!(19); // Cstar = 10^(q-x-1)
      c_star.w[1] = 0;
      *incr_exp = 1;
    } else {
      *incr_exp = 0;
    }
  } else if ind <= 38 {
    // if 21 <= ind <= 38
    if c_star.w[3] == 0 && c_star.w[2] == 0 && c_star.w[1] == bid_ten2k128!(ind - 20).w[1] && c_star.w[0] == bid_ten2k128!(ind - 20).w[0] {
      // if  Cstar = 10^(q-x)
      c_star.w[0] = bid_ten2k128!(ind - 21).w[0]; // Cstar = 10^(q-x-1)
      c_star.w[1] = bid_ten2k128!(ind - 21).w[1];
      *incr_exp = 1;
    } else {
      *incr_exp = 0;
    }
  } else if ind == 39 {
    if c_star.w[3] == 0 && c_star.w[2] == bid_ten2k256!(0).w[2] && c_star.w[1] == bid_ten2k256!(0).w[1] && c_star.w[0] == bid_ten2k256!(0).w[0] {
      // if  Cstar = 10^(q-x)
      c_star.w[0] = bid_ten2k128!(18).w[0]; // Cstar = 10^(q-x-1)
      c_star.w[1] = bid_ten2k128!(18).w[1];
      c_star.w[2] = 0;
      *incr_exp = 1;
    } else {
      *incr_exp = 0;
    }
  } else if ind <= 57 {
    // if 40 <= ind <= 57
    if c_star.w[3] == 0 && c_star.w[2] == bid_ten2k256!(ind - 39).w[2] && c_star.w[1] == bid_ten2k256!(ind - 39).w[1] && c_star.w[0] == bid_ten2k256!(ind - 39).w[0] {
      // if  Cstar = 10^(q-x)
      c_star.w[0] = bid_ten2k256!(ind - 40).w[0]; // Cstar = 10^(q-x-1)
      c_star.w[1] = bid_ten2k256!(ind - 40).w[1];
      c_star.w[2] = bid_ten2k256!(ind - 40).w[2];
      *incr_exp = 1;
    } else {
      *incr_exp = 0;
    }
  // else if (ind == 58) is not needed becauae we do not have ten2k192[] yet
  } else {
    // if 58 <= ind <= 77 (actually 58 <= ind <= 74)
    if c_star.w[3] == bid_ten2k256!(ind - 39).w[3] && c_star.w[2] == bid_ten2k256!(ind - 39).w[2] && c_star.w[1] == bid_ten2k256!(ind - 39).w[1] && c_star.w[0] == bid_ten2k256!(ind - 39).w[0] {
      // if  Cstar = 10^(q-x)
      c_star.w[0] = bid_ten2k256!(ind - 40).w[0]; // Cstar = 10^(q-x-1)
      c_star.w[1] = bid_ten2k256!(ind - 40).w[1];
      c_star.w[2] = bid_ten2k256!(ind - 40).w[2];
      c_star.w[3] = bid_ten2k256!(ind - 40).w[3];
      *incr_exp = 1;
    } else {
      *incr_exp = 0;
    }
  }
  ptr_c_star.w[3] = c_star.w[3];
  ptr_c_star.w[2] = c_star.w[2];
  ptr_c_star.w[1] = c_star.w[1];
  ptr_c_star.w[0] = c_star.w[0];
}
