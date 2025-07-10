use crate::bid_internal::*;
use crate::bid_round::*;
use crate::bid128::*;
use crate::bid128_common::*;
use crate::{
  BID_INEXACT_EXCEPTION, BID_INVALID_EXCEPTION, BID_OVERFLOW_EXCEPTION, BID_ROUNDING_DOWN, BID_ROUNDING_TIES_AWAY, BID_ROUNDING_TO_NEAREST, BID_ROUNDING_TO_ZERO, BID_ROUNDING_UP, BID_UNDERFLOW_EXCEPTION, BidUint64, BidUint128, BidUint192,
  BidUint256, IdecFlags, IdecRound,
};
use core::mem::swap;

#[allow(clippy::too_many_arguments)]
fn bid_rounding_correction(rnd_mode: u32, is_inexact_lt_midpoint: bool, is_inexact_gt_midpoint: bool, is_midpoint_lt_even: bool, is_midpoint_gt_even: bool, mut unbexp: i32, ptrres: &mut BidUint128, ptrfpsf: &mut IdecFlags) {
  // unbiased true exponent unbexp may be larger than emax

  let mut res: BidUint128 = *ptrres; // expected to have the correct sign and coefficient
  // (the exponent field is ignored, as unbexp is used instead)
  let mut exp: BidUint64;
  let mut c_hi: BidUint64;
  let mut c_lo: BidUint64;

  // general correction from RN to RA, RM, RP, RZ
  // Note: if the result is negative, then is_inexact_lt_midpoint,
  // is_inexact_gt_midpoint, is_midpoint_lt_even, and is_midpoint_gt_even
  // have to be considered as if determined for the absolute value of the
  // result (so they seem to be reversed)

  if is_inexact_lt_midpoint || is_inexact_gt_midpoint || is_midpoint_lt_even || is_midpoint_gt_even {
    set_status_flags!(ptrfpsf, BID_INEXACT_EXCEPTION);
  }
  // apply correction to result calculated with unbounded exponent
  let sign = res.w[1] & MASK_SIGN;
  exp = (unbexp.wrapping_add(6176) as u64) << 49; // valid only if expmin<=unbexp<=expmax
  c_hi = res.w[1] & MASK_COEFF;
  c_lo = res.w[0];
  if (sign == 0 && ((rnd_mode == BID_ROUNDING_UP && is_inexact_lt_midpoint) || ((rnd_mode == BID_ROUNDING_TIES_AWAY || rnd_mode == BID_ROUNDING_UP) && is_midpoint_gt_even)))
    || (sign > 0 && ((rnd_mode == BID_ROUNDING_DOWN && is_inexact_lt_midpoint) || ((rnd_mode == BID_ROUNDING_TIES_AWAY || rnd_mode == BID_ROUNDING_DOWN) && is_midpoint_gt_even)))
  {
    // C = C + 1
    inc!(c_lo);
    if c_lo == 0 {
      inc!(c_hi);
    }
    if c_hi == 0x0001ed09bead87c0 && c_lo == 0x378d8e6400000000 {
      // C = 10^34 => rounding overflow
      c_hi = 0x0000314dc6448d93;
      c_lo = 0x38c15b0a00000000; // 10^33
      // exp = exp + EXP_P1;
      inc!(unbexp);
      exp = (unbexp.wrapping_add(6176) as u64) << 49;
    }
  } else if (is_midpoint_lt_even || is_inexact_gt_midpoint) && ((sign > 0 && (rnd_mode == BID_ROUNDING_UP || rnd_mode == BID_ROUNDING_TO_ZERO)) || (sign == 0 && (rnd_mode == BID_ROUNDING_DOWN || rnd_mode == BID_ROUNDING_TO_ZERO))) {
    // C = C - 1
    dec!(c_lo);
    if c_lo == 0xffffffffffffffff {
      dec!(c_hi);
    }
    // check if we crossed into the lower decade
    if c_hi == 0x0000314dc6448d93 && c_lo == 0x38c15b09ffffffff {
      // C = 10^33 - 1
      if exp > 0 {
        c_hi = 0x0001ed09bead87c0; // 10^34 - 1
        c_lo = 0x378d8e63ffffffff;
        // exp = exp - EXP_P1;
        dec!(unbexp);
        exp = (unbexp.wrapping_add(6176) as u64) << 49;
      } else {
        // if exp = 0 the result is tiny & inexact
        *ptrfpsf |= BID_UNDERFLOW_EXCEPTION;
      }
    }
  } else { // the result is already correct
  }
  if unbexp > EXPMAX {
    // 6111
    set_status_flags!(ptrfpsf, BID_INEXACT_EXCEPTION | BID_OVERFLOW_EXCEPTION);
    exp = 0;
    if sign == 0 {
      // result is positive
      if rnd_mode == BID_ROUNDING_UP || rnd_mode == BID_ROUNDING_TIES_AWAY {
        // +inf
        c_hi = 0x7800000000000000;
        c_lo = 0x0000000000000000;
      } else {
        // res = +MAXFP = (10^34-1) * 10^emax
        c_hi = 0x5fffed09bead87c0;
        c_lo = 0x378d8e63ffffffff;
      }
    } else {
      // result is negative
      if rnd_mode == BID_ROUNDING_DOWN || rnd_mode == BID_ROUNDING_TIES_AWAY {
        // -inf
        c_hi = 0xf800000000000000;
        c_lo = 0x0000000000000000;
      } else {
        // res = -MAXFP = -(10^34-1) * 10^emax
        c_hi = 0xdfffed09bead87c0;
        c_lo = 0x378d8e63ffffffff;
      }
    }
  }
  // assemble the result
  res.w[1] = sign | exp | c_hi;
  res.w[0] = c_lo;
  *ptrres = res;
}

fn bid_add256(mut x: BidUint256, y: BidUint256, pz: &mut BidUint256) {
  // *z = x + yl assume the sum fits in 256 bits
  let mut z: BidUint256 = Default::default();
  z.w[0] = x.w[0].wrapping_add(y.w[0]);
  if z.w[0] < x.w[0] {
    inc!(x.w[1]);
    if x.w[1] == 0x0000000000000000 {
      inc!(x.w[2]);
      if x.w[2] == 0x0000000000000000 {
        inc!(x.w[3]);
      }
    }
  }
  z.w[1] = x.w[1].wrapping_add(y.w[1]);
  if z.w[1] < x.w[1] {
    inc!(x.w[2]);
    if x.w[2] == 0x0000000000000000 {
      inc!(x.w[3]);
    }
  }
  z.w[2] = x.w[2].wrapping_add(y.w[2]);
  if z.w[2] < x.w[2] {
    inc!(x.w[3]);
  }
  z.w[3] = x.w[3].wrapping_add(y.w[3]); // it was assumed that no carry is possible
  *pz = z;
}

fn bid_sub256(mut x: BidUint256, y: BidUint256, pz: &mut BidUint256) {
  // *z = x - y; assume x >= y
  let mut z: BidUint256 = Default::default();
  z.w[0] = x.w[0].wrapping_sub(y.w[0]);
  if z.w[0] > x.w[0] {
    dec!(x.w[1]);
    if x.w[1] == 0xffffffffffffffff {
      dec!(x.w[2]);
      if x.w[2] == 0xffffffffffffffff {
        dec!(x.w[3]);
      }
    }
  }
  z.w[1] = x.w[1].wrapping_sub(y.w[1]);
  if z.w[1] > x.w[1] {
    dec!(x.w[2]);
    if x.w[2] == 0xffffffffffffffff {
      dec!(x.w[3]);
    }
  }
  z.w[2] = x.w[2].wrapping_sub(y.w[2]);
  if z.w[2] > x.w[2] {
    dec!(x.w[3]);
  }
  z.w[3] = x.w[3].wrapping_sub(y.w[3]); // no borrow possible, because x >= y
  *pz = z;
}

/// FMA.
pub fn bid128_fma(x: BidUint128, y: BidUint128, z: BidUint128, rounding: IdecRound, flags: &mut IdecFlags) -> BidUint128 {
  let mut is_midpoint_lt_even = false;
  let mut is_midpoint_gt_even = false;
  let mut is_inexact_lt_midpoint = false;
  let mut is_inexact_gt_midpoint = false;
  bid128_ext_fma(&mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint, x, y, z, rounding, flags)
}

fn bid_bid_nr_digits256(r256: BidUint256) -> i32 {
  let mut ind: i32;
  // determine the number of decimal digits in r256
  if r256.w[3] == 0 && r256.w[2] == 0 && r256.w[1] == 0 {
    // between 1 and 19 digits
    ind = 1;
    while ind <= 19 {
      if r256.w[0] < bid_ten2k64![ind] {
        break;
      }
      ind += 1;
    }
  // ind digits
  } else if r256.w[3] == 0x0 && r256.w[2] == 0 && (r256.w[1] < bid_ten2k128![0].w[1] || (r256.w[1] == bid_ten2k128![0].w[1] && r256.w[0] < bid_ten2k128![0].w[0])) {
    // 20 digits
    ind = 20;
  } else if r256.w[3] == 0x0 && r256.w[2] == 0 {
    // between 21 and 38 digits
    ind = 1;
    while ind <= 18 {
      if r256.w[1] < bid_ten2k128![ind].w[1] || (r256.w[1] == bid_ten2k128![ind].w[1] && r256.w[0] < bid_ten2k128![ind].w[0]) {
        break;
      }
      ind += 1;
    }
    // ind + 20 digits
    ind += 20;
  } else if r256.w[3] == 0
    && (r256.w[2] < bid_ten2k256![0].w[2] || (r256.w[2] == bid_ten2k256![0].w[2] && r256.w[1] < bid_ten2k256![0].w[1]) || (r256.w[2] == bid_ten2k256![0].w[2] && r256.w[1] == bid_ten2k256![0].w[1] && r256.w[0] < bid_ten2k256![0].w[0]))
  {
    // 39 digits
    ind = 39;
  } else {
    // between 40 and 68 digits
    ind = 1;
    while ind <= 29 {
      if r256.w[3] < bid_ten2k256![ind].w[3]
        || (r256.w[3] == bid_ten2k256![ind].w[3] && r256.w[2] < bid_ten2k256![ind].w[2])
        || (r256.w[3] == bid_ten2k256![ind].w[3] && r256.w[2] == bid_ten2k256![ind].w[2] && r256.w[1] < bid_ten2k256![ind].w[1])
        || (r256.w[3] == bid_ten2k256![ind].w[3] && r256.w[2] == bid_ten2k256![ind].w[2] && r256.w[1] == bid_ten2k256![ind].w[1] && r256.w[0] < bid_ten2k256![ind].w[0])
      {
        break;
      }
      ind += 1;
    }
    // ind + 39 digits
    ind += 39;
  }
  ind
}

/// add/subtract c4 and c3 * 10^scale; this may follow a previous rounding, so
/// use the rounding information from ptr_is_* to avoid a double rounding error
#[allow(clippy::too_many_arguments)]
fn bid_add_and_round(
  q3: i32,
  q4: i32,
  mut e4: i32,
  delta: i32,
  p34: i32,
  z_sign: BidUint64,
  mut p_sign: BidUint64,
  c3: BidUint128,
  c4: BidUint256,
  rnd_mode: u32,
  ptr_is_midpoint_lt_even: &mut bool,
  ptr_is_midpoint_gt_even: &mut bool,
  ptr_is_inexact_lt_midpoint: &mut bool,
  ptr_is_inexact_gt_midpoint: &mut bool,
  ptrfpsf: &mut IdecFlags,
  ptrres: &mut BidUint128,
) {
  let mut x0: i32;
  let mut ind: i32;
  let mut r64: BidUint64 = 0;
  let mut p128: BidUint128 = Default::default();
  let mut r128: BidUint128 = Default::default();
  let mut p192: BidUint192 = Default::default();
  let mut r192: BidUint192 = Default::default();
  let mut r256: BidUint256 = Default::default();
  let mut is_midpoint_lt_even = false;
  let mut is_midpoint_gt_even = false;
  let mut is_inexact_lt_midpoint = false;
  let mut is_inexact_gt_midpoint = false;
  let is_midpoint_lt_even0;
  let is_midpoint_gt_even0;
  let is_inexact_lt_midpoint0;
  let is_inexact_gt_midpoint0;
  let mut incr_exp: i32 = 0;
  let mut is_tiny = false;
  let mut lt_half_ulp = false;
  let mut eq_half_ulp = false;
  let mut res: BidUint128 = *ptrres;

  // scale c3 up by 10^(q4-delta-q3), 0 <= q4-delta-q3 <= 2*P34-2 = 66
  let scale = q4 - delta - q3; // 0 <= scale <= 66 (or 0 <= scale <= 68 if this
  // comes from Cases (2), (3), (4), (5), (6), with 0 <= |delta| <= 1

  // calculate c3 * 10^scale in r256 (it has at most 67 decimal digits for
  // Cases (15),(16),(17) and at most 69 for Cases (2),(3),(4),(5),(6))
  if scale == 0 {
    r256.w[3] = 0;
    r256.w[2] = 0;
    r256.w[1] = c3.w[1];
    r256.w[0] = c3.w[0];
  } else if scale <= 19 {
    // 10^scale fits in 64 bits
    p128.w[1] = 0;
    p128.w[0] = bid_ten2k64![scale];
    mul_128x128_to_256!(r256, p128, c3);
  } else if scale <= 38 {
    // 10^scale fits in 128 bits
    mul_128x128_to_256!(r256, bid_ten2k128![scale - 20], c3);
  } else if scale <= 57 {
    // 39 <= scale <= 57
    // 10^scale fits in 192 bits but c3 * 10^scale fits in 223 or 230 bits
    // (10^67 has 223 bits; 10^69 has 230 bits);
    // must split the computation:
    // 10^scale * c3 = 10*38 * 10^(scale-38) * c3 where 10^38 takes 127
    // bits and so 10^(scale-38) * c3 fits in 128 bits with certainty
    // Note that 1 <= scale - 38 <= 19 => 10^(scale-38) fits in 64 bits
    mul_64x128_to_128!(r128, bid_ten2k64![scale - 38], c3);
    // now multiply r128 by 10^38
    mul_128x128_to_256!(r256, r128, bid_ten2k128![18]);
  } else {
    // 58 <= scale <= 66
    // 10^scale takes between 193 and 220 bits,
    // and c3 * 10^scale fits in 223 bits (10^67/10^69 has 223/230 bits)
    // must split the computation:
    // 10^scale * c3 = 10*38 * 10^(scale-38) * c3 where 10^38 takes 127
    // bits and so 10^(scale-38) * c3 fits in 128 bits with certainty
    // Note that 20 <= scale - 38 <= 30 => 10^(scale-38) fits in 128 bits
    // Calculate first 10^(scale-38) * c3, which fits in 128 bits; because
    // 10^(scale-38) takes more than 64 bits, c3 will take less than 64
    mul_64x128_to_128!(r128, c3.w[0], bid_ten2k128![scale - 58]);
    // now calculate 10*38 * 10^(scale-38) * c3
    mul_128x128_to_256!(r256, r128, bid_ten2k128![18]);
  }
  // c3 * 10^scale is now in r256

  // for Cases (15), (16), (17) c4 > c3 * 10^scale because c4 has at least
  // one extra digit; for Cases (2), (3), (4), (5), or (6) any order is
  // possible
  // add/subtract c4 and c3 * 10^scale; the exponent is e4
  if p_sign == z_sign {
    // r256 = c4 + r256
    // calculate r256 = c4 + c3 * 10^scale = c4 + r256 which is exact,
    // but may require rounding
    bid_add256(c4, r256, &mut r256);
  } else {
    // if (p_sign != z_sign) { // r256 = c4 - r256
    // calculate r256 = c4 - c3 * 10^scale = c4 - r256 or
    // r256 = c3 * 10^scale - c4 = r256 - c4 which is exact,
    // but may require rounding

    // compare first r256 = c3 * 10^scale and c4
    if r256.w[3] > c4.w[3]
      || (r256.w[3] == c4.w[3] && r256.w[2] > c4.w[2])
      || (r256.w[3] == c4.w[3] && r256.w[2] == c4.w[2] && r256.w[1] > c4.w[1])
      || (r256.w[3] == c4.w[3] && r256.w[2] == c4.w[2] && r256.w[1] == c4.w[1] && r256.w[0] >= c4.w[0])
    {
      // c3 * 10^scale >= c4
      // calculate r256 = c3 * 10^scale - c4 = r256 - c4, which is exact,
      // but may require rounding
      bid_sub256(r256, c4, &mut r256);
      // flip p_sign too, because the result has the sign of z
      p_sign = z_sign;
    } else {
      // if c4 > c3 * 10^scale
      // calculate r256 = c4 - c3 * 10^scale = c4 - r256, which is exact,
      // but may require rounding
      bid_sub256(c4, r256, &mut r256);
    }
    // if the result is pure zero, the sign depends on the rounding mode
    // (x*y and z had opposite signs)
    if r256.w[3] == 0 && r256.w[2] == 0 && r256.w[1] == 0 && r256.w[0] == 0 {
      if rnd_mode != BID_ROUNDING_DOWN {
        p_sign = 0x0000000000000000;
      } else {
        p_sign = 0x8000000000000000;
      }
      // the exponent is max (e4, expmin)
      if e4 < -6176 {
        e4 = EXPMIN;
      }
      // assemble result
      res.w[1] = p_sign | (e4.wrapping_add(6176) as u64) << 49;
      res.w[0] = 0;
      *ptrres = res;
      return;
    }
  }

  // determine the number of decimal digits in r256
  ind = bid_bid_nr_digits256(r256);

  // the exact result is (-1)^p_sign * r256 * 10^e4 where q (r256) = ind;
  // round to the destination precision, with unbounded exponent

  if ind <= p34 {
    // result rounded to the destination precision with unbounded exponent
    // is exact
    if ind + e4 < p34 + EXPMIN {
      is_tiny = true; // applies to all rounding modes
      // (regardless of the tininess detection method)
    }
    res.w[1] = p_sign | (e4.wrapping_add(6176) as u64) << 49 | r256.w[1];
    res.w[0] = r256.w[0];
  // Note: res is correct only if expmin <= e4 <= expmax
  } else {
    // if (ind > p34)
    // if more than P digits, round to nearest to P digits
    // round r256 to p34 digits
    x0 = ind - p34; // 1 <= x0 <= 34 as 35 <= ind <= 68
    if ind <= 38 {
      p128.w[1] = r256.w[1];
      p128.w[0] = r256.w[0];
      bid_round128_19_38(ind, x0, p128, &mut r128, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
    } else if ind <= 57 {
      p192.w[2] = r256.w[2];
      p192.w[1] = r256.w[1];
      p192.w[0] = r256.w[0];
      bid_round192_39_57(ind, x0, p192, &mut r192, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
      r128.w[1] = r192.w[1];
      r128.w[0] = r192.w[0];
    } else {
      // if (ind <= 68)
      bid_round256_58_76(ind, x0, r256, &mut r256, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
      r128.w[1] = r256.w[1];
      r128.w[0] = r256.w[0];
    }
    if !cfg!(feature = "decimal-tiny-detection-after-rounding") {
      if e4 + x0 < EXPMIN {
        // for all rounding modes
        is_tiny = true;
      } else {
        //
      }
    }
    // the rounded result has p34 = 34 digits
    e4 = e4 + x0 + incr_exp;
    if rnd_mode == BID_ROUNDING_TO_NEAREST {
      if cfg!(feature = "decimal-tiny-detection-after-rounding") {
        if e4 < EXPMIN {
          is_tiny = true; // for other rounding modes apply correction
        }
      } else {
        //
      }
    } else {
      // for RM, RP, RZ, RA apply correction in order to determine tininess
      // but do not save the result; apply the correction to
      // (-1)^p_sign * significand * 10^0
      p128.w[1] = p_sign | 0x3040000000000000 | r128.w[1];
      p128.w[0] = r128.w[0];
      bid_rounding_correction(rnd_mode, is_inexact_lt_midpoint, is_inexact_gt_midpoint, is_midpoint_lt_even, is_midpoint_gt_even, 0, &mut p128, ptrfpsf);
      // the number of digits in the significand is p34 = 34
      if cfg!(feature = "decimal-tiny-detection-after-rounding") {
        let tmp_scale = (((p128.w[1] & MASK_EXP) >> 49) as i32).wrapping_sub(6176); // -1, 0, or +1
        if e4 + tmp_scale < EXPMIN {
          is_tiny = true;
        }
      }
    }
    ind = p34; // the number of decimal digits in the signifcand of res
    res.w[1] = p_sign | (e4.wrapping_add(6176) as u64) << 49 | r128.w[1]; // RN
    res.w[0] = r128.w[0];
    // Note: res is correct only if expmin <= e4 <= expmax
    // set the inexact flag after rounding with bounded exponent, if any
  }
  // at this point we have the result rounded with unbounded exponent in
  // res and we know its tininess:
  // res = (-1)^p_sign * significand * 10^e4,
  // where q (significand) = ind <= p34
  // Note: res is correct only if expmin <= e4 <= expmax

  // check for overflow if RN
  if rnd_mode == BID_ROUNDING_TO_NEAREST && (ind + e4) > (p34 + EXPMAX) {
    res.w[1] = p_sign | 0x7800000000000000;
    res.w[0] = 0x0000000000000000;
    *ptrres = res;
    set_status_flags!(ptrfpsf, BID_INEXACT_EXCEPTION | BID_OVERFLOW_EXCEPTION);
    return; // BID_RETURN (res)
  } // else not overflow or not RN, so continue

  // if (e4 >= expmin) we have the result rounded with bounded exponent
  if e4 < EXPMIN {
    x0 = EXPMIN - e4; // x0 >= 1; the number of digits to chop off of res
    // where the result rounded [at most] once is
    //   (-1)^p_sign * significand_res * 10^e4

    // avoid double rounding error
    is_inexact_lt_midpoint0 = is_inexact_lt_midpoint;
    is_inexact_gt_midpoint0 = is_inexact_gt_midpoint;
    is_midpoint_lt_even0 = is_midpoint_lt_even;
    is_midpoint_gt_even0 = is_midpoint_gt_even;
    is_inexact_lt_midpoint = false;
    is_inexact_gt_midpoint = false;
    is_midpoint_lt_even = false;
    is_midpoint_gt_even = false;

    if x0 > ind {
      // nothing is left of res when moving the decimal point left x0 digits
      is_inexact_lt_midpoint = true;
      res.w[1] = p_sign;
      res.w[0] = 0x0000000000000000;
      e4 = EXPMIN;
    } else if x0 == ind {
      // 1 <= x0 = ind <= p34 = 34
      // this is <, =, or > 1/2 ulp
      // compare the ind-digit value in the significand of res with
      // 1/2 ulp = 5*10^(ind-1), i.e. determine whether it is
      // less than, equal to, or greater than 1/2 ulp (significand of res)
      r128.w[1] = res.w[1] & MASK_COEFF;
      r128.w[0] = res.w[0];
      if ind <= 19 {
        if r128.w[0] < bid_midpoint64![ind - 1] {
          // < 1/2 ulp
          lt_half_ulp = true;
          is_inexact_lt_midpoint = true;
        } else if r128.w[0] == bid_midpoint64![ind - 1] {
          // = 1/2 ulp
          eq_half_ulp = true;
          is_midpoint_gt_even = true;
        } else {
          // > 1/2 ulp
          // gt_half_ulp = 1;
          is_inexact_gt_midpoint = true;
        }
      } else {
        // if (ind <= 38) {
        if r128.w[1] < bid_midpoint128![ind - 20].w[1] || (r128.w[1] == bid_midpoint128![ind - 20].w[1] && r128.w[0] < bid_midpoint128![ind - 20].w[0]) {
          // < 1/2 ulp
          lt_half_ulp = true;
          is_inexact_lt_midpoint = true;
        } else if r128.w[1] == bid_midpoint128![ind - 20].w[1] && r128.w[0] == bid_midpoint128![ind - 20].w[0] {
          // = 1/2 ulp
          eq_half_ulp = true;
          is_midpoint_gt_even = true;
        } else {
          // > 1/2 ulp
          // gt_half_ulp = 1;
          is_inexact_gt_midpoint = true;
        }
      }
      if lt_half_ulp || eq_half_ulp {
        // res = +0.0 * 10^expmin
        res.w[1] = 0x0000000000000000;
        res.w[0] = 0x0000000000000000;
      } else {
        // if (gt_half_ulp)
        // res = +1 * 10^expmin
        res.w[1] = 0x0000000000000000;
        res.w[0] = 0x0000000000000001;
      }
      res.w[1] |= p_sign;
      e4 = EXPMIN;
    } else {
      // if (1 <= x0 <= ind - 1 <= 33)
      // round the ind-digit result to ind - x0 digits

      if ind <= 18 {
        // 2 <= ind <= 18
        bid_round64_2_18(ind, x0, res.w[0], &mut r64, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
        res.w[1] = 0x0;
        res.w[0] = r64;
      } else if ind <= 38 {
        p128.w[1] = res.w[1] & MASK_COEFF;
        p128.w[0] = res.w[0];
        bid_round128_19_38(ind, x0, p128, &mut res, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
      }
      e4 = e4.wrapping_add(x0); // expmin
      // we want the exponent to be expmin, so if incr_exp = 1 then
      // multiply the rounded result by 10 - it will still fit in 113 bits
      if incr_exp > 0 {
        // 64 x 128 -> 128
        p128.w[1] = res.w[1] & MASK_COEFF;
        p128.w[0] = res.w[0];
        mul_64x128_to_128!(res, bid_ten2k64![1], p128);
      }
      res.w[1] = p_sign | (e4.wrapping_add(6176) as u64) << 49 | (res.w[1] & MASK_COEFF);
      // avoid a double rounding error
      if (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) && is_midpoint_lt_even {
        // double rounding error upward
        // res = res - 1
        dec!(res.w[0]);
        if res.w[0] == 0xffffffffffffffff {
          dec!(res.w[1]);
        }
        // Note: a double rounding error upward is not possible; for this
        // the result after the first rounding would have to be 99...95
        // (35 digits in all), possibly followed by a number of zeros; this
        // is not possible in Cases (2)-(6) or (15)-(17) which may get here
        is_midpoint_lt_even = false;
        is_inexact_lt_midpoint = true;
      } else if (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) && is_midpoint_gt_even {
        // double rounding error downward
        // res = res + 1
        inc!(res.w[0]);
        if res.w[0] == 0 {
          inc!(res.w[1]);
        }
        is_midpoint_gt_even = false;
        is_inexact_gt_midpoint = true;
      } else if !is_midpoint_lt_even && !is_midpoint_gt_even && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint {
        // if this second rounding was exact the result may still be
        // inexact because of the first rounding
        if is_inexact_gt_midpoint0 || is_midpoint_lt_even0 {
          is_inexact_gt_midpoint = true;
        }
        if is_inexact_lt_midpoint0 || is_midpoint_gt_even0 {
          is_inexact_lt_midpoint = true;
        }
      } else if is_midpoint_gt_even && (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) {
        // pulled up to a midpoint
        is_inexact_lt_midpoint = true;
        is_inexact_gt_midpoint = false;
        is_midpoint_lt_even = false;
        is_midpoint_gt_even = false;
      } else if is_midpoint_lt_even && (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) {
        // pulled down to a midpoint
        is_inexact_lt_midpoint = false;
        is_inexact_gt_midpoint = true;
        is_midpoint_lt_even = false;
        is_midpoint_gt_even = false;
      }
    }
  }
  // res contains the correct result
  // apply correction if not rounding to nearest
  if rnd_mode != BID_ROUNDING_TO_NEAREST {
    bid_rounding_correction(rnd_mode, is_inexact_lt_midpoint, is_inexact_gt_midpoint, is_midpoint_lt_even, is_midpoint_gt_even, e4, &mut res, ptrfpsf);
  }
  if is_midpoint_lt_even || is_midpoint_gt_even || is_inexact_lt_midpoint || is_inexact_gt_midpoint {
    // set the inexact flag
    set_status_flags!(ptrfpsf, BID_INEXACT_EXCEPTION);
    if is_tiny {
      set_status_flags!(ptrfpsf, BID_UNDERFLOW_EXCEPTION);
    }
  }

  *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
  *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
  *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
  *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;
  *ptrres = res;
}

/// Ext FMA.
#[allow(clippy::too_many_arguments)]
pub fn bid128_ext_fma(
  ptr_is_midpoint_lt_even: &mut bool,
  ptr_is_midpoint_gt_even: &mut bool,
  ptr_is_inexact_lt_midpoint: &mut bool,
  ptr_is_inexact_gt_midpoint: &mut bool,
  mut x: BidUint128,
  mut y: BidUint128,
  mut z: BidUint128,
  rounding: IdecRound,
  flags: &mut IdecFlags,
) -> BidUint128 {
  let mut res = BidUint128 { w: [0xbaddbaddbaddbadd, 0xbaddbaddbaddbadd] };
  let mut z_sign: BidUint64;
  let mut p_sign: BidUint64;
  let mut tmp_sign: BidUint64;
  let mut x_exp: BidUint64 = 0;
  let mut y_exp: BidUint64 = 0;
  let mut z_exp: BidUint64 = 0;
  let mut p_exp: BidUint64;
  let mut c1: BidUint128 = Default::default();
  let mut c2: BidUint128 = Default::default();
  let mut c3: BidUint128 = Default::default();
  let mut c4: BidUint256 = Default::default();
  let mut q1: i32 = 0;
  let mut q2: i32 = 0;
  let mut q3: i32 = 0;
  let mut q4: i32;
  let x_nr_bits: i32;
  let y_nr_bits: i32;
  let z_nr_bits: i32;
  let mut e3: i32;
  let mut e4: i32;
  let mut scale: i32;
  let mut ind: i32;
  let mut delta: i32;
  let mut x0: i32;
  let mut lsb: i32;
  let mut is_midpoint_lt_even = false;
  let mut is_midpoint_gt_even = false;
  let mut is_inexact_lt_midpoint = false;
  let mut is_inexact_gt_midpoint = false;
  let mut is_midpoint_lt_even0;
  let mut is_midpoint_gt_even0;
  let mut is_inexact_lt_midpoint0;
  let mut is_inexact_gt_midpoint0;
  let mut is_tiny = false;
  let mut lt_half_ulp = false;
  let mut eq_half_ulp = false;
  let mut gt_half_ulp = false;
  let mut incr_exp: i32 = 0;
  let mut r64: BidUint64 = 0;
  let mut tmp64: BidUint64;
  let mut p128: BidUint128 = Default::default();
  let mut r128: BidUint128 = Default::default();
  let mut p192: BidUint192 = Default::default();
  let mut r192: BidUint192 = Default::default();
  let mut r256: BidUint256 = Default::default();
  let c4gt5toq4m1: bool;

  // The following are based on the table of special cases for fma;
  // the NaN behavior is similar to that of the IA-64 Architecture fma.

  // Identify cases where at least one operand is NaN.

  if (y.w[1] & MASK_NAN) == MASK_NAN {
    // y is NAN
    // If x = {0, f, inf, NaN}, y = NaN, z = {0, f, inf, NaN} then res = Q (y)
    // check first for non-canonical NaN payload.
    if ((y.w[1] & 0x00003fffffffffff) > 0x0000314dc6448d93) || (((y.w[1] & 0x00003fffffffffff) == 0x0000314dc6448d93) && (y.w[0] > 0x38c15b09ffffffff)) {
      y.w[1] &= &0xffffc00000000000;
      y.w[0] = 0x0;
    }
    if (y.w[1] & MASK_SNAN) == MASK_SNAN {
      // y is SNAN
      // set invalid flag
      set_status_flags!(flags, BID_INVALID_EXCEPTION);
      // return quiet (y)
      res.w[1] = y.w[1] & 0xfc003fffffffffff; // clear out also G[6]-G[16]
      res.w[0] = y.w[0];
    } else {
      // y is QNaN
      // return y
      res.w[1] = y.w[1] & 0xfc003fffffffffff; // clear out G[6]-G[16]
      res.w[0] = y.w[0];
      // if z = SNaN or x = SNaN signal invalid exception
      if (z.w[1] & MASK_SNAN) == MASK_SNAN || (x.w[1] & MASK_SNAN) == MASK_SNAN {
        // set invalid flag
        set_status_flags!(flags, BID_INVALID_EXCEPTION);
      }
    }
    *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
    *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
    *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
    *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

    return res;
  } else if (z.w[1] & MASK_NAN) == MASK_NAN {
    // z is NAN
    // if x = {0, f, inf, NaN}, y = {0, f, inf}, z = NaN then res = Q (z)
    // check first for non-canonical NaN payload
    if ((z.w[1] & 0x00003fffffffffff) > 0x0000314dc6448d93) || (((z.w[1] & 0x00003fffffffffff) == 0x0000314dc6448d93) && (z.w[0] > 0x38c15b09ffffffff)) {
      z.w[1] &= 0xffffc00000000000;
      z.w[0] = 0;
    }
    if (z.w[1] & MASK_SNAN) == MASK_SNAN {
      // z is SNAN
      // set invalid flag
      set_status_flags!(flags, BID_INVALID_EXCEPTION);
      // return quiet (z)
      res.w[1] = z.w[1] & 0xfc003fffffffffff; // clear out also G[6]-G[16]
      res.w[0] = z.w[0];
    } else {
      // z is QNaN
      // return z
      res.w[1] = z.w[1] & 0xfc003fffffffffff; // clear out G[6]-G[16]
      res.w[0] = z.w[0];
      // if x = SNaN signal invalid exception
      if (x.w[1] & MASK_SNAN) == MASK_SNAN {
        // set invalid flag
        set_status_flags!(flags, BID_INVALID_EXCEPTION);
      }
    }
    *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
    *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
    *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
    *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;
    return res;
  } else if (x.w[1] & MASK_NAN) == MASK_NAN {
    // x is NAN
    // if x = NaN, y = {0, f, inf}, z = {0, f, inf} then res = Q (x)
    // check first for non-canonical NaN payload
    if ((x.w[1] & 0x00003fffffffffff) > 0x0000314dc6448d93) || (((x.w[1] & 0x00003fffffffffff) == 0x0000314dc6448d93) && (x.w[0] > 0x38c15b09ffffffff)) {
      x.w[1] &= 0xffffc00000000000;
      x.w[0] = 0;
    }
    if (x.w[1] & MASK_SNAN) == MASK_SNAN {
      // x is SNAN
      // set invalid flag
      set_status_flags!(flags, BID_INVALID_EXCEPTION);
      // return quiet (x)
      res.w[1] = x.w[1] & 0xfc003fffffffffff; // clear out also G[6]-G[16]
      res.w[0] = x.w[0];
    } else {
      // x is QNaN
      // return x
      res.w[1] = x.w[1] & 0xfc003fffffffffff; // clear out G[6]-G[16]
      res.w[0] = x.w[0];
    }
    *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
    *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
    *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
    *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;
    return res;
  }

  // x, y, z are 0, f, or inf but not NaN => unpack the arguments and check for non-canonical values.
  let x_sign = x.w[1] & MASK_SIGN; // 0 for positive, MASK_SIGN for negative
  c1.w[1] = x.w[1] & MASK_COEFF;
  c1.w[0] = x.w[0];
  if (x.w[1] & MASK_ANY_INF) != MASK_INF {
    // x != inf
    // if x is not infinity check for non-canonical values - treated as zero
    if (x.w[1] & 0x6000000000000000) == 0x6000000000000000 {
      // G0_G1=11
      // non-canonical
      x_exp = (x.w[1] << 2) & MASK_EXP; // biased and shifted left 49 bits
      c1.w[1] = 0; // significand high
      c1.w[0] = 0; // significand low
    } else {
      // G0_G1 != 11
      x_exp = x.w[1] & MASK_EXP; // biased and shifted left 49 bits
      if c1.w[1] > 0x0001ed09bead87c0 || (c1.w[1] == 0x0001ed09bead87c0 && c1.w[0] > 0x378d8e63ffffffff) {
        // x is non-canonical if coefficient is larger than 10^34 -1
        c1.w[1] = 0;
        c1.w[0] = 0;
      } else { // canonical
      }
    }
  }

  let y_sign = y.w[1] & MASK_SIGN; // 0 for positive, MASK_SIGN for negative
  c2.w[1] = y.w[1] & MASK_COEFF;
  c2.w[0] = y.w[0];
  if (y.w[1] & MASK_ANY_INF) != MASK_INF {
    // y != inf
    // if y is not infinity check for non-canonical values - treated as zero
    if (y.w[1] & 0x6000000000000000) == 0x6000000000000000 {
      // G0_G1=11
      // non-canonical
      y_exp = (y.w[1] << 2) & MASK_EXP; // biased and shifted left 49 bits
      c2.w[1] = 0; // significand high
      c2.w[0] = 0; // significand low
    } else {
      // G0_G1 != 11
      y_exp = y.w[1] & MASK_EXP; // biased and shifted left 49 bits
      if c2.w[1] > 0x0001ed09bead87c0 || (c2.w[1] == 0x0001ed09bead87c0 && c2.w[0] > 0x378d8e63ffffffff) {
        // y is non-canonical if coefficient is larger than 10^34 -1
        c2.w[1] = 0;
        c2.w[0] = 0;
      } else { // canonical
      }
    }
  }
  z_sign = z.w[1] & MASK_SIGN; // 0 for positive, MASK_SIGN for negative
  c3.w[1] = z.w[1] & MASK_COEFF;
  c3.w[0] = z.w[0];
  if (z.w[1] & MASK_ANY_INF) != MASK_INF {
    // z != inf
    // if z is not infinity check for non-canonical values - treated as zero
    if (z.w[1] & 0x6000000000000000) == 0x6000000000000000 {
      // G0_G1=11
      // non-canonical
      z_exp = (z.w[1] << 2) & MASK_EXP; // biased and shifted left 49 bits
      c3.w[1] = 0; // significand high
      c3.w[0] = 0; // significand low
    } else {
      // G0_G1 != 11
      z_exp = z.w[1] & MASK_EXP; // biased and shifted left 49 bits
      if c3.w[1] > 0x0001ed09bead87c0 || (c3.w[1] == 0x0001ed09bead87c0 && c3.w[0] > 0x378d8e63ffffffff) {
        // z is non-canonical if coefficient is larger than 10^34 -1
        c3.w[1] = 0;
        c3.w[0] = 0;
      } else { // canonical
      }
    }
  }

  p_sign = x_sign ^ y_sign; // Sign of the product.

  // Identify cases where at least one operand is infinity.

  if (x.w[1] & MASK_ANY_INF) == MASK_INF {
    // x = inf
    if (y.w[1] & MASK_ANY_INF) == MASK_INF {
      // y = inf
      if (z.w[1] & MASK_ANY_INF) == MASK_INF {
        // z = inf
        if p_sign == z_sign {
          res.w[1] = z_sign | MASK_INF;
          res.w[0] = 0;
        } else {
          // return QNaN Indefinite
          res.w[1] = 0x7c00000000000000;
          res.w[0] = 0x0000000000000000;
          // set invalid flag
          set_status_flags!(flags, BID_INVALID_EXCEPTION);
        }
      } else {
        // z = 0 or z = f
        res.w[1] = p_sign | MASK_INF;
        res.w[0] = 0;
      }
    } else if c2.w[1] != 0 || c2.w[0] != 0 {
      // y = f
      if (z.w[1] & MASK_ANY_INF) == MASK_INF {
        // z = inf
        if p_sign == z_sign {
          res.w[1] = z_sign | MASK_INF;
          res.w[0] = 0x0;
        } else {
          // return QNaN Indefinite
          res.w[1] = 0x7c00000000000000;
          res.w[0] = 0x0000000000000000;
          // set invalid flag
          set_status_flags!(flags, BID_INVALID_EXCEPTION);
        }
      } else {
        // z = 0 or z = f
        res.w[1] = p_sign | MASK_INF;
        res.w[0] = 0x0;
      }
    } else {
      // y = 0
      // return QNaN Indefinite
      res.w[1] = 0x7c00000000000000;
      res.w[0] = 0x0000000000000000;
      // set invalid flag
      set_status_flags!(flags, BID_INVALID_EXCEPTION);
    }
    *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
    *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
    *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
    *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;
    return res;
  } else if (y.w[1] & MASK_ANY_INF) == MASK_INF {
    // y = inf
    if (z.w[1] & MASK_ANY_INF) == MASK_INF {
      // z = inf
      // x = f, necessarily
      if (p_sign != z_sign) || (c1.w[1] == 0 && c1.w[0] == 0) {
        // return QNaN Indefinite
        res.w[1] = 0x7c00000000000000;
        res.w[0] = 0x0000000000000000;
        // set invalid flag
        set_status_flags!(flags, BID_INVALID_EXCEPTION);
      } else {
        res.w[1] = z_sign | MASK_INF;
        res.w[0] = 0x0;
      }
    } else if c1.w[1] == 0x0 && c1.w[0] == 0x0 {
      // x = 0
      // z = 0, f, inf
      // return QNaN Indefinite
      res.w[1] = 0x7c00000000000000;
      res.w[0] = 0x0000000000000000;
      // set invalid flag
      set_status_flags!(flags, BID_INVALID_EXCEPTION);
    } else {
      // x = f and z = 0, f, necessarily
      res.w[1] = p_sign | MASK_INF;
      res.w[0] = 0x0;
    }
    *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
    *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
    *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
    *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;
    return res;
  } else if (z.w[1] & MASK_ANY_INF) == MASK_INF {
    // z = inf
    // x = 0, f and y = 0, f, necessarily
    res.w[1] = z_sign | MASK_INF;
    res.w[0] = 0x0;
    *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
    *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
    *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
    *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;
    return res;
  }

  let true_p_exp = ((x_exp >> 49) as i32).wrapping_sub(6176).wrapping_add(((y_exp >> 49) as i32).wrapping_sub(6176));
  if true_p_exp < -6176 {
    p_exp = 0; // cannot be less than EXP_MIN
  } else {
    p_exp = (true_p_exp.wrapping_add(6176) as u64) << 49;
  }

  if ((c1.w[1] == 0 && c1.w[0] == 0) || (c2.w[1] == 0 && c2.w[0] == 0)) && c3.w[1] == 0 && c3.w[0] == 0 {
    // (x = 0 or y = 0) and z = 0
    // the result is 0
    if p_exp < z_exp {
      res.w[1] = p_exp; // preferred exponent
    } else {
      res.w[1] = z_exp; // preferred exponent
    }
    if p_sign == z_sign {
      res.w[1] |= z_sign;
      res.w[0] = 0;
    } else {
      // x * y and z have opposite signs
      if rounding == BID_ROUNDING_DOWN {
        // res = -0.0
        res.w[1] |= MASK_SIGN;
        res.w[0] = 0;
      } else {
        // res = +0.0
        // res.w[1] |= 0x0;
        res.w[0] = 0;
      }
    }
    *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
    *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
    *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
    *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;
    return res;
  }

  // From this point on, we may need to know the number of decimal digits
  // in the significands of x, y, z when x, y, z != 0

  if c1.w[1] != 0 || c1.w[0] != 0 {
    // x = f (non-zero finite)
    // q1 = nr. of decimal digits in x
    // determine first the nr. of bits in x
    if c1.w[1] == 0 {
      if c1.w[0] >= 0x0020000000000000 {
        // x >= 2^53
        // split the 64-bit value in two 32-bit halves to avoid rounding errors
        x_nr_bits = 33 + bits!(c1.w[0] >> 32); // exact conversion
      } else {
        // if x < 2^53
        x_nr_bits = 1 + bits!(c1.w[0]); // exact conversion
      }
    } else {
      // c1.w[1] != 0 => nr. bits = 64 + nr_bits (c1.w[1])
      x_nr_bits = 65 + bits!(c1.w[1]);
    }
    q1 = bid_nr_digits!(x_nr_bits - 1).digits as i32;
    if q1 == 0 {
      q1 = bid_nr_digits!(x_nr_bits - 1).digits1 as i32;
      if c1.w[1] > bid_nr_digits!(x_nr_bits - 1).threshold_hi || (c1.w[1] == bid_nr_digits!(x_nr_bits - 1).threshold_hi && c1.w[0] >= bid_nr_digits!(x_nr_bits - 1).threshold_lo) {
        q1 = q1.wrapping_add(1);
      }
    }
  }

  if c2.w[1] != 0 || c2.w[0] != 0 {
    // y = f (non-zero finite)
    // q2 = nr. of decimal digits in y
    // determine first the nr. of bits in y
    if c2.w[1] == 0 {
      if c2.w[0] >= 0x0020000000000000 {
        // y >= 2^53
        // split the 64-bit value in two 32-bit halves to avoid rounding errors
        y_nr_bits = 33 + bits!(c2.w[0] >> 32); // exact conversion
      } else {
        // if y < 2^53
        y_nr_bits = 1 + bits!(c2.w[0]); // exact conversion
      }
    } else {
      // c2.w[1] != 0 => nr. bits = 64 + nr_bits (c2.w[1])
      y_nr_bits = 65 + bits!(c2.w[1]); // exact conversion
    }
    q2 = bid_nr_digits!(y_nr_bits - 1).digits as i32;
    if q2 == 0 {
      q2 = bid_nr_digits!(y_nr_bits - 1).digits1 as i32;
      if c2.w[1] > bid_nr_digits!(y_nr_bits - 1).threshold_hi || (c2.w[1] == bid_nr_digits!(y_nr_bits - 1).threshold_hi && c2.w[0] >= bid_nr_digits!(y_nr_bits - 1).threshold_lo) {
        q2 = q2.wrapping_add(1);
      }
    }
  }

  if c3.w[1] != 0 || c3.w[0] != 0 {
    // z = f (non-zero finite)
    // q3 = nr. of decimal digits in z
    // determine first the nr. of bits in z
    if c3.w[1] == 0 {
      if c3.w[0] >= 0x0020000000000000 {
        // z >= 2^53
        // split the 64-bit value in two 32-bit halves to avoid rounding errors
        z_nr_bits = 33 + bits!(c3.w[0] >> 32); // exact conversion
      } else {
        // if z < 2^53
        z_nr_bits = 1 + bits!(c3.w[0]); // exact conversion
      }
    } else {
      // c3.w[1] != 0 => nr. bits = 64 + nr_bits (c3.w[1])
      z_nr_bits = 65 + bits!(c3.w[1]); // exact conversion
    }
    q3 = bid_nr_digits!(z_nr_bits - 1).digits as i32;
    if q3 == 0 {
      q3 = bid_nr_digits!(z_nr_bits - 1).digits1 as i32;
      if c3.w[1] > bid_nr_digits!(z_nr_bits - 1).threshold_hi || (c3.w[1] == bid_nr_digits!(z_nr_bits - 1).threshold_hi && c3.w[0] >= bid_nr_digits!(z_nr_bits - 1).threshold_lo) {
        q3 = q3.wrapping_add(1);
      }
    }
  }

  if (c1.w[1] == 0x0 && c1.w[0] == 0x0) || (c2.w[1] == 0x0 && c2.w[0] == 0x0) {
    // x = 0 or y = 0
    // z = f, necessarily; for 0 + z return z, with the preferred exponent
    // the result is z, but need to get the preferred exponent
    if z_exp <= p_exp {
      // the preferred exponent is z_exp
      res.w[1] = z_sign | (z_exp & MASK_EXP) | c3.w[1];
      res.w[0] = c3.w[0];
    } else {
      // if (p_exp < z_exp) the preferred exponent is p_exp
      // return (c3 * 10^scale) * 10^(z_exp - scale)
      // where scale = min (P34-q3, (z_exp-p_exp) >> 49)
      scale = P34 - q3;
      ind = ((z_exp - p_exp) >> 49) as i32;
      if ind < scale {
        scale = ind;
      }
      if scale == 0 {
        res.w[1] = z.w[1]; // & MASK_COEFF, which is redundant
        res.w[0] = z.w[0];
      } else if q3 <= 19 {
        // z fits in 64 bits
        if scale <= 19 {
          // 10^scale fits in 64 bits
          // 64 x 64 c3.w[0] * bid_ten2k64!(scale]
          mul_64x64_to_128mach!(res, c3.w[0], bid_ten2k64!(scale));
        } else {
          // 10^scale fits in 128 bits
          // 64 x 128 c3.w[0] * bid_ten2k128!(scale - 20]
          mul_128x64_to_128!(res, c3.w[0], bid_ten2k128!(scale - 20));
        }
      } else {
        // z fits in 128 bits, but 10^scale must fit in 64 bits
        // 64 x 128 bid_ten2k64!(scale] * c3
        mul_128x64_to_128!(res, bid_ten2k64!(scale), c3);
      }
      // subtract scale from the exponent
      z_exp = z_exp.wrapping_sub((scale as u64) << 49);
      res.w[1] |= z_sign | (z_exp & MASK_EXP);
    }
    *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
    *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
    *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
    *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;
    return res;
  }
  let e1 = ((x_exp >> 49) as i32).wrapping_sub(6176); // unbiased exponent of x
  let e2 = ((y_exp >> 49) as i32).wrapping_sub(6176); // unbiased exponent of y
  e3 = ((z_exp >> 49) as i32).wrapping_sub(6176); // unbiased exponent of z
  e4 = e1.wrapping_add(e2); // unbiased exponent of the exact x * y

  // calculate c1 * c2 and its number of decimal digits, q4

  // the exact product has either q1 + q2 - 1 or q1 + q2 decimal digits
  // where 2 <= q1 + q2 <= 68
  // calculate c4 = c1 * c2 and determine q
  c4.w[3] = 0;
  c4.w[2] = 0;
  c4.w[1] = 0;
  c4.w[0] = 0;
  if q1 + q2 <= 19 {
    // if 2 <= q1 + q2 <= 19, c4 = c1 * c2 fits in 64 bits
    c4.w[0] = c1.w[0] * c2.w[0];
    // if c4 < 10^(q1+q2-1) then q4 = q1 + q2 - 1 else q4 = q1 + q2
    if c4.w[0] < bid_ten2k64!(q1 + q2 - 1) {
      q4 = q1.wrapping_add(q2).wrapping_sub(1); // q4 in [1, 18]
    } else {
      q4 = q1.wrapping_add(q2); // q4 in [2, 19]
    }

    // length of c1 * c2 rounded up to a multiple of 64 bits is len = 64;
  } else if q1 + q2 == 20 {
    // c4 = c1 * c2 fits in 64 or 128 bits
    // q1 <= 19 and q2 <= 19 so both c1 and c2 fit in 64 bits
    mul_64x64_to_128mach!(c4, c1.w[0], c2.w[0]);
    // if c4 < 10^(q1+q2-1) = 10^19 then q4 = q1+q2-1 = 19 else q4 = q1+q2 = 20
    if c4.w[1] == 0 && c4.w[0] < bid_ten2k64!(19) {
      // 19 = q1+q2-1
      // length of c1 * c2 rounded up to a multiple of 64 bits is len = 64;
      q4 = 19; // 19 = q1 + q2 - 1
    } else {
      // if (c4.w[1] == 0)
      //   length of c1 * c2 rounded up to a multiple of 64 bits is len = 64;
      // else
      //   length of c1 * c2 rounded up to a multiple of 64 bits is len = 128;
      q4 = 20; // 20 = q1 + q2
    }
  } else if q1 + q2 <= 38 {
    // 21 <= q1 + q2 <= 38
    // c4 = c1 * c2 fits in 64 or 128 bits
    // (64 bits possibly, but only when q1 + q2 = 21 and c4 has 20 digits)
    // at least one of c1, c2 has at most 19 decimal digits & fits in 64 bits
    if q1 <= 19 {
      mul_128x64_to_128!(c4, c1.w[0], c2);
    } else {
      // q2 <= 19
      mul_128x64_to_128!(c4, c2.w[0], c1);
    }
    // if c4 < 10^(q1+q2-1) then q4 = q1 + q2 - 1 else q4 = q1 + q2
    if c4.w[1] < bid_ten2k128!(q1 + q2 - 21).w[1] || (c4.w[1] == bid_ten2k128!(q1 + q2 - 21).w[1] && c4.w[0] < bid_ten2k128!(q1 + q2 - 21).w[0]) {
      // if (c4.w[1] == 0) // q4 = 20, necessarily
      //   length of c1 * c2 rounded up to a multiple of 64 bits is len = 64;
      // else
      //   length of c1 * c2 rounded up to a multiple of 64 bits is len = 128;
      q4 = q1 + q2 - 1; // q4 in [20, 37]
    } else {
      // length of c1 * c2 rounded up to a multiple of 64 bits is len = 128;
      q4 = q1 + q2; // q4 in [21, 38]
    }
  } else if q1 + q2 == 39 {
    // c4 = c1 * c2 fits in 128 or 192 bits
    // both c1 and c2 fit in 128 bits (actually in 113 bits)
    // may replace this by 128x128_to192
    mul_128x128_to_256!(c4, c1, c2); // c4.w[3] is 0
    // if c4 < 10^(q1+q2-1) = 10^38 then q4 = q1+q2-1 = 38 else q4 = q1+q2 = 39
    if c4.w[2] == 0 && (c4.w[1] < bid_ten2k128!(18).w[1] || (c4.w[1] == bid_ten2k128!(18).w[1] && c4.w[0] < bid_ten2k128!(18).w[0])) {
      // 18 = 38 - 20 = q1+q2-1 - 20
      // length of c1 * c2 rounded up to a multiple of 64 bits is len = 128;
      q4 = 38; // 38 = q1 + q2 - 1
    } else {
      // if (c4.w[2] == 0)
      // length of c1 * c2 rounded up to a multiple of 64 bits is len = 128;
      // else
      //   length of c1 * c2 rounded up to a multiple of 64 bits is len = 192;
      q4 = 39; // 39 = q1 + q2
    }
  } else if q1 + q2 <= 57 {
    // 40 <= q1 + q2 <= 57
    // c4 = c1 * c2 fits in 128 or 192 bits
    // (128 bits possibly, but only when q1 + q2 = 40 and c4 has 39 digits)
    // both c1 and c2 fit in 128 bits (actually in 113 bits); at most one
    // may fit in 64 bits
    if c1.w[1] == 0 {
      // c1 fits in 64 bits
      // mul_64x128_full! (REShi64, RESlo128, A64, B128)
      mul_64x128_full!(c4.w[2], c4, c1.w[0], c2);
    } else if c2.w[1] == 0 {
      // c2 fits in 64 bits
      // mul_64x128_full! (REShi64, RESlo128, A64, B128)
      mul_64x128_full!(c4.w[2], c4, c2.w[0], c1);
    } else {
      // both c1 and c2 require 128 bits
      // may use __mul_128x128_to_192 (c4.w[2], c4.w[0], c2.w[0], c1);
      mul_128x128_to_256!(c4, c1, c2); // c4.w[3] = 0
    }
    // if c4 < 10^(q1+q2-1) then q4 = q1 + q2 - 1 else q4 = q1 + q2
    if c4.w[2] < bid_ten2k256!(q1 + q2 - 40).w[2]
      || (c4.w[2] == bid_ten2k256!(q1 + q2 - 40).w[2] && (c4.w[1] < bid_ten2k256!(q1 + q2 - 40).w[1] || (c4.w[1] == bid_ten2k256!(q1 + q2 - 40).w[1] && c4.w[0] < bid_ten2k256!(q1 + q2 - 40).w[0])))
    {
      // if (c4.w[2] == 0) // q4 = 39, necessarily
      //   length of c1 * c2 rounded up to a multiple of 64 bits is len = 128;
      // else
      //   length of c1 * c2 rounded up to a multiple of 64 bits is len = 192;
      q4 = q1 + q2 - 1; // q4 in [39, 56]
    } else {
      // length of c1 * c2 rounded up to a multiple of 64 bits is len = 192;
      q4 = q1 + q2; // q4 in [40, 57]
    }
  } else if q1 + q2 == 58 {
    // c4 = c1 * c2 fits in 192 or 256 bits;
    // both c1 and c2 fit in 128 bits (actually in 113 bits); none can
    // fit in 64 bits, because each number must have at least 24 decimal
    // digits for the sum to have 58 (as the max. nr. of digits is 34) =>
    // c1.w[1] != 0 and c2.w[1] != 0
    mul_128x128_to_256!(c4, c1, c2);
    // if c4 < 10^(q1+q2-1) = 10^57 then q4 = q1+q2-1 = 57 else q4 = q1+q2 = 58
    if c4.w[3] == 0 && (c4.w[2] < bid_ten2k256!(18).w[2] || (c4.w[2] == bid_ten2k256!(18).w[2] && (c4.w[1] < bid_ten2k256!(18).w[1] || (c4.w[1] == bid_ten2k256!(18).w[1] && c4.w[0] < bid_ten2k256!(18).w[0])))) {
      // 18 = 57 - 39 = q1+q2-1 - 39
      // length of c1 * c2 rounded up to a multiple of 64 bits is len = 192;
      q4 = 57; // 57 = q1 + q2 - 1
    } else {
      // if (c4.w[3] == 0)
      //   length of c1 * c2 rounded up to a multiple of 64 bits is len = 192;
      // else
      //   length of c1 * c2 rounded up to a multiple of 64 bits is len = 256;
      q4 = 58; // 58 = q1 + q2
    }
  } else {
    // if 59 <= q1 + q2 <= 68
    // c4 = c1 * c2 fits in 192 or 256 bits
    // (192 bits possibly, but only when q1 + q2 = 59 and c4 has 58 digits)
    // both c1 and c2 fit in 128 bits (actually in 113 bits); none fits in
    // 64 bits
    // may use __mul_128x128_to_192 (c4.w[2], c4.w[0], c2.w[0], c1);
    mul_128x128_to_256!(c4, c1, c2); // c4.w[3] = 0
    // if c4 < 10^(q1+q2-1) then q4 = q1 + q2 - 1 else q4 = q1 + q2
    if c4.w[3] < bid_ten2k256!(q1 + q2 - 40).w[3]
      || (c4.w[3] == bid_ten2k256!(q1 + q2 - 40).w[3]
        && (c4.w[2] < bid_ten2k256!(q1 + q2 - 40).w[2]
          || (c4.w[2] == bid_ten2k256!(q1 + q2 - 40).w[2] && (c4.w[1] < bid_ten2k256!(q1 + q2 - 40).w[1] || (c4.w[1] == bid_ten2k256!(q1 + q2 - 40).w[1] && c4.w[0] < bid_ten2k256!(q1 + q2 - 40).w[0])))))
    {
      // if (c4.w[3] == 0) // q4 = 58, necessarily
      //   length of c1 * c2 rounded up to a multiple of 64 bits is len = 192;
      // else
      //   length of c1 * c2 rounded up to a multiple of 64 bits is len = 256;
      q4 = q1 + q2 - 1; // q4 in [58, 67]
    } else {
      // length of c1 * c2 rounded up to a multiple of 64 bits is len = 256;
      q4 = q1 + q2; // q4 in [59, 68]
    }
  }

  if c3.w[1] == 0 && c3.w[0] == 0 {
    // x = f, y = f, z = 0
    let save_fpsf: IdecFlags = *flags; // sticky bits - caller value must be preserved
    *flags = 0;

    if q4 > P34 {
      // truncate c4 to P34 digits into res
      // x = q4-P34, 1 <= x <= 34 because 35 <= q4 <= 68
      x0 = q4 - P34;
      if q4 <= 38 {
        p128.w[1] = c4.w[1];
        p128.w[0] = c4.w[0];
        bid_round128_19_38(q4, x0, p128, &mut res, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
      } else if q4 <= 57 {
        // 35 <= q4 <= 57
        p192.w[2] = c4.w[2];
        p192.w[1] = c4.w[1];
        p192.w[0] = c4.w[0];
        bid_round192_39_57(q4, x0, p192, &mut r192, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
        res.w[0] = r192.w[0];
        res.w[1] = r192.w[1];
      } else {
        // q4 <= 68
        bid_round256_58_76(q4, x0, c4, &mut r256, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
        res.w[0] = r256.w[0];
        res.w[1] = r256.w[1];
      }
      inc!(e4, x0);
      q4 = P34;
      if incr_exp > 0 {
        inc!(e4);
        if cfg!(feature = "decimal-tiny-detection-after-rounding") {
          if q4 + e4 == EXPMIN + P34 {
            set_status_flags!(flags, BID_INEXACT_EXCEPTION | BID_UNDERFLOW_EXCEPTION);
          }
        } else {
          //
        }
      }
      // res is now the coefficient of the result rounded to the destination
      // precision, with unbounded exponent; the exponent is e4; q4=digits(res)
    } else {
      // if (q4 <= P34)
      // c4 * 10^e4 is the result rounded to the destination precision, with
      // unbounded exponent (which is exact)

      if (q4 + e4 <= P34 + EXPMAX) && (e4 > EXPMAX) {
        // e4 is too large, but can be brought within range by scaling up c4
        scale = e4 - EXPMAX; // 1 <= scale < P-q4 <= P-1 => 1 <= scale <= P-2
        // res = (c4 * 10^scale) * 10^EXPMAX
        if q4 <= 19 {
          // c4 fits in 64 bits
          if scale <= 19 {
            // 10^scale fits in 64 bits
            // 64 x 64 c4.w[0] * bid_ten2k64!(scale]
            mul_64x64_to_128mach!(res, c4.w[0], bid_ten2k64!(scale));
          } else {
            // 10^scale fits in 128 bits
            // 64 x 128 c4.w[0] * bid_ten2k128!(scale - 20]
            mul_128x64_to_128!(res, c4.w[0], bid_ten2k128!(scale - 20));
          }
        } else {
          // c4 fits in 128 bits, but 10^scale must fit in 64 bits
          // 64 x 128 bid_ten2k64!(scale] * CC43
          mul_128x64_to_128!(res, bid_ten2k64!(scale), c4);
        }
        dec!(e4, scale); // EXPMAX
        inc!(q4, scale);
      } else {
        res.w[1] = c4.w[1];
        res.w[0] = c4.w[0];
      }
      // 'res' is the coefficient of the result rounded to the destination precision,
      // with unbounded exponent (it has q4 digits); the exponent is e4 (exact result).
    }

    // check for overflow
    if q4 + e4 > P34 + EXPMAX {
      if rounding == BID_ROUNDING_TO_NEAREST {
        res.w[1] = p_sign | 0x7800000000000000; // +/-inf
        res.w[0] = 0x0000000000000000;
        set_status_flags!(flags, BID_INEXACT_EXCEPTION | BID_OVERFLOW_EXCEPTION);
      } else {
        res.w[1] |= p_sign;
        bid_rounding_correction(rounding, is_inexact_lt_midpoint, is_inexact_gt_midpoint, is_midpoint_lt_even, is_midpoint_gt_even, e4, &mut res, flags);
      }
      *flags |= save_fpsf;
      *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
      *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
      *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
      *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;
      return res;
    }
    // check for underflow
    if q4 + e4 < EXPMIN + P34 {
      // (good also for most cases if 'before rounding')
      if e4 < EXPMIN {
        // if e4 < EXPMIN, we must truncate more of res
        x0 = EXPMIN - e4; // x0 >= 1
        is_inexact_lt_midpoint0 = is_inexact_lt_midpoint;
        is_inexact_gt_midpoint0 = is_inexact_gt_midpoint;
        is_midpoint_lt_even0 = is_midpoint_lt_even;
        is_midpoint_gt_even0 = is_midpoint_gt_even;
        is_inexact_lt_midpoint = false;
        is_inexact_gt_midpoint = false;
        is_midpoint_lt_even = false;
        is_midpoint_gt_even = false;
        // the number of decimal digits in res is q4
        if x0 < q4 {
          // 1 <= x0 <= q4-1 => round res to q4 - x0 digits
          if q4 <= 18 {
            // 2 <= q4 <= 18, 1 <= x0 <= 17
            bid_round64_2_18(q4, x0, res.w[0], &mut r64, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
            if incr_exp > 0 {
              // r64 = 10^(q4-x0), 1 <= q4 - x0 <= q4 - 1, 1 <= q4 - x0 <= 17
              r64 = bid_ten2k64!(q4 - x0);
            }
            // res.w[1] = 0; (from above)
            res.w[0] = r64;
          } else {
            // if (q4 <= 34)
            // 19 <= q4 <= 38
            p128.w[1] = res.w[1];
            p128.w[0] = res.w[0];
            bid_round128_19_38(q4, x0, p128, &mut res, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
            if incr_exp > 0 {
              // increase coefficient by a factor of 10; this will be <= 10^33
              // r128 = 10^(q4-x0), 1 <= q4 - x0 <= q4 - 1, 1 <= q4 - x0 <= 37
              if q4 - x0 <= 19 {
                // 1 <= q4 - x0 <= 19
                // res.w[1] = 0;
                res.w[0] = bid_ten2k64!(q4 - x0);
              } else {
                // 20 <= q4 - x0 <= 37
                res.w[0] = bid_ten2k128!(q4 - x0 - 20).w[0];
                res.w[1] = bid_ten2k128!(q4 - x0 - 20).w[1];
              }
            }
          }
          inc!(e4, x0); // EXPMIN
        } else if x0 == q4 {
          // the second rounding is for 0.d(0)d(1)...d(q4-1) * 10^emin
          // determine relationship with 1/2 ulp
          if q4 <= 19 {
            if res.w[0] < bid_midpoint64!(q4 - 1) {
              // < 1/2 ulp
              lt_half_ulp = true;
              is_inexact_lt_midpoint = true;
            } else if res.w[0] == bid_midpoint64!(q4 - 1) {
              // = 1/2 ulp
              eq_half_ulp = true;
              is_midpoint_gt_even = true;
            } else {
              // > 1/2 ulp
              // gt_half_ulp = 1;
              is_inexact_gt_midpoint = true;
            }
          } else {
            // if (q4 <= 34)
            if res.w[1] < bid_midpoint128!(q4 - 20).w[1] || (res.w[1] == bid_midpoint128!(q4 - 20).w[1] && res.w[0] < bid_midpoint128!(q4 - 20).w[0]) {
              // < 1/2 ulp
              lt_half_ulp = true;
              is_inexact_lt_midpoint = true;
            } else if res.w[1] == bid_midpoint128!(q4 - 20).w[1] && res.w[0] == bid_midpoint128!(q4 - 20).w[0] {
              // = 1/2 ulp
              eq_half_ulp = true;
              is_midpoint_gt_even = true;
            } else {
              // > 1/2 ulp
              // gt_half_ulp = 1;
              is_inexact_gt_midpoint = true;
            }
          }
          if lt_half_ulp || eq_half_ulp {
            // res = +0.0 * 10^EXPMIN
            res.w[1] = 0x0000000000000000;
            res.w[0] = 0x0000000000000000;
          } else {
            // if (gt_half_ulp)
            // res = +1 * 10^EXPMIN
            res.w[1] = 0x0000000000000000;
            res.w[0] = 0x0000000000000001;
          }
          e4 = EXPMIN;
        } else {
          // if (x0 > q4)
          // the second rounding is for 0.0...d(0)d(1)...d(q4-1) * 10^emin
          res.w[1] = 0;
          res.w[0] = 0;
          e4 = EXPMIN;
          is_inexact_lt_midpoint = true;
        }
        // avoid a double rounding error
        if (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) && is_midpoint_lt_even {
          // double rounding error upward
          // res = res - 1
          res.w[0] = res.w[0].wrapping_sub(1);
          if res.w[0] == 0xffffffffffffffff {
            res.w[1] = res.w[1].wrapping_sub(1);
          }
          // Note: a double rounding error upward is not possible; for this
          // the result after the first rounding would have to be 99...95
          // (35 digits in all), possibly followed by a number of zeros; this
          // not possible for f * f + 0
          is_midpoint_lt_even = false;
          is_inexact_lt_midpoint = true;
        } else if (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) && is_midpoint_gt_even {
          // double rounding error downward
          // res = res + 1
          inc!(res.w[0]);
          if res.w[0] == 0 {
            inc!(res.w[1]);
          }
          is_midpoint_gt_even = false;
          is_inexact_gt_midpoint = true;
        } else if !is_midpoint_lt_even && !is_midpoint_gt_even && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint {
          // if this second rounding was exact the result may still be
          // inexact because of the first rounding
          if is_inexact_gt_midpoint0 || is_midpoint_lt_even0 {
            is_inexact_gt_midpoint = true;
          }
          if is_inexact_lt_midpoint0 || is_midpoint_gt_even0 {
            is_inexact_lt_midpoint = true;
          }
        } else if is_midpoint_gt_even && (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) {
          // pulled up to a midpoint
          is_inexact_lt_midpoint = true;
          is_inexact_gt_midpoint = false;
          is_midpoint_lt_even = false;
          is_midpoint_gt_even = false;
        } else if is_midpoint_lt_even && (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) {
          // pulled down to a midpoint
          is_inexact_lt_midpoint = false;
          is_inexact_gt_midpoint = true;
          is_midpoint_lt_even = false;
          is_midpoint_gt_even = false;
        }
      } else {
        // if e4 >= emin then q4 < P and the result is tiny and exact
        if e3 < e4 {
          // if (e3 < e4) the preferred exponent is e3
          // return (c4 * 10^scale) * 10^(e4 - scale)
          // where scale = min (P34-q4, (e4 - e3))
          scale = P34 - q4;
          ind = e4 - e3;
          if ind < scale {
            scale = ind;
          }
          if scale == 0 { // res and e4 are unchanged
          } else if q4 <= 19 {
            // c4 fits in 64 bits
            if scale <= 19 {
              // 10^scale fits in 64 bits
              // 64 x 64 res.w[0] * bid_ten2k64!(scale]
              mul_64x64_to_128mach!(res, res.w[0], bid_ten2k64!(scale));
            } else {
              // 10^scale fits in 128 bits
              // 64 x 128 res.w[0] * bid_ten2k128!(scale - 20]
              mul_128x64_to_128!(res, res.w[0], bid_ten2k128!(scale - 20));
            }
          } else {
            // res fits in 128 bits, but 10^scale must fit in 64 bits
            // 64 x 128 bid_ten2k64!(scale] * c3
            mul_128x64_to_128!(res, bid_ten2k64!(scale), res);
          }
          // subtract scale from the exponent
          dec!(e4, scale);
        }
      }

      // check for inexact result
      if is_inexact_lt_midpoint || is_inexact_gt_midpoint || is_midpoint_lt_even || is_midpoint_gt_even {
        // set the inexact flag and the underflow flag
        set_status_flags!(flags, BID_INEXACT_EXCEPTION | BID_UNDERFLOW_EXCEPTION);
      }
      res.w[1] |= p_sign | (e4.wrapping_add(6176) as u64) << 49;
      if rounding != BID_ROUNDING_TO_NEAREST {
        bid_rounding_correction(rounding, is_inexact_lt_midpoint, is_inexact_gt_midpoint, is_midpoint_lt_even, is_midpoint_gt_even, e4, &mut res, flags);
      }
      *flags |= save_fpsf;
      *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
      *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
      *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
      *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

      return res;
    }
    // no overflow, and no underflow for rounding to nearest
    // (although if tininess is detected 'before rounding', we may
    // get here if incr_exp = 1 and then q4 + e4 == EXPMIN + P34)
    res.w[1] |= p_sign | (e4.wrapping_add(6176) as u64) << 49;

    if rounding != BID_ROUNDING_TO_NEAREST {
      bid_rounding_correction(rounding, is_inexact_lt_midpoint, is_inexact_gt_midpoint, is_midpoint_lt_even, is_midpoint_gt_even, e4, &mut res, flags);
      // if e4 = EXPMIN && significand < 10^33 => result is tiny (for RD, RZ)
      if e4 == EXPMIN && ((res.w[1] & MASK_COEFF) < 0x0000314dc6448d93 || ((res.w[1] & MASK_COEFF) == 0x0000314dc6448d93 && res.w[0] < 0x38c15b0a00000000)) {
        is_tiny = true;
      }
    }

    if is_inexact_lt_midpoint || is_inexact_gt_midpoint || is_midpoint_lt_even || is_midpoint_gt_even {
      // set the inexact flag
      *flags |= BID_INEXACT_EXCEPTION;
      if is_tiny {
        *flags |= BID_UNDERFLOW_EXCEPTION;
      }
    }

    if (*flags & BID_INEXACT_EXCEPTION) == 0 {
      // x * y is exact
      // need to ensure that the result has the preferred exponent
      p_exp = res.w[1] & MASK_EXP;
      if z_exp < p_exp {
        // the preferred exponent is z_exp
        // signficand of res in c3
        c3.w[1] = res.w[1] & MASK_COEFF;
        c3.w[0] = res.w[0];
        // the number of decimal digits of x * y is q4 <= 34
        // Note: the coefficient fits in 128 bits

        // return (c3 * 10^scale) * 10^(p_exp - scale)
        // where scale = min (P34-q4, (p_exp-z_exp) >> 49)
        scale = P34 - q4;
        ind = ((p_exp - z_exp) >> 49) as i32;
        if ind < scale {
          scale = ind;
        }
        // subtract scale from the exponent
        p_exp = p_exp.wrapping_sub((scale as u64) << 49);
        if scale == 0 {
          // leave res unchanged
        } else if q4 <= 19 {
          // x * y fits in 64 bits
          if scale <= 19 {
            // 10^scale fits in 64 bits
            // 64 x 64 c3.w[0] * bid_ten2k64!(scale]
            mul_64x64_to_128mach!(res, c3.w[0], bid_ten2k64!(scale));
          } else {
            // 10^scale fits in 128 bits
            // 64 x 128 c3.w[0] * bid_ten2k128!(scale - 20]
            mul_128x64_to_128!(res, c3.w[0], bid_ten2k128!(scale - 20));
          }
          res.w[1] |= p_sign | (p_exp & MASK_EXP);
        } else {
          // x * y fits in 128 bits, but 10^scale must fit in 64 bits
          // 64 x 128 bid_ten2k64!(scale] * c3
          mul_128x64_to_128!(res, bid_ten2k64!(scale), c3);
          res.w[1] |= p_sign | (p_exp & MASK_EXP);
        }
      } // else leave the result as it is, because p_exp <= z_exp
    }
    *flags |= save_fpsf;
    *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
    *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
    *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
    *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

    return res;
  } // else we have f * f + f

  // continue with x = f, y = f, z = f

  delta = q3 + e3 - q4 - e4;

  'delta_ge_zero: loop {
    if delta >= 0 {
      return if P34 < delta || (P34 == delta && e3 + 6176 < P34 - q3)
      // Case (1')
      {
        // Case (1''A)
        // check for overflow, which can occur only in Case (1')
        if (q3 + e3) > (P34 + EXPMAX) && P34 < delta {
          // e3 > EXPMAX implies P34 <= delta-1 and e3 > EXPMAX is a necessary
          // condition for (q3 + e3) > (P34 + EXPMAX)
          if rounding == BID_ROUNDING_TO_NEAREST {
            res.w[1] = z_sign | 0x7800000000000000; // +/-inf
            res.w[0] = 0x0000000000000000;
            set_status_flags!(flags, BID_INEXACT_EXCEPTION | BID_OVERFLOW_EXCEPTION);
          } else {
            if p_sign == z_sign {
              is_inexact_lt_midpoint = true;
            } else {
              is_inexact_gt_midpoint = true;
            }
            // q3 <= P34; if (q3 < P34) scale c3 up by 10^(P34-q3)
            scale = P34 - q3;
            if scale == 0 {
              res.w[1] = z_sign | c3.w[1];
              res.w[0] = c3.w[0];
            } else if q3 <= 19 {
              // c3 fits in 64 bits
              if scale <= 19 {
                // 10^scale fits in 64 bits
                // 64 x 64 c3.w[0] * bid_ten2k64!(scale]
                mul_64x64_to_128mach!(res, c3.w[0], bid_ten2k64!(scale));
              } else {
                // 10^scale fits in 128 bits
                // 64 x 128 c3.w[0] * bid_ten2k128!(scale - 20]
                mul_128x64_to_128!(res, c3.w[0], bid_ten2k128!(scale - 20));
              }
            } else {
              // c3 fits in 128 bits, but 10^scale must fit in 64 bits
              // 64 x 128 bid_ten2k64!(scale] * c3
              mul_128x64_to_128!(res, bid_ten2k64!(scale), c3);
            }
            dec!(e3, scale);
            res.w[1] |= z_sign;
            bid_rounding_correction(rounding, is_inexact_lt_midpoint, is_inexact_gt_midpoint, is_midpoint_lt_even, is_midpoint_gt_even, e3, &mut res, flags);
          }
          *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
          *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
          *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
          *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

          return res;
        }
        // res = z
        if q3 < P34 {
          // the preferred exponent is z_exp - (P34 - q3)
          // return (c3 * 10^scale) * 10^(z_exp - scale)
          // where scale = min (P34-q3, z_exp-EMIN)
          scale = P34 - q3;
          ind = e3 + 6176;
          if ind < scale {
            scale = ind;
          }
          if scale == 0 {
            res.w[1] = c3.w[1];
            res.w[0] = c3.w[0];
          } else if q3 <= 19 {
            // z fits in 64 bits
            if scale <= 19 {
              // 10^scale fits in 64 bits
              // 64 x 64 c3.w[0] * bid_ten2k64!(scale]
              mul_64x64_to_128mach!(res, c3.w[0], bid_ten2k64!(scale));
            } else {
              // 10^scale fits in 128 bits
              // 64 x 128 c3.w[0] * bid_ten2k128!(scale - 20]
              mul_128x64_to_128!(res, c3.w[0], bid_ten2k128!(scale - 20));
            }
          } else {
            // z fits in 128 bits, but 10^scale must fit in 64 bits
            // 64 x 128 bid_ten2k64!(scale] * c3
            mul_128x64_to_128!(res, bid_ten2k64!(scale), c3);
          }
          // the coefficient in res has q3 + scale digits
          // subtract scale from the exponent
          dec!(z_exp, (scale as u64) << 49);
          dec!(e3, scale);
          res.w[1] |= z_sign | (z_exp & MASK_EXP);
          if scale + q3 < P34 {
            set_status_flags!(flags, BID_UNDERFLOW_EXCEPTION); // OK for tininess detection
          }
          // before or after rounding, because the exponent of the
          // rounded result with unbounded exponent does not change
          // due to rounding overflow
        } else {
          // if q3 = P34
          scale = 0;
          res.w[1] = z_sign | (e3.wrapping_add(6176) as u64) << 49 | c3.w[1];
          res.w[0] = c3.w[0];
        }

        // use the following to avoid double rounding errors when operating on
        // mixed formats in rounding to nearest, and for correcting the result
        // if not rounding to nearest
        if (p_sign != z_sign) && (delta == (q3 + scale + 1)) {
          // there is a gap of exactly one digit between the scaled c3 and c4
          // c3 * 10^ scale = 10^(q3+scale-1) <=> c3 = 10^(q3-1) is a special case
          if (q3 <= 19 && c3.w[0] != bid_ten2k64![q3 - 1]) || (q3 == 20 && (c3.w[1] != 0 || c3.w[0] != bid_ten2k64![19])) || (q3 >= 21 && (c3.w[1] != bid_ten2k128![q3 - 21].w[1] || c3.w[0] != bid_ten2k128![q3 - 21].w[0])) {
            // c3 * 10^ scale != 10^(q3-1)
            // if ((res.w[1] & MASK_COEFF) != 0x0000314dc6448d93ull ||
            // res.w[0] != 0x38c15b0a00000000ull) { // c3 * 10^scale != 10^33
            is_inexact_gt_midpoint = true; // if (z_sign), set as if for abs. value
          } else {
            // if c3 * 10^scale = 10^(q3+scale-1)
            // ok from above e3 = (z_exp >> 49) - 6176;
            // the result is always inexact
            if q4 == 1 {
              r64 = c4.w[0];
            } else {
              // if q4 > 1 then truncate c4 from q4 digits to 1 digit;
              // x = q4-1, 1 <= x <= 67 and check if this operation is exact
              if q4 <= 18 {
                // 2 <= q4 <= 18
                bid_round64_2_18(q4, q4 - 1, c4.w[0], &mut r64, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
              } else if q4 <= 38 {
                p128.w[1] = c4.w[1];
                p128.w[0] = c4.w[0];
                bid_round128_19_38(q4, q4 - 1, p128, &mut r128, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
                r64 = r128.w[0]; // one decimal digit
              } else if q4 <= 57 {
                p192.w[2] = c4.w[2];
                p192.w[1] = c4.w[1];
                p192.w[0] = c4.w[0];
                bid_round192_39_57(q4, q4 - 1, p192, &mut r192, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
                r64 = r192.w[0]; // one decimal digit
              } else {
                // if (q4 <= 68)
                bid_round256_58_76(q4, q4 - 1, c4, &mut r256, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
                r64 = r256.w[0]; // one decimal digit
              }
              if incr_exp > 0 {
                r64 = 10;
              }
            }
            if r64 == 5 && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint && !is_midpoint_lt_even && !is_midpoint_gt_even {
              is_inexact_lt_midpoint = false;
              is_inexact_gt_midpoint = false;
              is_midpoint_lt_even = true;
              is_midpoint_gt_even = false;
            } else if (e3 == EXPMIN) || r64 < 5 || (r64 == 5 && is_inexact_gt_midpoint) {
              // result does not change
              is_inexact_lt_midpoint = false;
              is_inexact_gt_midpoint = true;
              is_midpoint_lt_even = false;
              is_midpoint_gt_even = false;
            } else {
              is_inexact_lt_midpoint = true;
              is_inexact_gt_midpoint = false;
              is_midpoint_lt_even = false;
              is_midpoint_gt_even = false;
              // result decremented is 10^(q3+scale) - 1
              if (q3 + scale) <= 19 {
                res.w[1] = 0;
                res.w[0] = bid_ten2k64!(q3 + scale);
              } else {
                // if ((q3 + scale + 1) <= 35)
                res.w[1] = bid_ten2k128!(q3 + scale - 20).w[1];
                res.w[0] = bid_ten2k128!(q3 + scale - 20).w[0];
              }
              dec!(res.w[0]); // borrow never occurs
              dec!(e3);
              res.w[1] |= z_sign | (e3.wrapping_add(6176) as u64) << 49;
            }
            if e3 == EXPMIN {
              if cfg!(feature = "decimal-tiny-detection-after-rounding") {
                if r64 < 5 || (r64 == 5 && !is_inexact_lt_midpoint) {
                  // result not tiny (in round-to-nearest mode)
                  // rounds to 10^33 * 10^emin
                } else {
                  set_status_flags!(flags, BID_UNDERFLOW_EXCEPTION);
                }
              } else {
                set_status_flags!(flags, BID_UNDERFLOW_EXCEPTION); // tiny if detected before rounding
              }
            }
          } // end 10^(q3+scale-1)
          // set the inexact flag
          *flags |= BID_INEXACT_EXCEPTION;
        } else {
          if p_sign == z_sign {
            // if (z_sign), set as if for absolute value
            is_inexact_lt_midpoint = true;
          } else {
            // if (p_sign != z_sign)
            // if (z_sign), set as if for absolute value
            is_inexact_gt_midpoint = true;
          }
          *flags |= BID_INEXACT_EXCEPTION;
        }
        // the result is always inexact => set the inexact flag
        // Determine tininess:
        //    if (exp > EXPMIN)
        //      the result is not tiny
        //    else // if exp = emin
        //      if (q3 + scale < P34)
        //        the result is tiny
        //      else // if (q3 + scale = P34)
        //        if (c3 * 10^scale > 10^33)
        //          the result is not tiny
        //        else // if c3 * 10^scale = 10^33
        //          if (xy * z > 0)
        //            the result is not tiny
        //          else // if (xy * z < 0)
        //            if (rounding = RN || rounding = RA) and (delta = P+1) and
        //                c4 > 5 * 10^(q4-1)
        //              the result is tiny
        //            else
        //              the result is not tiny
        //          endif
        //        endif
        //      endif
        //    endif

        if cfg!(feature = "decimal-tiny-detection-after-rounding") {
          // determine if c4 > 5 * 10^(q4-1)
          if q4 <= 19 {
            c4gt5toq4m1 = c4.w[0] > bid_midpoint64!(q4 - 1);
          } else if q4 <= 38 {
            c4gt5toq4m1 = c4.w[1] > bid_midpoint128!(q4 - 20).w[1] || (c4.w[1] == bid_midpoint128!(q4 - 20).w[1] && c4.w[0] > bid_midpoint128!(q4 - 20).w[0]);
          } else if q4 <= 58 {
            c4gt5toq4m1 = c4.w[2] > bid_midpoint192!(q4 - 39).w[2]
              || (c4.w[2] == bid_midpoint192!(q4 - 39).w[2] && c4.w[1] > bid_midpoint192!(q4 - 39).w[1])
              || (c4.w[2] == bid_midpoint192!(q4 - 39).w[2] && c4.w[1] == bid_midpoint192!(q4 - 39).w[1] && c4.w[0] > bid_midpoint192!(q4 - 39).w[0]);
          } else {
            // if (q4 <= 68)
            c4gt5toq4m1 = c4.w[3] > bid_midpoint256![q4 - 59].w[3]
              || (c4.w[3] == bid_midpoint256![q4 - 59].w[3] && c4.w[2] > bid_midpoint256![q4 - 59].w[2])
              || (c4.w[3] == bid_midpoint256![q4 - 59].w[3] && c4.w[2] == bid_midpoint256![q4 - 59].w[2] && c4.w[1] > bid_midpoint256![q4 - 59].w[1])
              || (c4.w[3] == bid_midpoint256![q4 - 59].w[3] && c4.w[2] == bid_midpoint256![q4 - 59].w[2] && c4.w[1] == bid_midpoint256![q4 - 59].w[1] && c4.w[0] > bid_midpoint256![q4 - 59].w[0]);
          }

          if (e3 == EXPMIN && (q3 + scale) < P34)
            || (e3 == EXPMIN && (q3 + scale) == P34 &&
            (res.w[1] & MASK_COEFF) == 0x0000314dc6448d93 &&  // 10^33_high
            res.w[0] == 0x38c15b0a00000000 &&  // 10^33_low
            z_sign != p_sign &&
            (rounding == BID_ROUNDING_TO_NEAREST || rounding == BID_ROUNDING_TIES_AWAY) &&
            (delta == (P34 + 1)) && c4gt5toq4m1)
          {
            *flags |= BID_UNDERFLOW_EXCEPTION;
          }
        } else if (e3 == EXPMIN && (q3 + scale) < P34)
          || (e3 == EXPMIN && (q3 + scale) == P34 &&
            (res.w[1] & MASK_COEFF) == 0x0000314dc6448d93 &&  // 10^33_high
            res.w[0] == 0x38c15b0a00000000 &&  // 10^33_low
            z_sign != p_sign)
        {
          *flags |= BID_UNDERFLOW_EXCEPTION; // for all rounding modes
        }

        if rounding != BID_ROUNDING_TO_NEAREST {
          bid_rounding_correction(rounding, is_inexact_lt_midpoint, is_inexact_gt_midpoint, is_midpoint_lt_even, is_midpoint_gt_even, e3, &mut res, flags);
        }
        *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
        *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
        *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
        *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

        res
      } else if P34 == delta {
        // Case (1''B)

        // because Case (1''A) was treated above, e3 + 6176 >= P34 - q3
        // and c3 can be scaled up to P34 digits if needed

        // scale c3 to P34 digits if needed
        scale = P34 - q3; // 0 <= scale <= P34 - 1
        if scale == 0 {
          res.w[1] = c3.w[1];
          res.w[0] = c3.w[0];
        } else if q3 <= 19 {
          // z fits in 64 bits
          if scale <= 19 {
            // 10^scale fits in 64 bits
            // 64 x 64 c3.w[0] * bid_ten2k64!(scale]
            mul_64x64_to_128mach!(res, c3.w[0], bid_ten2k64!(scale));
          } else {
            // 10^scale fits in 128 bits
            // 64 x 128 c3.w[0] * bid_ten2k128!(scale - 20]
            mul_128x64_to_128!(res, c3.w[0], bid_ten2k128!(scale - 20));
          }
        } else {
          // z fits in 128 bits, but 10^scale must fit in 64 bits
          // 64 x 128 bid_ten2k64!(scale] * c3
          mul_128x64_to_128!(res, bid_ten2k64!(scale), c3);
        }
        // subtract scale from the exponent
        dec!(z_exp, (scale as u64) << 49);
        dec!(e3, scale);
        // now z_sign, z_exp, and res correspond to a z scaled to P34 = 34 digits

        // determine whether x * y is less than, equal to, or greater than
        // 1/2 ulp (z)
        if q4 <= 19 {
          if c4.w[0] < bid_midpoint64!(q4 - 1) {
            // < 1/2 ulp
            lt_half_ulp = true;
          } else if c4.w[0] == bid_midpoint64!(q4 - 1) {
            // = 1/2 ulp
            eq_half_ulp = true;
          } else {
            // > 1/2 ulp
            gt_half_ulp = true;
          }
        } else if q4 <= 38 {
          if c4.w[2] == 0 && (c4.w[1] < bid_midpoint128!(q4 - 20).w[1] || (c4.w[1] == bid_midpoint128!(q4 - 20).w[1] && c4.w[0] < bid_midpoint128!(q4 - 20).w[0])) {
            // < 1/2 ulp
            lt_half_ulp = true;
          } else if c4.w[2] == 0 && c4.w[1] == bid_midpoint128!(q4 - 20).w[1] && c4.w[0] == bid_midpoint128!(q4 - 20).w[0] {
            // = 1/2 ulp
            eq_half_ulp = true;
          } else {
            // > 1/2 ulp
            gt_half_ulp = true;
          }
        } else if q4 <= 58 {
          if c4.w[3] == 0
            && (c4.w[2] < bid_midpoint192!(q4 - 39).w[2]
              || (c4.w[2] == bid_midpoint192!(q4 - 39).w[2] && c4.w[1] < bid_midpoint192!(q4 - 39).w[1])
              || (c4.w[2] == bid_midpoint192!(q4 - 39).w[2] && c4.w[1] == bid_midpoint192!(q4 - 39).w[1] && c4.w[0] < bid_midpoint192!(q4 - 39).w[0]))
          {
            // < 1/2 ulp
            lt_half_ulp = true;
          } else if c4.w[3] == 0 && c4.w[2] == bid_midpoint192![q4 - 39].w[2] && c4.w[1] == bid_midpoint192!(q4 - 39).w[1] && c4.w[0] == bid_midpoint192!(q4 - 39).w[0] {
            // = 1/2 ulp
            eq_half_ulp = true;
          } else {
            // > 1/2 ulp
            gt_half_ulp = true;
          }
        } else if c4.w[3] < bid_midpoint256![q4 - 59].w[3]
          || (c4.w[3] == bid_midpoint256![q4 - 59].w[3] && c4.w[2] < bid_midpoint256![q4 - 59].w[2])
          || (c4.w[3] == bid_midpoint256![q4 - 59].w[3] && c4.w[2] == bid_midpoint256![q4 - 59].w[2] && c4.w[1] < bid_midpoint256![q4 - 59].w[1])
          || (c4.w[3] == bid_midpoint256![q4 - 59].w[3] && c4.w[2] == bid_midpoint256![q4 - 59].w[2] && c4.w[1] == bid_midpoint256![q4 - 59].w[1] && c4.w[0] < bid_midpoint256![q4 - 59].w[0])
        {
          // < 1/2 ulp
          lt_half_ulp = true;
        } else if c4.w[3] == bid_midpoint256![q4 - 59].w[3] && c4.w[2] == bid_midpoint256![q4 - 59].w[2] && c4.w[1] == bid_midpoint256![q4 - 59].w[1] && c4.w[0] == bid_midpoint256![q4 - 59].w[0] {
          // = 1/2 ulp
          eq_half_ulp = true;
        } else {
          // > 1/2 ulp
          gt_half_ulp = true;
        }

        if p_sign == z_sign {
          if lt_half_ulp {
            res.w[1] |= z_sign | (z_exp & MASK_EXP);
            // use the following to avoid double rounding errors when operating on
            // mixed formats in rounding to nearest
            is_inexact_lt_midpoint = true; // if (z_sign), as if for absolute value
          } else if (eq_half_ulp && (res.w[0] & 0x01 > 0)) || gt_half_ulp {
            // add 1 ulp to the significand
            inc!(res.w[0]);
            if res.w[0] == 0 {
              inc!(res.w[1]);
            }
            // check for rounding overflow, when coeff == 10^34
            if (res.w[1] & MASK_COEFF) == 0x0001ed09bead87c0 && res.w[0] == 0x378d8e6400000000 {
              // coefficient = 10^34
              inc!(e3);
              // coeff = 10^33
              z_exp = ((e3.wrapping_add(6176) as u64) << 49) & MASK_EXP;
              res.w[1] = 0x0000314dc6448d93;
              res.w[0] = 0x38c15b0a00000000;
            }
            // end add 1 ulp
            res.w[1] |= z_sign | (z_exp & MASK_EXP);
            if eq_half_ulp {
              is_midpoint_lt_even = true; // if (z_sign), as if for absolute value
            } else {
              is_inexact_gt_midpoint = true; // if (z_sign), as if for absolute value
            }
          } else {
            // if (eq_half_ulp && !(res.w[0] & 0x01))
            // leave unchanged
            res.w[1] |= z_sign | (z_exp & MASK_EXP);
            is_midpoint_gt_even = true; // if (z_sign), as if for absolute value
          }
          // the result is always inexact, and never tiny
          // set the inexact flag
          *flags |= BID_INEXACT_EXCEPTION;
          // check for overflow
          if e3 > EXPMAX && rounding == BID_ROUNDING_TO_NEAREST {
            res.w[1] = z_sign | 0x7800000000000000; // +/-inf
            res.w[0] = 0x0000000000000000;
            set_status_flags!(flags, BID_INEXACT_EXCEPTION | BID_OVERFLOW_EXCEPTION);
            *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
            *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
            *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
            *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

            return res;
          }
          if rounding != BID_ROUNDING_TO_NEAREST {
            bid_rounding_correction(rounding, is_inexact_lt_midpoint, is_inexact_gt_midpoint, is_midpoint_lt_even, is_midpoint_gt_even, e3, &mut res, flags);
            z_exp = res.w[1] & MASK_EXP;
          }
        } else {
          // if (p_sign != z_sign)
          // consider two cases, because c3 * 10^scale = 10^33 is a special case
          if res.w[1] != 0x0000314dc6448d93 || res.w[0] != 0x38c15b0a00000000 {
            // c3 * 10^scale != 10^33
            if lt_half_ulp {
              res.w[1] |= z_sign | (z_exp & MASK_EXP);
              // use the following to avoid double rounding errors when operating
              // on mixed formats in rounding to nearest
              is_inexact_gt_midpoint = true; // if (z_sign), as if for absolute value
            } else if (eq_half_ulp && (res.w[0] & 0x01 > 0)) || gt_half_ulp {
              // subtract 1 ulp from the significand
              dec!(res.w[0]);
              if res.w[0] == 0xffffffffffffffff {
                dec!(res.w[1]);
              }
              res.w[1] |= z_sign | (z_exp & MASK_EXP);
              if eq_half_ulp {
                is_midpoint_gt_even = true; // if (z_sign), as if for absolute value
              } else {
                is_inexact_lt_midpoint = true; //if(z_sign), as if for absolute value
              }
            } else {
              // if (eq_half_ulp && !(res.w[0] & 0x01))
              // leave unchanged
              res.w[1] |= z_sign | (z_exp & MASK_EXP);
              is_midpoint_lt_even = true; // if (z_sign), as if for absolute value
            }
            // the result is always inexact, and never tiny
            // check for overflow for RN
            if e3 > EXPMAX {
              if rounding == BID_ROUNDING_TO_NEAREST {
                res.w[1] = z_sign | 0x7800000000000000; // +/-inf
                res.w[0] = 0x0000000000000000;
                set_status_flags!(flags, BID_INEXACT_EXCEPTION | BID_OVERFLOW_EXCEPTION);
              } else {
                bid_rounding_correction(rounding, is_inexact_lt_midpoint, is_inexact_gt_midpoint, is_midpoint_lt_even, is_midpoint_gt_even, e3, &mut res, flags);
              }
              *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
              *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
              *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
              *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

              return res;
            }
            // set the inexact flag
            *flags |= BID_INEXACT_EXCEPTION;
            if rounding != BID_ROUNDING_TO_NEAREST {
              bid_rounding_correction(rounding, is_inexact_lt_midpoint, is_inexact_gt_midpoint, is_midpoint_lt_even, is_midpoint_gt_even, e3, &mut res, flags);
            }
            z_exp = res.w[1] & MASK_EXP;
          } else {
            // if c3 * 10^scale = 10^33
            e3 = (z_exp >> 49).wrapping_sub(6176) as i32;
            if e3 > EXPMIN {
              // the result is exact if exp > EXPMIN and c4 = d*10^(q4-1),
              // where d = 1, 2, 3, ..., 9; it could be tiny too, but exact
              if q4 == 1 {
                // if q4 = 1 the result is exact
                // result coefficient = 10^34 - c4
                res.w[1] = 0x0001ed09bead87c0;
                res.w[0] = 0x378d8e6400000000 - c4.w[0];
                dec!(z_exp, EXP_P1);
                res.w[1] |= z_sign | (z_exp & MASK_EXP);
              } else {
                // if q4 > 1 then truncate c4 from q4 digits to 1 digit;
                // x = q4-1, 1 <= x <= 67 and check if this operation is exact
                if q4 <= 18 {
                  // 2 <= q4 <= 18
                  bid_round64_2_18(q4, q4 - 1, c4.w[0], &mut r64, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
                } else if q4 <= 38 {
                  p128.w[1] = c4.w[1];
                  p128.w[0] = c4.w[0];
                  bid_round128_19_38(q4, q4 - 1, p128, &mut r128, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
                  r64 = r128.w[0]; // one decimal digit
                } else if q4 <= 57 {
                  p192.w[2] = c4.w[2];
                  p192.w[1] = c4.w[1];
                  p192.w[0] = c4.w[0];
                  bid_round192_39_57(q4, q4 - 1, p192, &mut r192, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
                  r64 = r192.w[0]; // one decimal digit
                } else {
                  // if (q4 <= 68)
                  bid_round256_58_76(q4, q4 - 1, c4, &mut r256, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
                  r64 = r256.w[0]; // one decimal digit
                }
                if !is_midpoint_lt_even && !is_midpoint_gt_even && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint {
                  // the result is exact: 10^34 - r64
                  // incr_exp = 0 with certainty
                  dec!(z_exp, EXP_P1);
                  res.w[1] = z_sign | (z_exp & MASK_EXP) | 0x0001ed09bead87c0;
                  res.w[0] = 0x378d8e6400000000 - r64;
                } else {
                  // We want r64 to be the top digit of c4, but we actually
                  // obtained (c4 * 10^(-q4+1))RN; a correction may be needed,
                  // because the top digit is (c4 * 10^(-q4+1))RZ
                  // however, if incr_exp = 1 then r64 = 10 with certainty
                  if incr_exp > 0 {
                    r64 = 10;
                  }
                  // the result is inexact as c4 has more than 1 significant digit
                  // and c3 * 10^scale = 10^33
                  // example of case that is treated here:
                  // 100...0 * 10^e3 - 0.41 * 10^e3 =
                  // 0999...9.59 * 10^e3 -> rounds to 99...96*10^(e3-1)
                  // note that (e3 > EXPMIN}
                  // in order to round, subtract r64 from 10^34 and then compare
                  // c4 - r64 * 10^(q4-1) with 1/2 ulp
                  // calculate 10^34 - r64
                  res.w[1] = 0x0001ed09bead87c0;
                  res.w[0] = 0x378d8e6400000000 - r64;
                  // calculate c4 - r64 * 10^(q4-1); this is a rare case and
                  // r64 is small, 1 <= r64 <= 9
                  dec!(e3);
                  if is_inexact_lt_midpoint {
                    is_inexact_lt_midpoint = false;
                    is_inexact_gt_midpoint = true;
                  } else if is_inexact_gt_midpoint {
                    is_inexact_gt_midpoint = false;
                    is_inexact_lt_midpoint = true;
                  } else if is_midpoint_lt_even {
                    is_midpoint_lt_even = false;
                    is_midpoint_gt_even = true;
                  } else if is_midpoint_gt_even {
                    is_midpoint_gt_even = false;
                    is_midpoint_lt_even = true;
                  }
                  // the result is always inexact, and never tiny
                  // check for overflow for RN
                  if e3 > EXPMAX {
                    if rounding == BID_ROUNDING_TO_NEAREST {
                      res.w[1] = z_sign | 0x7800000000000000; // +/-inf
                      res.w[0] = 0x0000000000000000;
                      set_status_flags!(flags, BID_INEXACT_EXCEPTION | BID_OVERFLOW_EXCEPTION);
                    } else {
                      bid_rounding_correction(rounding, is_inexact_lt_midpoint, is_inexact_gt_midpoint, is_midpoint_lt_even, is_midpoint_gt_even, e3, &mut res, flags);
                    }
                    *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
                    *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
                    *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
                    *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

                    return res;
                  }
                  // set the inexact flag
                  *flags |= BID_INEXACT_EXCEPTION;
                  res.w[1] |= z_sign | (e3.wrapping_add(6176) as u64) << 49;
                  if rounding != BID_ROUNDING_TO_NEAREST {
                    bid_rounding_correction(rounding, is_inexact_lt_midpoint, is_inexact_gt_midpoint, is_midpoint_lt_even, is_midpoint_gt_even, e3, &mut res, flags);
                  }
                  z_exp = res.w[1] & MASK_EXP;
                } // end result is inexact
              } // end q4 > 1
            } else {
              // if (e3 = emin)
              // if e3 = EXPMIN the result is also tiny (the condition for
              // tininess is c4 > 050...0 [q4 digits] which is met because
              // the msd of c4 is not zero)
              // the result is tiny and inexact in all rounding modes;
              // it is either 100...0 or 0999...9 (use lt_half_ulp, eq_half_ulp,
              // gt_half_ulp to calculate)
              // if (lt_half_ulp || eq_half_ulp) res = 10^33 stays unchanged

              // p_sign != z_sign so swap gt_half_ulp and lt_half_ulp
              if gt_half_ulp {
                // res = 10^33 - 1
                res.w[1] = 0x0000314dc6448d93;
                res.w[0] = 0x38c15b09ffffffff;
              } else {
                res.w[1] = 0x0000314dc6448d93;
                res.w[0] = 0x38c15b0a00000000;
              }
              res.w[1] |= z_sign | (z_exp & MASK_EXP);
              *flags |= BID_UNDERFLOW_EXCEPTION; // inexact is set later

              if eq_half_ulp {
                is_midpoint_lt_even = true; // if (z_sign), as if for absolute value
              } else if lt_half_ulp {
                is_inexact_gt_midpoint = true; //if(z_sign), as if for absolute value
              } else {
                // if (gt_half_ulp)
                is_inexact_lt_midpoint = true; //if(z_sign), as if for absolute value
              }

              if rounding != BID_ROUNDING_TO_NEAREST {
                bid_rounding_correction(rounding, is_inexact_lt_midpoint, is_inexact_gt_midpoint, is_midpoint_lt_even, is_midpoint_gt_even, e3, &mut res, flags);
                z_exp = res.w[1] & MASK_EXP;
              }
            } // end e3 = emin
            // set the inexact flag (if the result was not exact)
            if is_inexact_lt_midpoint || is_inexact_gt_midpoint || is_midpoint_lt_even || is_midpoint_gt_even {
              set_status_flags!(flags, BID_INEXACT_EXCEPTION);
            }
          } // end 10^33
        } // end if (p_sign != z_sign)
        res.w[1] |= z_sign | (z_exp & MASK_EXP);
        *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
        *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
        *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
        *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

        res
      } else if ((q3 <= delta && delta < P34 && P34 < delta + q4) || // Case (2)
        (q3 <= delta && delta + q4 <= P34) || // Case (3)
        (delta < q3 && P34 < delta + q4) || // Case (4)
        (delta < q3 && q3 <= delta + q4 && delta + q4 <= P34) || // Case (5)
        (delta + q4 < q3)) && // Case (6)
        !(delta <= 1 && p_sign != z_sign)
      {
        // Case (2), (3), (4), (5) or (6)

        // the result has the sign of z

        if (q3 <= delta && delta < P34 && P34 < delta + q4) || // Case (2)
          (delta < q3 && P34 < delta + q4)
        {
          // Case (4)
          // round first the sum x * y + z with unbounded exponent
          // scale c3 up by scale = P34 - q3, 1 <= scale <= P34-1,
          // 1 <= scale <= 33
          // calculate res = c3 * 10^scale
          scale = P34 - q3;
          x0 = delta + q4 - P34;
        } else if delta + q4 < q3 {
          // Case (6)
          // make Case (6) look like Case (3) or Case (5) with scale = 0
          // by scaling up c4 by 10^(q3 - delta - q4)
          scale = q3 - delta - q4; // 1 <= scale <= 33
          if q4 <= 19 {
            // 1 <= scale <= 19; c4 fits in 64 bits
            if scale <= 19 {
              // 10^scale fits in 64 bits
              // 64 x 64 c4.w[0] * bid_ten2k64!(scale]
              mul_64x64_to_128mach!(p128, c4.w[0], bid_ten2k64!(scale));
            } else {
              // 10^scale fits in 128 bits
              // 64 x 128 c4.w[0] * bid_ten2k128!(scale - 20]
              mul_128x64_to_128!(p128, c4.w[0], bid_ten2k128!(scale - 20));
            }
          } else {
            // c4 fits in 128 bits, but 10^scale must fit in 64 bits
            // 64 x 128 bid_ten2k64!(scale] * c4
            mul_128x64_to_128!(p128, bid_ten2k64!(scale), c4);
          }
          c4.w[0] = p128.w[0];
          c4.w[1] = p128.w[1];
          // e4 does not need adjustment, as it is not used from this point on
          scale = 0;
          x0 = 0;
          // now Case (6) looks like Case (3) or Case (5) with scale = 0
        } else {
          // if Case (3) or Case (5)
          // Note: Case (3) is similar to Case (2), but scale differs and the
          // result is exact, unless it is tiny (so x0 = 0 when calculating the
          // result with unbounded exponent)

          // calculate first the sum x * y + z with unbounded exponent (exact)
          // scale c3 up by scale = delta + q4 - q3, 1 <= scale <= P34-1,
          // 1 <= scale <= 33
          // calculate res = c3 * 10^scale
          scale = delta + q4 - q3;
          x0 = 0;
          // Note: the comments which follow refer [mainly] to Case (2)]
        }

        'case2_repeat: loop {
          if scale == 0 {
            // this could happen e.g. if we return to case2_repeat
            // or in Case (4)
            res.w[1] = c3.w[1];
            res.w[0] = c3.w[0];
          } else if q3 <= 19 {
            // 1 <= scale <= 19; z fits in 64 bits
            if scale <= 19 {
              // 10^scale fits in 64 bits
              // 64 x 64 c3.w[0] * bid_ten2k64!(scale]
              mul_64x64_to_128mach!(res, c3.w[0], bid_ten2k64!(scale));
            } else {
              // 10^scale fits in 128 bits
              // 64 x 128 c3.w[0] * bid_ten2k128!(scale - 20]
              mul_128x64_to_128!(res, c3.w[0], bid_ten2k128!(scale - 20));
            }
          } else {
            // z fits in 128 bits, but 10^scale must fit in 64 bits
            // 64 x 128 bid_ten2k64!(scale] * c3
            mul_128x64_to_128!(res, bid_ten2k64!(scale), c3);
          }
          // e3 is already calculated
          dec!(e3, scale);
          // now res = c3 * 10^scale and e3 = e3 - scale
          // Note: c3 * 10^scale could be 10^34 if we returned to case2_repeat
          // because the result was too small

          // round c4 to nearest to q4 - x0 digits, where x0 = delta + q4 - P34,
          // 1 <= x0 <= min (q4 - 1, 2 * P34 - 1) <=> 1 <= x0 <= min (q4 - 1, 67)
          // Also: 1 <= q4 - x0 <= P34 -1 => 1 <= q4 - x0 <= 33 (so the result of
          // the rounding fits in 128 bits!)
          // x0 = delta + q4 - P34 (calculated before reaching case2_repeat)
          // because q3 + q4 - x0 <= P => x0 >= q3 + q4 - P34
          if x0 == 0 {
            // this could happen only if we return to case2_repeat, or
            // for Case (3) or Case (6)
            r128.w[1] = c4.w[1];
            r128.w[0] = c4.w[0];
          } else if q4 <= 18 {
            // 2 <= q4 <= 18, max(1, q3+q4-P34) <= x0 <= q4 - 1, 1 <= x0 <= 17
            bid_round64_2_18(q4, x0, c4.w[0], &mut r64, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
            if incr_exp > 0 {
              // r64 = 10^(q4-x0), 1 <= q4 - x0 <= q4 - 1, 1 <= q4 - x0 <= 17
              r64 = bid_ten2k64!(q4 - x0);
            }
            r128.w[1] = 0;
            r128.w[0] = r64;
          } else if q4 <= 38 {
            // 19 <= q4 <= 38, max(1, q3+q4-P34) <= x0 <= q4 - 1, 1 <= x0 <= 37
            p128.w[1] = c4.w[1];
            p128.w[0] = c4.w[0];
            bid_round128_19_38(q4, x0, p128, &mut r128, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
            if incr_exp > 0 {
              // r128 = 10^(q4-x0), 1 <= q4 - x0 <= q4 - 1, 1 <= q4 - x0 <= 37
              if q4 - x0 <= 19 {
                // 1 <= q4 - x0 <= 19
                r128.w[0] = bid_ten2k64!(q4 - x0);
                // r128.w[1] stays 0
              } else {
                // 20 <= q4 - x0 <= 37
                r128.w[0] = bid_ten2k128!(q4 - x0 - 20).w[0];
                r128.w[1] = bid_ten2k128!(q4 - x0 - 20).w[1];
              }
            }
          } else if q4 <= 57 {
            // 38 <= q4 <= 57, max(1, q3+q4-P34) <= x0 <= q4 - 1, 5 <= x0 <= 56
            p192.w[2] = c4.w[2];
            p192.w[1] = c4.w[1];
            p192.w[0] = c4.w[0];
            bid_round192_39_57(q4, x0, p192, &mut r192, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
            // r192.w[2] is always 0
            if incr_exp > 0 {
              // r192 = 10^(q4-x0), 1 <= q4 - x0 <= q4 - 5, 1 <= q4 - x0 <= 52
              if q4 - x0 <= 19 {
                // 1 <= q4 - x0 <= 19
                r192.w[0] = bid_ten2k64!(q4 - x0);
                // r192.w[1] stays 0
                // r192.w[2] stays 0
              } else {
                // 20 <= q4 - x0 <= 33
                r192.w[0] = bid_ten2k128!(q4 - x0 - 20).w[0];
                r192.w[1] = bid_ten2k128!(q4 - x0 - 20).w[1];
                // r192.w[2] stays 0
              }
            }
            r128.w[1] = r192.w[1];
            r128.w[0] = r192.w[0];
          } else {
            // 58 <= q4 <= 68, max(1, q3+q4-P34) <= x0 <= q4 - 1, 25 <= x0 <= 67
            bid_round256_58_76(q4, x0, c4, &mut r256, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
            // r256.w[3] and r256.w[2] are always 0
            if incr_exp > 0 {
              // r256 = 10^(q4-x0), 1 <= q4 - x0 <= q4 - 25, 1 <= q4 - x0 <= 43
              if q4 - x0 <= 19 {
                // 1 <= q4 - x0 <= 19
                r256.w[0] = bid_ten2k64!(q4 - x0);
                // r256.w[1] stays 0
                // r256.w[2] stays 0
                // r256.w[3] stays 0
              } else {
                // 20 <= q4 - x0 <= 33
                r256.w[0] = bid_ten2k128!(q4 - x0 - 20).w[0];
                r256.w[1] = bid_ten2k128!(q4 - x0 - 20).w[1];
                // r256.w[2] stays 0
                // r256.w[3] stays 0
              }
            }
            r128.w[1] = r256.w[1];
            r128.w[0] = r256.w[0];
          }
          // now add c3 * 10^scale in res and the signed top (q4-x0) digits of c4,
          // rounded to nearest, which were copied into r128
          if z_sign == p_sign {
            lsb = (res.w[0] & 0x01) as i32; // lsb of c3 * 10^scale
            // the sum can result in [up to] P34 or P34 + 1 digits
            inc!(res.w[0], r128.w[0]);
            inc!(res.w[1], r128.w[1]);
            if res.w[0] < r128.w[0] {
              inc!(res.w[1]); // carry
            }
            // if res > 10^34 - 1 need to increase x0 and decrease scale by 1
            if res.w[1] > 0x0001ed09bead87c0 || (res.w[1] == 0x0001ed09bead87c0 && res.w[0] > 0x378d8e63ffffffff) {
              // avoid double rounding error
              is_inexact_lt_midpoint0 = is_inexact_lt_midpoint;
              is_inexact_gt_midpoint0 = is_inexact_gt_midpoint;
              is_midpoint_lt_even0 = is_midpoint_lt_even;
              is_midpoint_gt_even0 = is_midpoint_gt_even;
              is_inexact_lt_midpoint = false;
              is_inexact_gt_midpoint = false;
              is_midpoint_lt_even = false;
              is_midpoint_gt_even = false;
              p128.w[1] = res.w[1];
              p128.w[0] = res.w[0];
              bid_round128_19_38(35, 1, p128, &mut res, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
              // incr_exp is 0 with certainty in this case
              // avoid a double rounding error
              if (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) && is_midpoint_lt_even {
                // double rounding error upward
                // res = res - 1
                dec!(res.w[0]);
                if res.w[0] == 0xffffffffffffffff {
                  dec!(res.w[1]);
                }
                // Note: a double rounding error upward is not possible; for this
                // the result after the first rounding would have to be 99...95
                // (35 digits in all), possibly followed by a number of zeros; this
                // not possible in Cases (2)-(6) or (15)-(17) which may get here
                is_midpoint_lt_even = false;
                is_inexact_lt_midpoint = true;
              } else if (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) && is_midpoint_gt_even {
                // double rounding error downward
                // res = res + 1
                inc!(res.w[0]);
                if res.w[0] == 0 {
                  inc!(res.w[1]);
                }
                is_midpoint_gt_even = false;
                is_inexact_gt_midpoint = true;
              } else if !is_midpoint_lt_even && !is_midpoint_gt_even && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint {
                // if this second rounding was exact the result may still be
                // inexact because of the first rounding
                if is_inexact_gt_midpoint0 || is_midpoint_lt_even0 {
                  is_inexact_gt_midpoint = true;
                }
                if is_inexact_lt_midpoint0 || is_midpoint_gt_even0 {
                  is_inexact_lt_midpoint = true;
                }
              } else if is_midpoint_gt_even && (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) {
                // pulled up to a midpoint
                is_inexact_lt_midpoint = true;
                is_inexact_gt_midpoint = false;
                is_midpoint_lt_even = false;
                is_midpoint_gt_even = false;
              } else if is_midpoint_lt_even && (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) {
                // pulled down to a midpoint
                is_inexact_lt_midpoint = false;
                is_inexact_gt_midpoint = true;
                is_midpoint_lt_even = false;
                is_midpoint_gt_even = false;
              }
              // adjust exponent
              inc!(e3);
              if !is_midpoint_lt_even && !is_midpoint_gt_even && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint && (is_midpoint_lt_even0 || is_midpoint_gt_even0 || is_inexact_lt_midpoint0 || is_inexact_gt_midpoint0) {
                is_inexact_lt_midpoint = true;
              }
            } else {
              // This is the result rounded with unbounded exponent, unless a correction is needed.
              res.w[1] &= MASK_COEFF;
              if lsb == 1 {
                if is_midpoint_gt_even {
                  // res = res + 1
                  is_midpoint_gt_even = false;
                  is_midpoint_lt_even = true;
                  inc!(res.w[0]);
                  if res.w[0] == 0x0 {
                    inc!(res.w[1]);
                  }
                  // check for rounding overflow
                  if res.w[1] == 0x0001ed09bead87c0 && res.w[0] == 0x378d8e6400000000 {
                    // res = 10^34 => rounding overflow
                    res.w[1] = 0x0000314dc6448d93;
                    res.w[0] = 0x38c15b0a00000000; // 10^33
                    inc!(e3);
                  }
                } else if is_midpoint_lt_even {
                  // res = res - 1
                  is_midpoint_lt_even = false;
                  is_midpoint_gt_even = true;
                  dec!(res.w[0]);
                  if res.w[0] == 0xffffffffffffffff {
                    dec!(res.w[1]);
                  }
                  // if the result is pure zero, the sign depends on the rounding
                  // mode (x*y and z had opposite signs)
                  if res.w[1] == 0 && res.w[0] == 0 {
                    // the exponent is max (e3, EXPMIN)
                    res.w[1] = 0;
                    res.w[0] = 0;
                    *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
                    *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
                    *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
                    *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;
                    return res;
                  }
                }
              }
            }
          } else {
            // if (z_sign != p_sign)
            lsb = (res.w[0] & 0x01) as i32; // lsb of c3 * 10^scale; r128 contains rounded c4
            // used to swap rounding indicators if p_sign != z_sign
            // the sum can result in [up to] P34 or P34 - 1 digits
            tmp64 = res.w[0];
            dec!(res.w[0], r128.w[0]);
            dec!(res.w[1], r128.w[1]);
            if res.w[0] > tmp64 {
              dec!(res.w[1]); // borrow
            }
            // if res < 10^33 and exp > EXPMIN need to decrease x0 and
            // increase scale by 1
            if e3 > EXPMIN
              && ((res.w[1] < 0x0000314dc6448d93 || (res.w[1] == 0x0000314dc6448d93 && res.w[0] < 0x38c15b0a00000000)) || ((is_inexact_lt_midpoint | is_midpoint_gt_even) && res.w[1] == 0x0000314dc6448d93 && res.w[0] == 0x38c15b0a00000000))
              && x0 >= 1
            {
              dec!(x0);
              // first restore e3, otherwise it will be too small
              inc!(e3, scale);
              inc!(scale);
              is_inexact_lt_midpoint = false;
              is_inexact_gt_midpoint = false;
              is_midpoint_lt_even = false;
              is_midpoint_gt_even = false;
              incr_exp = 0;
              continue 'case2_repeat;
            }
            // else this is the result rounded with unbounded exponent;
            // because the result has opposite sign to that of c4 which was
            // rounded, need to change the rounding indicators
            if is_inexact_lt_midpoint {
              is_inexact_lt_midpoint = false;
              is_inexact_gt_midpoint = true;
            } else if is_inexact_gt_midpoint {
              is_inexact_gt_midpoint = false;
              is_inexact_lt_midpoint = true;
            } else if lsb == 0 {
              if is_midpoint_lt_even {
                is_midpoint_lt_even = false;
                is_midpoint_gt_even = true;
              } else if is_midpoint_gt_even {
                is_midpoint_gt_even = false;
                is_midpoint_lt_even = true;
              }
            } else if lsb == 1 {
              if is_midpoint_lt_even {
                // res = res + 1
                inc!(res.w[0]);
                if res.w[0] == 0 {
                  inc!(res.w[1]);
                }
                // check for rounding overflow
                if res.w[1] == 0x0001ed09bead87c0 && res.w[0] == 0x378d8e6400000000 {
                  // res = 10^34 => rounding overflow
                  res.w[1] = 0x0000314dc6448d93;
                  res.w[0] = 0x38c15b0a00000000; // 10^33
                  inc!(e3);
                }
              } else if is_midpoint_gt_even {
                // res = res - 1
                dec!(res.w[0]);
                if res.w[0] == 0xffffffffffffffff {
                  dec!(res.w[1]);
                }
                // if the result is pure zero, the sign depends on the rounding
                // mode (x*y and z had opposite signs)
                if res.w[1] == 0 && res.w[0] == 0 {
                  // the exponent is max (e3, EXPMIN)
                  res.w[1] = 0;
                  res.w[0] = 0;
                  *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
                  *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
                  *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
                  *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;
                  return res;
                }
              }
            }
          }
          break 'case2_repeat;
        }

        // check for underflow
        if e3 == EXPMIN {
          // and if significand < 10^33 => result is tiny
          if (res.w[1] & MASK_COEFF) < 0x0000314dc6448d93 || ((res.w[1] & MASK_COEFF) == 0x0000314dc6448d93 && res.w[0] < 0x38c15b0a00000000) {
            is_tiny = true;
          }
          if cfg!(feature = "decimal-tiny-detection-after-rounding") {
            if ((res.w[1] & 0x7fffffffffffffff) == 0x0000314dc6448d93) && (res.w[0] == 0x38c15b0a00000000) && /* 10^33*10^-6176 */ (z_sign != p_sign) {
              is_tiny = true;
            }
          } else {
            //
          }
        } else if e3 < EXPMIN {
          // the result is tiny, so we must truncate more of res
          is_tiny = true;
          x0 = EXPMIN - e3;
          is_inexact_lt_midpoint0 = is_inexact_lt_midpoint;
          is_inexact_gt_midpoint0 = is_inexact_gt_midpoint;
          is_midpoint_lt_even0 = is_midpoint_lt_even;
          is_midpoint_gt_even0 = is_midpoint_gt_even;
          is_inexact_lt_midpoint = false;
          is_inexact_gt_midpoint = false;
          is_midpoint_lt_even = false;
          is_midpoint_gt_even = false;
          // determine the number of decimal digits in res
          if res.w[1] == 0 {
            // between 1 and 19 digits
            ind = 1;
            while ind <= 19 {
              if res.w[0] < bid_ten2k64!(ind) {
                break;
              }
              ind += 1;
            }
            // ind digits
          } else if res.w[1] < bid_ten2k128!(0).w[1] || (res.w[1] == bid_ten2k128!(0).w[1] && res.w[0] < bid_ten2k128!(0).w[0]) {
            // 20 digits
            ind = 20;
          } else {
            // between 21 and 38 digits
            ind = 1;
            while ind <= 18 {
              if res.w[1] < bid_ten2k128!(ind).w[1] || (res.w[1] == bid_ten2k128!(ind).w[1] && res.w[0] < bid_ten2k128!(ind).w[0]) {
                break;
              }
              ind += 1;
            }
            // ind + 20 digits
            ind += 20;
          }

          // at this point ind >= x0; because delta >= 2 on this path, the case
          // ind = x0 can occur only in Case (2) or case (3), when c3 has one
          // digit (q3 = 1) equal to 1 (c3 = 1), e3 is EXPMIN (e3 = EXPMIN),
          // the signs of x * y and z are opposite, and through cancellation
          // the most significant decimal digit in res has the weight
          // 10^(emin-1); however, it is clear that in this case the most
          // significant digit is 9, so the result before rounding is
          // 0.9... * 10^emin
          // Otherwise, ind > x0 because there are non-zero decimal digits in the
          // result with weight of at least 10^emin, and correction for underflow
          //  can be carried out using the round*_*_2_* () routines
          if x0 == ind {
            // the result before rounding is 0.9... * 10^emin
            res.w[1] = 0x0;
            res.w[0] = 0x1;
            is_inexact_gt_midpoint = true;
          } else if ind <= 18 {
            // check that 2 <= ind
            // 2 <= ind <= 18, 1 <= x0 <= 17
            bid_round64_2_18(ind, x0, res.w[0], &mut r64, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
            if incr_exp > 0 {
              // r64 = 10^(ind-x0), 1 <= ind - x0 <= ind - 1, 1 <= ind - x0 <= 17
              r64 = bid_ten2k64!(ind - x0);
            }
            res.w[1] = 0;
            res.w[0] = r64;
          } else if ind <= 38 {
            // 19 <= ind <= 38
            p128.w[1] = res.w[1];
            p128.w[0] = res.w[0];
            bid_round128_19_38(ind, x0, p128, &mut res, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
            if incr_exp > 0 {
              // r128 = 10^(ind-x0), 1 <= ind - x0 <= ind - 1, 1 <= ind - x0 <= 37
              if ind - x0 <= 19 {
                // 1 <= ind - x0 <= 19
                res.w[0] = bid_ten2k64!(ind - x0);
                // res.w[1] stays 0
              } else {
                // 20 <= ind - x0 <= 37
                res.w[0] = bid_ten2k128!(ind - x0 - 20).w[0];
                res.w[1] = bid_ten2k128!(ind - x0 - 20).w[1];
              }
            }
          }
          // avoid a double rounding error
          if (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) && is_midpoint_lt_even {
            // double rounding error upward
            // res = res - 1
            dec!(res.w[0]);
            if res.w[0] == 0xffffffffffffffff {
              dec!(res.w[1]);
            }
            // Note: a double rounding error upward is not possible; for this
            // the result after the first rounding would have to be 99...95
            // (35 digits in all), possibly followed by a number of zeros; this
            // not possible in Cases (2)-(6) which may get here
            is_midpoint_lt_even = false;
            is_inexact_lt_midpoint = true;
          } else if (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) && is_midpoint_gt_even {
            // double rounding error downward
            // res = res + 1
            inc!(res.w[0]);
            if res.w[0] == 0 {
              inc!(res.w[1]);
            }
            is_midpoint_gt_even = false;
            is_inexact_gt_midpoint = true;
          } else if !is_midpoint_lt_even && !is_midpoint_gt_even && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint {
            // if this second rounding was exact the result may still be
            // inexact because of the first rounding
            if is_inexact_gt_midpoint0 || is_midpoint_lt_even0 {
              is_inexact_gt_midpoint = true;
            }
            if is_inexact_lt_midpoint0 || is_midpoint_gt_even0 {
              is_inexact_lt_midpoint = true;
            }
          } else if is_midpoint_gt_even && (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) {
            // pulled up to a midpoint
            is_inexact_lt_midpoint = true;
            is_inexact_gt_midpoint = false;
            is_midpoint_lt_even = false;
            is_midpoint_gt_even = false;
          } else if is_midpoint_lt_even && (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) {
            // pulled down to a midpoint
            is_inexact_lt_midpoint = false;
            is_inexact_gt_midpoint = true;
            is_midpoint_lt_even = false;
            is_midpoint_gt_even = false;
          }
          // adjust exponent
          inc!(e3, x0);
          if (!is_midpoint_lt_even && !is_midpoint_gt_even && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint) && (is_midpoint_lt_even0 || is_midpoint_gt_even0 || is_inexact_lt_midpoint0 || is_inexact_gt_midpoint0) {
            is_inexact_lt_midpoint = true;
          }
        } else { // not underflow
        }
        // check for inexact result
        if is_inexact_lt_midpoint || is_inexact_gt_midpoint || is_midpoint_lt_even || is_midpoint_gt_even {
          // set the inexact flag
          set_status_flags!(flags, BID_INEXACT_EXCEPTION);
          if is_tiny {
            set_status_flags!(flags, BID_UNDERFLOW_EXCEPTION);
          }
        }
        // now check for significand = 10^34 (may have resulted from going
        // back to case2_repeat)
        if res.w[1] == 0x0001ed09bead87c0 && res.w[0] == 0x378d8e6400000000 {
          // if  res = 10^34
          res.w[1] = 0x0000314dc6448d93; // res = 10^33
          res.w[0] = 0x38c15b0a00000000;
          inc!(e3);
        }
        res.w[1] |= z_sign | (e3.wrapping_add(6176) as u64) << 49;
        // check for overflow
        if rounding == BID_ROUNDING_TO_NEAREST && e3 > EXPMAX {
          res.w[1] = z_sign | 0x7800000000000000; // +/-inf
          res.w[0] = 0;
          set_status_flags!(flags, BID_INEXACT_EXCEPTION | BID_OVERFLOW_EXCEPTION);
        }
        if rounding != BID_ROUNDING_TO_NEAREST {
          bid_rounding_correction(rounding, is_inexact_lt_midpoint, is_inexact_gt_midpoint, is_midpoint_lt_even, is_midpoint_gt_even, e3, &mut res, flags);
        }
        *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
        *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
        *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
        *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

        res
      } else {
        // we get here only if delta <= 1 in Cases (2), (3), (4), (5), or (6) and
        // the signs of x*y and z are opposite; in these cases massive
        // cancellation can occur, so it is better to scale either c3 or c4 and
        // to perform the subtraction before rounding; rounding is performed
        // next, depending on the number of decimal digits in the result and on
        // the exponent value
        // Note: overlow is not possible in this case
        // this is similar to Cases (15), (16), and (17)

        if delta + q4 < q3 {
          // from Case (6)
          // Case (6) with 0<= delta <= 1 is similar to Cases (15), (16), and
          // (17) if we swap (c3, c4), (q3, q4), (e3, e4), (z_sign, p_sign)
          // and call bid_add_and_round; delta stays positive
          // c4.w[3] = 0 and c4.w[2] = 0, so swap just the low part of c4 with c3
          p128.w[1] = c3.w[1];
          p128.w[0] = c3.w[0];
          c3.w[1] = c4.w[1];
          c3.w[0] = c4.w[0];
          c4.w[1] = p128.w[1];
          c4.w[0] = p128.w[0];
          ind = q3;
          q3 = q4;
          q4 = ind;
          ind = e3;
          e4 = ind;
          tmp_sign = z_sign;
          z_sign = p_sign;
          p_sign = tmp_sign;
        } else {
          // from Cases (2), (3), (4), (5)
          // In Cases (2), (3), (4), (5) with 0 <= delta <= 1 c3 has to be
          // scaled up by q4 + delta - q3; this is the same as in Cases (15),
          // (16), and (17) if we just change the sign of delta
          delta = -delta;
        }
        bid_add_and_round(q3, q4, e4, delta, P34, z_sign, p_sign, c3, c4, rounding, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint, flags, &mut res);
        *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
        *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
        *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
        *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

        res
      };
    } else {
      // if delta < 0

      delta = -delta;

      if P34 < q4 && q4 <= delta {
        // Case (7)

        // truncate c4 to P34 digits into res
        // x = q4-P34, 1 <= x <= 34 because 35 <= q4 <= 68
        x0 = q4 - P34;
        if q4 <= 38 {
          p128.w[1] = c4.w[1];
          p128.w[0] = c4.w[0];
          bid_round128_19_38(q4, x0, p128, &mut res, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
        } else if q4 <= 57 {
          // 35 <= q4 <= 57
          p192.w[2] = c4.w[2];
          p192.w[1] = c4.w[1];
          p192.w[0] = c4.w[0];
          bid_round192_39_57(q4, x0, p192, &mut r192, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
          res.w[0] = r192.w[0];
          res.w[1] = r192.w[1];
        } else {
          // if (q4 <= 68)
          bid_round256_58_76(q4, x0, c4, &mut r256, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
          res.w[0] = r256.w[0];
          res.w[1] = r256.w[1];
        }
        inc!(e4, x0);
        if incr_exp > 0 {
          inc!(e4);
        }
        if !is_midpoint_lt_even && !is_midpoint_gt_even && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint {
          // if c4 rounded to P34 digits is exact then the result is inexact,
          // in a way that depends on the signs of x * y and z
          if p_sign == z_sign {
            is_inexact_lt_midpoint = true;
          } else {
            // if (p_sign != z_sign)
            if res.w[1] != 0x0000314dc6448d93 || res.w[0] != 0x38c15b0a00000000 {
              // res != 10^33
              is_inexact_gt_midpoint = true;
            } else {
              // res = 10^33 and exact is a special case
              // if c3 < 1/2 ulp then res = 10^33 and is_inexact_gt_midpoint = 1
              // if c3 = 1/2 ulp then res = 10^33 and is_midpoint_lt_even = 1
              // if c3 > 1/2 ulp then res = 10^34-1 and is_inexact_lt_midpoint = 1
              // Note: ulp is really ulp/10 (after borrow which propagates to msd)
              if delta > P34 + 1 {
                // c3 < 1/2
                // res = 10^33, unchanged
                is_inexact_gt_midpoint = true;
              } else {
                // if (delta == P34 + 1)
                if q3 <= 19 {
                  if c3.w[0] < bid_midpoint64!(q3 - 1) {
                    // c3 < 1/2 ulp
                    // res = 10^33, unchanged
                    is_inexact_gt_midpoint = true;
                  } else if c3.w[0] == bid_midpoint64!(q3 - 1) {
                    // c3 = 1/2 ulp
                    // res = 10^33, unchanged
                    is_midpoint_lt_even = true;
                  } else {
                    // if (c3.w[0] > bid_midpoint64[q3-1]), c3 > 1/2 ulp
                    res.w[1] = 0x0001ed09bead87c0; // 10^34 - 1
                    res.w[0] = 0x378d8e63ffffffff;
                    dec!(e4);
                    is_inexact_lt_midpoint = true;
                  }
                } else {
                  // if (20 <= q3 <=34)
                  if c3.w[1] < bid_midpoint128!(q3 - 20).w[1] || (c3.w[1] == bid_midpoint128!(q3 - 20).w[1] && c3.w[0] < bid_midpoint128!(q3 - 20).w[0]) {
                    // c3 < 1/2 ulp
                    // res = 10^33, unchanged
                    is_inexact_gt_midpoint = true;
                  } else if c3.w[1] == bid_midpoint128!(q3 - 20).w[1] && c3.w[0] == bid_midpoint128!(q3 - 20).w[0] {
                    // c3 = 1/2 ulp
                    // res = 10^33, unchanged
                    is_midpoint_lt_even = true;
                  } else {
                    // if (c3 > bid_midpoint128!(q3-20]), c3 > 1/2 ulp
                    res.w[1] = 0x0001ed09bead87c0; // 10^34 - 1
                    res.w[0] = 0x378d8e63ffffffff;
                    dec!(e4);
                    is_inexact_lt_midpoint = true;
                  }
                }
              }
            }
          }
        } else if is_midpoint_lt_even {
          if z_sign != p_sign {
            // needs correction: res = res - 1
            dec!(res.w[0]);
            if res.w[0] == 0xffffffffffffffff {
              dec!(res.w[1]);
            }
            // if it is (10^33-1)*10^e4 then the corect result is
            // (10^34-1)*10(e4-1)
            if res.w[1] == 0x0000314dc6448d93 && res.w[0] == 0x38c15b09ffffffff {
              res.w[1] = 0x0001ed09bead87c0; // 10^34 - 1
              res.w[0] = 0x378d8e63ffffffff;
              dec!(e4);
            }
            is_midpoint_lt_even = false;
            is_inexact_lt_midpoint = true;
          } else {
            // if (z_sign == p_sign)
            is_midpoint_lt_even = false;
            is_inexact_gt_midpoint = true;
          }
        } else if is_midpoint_gt_even {
          if z_sign == p_sign {
            // needs correction: res = res + 1 (cannot cross in the next binade)
            inc!(res.w[0]);
            if res.w[0] == 0x0000000000000000 {
              inc!(res.w[1]);
            }
            is_midpoint_gt_even = false;
            is_inexact_gt_midpoint = true;
          } else {
            // if (z_sign != p_sign)
            is_midpoint_gt_even = false;
            is_inexact_lt_midpoint = true;
          }
        } else { // the rounded result is already correct
        }
        // check for overflow
        if rounding == BID_ROUNDING_TO_NEAREST && e4 > EXPMAX {
          res.w[1] = p_sign | 0x7800000000000000;
          res.w[0] = 0x0000000000000000;
          set_status_flags!(flags, BID_OVERFLOW_EXCEPTION | BID_INEXACT_EXCEPTION);
        } else {
          // no overflow or not RN
          p_exp = (e4.wrapping_add(6176) as u64) << 49;
          res.w[1] |= p_sign | (p_exp & MASK_EXP);
        }
        if rounding != BID_ROUNDING_TO_NEAREST {
          bid_rounding_correction(rounding, is_inexact_lt_midpoint, is_inexact_gt_midpoint, is_midpoint_lt_even, is_midpoint_gt_even, e4, &mut res, flags);
        }
        if is_inexact_lt_midpoint || is_inexact_gt_midpoint || is_midpoint_lt_even || is_midpoint_gt_even {
          // set the inexact flag
          *flags |= BID_INEXACT_EXCEPTION;
        }
        *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
        *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
        *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
        *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

        return res;
      } else if (q4 <= P34 && P34 <= delta) || // Case (8)
      (q4 <= delta && delta < P34 && P34 < delta + q3) || // Case (9)
      (q4 <= delta && delta + q3 <= P34) || // Case (10)
      (delta < q4 && q4 <= P34 && P34 < delta + q3) || // Case (13)
      (delta < q4 && q4 <= delta + q3 && delta + q3 <= P34) || // Case (14)
      (delta + q3 < q4 && q4 <= P34)
      {
        // Case (18)

        // Case (8) is similar to Case (1), with c3 and c4 swapped
        // Case (9) is similar to Case (2), with c3 and c4 swapped
        // Case (10) is similar to Case (3), with c3 and c4 swapped
        // Case (13) is similar to Case (4), with c3 and c4 swapped
        // Case (14) is similar to Case (5), with c3 and c4 swapped
        // Case (18) is similar to Case (6), with c3 and c4 swapped

        // swap (c3, c4), (q3, q4), (e3, 34), (z_sign, p_sign), (z_exp, p_exp)
        // and go back to delta_ge_zero
        // c4.w[3] = 0 and c4.w[2] = 0, so swap just the low part of c4 with c3
        p128.w[1] = c3.w[1];
        p128.w[0] = c3.w[0];
        c3.w[1] = c4.w[1];
        c3.w[0] = c4.w[0];
        c4.w[1] = p128.w[1];
        c4.w[0] = p128.w[0];
        ind = q3;
        q3 = q4;
        q4 = ind;
        ind = e3;
        e3 = e4;
        e4 = ind;
        tmp_sign = z_sign;
        z_sign = p_sign;
        p_sign = tmp_sign;
        swap(&mut z_exp, &mut p_exp);
        continue 'delta_ge_zero;
      } else if (P34 <= delta && delta < q4 && q4 < delta + q3) || /* Case (11) */ (delta < P34 && P34 < q4 && q4 < delta + q3) {
        // Case (12)

        // round c3 to nearest to q3 - x0 digits, where x0 = e4 - e3,
        // 1 <= x0 <= q3 - 1 <= P34 - 1
        x0 = e4 - e3; // or x0 = delta + q3 - q4
        if q3 <= 18 {
          // 2 <= q3 <= 18
          bid_round64_2_18(q3, x0, c3.w[0], &mut r64, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
          // c3.w[1] = 0;
          c3.w[0] = r64;
        } else if q3 <= 38 {
          bid_round128_19_38(q3, x0, c3, &mut r128, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
          c3.w[1] = r128.w[1];
          c3.w[0] = r128.w[0];
        }
        // the rounded result has q3 - x0 digits
        // we want the exponent to be e4, so if incr_exp = 1 then
        // multiply the rounded result by 10 - it will still fit in 113 bits
        if incr_exp > 0 {
          // 64 x 128 -> 128
          p128.w[1] = c3.w[1];
          p128.w[0] = c3.w[0];
          mul_64x128_to_128!(c3, bid_ten2k64!(1), p128);
        }
        // now add/subtract the 256-bit c4 and the new (and shorter) 128-bit c3;
        // the result will have the sign of x * y; the exponent is e4
        r256.w[3] = 0;
        r256.w[2] = 0;
        r256.w[1] = c3.w[1];
        r256.w[0] = c3.w[0];
        if p_sign == z_sign {
          // r256 = c4 + r256
          bid_add256(c4, r256, &mut r256);
        } else {
          // if (p_sign != z_sign) { // r256 = c4 - r256
          bid_sub256(c4, r256, &mut r256); // the result cannot be pure zero
          // because the result has opposite sign to that of r256 which was
          // rounded, need to change the rounding indicators
          lsb = (c4.w[0] & 0x01) as i32;
          if is_inexact_lt_midpoint {
            is_inexact_lt_midpoint = false;
            is_inexact_gt_midpoint = true;
          } else if is_inexact_gt_midpoint {
            is_inexact_gt_midpoint = false;
            is_inexact_lt_midpoint = true;
          } else if lsb == 0 {
            if is_midpoint_lt_even {
              is_midpoint_lt_even = false;
              is_midpoint_gt_even = true;
            } else if is_midpoint_gt_even {
              is_midpoint_gt_even = false;
              is_midpoint_lt_even = true;
            }
          } else if lsb == 1 {
            if is_midpoint_lt_even {
              // res = res + 1
              inc!(r256.w[0]);
              if r256.w[0] == 0x0 {
                inc!(r256.w[1]);
                if r256.w[1] == 0x0 {
                  inc!(r256.w[2]);
                  if r256.w[2] == 0x0 {
                    inc!(r256.w[3]);
                  }
                }
              }
              // no check for rounding overflow - r256 was a difference
            } else if is_midpoint_gt_even {
              // res = res - 1
              dec!(r256.w[0]);
              if r256.w[0] == 0xffffffffffffffff {
                dec!(r256.w[1]);
                if r256.w[1] == 0xffffffffffffffff {
                  dec!(r256.w[2]);
                  if r256.w[2] == 0xffffffffffffffff {
                    dec!(r256.w[3]);
                  }
                }
              }
            }
          }
        }
        // determine the number of decimal digits in r256
        ind = bid_bid_nr_digits256(r256); // ind >= P34
        // if r256 is sum, then ind > P34; if r256 is a difference, then
        // ind >= P34; this means that we can calculate the result rounded to
        // the destination precision, with unbounded exponent, starting from r256
        // and using the indicators from the rounding of c3 to avoid a double
        // rounding error

        if ind < P34 {
        } else if ind == P34 {
          // the result rounded to the destination precision with
          // unbounded exponent
          // is (-1)^p_sign * r256 * 10^e4
          res.w[1] = r256.w[1];
          res.w[0] = r256.w[0];
        } else {
          // if (ind > P34)
          // if more than P digits, round to nearest to P digits
          // round r256 to P34 digits
          x0 = ind - P34; // 1 <= x0 <= 34 as 35 <= ind <= 68
          // save c3 rounding indicators to help avoid double rounding error
          is_inexact_lt_midpoint0 = is_inexact_lt_midpoint;
          is_inexact_gt_midpoint0 = is_inexact_gt_midpoint;
          is_midpoint_lt_even0 = is_midpoint_lt_even;
          is_midpoint_gt_even0 = is_midpoint_gt_even;
          // initialize rounding indicators
          is_inexact_lt_midpoint = false;
          is_inexact_gt_midpoint = false;
          is_midpoint_lt_even = false;
          is_midpoint_gt_even = false;
          // round to P34 digits; the result fits in 113 bits
          if ind <= 38 {
            p128.w[1] = r256.w[1];
            p128.w[0] = r256.w[0];
            bid_round128_19_38(ind, x0, p128, &mut r128, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
          } else if ind <= 57 {
            p192.w[2] = r256.w[2];
            p192.w[1] = r256.w[1];
            p192.w[0] = r256.w[0];
            bid_round192_39_57(ind, x0, p192, &mut r192, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
            r128.w[1] = r192.w[1];
            r128.w[0] = r192.w[0];
          } else {
            // if (ind <= 68)
            bid_round256_58_76(ind, x0, r256, &mut r256, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
            r128.w[1] = r256.w[1];
            r128.w[0] = r256.w[0];
          }
          // the rounded result has P34 = 34 digits
          e4 = e4 + x0 + incr_exp;

          res.w[1] = r128.w[1];
          res.w[0] = r128.w[0];

          // avoid a double rounding error
          if (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) && is_midpoint_lt_even {
            // double rounding error upward
            // res = res - 1
            dec!(res.w[0]);
            if res.w[0] == 0xffffffffffffffff {
              dec!(res.w[1]);
            }
            is_midpoint_lt_even = false;
            is_inexact_lt_midpoint = true;
            // Note: a double rounding error upward is not possible; for this
            // the result after the first rounding would have to be 99...95
            // (35 digits in all), possibly followed by a number of zeros; this
            // not possible in Cases (2)-(6) or (15)-(17) which may get here
            // if this is 10^33 - 1 make it 10^34 - 1 and decrement exponent
            if res.w[1] == 0x0000314dc6448d93 && res.w[0] == 0x38c15b09ffffffff {
              // 10^33 - 1
              res.w[1] = 0x0001ed09bead87c0; // 10^34 - 1
              res.w[0] = 0x378d8e63ffffffff;
              dec!(e4);
            }
          } else if (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) && is_midpoint_gt_even {
            // double rounding error downward
            // res = res + 1
            inc!(res.w[0]);
            if res.w[0] == 0 {
              inc!(res.w[1]);
            }
            is_midpoint_gt_even = false;
            is_inexact_gt_midpoint = true;
          } else if !is_midpoint_lt_even && !is_midpoint_gt_even && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint {
            // if this second rounding was exact the result may still be
            // inexact because of the first rounding
            if is_inexact_gt_midpoint0 || is_midpoint_lt_even0 {
              is_inexact_gt_midpoint = true;
            }
            if is_inexact_lt_midpoint0 || is_midpoint_gt_even0 {
              is_inexact_lt_midpoint = true;
            }
          } else if is_midpoint_gt_even && (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) {
            // pulled up to a midpoint
            is_inexact_lt_midpoint = true;
            is_inexact_gt_midpoint = false;
            is_midpoint_lt_even = false;
            is_midpoint_gt_even = false;
          } else if is_midpoint_lt_even && (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) {
            // pulled down to a midpoint
            is_inexact_lt_midpoint = false;
            is_inexact_gt_midpoint = true;
            is_midpoint_lt_even = false;
            is_midpoint_gt_even = false;
          }
        }

        // determine tininess
        if rounding == BID_ROUNDING_TO_NEAREST {
          if e4 < EXPMIN {
            is_tiny = true; // for other rounding modes apply correction
          }
        } else {
          // for RM, RP, RZ, RA apply correction in order to determine tininess
          // but do not save the result; apply the correction to
          // (-1)^p_sign * res * 10^0
          p128.w[1] = p_sign | 0x3040000000000000 | res.w[1];
          p128.w[0] = res.w[0];
          bid_rounding_correction(rounding, is_inexact_lt_midpoint, is_inexact_gt_midpoint, is_midpoint_lt_even, is_midpoint_gt_even, 0, &mut p128, flags);
          scale = (((p128.w[1] & MASK_EXP) >> 49) as i32).wrapping_sub(6176); // -1, 0, or +1
          // the number of digits in the significand is P34 = 34
          if e4 + scale < EXPMIN {
            is_tiny = true;
          }
        }

        // the result rounded to the destination precision with unbounded exponent
        // is (-1)^p_sign * res * 10^e4
        res.w[1] |= p_sign | (e4.wrapping_add(6176) as u64) << 49; // RN
        // res.w[0] unchanged;
        // Note: res is correct only if EXPMIN <= e4 <= EXPMAX
        ind = P34; // the number of decimal digits in the signifcand of res

        // at this point we have the result rounded with unbounded exponent in
        // res and we know its tininess:
        // res = (-1)^p_sign * significand * 10^e4,
        // where q (significand) = ind = P34
        // Note: res is correct only if EXPMIN <= e4 <= EXPMAX

        // check for overflow if RN
        if rounding == BID_ROUNDING_TO_NEAREST && (ind + e4) > (P34 + EXPMAX) {
          res.w[1] = p_sign | 0x7800000000000000;
          res.w[0] = 0x0000000000000000;
          set_status_flags!(flags, BID_INEXACT_EXCEPTION | BID_OVERFLOW_EXCEPTION);
          *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
          *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
          *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
          *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;
          return res;
        } // else not overflow or not RN, so continue

        // from this point on this is similar to the last part of the computation
        // for Cases (15), (16), (17)

        // if (e4 >= EXPMIN) we have the result rounded with bounded exponent
        if e4 < EXPMIN {
          x0 = EXPMIN - e4; // x0 >= 1; the number of digits to chop off of res
          // where the result rounded [at most] once is
          //   (-1)^p_sign * significand_res * 10^e4

          // avoid double rounding error
          is_inexact_lt_midpoint0 = is_inexact_lt_midpoint;
          is_inexact_gt_midpoint0 = is_inexact_gt_midpoint;
          is_midpoint_lt_even0 = is_midpoint_lt_even;
          is_midpoint_gt_even0 = is_midpoint_gt_even;
          is_inexact_lt_midpoint = false;
          is_inexact_gt_midpoint = false;
          is_midpoint_lt_even = false;
          is_midpoint_gt_even = false;

          if x0 > ind {
            // nothing is left of res when moving the decimal point left x0 digits
            is_inexact_lt_midpoint = true;
            res.w[1] = p_sign;
            res.w[0] = 0x0000000000000000;
            e4 = EXPMIN;
          } else if x0 == ind {
            // 1 <= x0 = ind <= P34 = 34
            // this is <, =, or > 1/2 ulp
            // compare the ind-digit value in the significand of res with
            // 1/2 ulp = 5*10^(ind-1), i.e. determine whether it is
            // less than, equal to, or greater than 1/2 ulp (significand of res)
            r128.w[1] = res.w[1] & MASK_COEFF;
            r128.w[0] = res.w[0];
            if ind <= 19 {
              if r128.w[0] < bid_midpoint64!(ind - 1) {
                // < 1/2 ulp
                lt_half_ulp = true;
                is_inexact_lt_midpoint = true;
              } else if r128.w[0] == bid_midpoint64!(ind - 1) {
                // = 1/2 ulp
                eq_half_ulp = true;
                is_midpoint_gt_even = true;
              } else {
                // > 1/2 ulp
                is_inexact_gt_midpoint = true;
              }
            } else {
              // if (ind <= 38)
              if r128.w[1] < bid_midpoint128!(ind - 20).w[1] || (r128.w[1] == bid_midpoint128!(ind - 20).w[1] && r128.w[0] < bid_midpoint128!(ind - 20).w[0]) {
                // < 1/2 ulp
                lt_half_ulp = true;
                is_inexact_lt_midpoint = true;
              } else if r128.w[1] == bid_midpoint128!(ind - 20).w[1] && r128.w[0] == bid_midpoint128!(ind - 20).w[0] {
                // = 1/2 ulp
                eq_half_ulp = true;
                is_midpoint_gt_even = true;
              } else {
                // > 1/2 ulp
                is_inexact_gt_midpoint = true;
              }
            }
            if lt_half_ulp || eq_half_ulp {
              // res = +0.0 * 10^EXPMIN
              res.w[1] = 0x0000000000000000;
              res.w[0] = 0x0000000000000000;
            } else {
              // if (gt_half_ulp)
              // res = +1 * 10^EXPMIN
              res.w[1] = 0x0000000000000000;
              res.w[0] = 0x0000000000000001;
            }
            res.w[1] |= p_sign;
            e4 = EXPMIN;
          } else {
            // if (1 <= x0 <= ind - 1 <= 33)
            // round the ind-digit result to ind - x0 digits

            if ind <= 18 {
              // 2 <= ind <= 18
              bid_round64_2_18(ind, x0, res.w[0], &mut r64, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
              res.w[1] = 0x0;
              res.w[0] = r64;
            } else if ind <= 38 {
              p128.w[1] = res.w[1] & MASK_COEFF;
              p128.w[0] = res.w[0];
              bid_round128_19_38(ind, x0, p128, &mut res, &mut incr_exp, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
            }
            e4 = e4.wrapping_add(x0); // EXPMIN
            // we want the exponent to be EXPMIN, so if incr_exp = 1 then
            // multiply the rounded result by 10 - it will still fit in 113 bits
            if incr_exp > 0 {
              // 64 x 128 -> 128
              p128.w[1] = res.w[1] & MASK_COEFF;
              p128.w[0] = res.w[0];
              mul_64x128_to_128!(res, bid_ten2k64!(1), p128);
            }
            res.w[1] = p_sign | (e4.wrapping_add(6176) as u64) << 49 | (res.w[1] & MASK_COEFF);
            // avoid a double rounding error
            if (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) && is_midpoint_lt_even {
              // double rounding error upward
              // res = res - 1
              dec!(res.w[0]);
              if res.w[0] == 0xffffffffffffffff {
                dec!(res.w[1]);
              }
              // Note: a double rounding error upward is not possible; for this
              // the result after the first rounding would have to be 99...95
              // (35 digits in all), possibly followed by a number of zeros; this
              // not possible in this underflow case
              is_midpoint_lt_even = false;
              is_inexact_lt_midpoint = true;
            } else if (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) && is_midpoint_gt_even {
              // double rounding error downward
              // res = res + 1
              inc!(res.w[0]);
              if res.w[0] == 0 {
                inc!(res.w[1]);
              }
              is_midpoint_gt_even = false;
              is_inexact_gt_midpoint = true;
            } else if !is_midpoint_lt_even && !is_midpoint_gt_even && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint {
              // if this second rounding was exact the result may still be
              // inexact because of the first rounding
              if is_inexact_gt_midpoint0 || is_midpoint_lt_even0 {
                is_inexact_gt_midpoint = true;
              }
              if is_inexact_lt_midpoint0 || is_midpoint_gt_even0 {
                is_inexact_lt_midpoint = true;
              }
            } else if is_midpoint_gt_even && (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) {
              // pulled up to a midpoint
              is_inexact_lt_midpoint = true;
              is_inexact_gt_midpoint = false;
              is_midpoint_lt_even = false;
              is_midpoint_gt_even = false;
            } else if is_midpoint_lt_even && (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) {
              // pulled down to a midpoint
              is_inexact_lt_midpoint = false;
              is_inexact_gt_midpoint = true;
              is_midpoint_lt_even = false;
              is_midpoint_gt_even = false;
            }
          }
        }
        // res contains the correct result
        // apply correction if not rounding to nearest
        if rounding != BID_ROUNDING_TO_NEAREST {
          bid_rounding_correction(rounding, is_inexact_lt_midpoint, is_inexact_gt_midpoint, is_midpoint_lt_even, is_midpoint_gt_even, e4, &mut res, flags);
        }
        if cfg!(feature = "decimal-tiny-detection-after-rounding") {
          // correction needed for tininess detection before rounding
          if (((res.w[1] & 0x7fffffffffffffff) == 0x0000314dc6448d93) && // 10^33*10^-6176_high
          (res.w[0] == 0x38c15b0a00000000)) &&  // 10^33*10^-6176_low
          (((rounding == BID_ROUNDING_TO_NEAREST ||
            rounding == BID_ROUNDING_TIES_AWAY) &&
            (is_midpoint_lt_even || is_inexact_gt_midpoint)) ||
            ((((rounding == BID_ROUNDING_UP) && (res.w[1] & MASK_SIGN) == 0) ||
              ((rounding == BID_ROUNDING_DOWN) && (res.w[1] & MASK_SIGN) > 0))
              && (is_midpoint_lt_even || is_midpoint_gt_even ||
              is_inexact_lt_midpoint || is_inexact_gt_midpoint)))
          {
            is_tiny = true;
          }
        }

        if is_midpoint_lt_even || is_midpoint_gt_even || is_inexact_lt_midpoint || is_inexact_gt_midpoint {
          // set the inexact flag
          set_status_flags!(flags, BID_INEXACT_EXCEPTION);
          if is_tiny {
            set_status_flags!(flags, BID_UNDERFLOW_EXCEPTION);
          }
        }
        *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
        *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
        *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
        *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;
        return res;
      } else if (P34 <= delta && delta + q3 <= q4) || // Case (15)
      (delta < P34 && P34 < delta + q3 && delta + q3 <= q4) || //Case (16)
      (delta + q3 <= P34 && P34 < q4)
      {
        // Case (17)

        // calculate first the result rounded to the destination precision, with
        // unbounded exponent

        bid_add_and_round(q3, q4, e4, delta, P34, z_sign, p_sign, c3, c4, rounding, &mut is_midpoint_lt_even, &mut is_midpoint_gt_even, &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint, flags, &mut res);
        *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
        *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
        *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
        *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;
        return res;
      }
    }
    break 'delta_ge_zero;
  }

  *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
  *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
  *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
  *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;
  res
}
