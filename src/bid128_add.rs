//! # bid128_add

use crate::Bid128;
use crate::bid_conf::{IdecFlags, IdecRound};
use crate::bid_functions::*;
use crate::bid_internal::*;
use crate::bid64::Bid64;
use crate::bid128_common::*;
use crate::bid256::Bid256;

/// Adds two 128-bit decimal floating-point values.
pub fn bid128_add(mut x: Bid128, mut y: Bid128, rnd_mode: IdecRound, pfpsf: &mut IdecFlags) -> Bid128 {
  let mut res = Bid128 { w: [0xbaddbaddbaddbadd, 0xbaddbaddbaddbadd] };
  let mut c1 = Bid128::default();
  let mut c2 = Bid128::default();
  let mut x_sign = x.w[1] & MASK_SIGN; // 0 for positive and MASK_SIGN for negative.
  let mut y_sign = y.w[1] & MASK_SIGN; // 0 for positive and MASK_SIGN for negative.
  let mut tmp_sign: u64;
  let x_nr_bits: usize;
  let y_nr_bits: usize;
  let mut q1: i32;
  let mut q2: i32;
  let mut scale: i32;
  let mut x1: i32;
  let mut ind: i32;
  let mut shift: i32;
  let delta: i32;
  let halfulp64: u64;
  let halfulp128: Bid128;
  let mut highf2star = Bid128::default(); // Top 128 bits in f2*. Low 128 bits in r256[1], r256[0]
  let mut p256 = Bid256::default();
  let mut q256 = Bid256::default();
  let mut r256 = Bid256::default();
  let mut is_inexact = false;
  let mut is_midpoint_lt_even = false;
  let mut is_midpoint_gt_even = false;
  let mut is_inexact_lt_midpoint = false;
  let mut is_inexact_gt_midpoint = false;
  let mut tmp_inexact = false;
  let mut tmp64: u64;
  let mut tmp64a: u64;
  let mut tmp64b: u64;
  let mut ten2m1 = Bid128::default();
  let mut second_pass = false;

  // Check for NaN or Infinity.
  if x.w[1] & MASK_SPECIAL == MASK_SPECIAL || y.w[1] & MASK_SPECIAL == MASK_SPECIAL {
    // x is special or y is special
    return if (x.w[1] & MASK_NAN) == MASK_NAN {
      // x is NAN
      // Check first for non-canonical NaN payload.
      if ((x.w[1] & 0x00003fffffffffff) > 0x0000314dc6448d93) || (((x.w[1] & 0x00003fffffffffff) == 0x0000314dc6448d93) && (x.w[0] > 0x38c15b09ffffffff)) {
        x.w[1] &= 0xffffc00000000000;
        x.w[0] = 0x0;
      }
      if (x.w[1] & MASK_SNAN) == MASK_SNAN {
        // x is SNaN
        // Set invalid flag.
        *pfpsf |= BID_INVALID_EXCEPTION;
        // Return quiet (x).
        res.w[1] = x.w[1] & 0xfc003fffffffffff;
        // Clear out also G[6]-G[16].
        res.w[0] = x.w[0];
      } else {
        // x is QNaN
        // Return x.
        res.w[1] = x.w[1] & 0xfc003fffffffffff;
        // Clear out G[6]-G[16]
        res.w[0] = x.w[0];
        // If y = SNaN then signal invalid exception.
        if (y.w[1] & MASK_SNAN) == MASK_SNAN {
          // Set invalid flag.
          *pfpsf |= BID_INVALID_EXCEPTION;
        }
      }
      res
    } else if (y.w[1] & MASK_NAN) == MASK_NAN {
      // y is NAN
      // Check first for non-canonical NaN payload.
      if ((y.w[1] & 0x00003fffffffffff) > 0x0000314dc6448d93) || (((y.w[1] & 0x00003fffffffffff) == 0x0000314dc6448d93) && (y.w[0] > 0x38c15b09ffffffff)) {
        y.w[1] &= 0xffffc00000000000;
        y.w[0] = 0x0;
      }
      if (y.w[1] & MASK_SNAN) == MASK_SNAN {
        // y is SNAN
        // Set invalid flag
        *pfpsf |= BID_INVALID_EXCEPTION;
        // Return quiet (y).
        res.w[1] = y.w[1] & 0xfc003fffffffffff;
        // Clear out also G[6]-G[16].
        res.w[0] = y.w[0];
      } else {
        // y is QNaN
        // Return y.
        res.w[1] = y.w[1] & 0xfc003fffffffffff;
        // Clear out G[6]-G[16]
        res.w[0] = y.w[0];
      }
      res
    } else {
      // Neither x not y is NaN; at least one is infinity.
      if (x.w[1] & MASK_ANY_INF) == MASK_INF {
        // x is infinity.
        if (y.w[1] & MASK_ANY_INF) == MASK_INF {
          // y is infinity
          // If same sign, return either of them.
          if (x.w[1] & MASK_SIGN) == (y.w[1] & MASK_SIGN) {
            res.w[1] = x_sign | MASK_INF;
            res.w[0] = 0x0;
          } else {
            // x and y are infinities of opposite signs.
            // Set invalid flag.
            *pfpsf |= BID_INVALID_EXCEPTION;
            // return QNaN Indefinite
            res.w[1] = 0x7c00000000000000;
            res.w[0] = 0x0000000000000000;
          }
        } else {
          // y is 0 or finite.
          // Return x
          res.w[1] = x_sign | MASK_INF;
          res.w[0] = 0x0;
        }
      } else {
        // x is not NaN or infinity, so y must be infinity.
        res.w[1] = y_sign | MASK_INF;
        res.w[0] = 0x0;
      }
      res
    };
  }
  // Unpack the arguments.

  // Test for non-canonical values:
  //  - values whose encoding begins with x00, x01, or x10 and whose coefficient is larger than 10^34 -1, or
  //  - values whose encoding begins with x1100, x1101, x1110 (if NaNs and infinitis were eliminated already this test is reduced to checking for x10x)

  // Unpack x.
  let mut c1_hi = x.w[1] & MASK_COEFF;
  let mut c1_lo = x.w[0];
  let mut x_exp: Bid64;

  // x is not infinity; check for non-canonical values - treated as zero.
  if (x.w[1] & 0x6000000000000000) == 0x6000000000000000 {
    // G0_G1=11; non-canonical
    x_exp = (x.w[1] << 2) & MASK_EXP; // Biased and shifted left 49 bits.
    c1_hi = 0; // significand high
    c1_lo = 0; // significand low
  } else {
    // G0_G1 != 11
    x_exp = x.w[1] & MASK_EXP; // biased and shifted left 49 bits
    if c1_hi > 0x0001ed09bead87c0 || (c1_hi == 0x0001ed09bead87c0 && c1_lo > 0x378d8e63ffffffff) {
      // x is non-canonical if coefficient is larger than 10^34 -1
      c1_hi = 0;
      c1_lo = 0;
    } else {
      // canonical
    }
  }

  // Unpack y.
  let mut c2_hi = y.w[1] & MASK_COEFF;
  let mut c2_lo = y.w[0];
  let mut y_exp: Bid64;

  // y is not infinity; check for non-canonical values - treated as zero.
  if (y.w[1] & 0x6000000000000000) == 0x6000000000000000 {
    // G0_G1=11; non-canonical
    y_exp = (y.w[1] << 2) & MASK_EXP; // Biased and shifted left 49 bits.
    c2_hi = 0; // significand high
    c2_lo = 0; // significand low
  } else {
    // G0_G1 != 11
    y_exp = y.w[1] & MASK_EXP; // biased and shifted left 49 bits
    if c2_hi > 0x0001ed09bead87c0 || (c2_hi == 0x0001ed09bead87c0 && c2_lo > 0x378d8e63ffffffff) {
      // y is non-canonical if coefficient is larger than 10^34 -1
      c2_hi = 0;
      c2_lo = 0;
    } else {
      // canonical
    }
  }

  if (c1_hi == 0x0) && (c1_lo == 0x0) {
    // x is 0 and y is not special.
    // if y is 0 return 0 with the smaller exponent.
    if (c2_hi == 0x0) && (c2_lo == 0x0) {
      if x_exp < y_exp {
        res.w[1] = x_exp;
      } else {
        res.w[1] = y_exp;
      }
      if x_sign != 0 && y_sign != 0 {
        res.w[1] |= x_sign; // both negative
      } else if rnd_mode == BID_ROUNDING_DOWN && x_sign != y_sign {
        res.w[1] |= 0x8000000000000000; // -0
      }
      // else; // res = +0
      res.w[0] = 0;
    } else {
      // for 0 + y return y, with the preferred exponent
      if y_exp <= x_exp {
        res.w[1] = y.w[1];
        res.w[0] = y.w[0];
      } else {
        // if y_exp > x_exp
        // return (C2 * 10^scale) * 10^(y_exp - scale)
        // where scale = min (P34-q2, y_exp-x_exp)
        // determine q2 = nr. of decimal digits in y
        //  determine first the nr. of bits in y (y_nr_bits)
        if c2_hi == 0 {
          // y_bits is the nr. of bits in c2_lo
          if c2_lo >= 0x0020000000000000 {
            // y >= 2^53
            // Split the 64-bit value in two 32-bit halves to avoid rounding errors.
            y_nr_bits = 32 + bits(c2_lo >> 32);
          } else {
            // if y < 2^53
            y_nr_bits = bits(c2_lo);
          }
        } else {
          // c2_hi != 0 => nr. bits = 64 + nr_bits (c2_hi)
          y_nr_bits = 64 + bits(c2_hi)
        }
        q2 = BID_NR_DIGITS[y_nr_bits].digits;
        if q2 == 0 {
          q2 = BID_NR_DIGITS[y_nr_bits].digits1;
          if c2_hi > BID_NR_DIGITS[y_nr_bits].threshold_hi || (c2_hi == BID_NR_DIGITS[y_nr_bits].threshold_hi && c2_lo >= BID_NR_DIGITS[y_nr_bits].threshold_lo) {
            q2 += 1;
          }
        }
        // return (C2 * 10^scale) * 10^(y_exp - scale)
        // where scale = min (P34-q2, y_exp-x_exp)
        scale = P34 - q2;
        let ind = ((y_exp - x_exp) >> 49) as i32;
        if ind < scale {
          scale = ind;
        }
        if scale == 0 {
          res.w[1] = y.w[1];
          res.w[0] = y.w[0];
        } else if q2 <= 19 {
          // y fits in 64 bits
          if scale <= 19 {
            // 10^scale fits in 64 bits
            // 64 x 64 c2_lo * bid_ten2k64!(scale)
            mul_64x64_to_128mach(&mut res, c2_lo, bid_ten2k64!(scale));
          } else {
            // 10^scale fits in 128 bits
            // 64 x 128 c2_lo * bid_ten2k128(scale - 20]
            mul_128x64_to_128(&mut res, c2_lo, bid_ten2k128(scale - 20));
          }
        } else {
          // y fits in 128 bits, but 10^scale must fit in 64 bits
          // 64 x 128 bid_ten2k64!(scale) * C2
          c2.w[1] = c2_hi;
          c2.w[0] = c2_lo;
          mul_128x64_to_128(&mut res, bid_ten2k64!(scale), c2);
        }
        // subtract scale from the exponent
        y_exp = y_exp.wrapping_sub((scale as u64) << 49);
        res.w[1] = res.w[1] | y_sign | y_exp;
      }
    }
    res
  } else if (c2_hi == 0x0) && (c2_lo == 0x0) {
    // y is 0 and x is not special, and not zero.
    // For x + 0 return x, with the preferred exponent.
    if x_exp <= y_exp {
      res.w[1] = x.w[1];
      res.w[0] = x.w[0];
    } else {
      // if x_exp > y_exp
      // return (C1 * 10^scale) * 10^(x_exp - scale)
      // where scale = min (P34-q1, x_exp-y_exp)
      // determine q1 = nr. of decimal digits in x
      //  determine first the nr. of bits in x
      if c1_hi == 0 {
        // x_bits is the nr. of bits in c1_lo
        if c1_lo >= 0x0020000000000000 {
          // x >= 2^53
          // split the 64-bit value in two 32-bit halves to avoid
          // rounding errors
          x_nr_bits = 32 + bits(c1_lo >> 32);
        } else {
          // if x < 2^53
          x_nr_bits = bits(c1_lo);
        }
      } else {
        // c1_hi != 0 => nr. bits = 64 + nr_bits (c1_hi)
        x_nr_bits = 64 + bits(c1_hi);
      }
      q1 = BID_NR_DIGITS[x_nr_bits].digits;
      if q1 == 0 {
        q1 = BID_NR_DIGITS[x_nr_bits].digits1;
        if c1_hi > BID_NR_DIGITS[x_nr_bits].threshold_hi || (c1_hi == BID_NR_DIGITS[x_nr_bits].threshold_hi && c1_lo >= BID_NR_DIGITS[x_nr_bits].threshold_lo) {
          q1 = q1.wrapping_add(1);
        }
      }
      // return (C1 * 10^scale) * 10^(x_exp - scale)
      // where scale = min (P34-q1, x_exp-y_exp)
      scale = P34 - q1;
      let ind = ((x_exp - y_exp) >> 49) as i32;
      if ind < scale {
        scale = ind;
      }
      if scale == 0 {
        res.w[1] = x.w[1];
        res.w[0] = x.w[0];
      } else if q1 <= 19 {
        // x fits in 64 bits
        if scale <= 19 {
          // 10^scale fits in 64 bits
          // 64 x 64 c1_lo * bid_ten2k64!(scale)
          mul_64x64_to_128mach(&mut res, c1_lo, bid_ten2k64!(scale));
        } else {
          // 10^scale fits in 128 bits
          // 64 x 128 c1_lo * bid_ten2k128(scale - 20]
          mul_128x64_to_128(&mut res, c1_lo, bid_ten2k128(scale - 20));
        }
      } else {
        // x fits in 128 bits, but 10^scale must fit in 64 bits
        // 64 x 128 bid_ten2k64!(scale) * C1
        c1.w[1] = c1_hi;
        c1.w[0] = c1_lo;
        mul_128x64_to_128(&mut res, bid_ten2k64!(scale), c1);
      }
      // subtract scale from the exponent
      x_exp = x_exp.wrapping_sub((scale as u64) << 49);
      res.w[1] = res.w[1] | x_sign | x_exp;
    }
    res
  } else {
    // x and y are not canonical, not special, and are not zero.
    // Note that the result may still be zero, and then it has
    // to have the preferred exponent.
    if x_exp < y_exp {
      // If exp_x < exp_y then swap x and y.
      let tmp_sign = x_sign;
      let tmp_exp = x_exp;
      let tmp_signif_hi = c1_hi;
      let tmp_signif_lo = c1_lo;
      x_sign = y_sign;
      x_exp = y_exp;
      c1_hi = c2_hi;
      c1_lo = c2_lo;
      y_sign = tmp_sign;
      y_exp = tmp_exp;
      c2_hi = tmp_signif_hi;
      c2_lo = tmp_signif_lo;
    }
    // q1 is the number of decimal digits in x.
    // Determine first the number of bits in x.
    if c1_hi == 0 {
      // x_nr_bits is the number of bits in c1_lo
      if c1_lo >= 0x0020000000000000 {
        // x >= 2^53
        // Split the 64-bit value in two 32-bit halves to avoid rounding errors.
        x_nr_bits = 32 + bits(c1_lo >> 32)
      } else {
        // if x < 2^53
        x_nr_bits = bits(c1_lo);
      }
    } else {
      // when c1_hi != 0 then the number of bits = 64 + nr_bits (c1_hi)
      x_nr_bits = 64 + bits(c1_hi);
    }

    q1 = BID_NR_DIGITS[x_nr_bits].digits;
    if q1 == 0 {
      q1 = BID_NR_DIGITS[x_nr_bits].digits1;
      if c1_hi > BID_NR_DIGITS[x_nr_bits].threshold_hi || (c1_hi == BID_NR_DIGITS[x_nr_bits].threshold_hi && c1_lo >= BID_NR_DIGITS[x_nr_bits].threshold_lo) {
        q1 = q1.wrapping_add(1);
      }
    }
    // q2 is the number  of decimal digits in y.
    // Determine first the number of bits in y (y_nr_bits)
    if c2_hi == 0 {
      // y_nr_bits is the number of bits in c2_lo
      if c2_lo >= 0x0020000000000000 {
        // y >= 2^53
        // Split the 64-bit value in two 32-bit halves to avoid rounding errors.
        y_nr_bits = 32 + bits(c2_lo >> 32);
      } else {
        // if y < 2^53
        y_nr_bits = bits(c2_lo);
      }
    } else {
      // If c2_hi != 0 then the number of bits = 64 + nr_bits (c2_hi)
      y_nr_bits = 64 + bits(c2_hi);
    }

    q2 = BID_NR_DIGITS[y_nr_bits].digits;
    if q2 == 0 {
      q2 = BID_NR_DIGITS[y_nr_bits].digits1;
      if c2_hi > BID_NR_DIGITS[y_nr_bits].threshold_hi || (c2_hi == BID_NR_DIGITS[y_nr_bits].threshold_hi && c2_lo >= BID_NR_DIGITS[y_nr_bits].threshold_lo) {
        q2 = q2.wrapping_add(1);
      }
    }

    delta = q1 + (x_exp >> 49) as i32 - q2 - (y_exp >> 49) as i32;

    if delta >= P34 {
      // round the result directly because 0 < C2 < ulp (C1 * 10^(x_exp-e2))
      // n = C1 * 10^e1 or n = C1 +/- 10^(q1-P34)) * 10^e1
      // the result is inexact; the preferred exponent is the least possible

      if delta > P34 {
        // for RN the result is the operand with the larger magnitude,
        // possibly scaled up by 10^(P34-q1)
        // an overflow cannot occur in this case (rounding to nearest)
        if q1 < P34 {
          // scale C1 up by 10^(P34-q1)
          // Note: because delta >= P34+1 it is certain that
          //     x_exp - ((BID_UINT64)scale << 49) will stay above e_min
          scale = P34 - q1;
          if q1 <= 19 {
            // C1 fits in 64 bits
            // 1 <= q1 <= 19 => 15 <= scale <= 33
            if scale <= 19 {
              // 10^scale fits in 64 bits
              mul_64x64_to_128mach(&mut c1, bid_ten2k64!(scale), c1_lo);
            } else {
              // if 20 <= scale <= 33
              // C1 * 10^scale = (C1 * 10^(scale-19)) * 10^19 where
              // (C1 * 10^(scale-19)) fits in 64 bits
              c1_lo *= bid_ten2k64!(scale - 19);
              mul_64x64_to_128mach(&mut c1, bid_ten2k64!(19), c1_lo);
            }
          } else {
            //if 20 <= q1 <= 33=P34-1 then C1 fits only in 128 bits
            // => 1 <= P34 - q1 <= 14 so 10^(P34-q1) fits in 64 bits
            c1.w[1] = c1_hi;
            c1.w[0] = c1_lo;
            let c1copy = c1;
            mul_128x64_to_128(&mut c1, bid_ten2k64!(P34 - q1), c1copy);
            // C1 = bid_ten2k64!(P34 - q1) * C1
          }
          x_exp = x_exp.wrapping_sub((scale as u64) << 49);
          c1_hi = c1.w[1];
          c1_lo = c1.w[0];
        }
        // some special cases arise: if delta = P34 + 1 and C1 = 10^(P34-1)
        // (after scaling) and x_sign != y_sign and C2 > 5*10^(q2-1) =>
        // subtract 1 ulp
        // Note: do this only for rounding to nearest; for other rounding
        // modes the correction will be applied next
        if (rnd_mode == BID_ROUNDING_TO_NEAREST || rnd_mode == BID_ROUNDING_TIES_AWAY)
          && delta == (P34 + 1)
          && c1_hi == 0x0000314dc6448d93
          && c1_lo == 0x38c15b0a00000000
          && x_sign != y_sign
          && ((q2 <= 19 && c2_lo > bid_midpoint64(q2 - 1)) || (q2 >= 20 && (c2_hi > bid_midpoint128(q2 - 20).w[1] || (c2_hi == bid_midpoint128(q2 - 20).w[1] && c2_lo > bid_midpoint128(q2 - 20).w[0]))))
        {
          // C1 = 10^34 - 1 and decrement x_exp by 1 (no underflow possible)
          c1_hi = 0x0001ed09bead87c0;
          c1_lo = 0x378d8e63ffffffff;
          x_exp = x_exp.wrapping_sub(EXP_P1);
        }
        if rnd_mode != BID_ROUNDING_TO_NEAREST {
          if (rnd_mode == BID_ROUNDING_DOWN && x_sign > 0 && y_sign > 0) || (rnd_mode == BID_ROUNDING_UP && x_sign == 0 && y_sign == 0) {
            // add 1 ulp and then check for overflow
            c1_lo = c1_lo.wrapping_add(1);
            if c1_lo == 0 {
              // rounding overflow in the low 64 bits
              c1_hi = c1_hi.wrapping_add(1);
            }
            if c1_hi == 0x0001ed09bead87c0 && c1_lo == 0x378d8e6400000000 {
              // C1 = 10^34 => rounding overflow
              c1_hi = 0x0000314dc6448d93;
              c1_lo = 0x38c15b0a00000000; // 10^33
              x_exp = x_exp.wrapping_add(EXP_P1);
              if x_exp == EXP_MAX_P1 {
                // overflow
                c1_hi = 0x7800000000000000; // +inf
                c1_lo = 0x0;
                x_exp = 0; // x_sign is preserved
                // set overflow flag (the inexact flag was set too)
                *pfpsf |= BID_OVERFLOW_EXCEPTION;
              }
            }
          } else if (rnd_mode == BID_ROUNDING_DOWN && x_sign == 0 && y_sign > 0) || (rnd_mode == BID_ROUNDING_UP && x_sign > 0 && y_sign == 0) || (rnd_mode == BID_ROUNDING_TO_ZERO && x_sign != y_sign) {
            // subtract 1 ulp from C1
            // Note: because delta >= P34 + 1 the result cannot be zero
            c1_lo = c1_lo.wrapping_sub(1);
            if c1_lo == 0xffffffffffffffff {
              c1_hi = c1_hi.wrapping_sub(1);
            }
            // if the coefficient is 10^33 - 1 then make it 10^34 - 1 and
            // decrease the exponent by 1 (because delta >= P34 + 1 the
            // exponent will not become less than e_min)
            // 10^33 - 1 = 0x0000314dc6448d9338c15b09ffffffff
            // 10^34 - 1 = 0x0001ed09bead87c0378d8e63ffffffff
            if c1_hi == 0x0000314dc6448d93 && c1_lo == 0x38c15b09ffffffff {
              // make C1 = 10^34  - 1
              c1_hi = 0x0001ed09bead87c0;
              c1_lo = 0x378d8e63ffffffff;
              x_exp = x_exp.wrapping_sub(EXP_P1);
            }
          } else { // the result is already correct
          }
        }
        // set the inexact flag
        *pfpsf |= BID_INEXACT_EXCEPTION;
        // assemble the result
        res.w[1] = x_sign | x_exp | c1_hi;
        res.w[0] = c1_lo;
      } else {
        // delta = P34
        // in most cases, the smaller operand may be < or = or > 1/2 ulp of the
        // larger operand
        // however, the case C1 = 10^(q1-1) and x_sign != y_sign is special due
        // to accuracy loss after subtraction, and will be treated separately
        if x_sign == y_sign || (q1 <= 20 && (c1_hi != 0 || c1_lo != bid_ten2k64!(q1 - 1))) || (q1 >= 21 && (c1_hi != bid_ten2k128(q1 - 21).w[1] || c1_lo != bid_ten2k128(q1 - 21).w[0])) {
          // if x_sign == y_sign or C1 != 10^(q1-1)
          // compare C2 with 1/2 ulp = 5 * 10^(q2-1), the latter read from table
          // Note: cases q1<=19 and q1>=20 can be coalesced at some latency cost
          if q2 <= 19 {
            // C2 and 5*10^(q2-1) both fit in 64 bits
            halfulp64 = bid_midpoint64(q2 - 1); // 5 * 10^(q2-1)
            if c2_lo < halfulp64 {
              // n2 < 1/2 ulp (n1)
              // for RN the result is the operand with the larger magnitude,
              // possibly scaled up by 10^(P34-q1)
              // an overflow cannot occur in this case (rounding to nearest)
              if q1 < P34 {
                // scale C1 up by 10^(P34-q1)
                // Note: because delta = P34 it is certain that
                //     x_exp - ((BID_UINT64)scale << 49) will stay above e_min
                scale = P34 - q1;
                if q1 <= 19 {
                  // C1 fits in 64 bits
                  // 1 <= q1 <= 19 => 15 <= scale <= 33
                  if scale <= 19 {
                    // 10^scale fits in 64 bits
                    mul_64x64_to_128mach(&mut c1, bid_ten2k64!(scale), c1_lo);
                  } else {
                    // if 20 <= scale <= 33
                    // C1 * 10^scale = (C1 * 10^(scale-19)) * 10^19 where
                    // (C1 * 10^(scale-19)) fits in 64 bits
                    c1_lo *= bid_ten2k64!(scale - 19);
                    mul_64x64_to_128mach(&mut c1, bid_ten2k64!(19), c1_lo);
                  }
                } else {
                  //if 20 <= q1 <= 33=P34-1 then C1 fits only in 128 bits
                  // => 1 <= P34 - q1 <= 14 so 10^(P34-q1) fits in 64 bits
                  c1.w[1] = c1_hi;
                  c1.w[0] = c1_lo;
                  // C1 = bid_ten2k64!(P34 - q1] * C1
                  let c1copy = c1;
                  mul_128x64_to_128(&mut c1, bid_ten2k64!(P34 - q1), c1copy);
                }
                x_exp = x_exp.wrapping_add((scale as u64) << 49);
                c1_hi = c1.w[1];
                c1_lo = c1.w[0];
              }
              if rnd_mode != BID_ROUNDING_TO_NEAREST {
                if (rnd_mode == BID_ROUNDING_DOWN && x_sign > 0 && y_sign > 0) || (rnd_mode == BID_ROUNDING_UP && x_sign == 0 && y_sign == 0) {
                  // add 1 ulp and then check for overflow
                  c1_lo = c1_lo.wrapping_add(1);
                  if c1_lo == 0 {
                    // rounding overflow in the low 64 bits
                    c1_hi = c1_hi.wrapping_add(1);
                  }
                  if c1_hi == 0x0001ed09bead87c0 && c1_lo == 0x378d8e6400000000 {
                    // C1 = 10^34 => rounding overflow
                    c1_hi = 0x0000314dc6448d93;
                    c1_lo = 0x38c15b0a00000000; // 10^33
                    x_exp = x_exp.wrapping_add(EXP_P1);
                    if x_exp == EXP_MAX_P1 {
                      // overflow
                      c1_hi = 0x7800000000000000; // +inf
                      c1_lo = 0x0;
                      x_exp = 0; // x_sign is preserved
                      // set overflow flag (the inexact flag was set too)
                      *pfpsf |= BID_OVERFLOW_EXCEPTION;
                    }
                  }
                } else if (rnd_mode == BID_ROUNDING_DOWN && x_sign == 0 && y_sign > 0) || (rnd_mode == BID_ROUNDING_UP && x_sign > 0 && y_sign == 0) || (rnd_mode == BID_ROUNDING_TO_ZERO && x_sign != y_sign) {
                  // subtract 1 ulp from C1
                  // Note: because delta >= P34 + 1 the result cannot be zero
                  c1_lo = c1_lo.wrapping_sub(1);
                  if c1_lo == 0xffffffffffffffff {
                    c1_hi = c1_hi.wrapping_sub(1);
                  }
                  // if the coefficient is 10^33-1 then make it 10^34-1 and
                  // decrease the exponent by 1 (because delta >= P34 + 1 the
                  // exponent will not become less than e_min)
                  // 10^33 - 1 = 0x0000314dc6448d9338c15b09ffffffff
                  // 10^34 - 1 = 0x0001ed09bead87c0378d8e63ffffffff
                  if c1_hi == 0x0000314dc6448d93 && c1_lo == 0x38c15b09ffffffff {
                    // make C1 = 10^34  - 1
                    c1_hi = 0x0001ed09bead87c0;
                    c1_lo = 0x378d8e63ffffffff;
                    x_exp = x_exp.wrapping_sub(EXP_P1);
                  }
                } else {
                  // the result is already correct
                }
              }
              // set the inexact flag
              *pfpsf |= BID_INEXACT_EXCEPTION;
              // assemble the result
              res.w[1] = x_sign | x_exp | c1_hi;
              res.w[0] = c1_lo;
            } else if (c2_lo == halfulp64) && (q1 < P34 || ((c1_lo & 0x1) == 0)) {
              // n2 = 1/2 ulp (n1) and q1 < P34 or C1 is even
              // the result is the operand with the larger magnitude,
              // possibly scaled up by 10^(P34-q1)
              // an overflow cannot occur in this case (rounding to nearest)
              if q1 < P34 {
                // scale C1 up by 10^(P34-q1)
                // Note: because delta = P34 it is certain that
                //     x_exp - ((BID_UINT64)scale << 49) will stay above e_min
                scale = P34 - q1;
                if q1 <= 19 {
                  // C1 fits in 64 bits
                  // 1 <= q1 <= 19 => 15 <= scale <= 33
                  if scale <= 19 {
                    // 10^scale fits in 64 bits
                    mul_64x64_to_128mach(&mut c1, bid_ten2k64!(scale), c1_lo);
                  } else {
                    // if 20 <= scale <= 33
                    // C1 * 10^scale = (C1 * 10^(scale-19)) * 10^19 where
                    // (C1 * 10^(scale-19)) fits in 64 bits
                    c1_lo *= bid_ten2k64!(scale - 19);
                    mul_64x64_to_128mach(&mut c1, bid_ten2k64!(19), c1_lo);
                  }
                } else {
                  //if 20 <= q1 <= 33=P34-1 then C1 fits only in 128 bits
                  // => 1 <= P34 - q1 <= 14 so 10^(P34-q1) fits in 64 bits
                  c1.w[1] = c1_hi;
                  c1.w[0] = c1_lo;
                  // C1 = bid_ten2k64!(P34 - q1] * C1
                  let c1copy = c1;
                  mul_128x64_to_128(&mut c1, bid_ten2k64!(P34 - q1), c1copy);
                }
                x_exp = x_exp.wrapping_sub((scale as u64) << 49);
                c1_hi = c1.w[1];
                c1_lo = c1.w[0];
              }
              if (rnd_mode == BID_ROUNDING_TO_NEAREST && x_sign == y_sign && (c1_lo & 0x01) > 0)
                || (rnd_mode == BID_ROUNDING_TIES_AWAY && x_sign == y_sign)
                || (rnd_mode == BID_ROUNDING_UP && x_sign == 0 && y_sign == 0)
                || (rnd_mode == BID_ROUNDING_DOWN && x_sign > 0 && y_sign > 0)
              {
                // add 1 ulp and then check for overflow
                c1_lo = c1_lo.wrapping_add(1);
                if c1_lo == 0 {
                  // rounding overflow in the low 64 bits
                  c1_hi = c1_hi.wrapping_add(1);
                }
                if c1_hi == 0x0001ed09bead87c0 && c1_lo == 0x378d8e6400000000 {
                  // C1 = 10^34 => rounding overflow
                  c1_hi = 0x0000314dc6448d93;
                  c1_lo = 0x38c15b0a00000000; // 10^33
                  x_exp = x_exp.wrapping_add(EXP_P1);
                  if x_exp == EXP_MAX_P1 {
                    // overflow
                    c1_hi = 0x7800000000000000; // +inf
                    c1_lo = 0x0;
                    x_exp = 0; // x_sign is preserved
                    // set overflow flag (the inexact flag was set too)
                    *pfpsf |= BID_OVERFLOW_EXCEPTION;
                  }
                }
              } else if (rnd_mode == BID_ROUNDING_TO_NEAREST && x_sign != y_sign && (c1_lo & 0x01) > 0)
                || (rnd_mode == BID_ROUNDING_DOWN && x_sign == 0 && y_sign > 0)
                || (rnd_mode == BID_ROUNDING_UP && x_sign > 0 && y_sign == 0)
                || (rnd_mode == BID_ROUNDING_TO_ZERO && x_sign != y_sign)
              {
                // subtract 1 ulp from C1
                // Note: because delta >= P34 + 1 the result cannot be zero
                c1_lo = c1_lo.wrapping_sub(1);
                if c1_lo == 0xffffffffffffffff {
                  c1_hi = c1_hi.wrapping_sub(1);
                }
                // if the coefficient is 10^33 - 1 then make it 10^34 - 1
                // and decrease the exponent by 1 (because delta >= P34 + 1
                // the exponent will not become less than e_min)
                // 10^33 - 1 = 0x0000314dc6448d9338c15b09ffffffff
                // 10^34 - 1 = 0x0001ed09bead87c0378d8e63ffffffff
                if c1_hi == 0x0000314dc6448d93 && c1_lo == 0x38c15b09ffffffff {
                  // make C1 = 10^34  - 1
                  c1_hi = 0x0001ed09bead87c0;
                  c1_lo = 0x378d8e63ffffffff;
                  x_exp = x_exp.wrapping_sub(EXP_P1);
                }
              } else {
                // the result is already correct
              }
              // set the inexact flag
              *pfpsf |= BID_INEXACT_EXCEPTION;
              // assemble the result
              res.w[1] = x_sign | x_exp | c1_hi;
              res.w[0] = c1_lo;
            } else {
              // if c2_lo > halfulp64 ||
              // (c2_lo == halfulp64 && q1 == P34 && ((c1_lo & 0x1) == 1)), i.e.
              // 1/2 ulp(n1) < n2 < 1 ulp(n1) or n2 = 1/2 ulp(n1) and C1 odd
              // res = x+1 ulp if n1*n2 > 0 and res = x-1 ulp if n1*n2 < 0
              if q1 < P34 {
                // then 1 ulp = 10^(e1+q1-P34) < 10^e1
                // Note: if (q1 == P34) then 1 ulp = 10^(e1+q1-P34) = 10^e1
                // because q1 < P34 we must first replace C1 by
                // C1 * 10^(P34-q1), and must decrease the exponent by
                // (P34-q1) (it will still be at least e_min)
                scale = P34 - q1;
                if q1 <= 19 {
                  // C1 fits in 64 bits
                  // 1 <= q1 <= 19 => 15 <= scale <= 33
                  if scale <= 19 {
                    // 10^scale fits in 64 bits
                    mul_64x64_to_128mach(&mut c1, bid_ten2k64!(scale), c1_lo);
                  } else {
                    // if 20 <= scale <= 33
                    // C1 * 10^scale = (C1 * 10^(scale-19)) * 10^19 where
                    // (C1 * 10^(scale-19)) fits in 64 bits
                    c1_lo *= bid_ten2k64!(scale - 19);
                    mul_64x64_to_128mach(&mut c1, bid_ten2k64!(19), c1_lo);
                  }
                } else {
                  //if 20 <= q1 <= 33=P34-1 then C1 fits only in 128 bits
                  // => 1 <= P34 - q1 <= 14 so 10^(P34-q1) fits in 64 bits
                  c1.w[1] = c1_hi;
                  c1.w[0] = c1_lo;
                  // C1 = bid_ten2k64!(P34 - q1] * C1
                  let c1copy = c1;
                  mul_128x64_to_128(&mut c1, bid_ten2k64!(P34 - q1), c1copy);
                }
                x_exp = x_exp.wrapping_sub((scale as u64) << 49);
                c1_hi = c1.w[1];
                c1_lo = c1.w[0];
                // check for rounding overflow
                if c1_hi == 0x0001ed09bead87c0 && c1_lo == 0x378d8e6400000000 {
                  // C1 = 10^34 => rounding overflow
                  c1_hi = 0x0000314dc6448d93;
                  c1_lo = 0x38c15b0a00000000; // 10^33
                  x_exp = x_exp.wrapping_add(EXP_P1);
                }
              }
              if (rnd_mode == BID_ROUNDING_TO_NEAREST && x_sign != y_sign)
                || (rnd_mode == BID_ROUNDING_TIES_AWAY && x_sign != y_sign && c2_lo != halfulp64)
                || (rnd_mode == BID_ROUNDING_DOWN && x_sign == 0 && y_sign > 0)
                || (rnd_mode == BID_ROUNDING_UP && x_sign > 0 && y_sign == 0)
                || (rnd_mode == BID_ROUNDING_TO_ZERO && x_sign != y_sign)
              {
                // the result is x - 1
                // for RN n1 * n2 < 0; underflow not possible
                c1_lo = c1_lo.wrapping_sub(1);
                if c1_lo == 0xffffffffffffffff {
                  c1_hi = c1_hi.wrapping_sub(1);
                }
                // check if we crossed into the lower decade
                if c1_hi == 0x0000314dc6448d93 && c1_lo == 0x38c15b09ffffffff {
                  // 10^33 - 1
                  c1_hi = 0x0001ed09bead87c0; // 10^34 - 1
                  c1_lo = 0x378d8e63ffffffff;
                  x_exp = x_exp.wrapping_sub(EXP_P1); // no underflow, because n1 >> n2
                }
              } else if !(x_sign != y_sign || rnd_mode != BID_ROUNDING_TO_NEAREST && rnd_mode != BID_ROUNDING_TIES_AWAY)
                || (rnd_mode == BID_ROUNDING_DOWN && x_sign > 0 && y_sign > 0)
                || (rnd_mode == BID_ROUNDING_UP && x_sign == 0 && y_sign == 0)
              {
                // the result is x + 1
                // for RN x_sign = y_sign, i.e. n1*n2 > 0
                c1_lo = c1_lo.wrapping_add(1);
                if c1_lo == 0 {
                  // rounding overflow in the low 64 bits
                  c1_hi = c1_hi.wrapping_add(1);
                }
                if c1_hi == 0x0001ed09bead87c0 && c1_lo == 0x378d8e6400000000 {
                  // C1 = 10^34 => rounding overflow
                  c1_hi = 0x0000314dc6448d93;
                  c1_lo = 0x38c15b0a00000000; // 10^33
                  x_exp = x_exp.wrapping_add(EXP_P1);
                  if x_exp == EXP_MAX_P1 {
                    // overflow
                    c1_hi = 0x7800000000000000; // +inf
                    c1_lo = 0x0;
                    x_exp = 0; // x_sign is preserved
                    // set the overflow flag
                    *pfpsf |= BID_OVERFLOW_EXCEPTION;
                  }
                }
              } else { // the result is x
              }
              // set the inexact flag
              *pfpsf |= BID_INEXACT_EXCEPTION;
              // assemble the result
              res.w[1] = x_sign | x_exp | c1_hi;
              res.w[0] = c1_lo;
            }
          } else {
            // if q2 >= 20 then 5*10^(q2-1) and C2 (the latter in
            // most cases) fit only in more than 64 bits
            halfulp128 = bid_midpoint128(q2 - 20); // 5 * 10^(q2-1)
            if (c2_hi < halfulp128.w[1]) || (c2_hi == halfulp128.w[1] && c2_lo < halfulp128.w[0]) {
              // n2 < 1/2 ulp (n1)
              // the result is the operand with the larger magnitude,
              // possibly scaled up by 10^(P34-q1)
              // an overflow cannot occur in this case (rounding to nearest)
              if q1 < P34 {
                // scale C1 up by 10^(P34-q1)
                // Note: because delta = P34 it is certain that
                //     x_exp - ((BID_UINT64)scale << 49) will stay above e_min
                scale = P34 - q1;
                if q1 <= 19 {
                  // C1 fits in 64 bits
                  // 1 <= q1 <= 19 => 15 <= scale <= 33
                  if scale <= 19 {
                    // 10^scale fits in 64 bits
                    mul_64x64_to_128mach(&mut c1, bid_ten2k64!(scale), c1_lo);
                  } else {
                    // if 20 <= scale <= 33
                    // C1 * 10^scale = (C1 * 10^(scale-19)) * 10^19 where
                    // (C1 * 10^(scale-19)) fits in 64 bits
                    c1_lo *= bid_ten2k64!(scale - 19);
                    mul_64x64_to_128mach(&mut c1, bid_ten2k64!(19), c1_lo);
                  }
                } else {
                  //if 20 <= q1 <= 33=P34-1 then C1 fits only in 128 bits
                  // => 1 <= P34 - q1 <= 14 so 10^(P34-q1) fits in 64 bits
                  c1.w[1] = c1_hi;
                  c1.w[0] = c1_lo;
                  // C1 = bid_ten2k64!(P34 - q1] * C1
                  let c1copy = c1;
                  mul_128x64_to_128(&mut c1, bid_ten2k64!(P34 - q1), c1copy);
                }
                c1_hi = c1.w[1];
                c1_lo = c1.w[0];
                x_exp = x_exp.wrapping_sub((scale as u64) << 49);
              }
              if rnd_mode != BID_ROUNDING_TO_NEAREST {
                if (rnd_mode == BID_ROUNDING_DOWN && x_sign > 0 && y_sign > 0) || (rnd_mode == BID_ROUNDING_UP && x_sign == 0 && y_sign == 0) {
                  // add 1 ulp and then check for overflow
                  c1_lo = c1_lo.wrapping_add(1);
                  if c1_lo == 0 {
                    // rounding overflow in the low 64 bits
                    c1_hi = c1_hi.wrapping_add(1);
                  }
                  if c1_hi == 0x0001ed09bead87c0 && c1_lo == 0x378d8e6400000000 {
                    // C1 = 10^34 => rounding overflow
                    c1_hi = 0x0000314dc6448d93;
                    c1_lo = 0x38c15b0a00000000; // 10^33
                    x_exp = x_exp.wrapping_add(EXP_P1);
                    if x_exp == EXP_MAX_P1 {
                      // overflow
                      c1_hi = 0x7800000000000000; // +inf
                      c1_lo = 0x0;
                      x_exp = 0; // x_sign is preserved
                      // set overflow flag (the inexact flag was set too)
                      *pfpsf |= BID_OVERFLOW_EXCEPTION;
                    }
                  }
                } else if (rnd_mode == BID_ROUNDING_DOWN && x_sign == 0 && y_sign > 0) || (rnd_mode == BID_ROUNDING_UP && x_sign > 0 && y_sign == 0) || (rnd_mode == BID_ROUNDING_TO_ZERO && x_sign != y_sign) {
                  // subtract 1 ulp from C1
                  // Note: because delta >= P34 + 1 the result cannot be zero
                  c1_lo = c1_lo.wrapping_sub(1);
                  if c1_lo == 0xffffffffffffffff {
                    c1_hi = c1_hi.wrapping_sub(1);
                  }
                  // if the coefficient is 10^33-1 then make it 10^34-1 and
                  // decrease the exponent by 1 (because delta >= P34 + 1 the
                  // exponent will not become less than e_min)
                  // 10^33 - 1 = 0x0000314dc6448d9338c15b09ffffffff
                  // 10^34 - 1 = 0x0001ed09bead87c0378d8e63ffffffff
                  if c1_hi == 0x0000314dc6448d93 && c1_lo == 0x38c15b09ffffffff {
                    // make C1 = 10^34  - 1
                    c1_hi = 0x0001ed09bead87c0;
                    c1_lo = 0x378d8e63ffffffff;
                    x_exp = x_exp.wrapping_sub(EXP_P1);
                  }
                } else { // the result is already correct
                }
              }
              // set the inexact flag
              *pfpsf |= BID_INEXACT_EXCEPTION;
              // assemble the result
              res.w[1] = x_sign | x_exp | c1_hi;
              res.w[0] = c1_lo;
            } else if (c2_hi == halfulp128.w[1] && c2_lo == halfulp128.w[0]) && (q1 < P34 || ((c1_lo & 0x1) == 0)) {
              // set the inexact flag
              // midpoint & lsb in C1 is 0
              // n2 = 1/2 ulp (n1) and C1 is even
              // the result is the operand with the larger magnitude,
              // possibly scaled up by 10^(P34-q1)
              // an overflow cannot occur in this case (rounding to nearest)
              if q1 < P34 {
                // scale C1 up by 10^(P34-q1)
                // Note: because delta = P34 it is certain that
                //     x_exp - ((BID_UINT64)scale << 49) will stay above e_min
                scale = P34 - q1;
                if q1 <= 19 {
                  // C1 fits in 64 bits
                  // 1 <= q1 <= 19 => 15 <= scale <= 33
                  if scale <= 19 {
                    // 10^scale fits in 64 bits
                    mul_64x64_to_128mach(&mut c1, bid_ten2k64!(scale), c1_lo);
                  } else {
                    // if 20 <= scale <= 33
                    // C1 * 10^scale = (C1 * 10^(scale-19)) * 10^19 where
                    // (C1 * 10^(scale-19)) fits in 64 bits
                    c1_lo *= bid_ten2k64!(scale - 19);
                    mul_64x64_to_128mach(&mut c1, bid_ten2k64!(19), c1_lo);
                  }
                } else {
                  //if 20 <= q1 <= 33=P34-1 then C1 fits only in 128 bits
                  // => 1 <= P34 - q1 <= 14 so 10^(P34-q1) fits in 64 bits
                  c1.w[1] = c1_hi;
                  c1.w[0] = c1_lo;
                  // C1 = bid_ten2k64!(P34 - q1] * C1
                  let c1copy = c1;
                  mul_128x64_to_128(&mut c1, bid_ten2k64!(P34 - q1), c1copy);
                }
                x_exp = x_exp.wrapping_sub((scale as u64) << 49);
                c1_hi = c1.w[1];
                c1_lo = c1.w[0];
              }
              if rnd_mode != BID_ROUNDING_TO_NEAREST {
                if (rnd_mode == BID_ROUNDING_TIES_AWAY && x_sign == y_sign) || (rnd_mode == BID_ROUNDING_UP && x_sign == 0 && y_sign == 0) || (rnd_mode == BID_ROUNDING_DOWN && x_sign > 0 && y_sign > 0) {
                  // add 1 ulp and then check for overflow
                  c1_lo = c1_lo.wrapping_add(1);
                  if c1_lo == 0 {
                    // rounding overflow in the low 64 bits
                    c1_hi = c1_hi.wrapping_add(1);
                  }
                  if c1_hi == 0x0001ed09bead87c0 && c1_lo == 0x378d8e6400000000 {
                    // C1 = 10^34 => rounding overflow
                    c1_hi = 0x0000314dc6448d93;
                    c1_lo = 0x38c15b0a00000000; // 10^33
                    x_exp = x_exp.wrapping_add(EXP_P1);
                    if x_exp == EXP_MAX_P1 {
                      // overflow
                      c1_hi = 0x7800000000000000; // +inf
                      c1_lo = 0x0;
                      x_exp = 0; // x_sign is preserved
                      // set overflow flag (the inexact flag was set too)
                      *pfpsf |= BID_OVERFLOW_EXCEPTION;
                    }
                  }
                } else if (rnd_mode == BID_ROUNDING_DOWN && x_sign == 0 && y_sign == 0) || (rnd_mode == BID_ROUNDING_UP && x_sign > 0 && y_sign == 0) || (rnd_mode == BID_ROUNDING_TO_ZERO && x_sign != y_sign) {
                  // subtract 1 ulp from C1
                  // Note: because delta >= P34 + 1 the result cannot be zero
                  c1_lo = c1_lo.wrapping_sub(1);
                  if c1_lo == 0xffffffffffffffff {
                    c1_hi = c1_hi.wrapping_sub(1);
                  }
                  // if the coefficient is 10^33 - 1 then make it 10^34 - 1
                  // and decrease the exponent by 1 (because delta >= P34 + 1
                  // the exponent will not become less than e_min)
                  // 10^33 - 1 = 0x0000314dc6448d9338c15b09ffffffff
                  // 10^34 - 1 = 0x0001ed09bead87c0378d8e63ffffffff
                  if c1_hi == 0x0000314dc6448d93 && c1_lo == 0x38c15b09ffffffff {
                    // make C1 = 10^34  - 1
                    c1_hi = 0x0001ed09bead87c0;
                    c1_lo = 0x378d8e63ffffffff;
                    x_exp = x_exp.wrapping_sub(EXP_P1);
                  }
                } else { // the result is already correct
                }
              }
              // set the inexact flag
              *pfpsf |= BID_INEXACT_EXCEPTION;
              // assemble the result
              res.w[1] = x_sign | x_exp | c1_hi;
              res.w[0] = c1_lo;
            } else {
              // if C2 > halfulp128 ||
              // (C2 == halfulp128 && q1 == P34 && ((C1 & 0x1) == 1)), i.e.
              // 1/2 ulp(n1) < n2 < 1 ulp(n1) or n2 = 1/2 ulp(n1) and C1 odd
              // res = x+1 ulp if n1*n2 > 0 and res = x-1 ulp if n1*n2 < 0
              if q1 < P34 {
                // then 1 ulp = 10^(e1+q1-P34) < 10^e1
                // Note: if (q1 == P34) then 1 ulp = 10^(e1+q1-P34) = 10^e1
                // because q1 < P34 we must first replace C1 by C1*10^(P34-q1),
                // and must decrease the exponent by (P34-q1) (it will still be
                // at least e_min)
                scale = P34 - q1;
                if q1 <= 19 {
                  // C1 fits in 64 bits
                  // 1 <= q1 <= 19 => 15 <= scale <= 33
                  if scale <= 19 {
                    // 10^scale fits in 64 bits
                    mul_64x64_to_128mach(&mut c1, bid_ten2k64!(scale), c1_lo);
                  } else {
                    // if 20 <= scale <= 33
                    // C1 * 10^scale = (C1 * 10^(scale-19)) * 10^19 where
                    // (C1 * 10^(scale-19)) fits in 64 bits
                    c1_lo *= bid_ten2k64!(scale - 19);
                    mul_64x64_to_128mach(&mut c1, bid_ten2k64!(19), c1_lo);
                  }
                } else {
                  //if 20 <= q1 <= 33=P34-1 then C1 fits only in 128 bits
                  // => 1 <= P34 - q1 <= 14 so 10^(P34-q1) fits in 64 bits
                  c1.w[1] = c1_hi;
                  c1.w[0] = c1_lo;
                  // C1 = bid_ten2k64!(P34 - q1] * C1
                  let c1copy = c1;
                  mul_128x64_to_128(&mut c1, bid_ten2k64!(P34 - q1), c1copy);
                }
                c1_hi = c1.w[1];
                c1_lo = c1.w[0];
                x_exp = x_exp.wrapping_sub((scale as u64) << 49);
              }
              if (rnd_mode == BID_ROUNDING_TO_NEAREST && x_sign != y_sign)
                || (rnd_mode == BID_ROUNDING_TIES_AWAY && x_sign != y_sign && (c2_hi != halfulp128.w[1] || c2_lo != halfulp128.w[0]))
                || (rnd_mode == BID_ROUNDING_DOWN && x_sign == 0 && y_sign > 0)
                || (rnd_mode == BID_ROUNDING_UP && x_sign > 0 && y_sign == 0)
                || (rnd_mode == BID_ROUNDING_TO_ZERO && x_sign != y_sign)
              {
                // the result is x - 1
                // for RN n1 * n2 < 0; underflow not possible
                c1_lo = c1_lo.wrapping_sub(1);
                if c1_lo == 0xffffffffffffffff {
                  c1_hi = c1_hi.wrapping_sub(1);
                }
                // check if we crossed into the lower decade
                if c1_hi == 0x0000314dc6448d93 && c1_lo == 0x38c15b09ffffffff {
                  // 10^33 - 1
                  c1_hi = 0x0001ed09bead87c0; // 10^34 - 1
                  c1_lo = 0x378d8e63ffffffff;
                  x_exp = x_exp.wrapping_sub(EXP_P1); // no underflow, because n1 >> n2
                }
              } else if !(x_sign != y_sign || rnd_mode != BID_ROUNDING_TO_NEAREST && rnd_mode != BID_ROUNDING_TIES_AWAY)
                || (rnd_mode == BID_ROUNDING_DOWN && x_sign > 0 && y_sign > 0)
                || (rnd_mode == BID_ROUNDING_UP && x_sign == 0 && y_sign == 0)
              {
                // the result is x + 1
                // for RN x_sign = y_sign, i.e. n1*n2 > 0
                c1_lo = c1_lo.wrapping_add(1);
                if c1_lo == 0 {
                  // rounding overflow in the low 64 bits
                  c1_hi = c1_hi.wrapping_add(1);
                }
                if c1_hi == 0x0001ed09bead87c0 && c1_lo == 0x378d8e6400000000 {
                  // C1 = 10^34 => rounding overflow
                  c1_hi = 0x0000314dc6448d93;
                  c1_lo = 0x38c15b0a00000000; // 10^33
                  x_exp = x_exp.wrapping_add(EXP_P1);
                  if x_exp == EXP_MAX_P1 {
                    // overflow
                    c1_hi = 0x7800000000000000; // +inf
                    c1_lo = 0x0;
                    x_exp = 0; // x_sign is preserved
                    // set the overflow flag
                    *pfpsf |= BID_OVERFLOW_EXCEPTION;
                  }
                }
              } else { // the result is x
              }
              // set the inexact flag
              *pfpsf |= BID_INEXACT_EXCEPTION;
              // assemble the result
              res.w[1] = x_sign | x_exp | c1_hi;
              res.w[0] = c1_lo;
            }
          } // end q1 >= 20
        // end case where C1 != 10^(q1-1)
        } else {
          // C1 = 10^(q1-1) and x_sign != y_sign
          // instead of C' = (C1 * 10^(e1-e2) + C2)rnd,P34
          // calculate C' = C1 * 10^(e1-e2-x1) + (C2 * 10^(-x1))rnd,P34
          // where x1 = q2 - 1, 0 <= x1 <= P34 - 1
          // Because C1 = 10^(q1-1) and x_sign != y_sign, C' will have P34
          // digits and n = C' * 10^(e2+x1)
          // If the result has P34+1 digits, redo the steps above with x1+1
          // If the result has P34-1 digits or less, redo the steps above with
          // x1-1 but only if initially x1 >= 1
          // NOTE: these two steps can be improved, e.g. we could guess if
          // P34+1 or P34-1 digits will be obtained by adding/subtracting
          // just the top 64 bits of the two operands
          // The result cannot be zero, and it cannot overflow
          x1 = q2 - 1; // 0 <= x1 <= P34-1
          // Calculate C1 * 10^(e1-e2-x1) where 1 <= e1-e2-x1 <= P34
          // scale = (int)(e1 >> 49) - (int)(e2 >> 49) - x1; 0 <= scale <= P34-1
          scale = P34 - q1 + 1; // scale=e1-e2-x1 = P34+1-q1; 1<=scale<=P34
          // either C1 or 10^(e1-e2-x1) may not fit is 64 bits,
          // but their product fits with certainty in 128 bits
          if scale >= 20 {
            //10^(e1-e2-x1) doesn't fit in 64 bits, but C1 does
            mul_128x64_to_128(&mut c1, c1_lo, bid_ten2k128(scale - 20));
          } else {
            // if (scale >= 1
            // if 1 <= scale <= 19 then 10^(e1-e2-x1) fits in 64 bits
            if q1 <= 19 {
              // C1 fits in 64 bits
              mul_64x64_to_128mach(&mut c1, c1_lo, bid_ten2k64!(scale));
            } else {
              // q1 >= 20
              c1.w[1] = c1_hi;
              c1.w[0] = c1_lo;
              let c1copy = c1;
              mul_128x64_to_128(&mut c1, bid_ten2k64!(scale), c1copy);
            }
          }
          let tmp64 = c1.w[0]; // c1.w[1], c1.w[0] contains C1 * 10^(e1-e2-x1)

          // now round C2 to q2-x1 = 1 decimal digit
          // C2' = C2 + 1/2 * 10^x1 = C2 + 5 * 10^(x1-1)
          ind = x1 - 1; // -1 <= ind <= P34 - 2
          if ind >= 0 {
            // if (x1 >= 1)
            c2.w[0] = c2_lo;
            c2.w[1] = c2_hi;
            if ind <= 18 {
              c2.w[0] = c2.w[0].wrapping_add(bid_midpoint64(ind));
              if c2.w[0] < c2_lo {
                c2.w[1] = c2.w[1].wrapping_add(1);
              }
            } else {
              // 19 <= ind <= 32
              c2.w[0] = c2.w[0].wrapping_add(bid_midpoint128(ind - 19).w[0]);
              c2.w[1] = c2.w[1].wrapping_add(bid_midpoint128(ind - 19).w[1]);
              if c2.w[0] < c2_lo {
                c2.w[1] = c2.w[1].wrapping_add(1);
              }
            }
            // the approximation of 10^(-x1) was rounded up to 118 bits
            mul_128x128_to_256(&mut r256, c2, bid_ten2mk128(ind)); // R256 = C2*, f2*
            // calculate C2* and f2*
            // C2* is actually floor(C2*) in this case
            // C2* and f2* need shifting and masking, as shown by
            // bid_shiftright128[] and bid_maskhigh128[]
            // the top Ex bits of 10^(-x1) are T* = bid_ten2mk128trunc(ind), e.g.
            // if x1=1, T*=bid_ten2mk128trunc[0]=0x19999999999999999999999999999999
            // if (0 < f2* < 10^(-x1)) then
            //   if floor(C1+C2*) is even then C2* = floor(C2*) - logical right
            //       shift; C2* has p decimal digits, correct by Prop. 1)
            //   else if floor(C1+C2*) is odd C2* = floor(C2*)-1 (logical right
            //       shift; C2* has p decimal digits, correct by Pr. 1)
            // else
            //   C2* = floor(C2*) (logical right shift; C has p decimal digits,
            //       correct by Property 1)
            // n = C2* * 10^(e2+x1)

            if ind <= 2 {
              highf2star.w[1] = 0x0;
              highf2star.w[0] = 0x0; // low f2* ok
            } else if ind <= 21 {
              highf2star.w[1] = 0x0;
              highf2star.w[0] = r256.w[2] & bid_maskhigh128(ind); // low f2* ok
            } else {
              highf2star.w[1] = r256.w[3] & bid_maskhigh128(ind);
              highf2star.w[0] = r256.w[2]; // low f2* is ok
            }
            // shift right C2* by Ex-128 = bid_shiftright128(ind)
            if ind >= 3 {
              shift = bid_shiftright128(ind);
              if shift < 64 {
                // 3 <= shift <= 63
                r256.w[2] = (r256.w[2] >> shift) | (r256.w[3] << (64 - shift));
                r256.w[3] >>= shift;
              } else {
                // 66 <= shift <= 102
                r256.w[2] = r256.w[3] >> (shift - 64);
                r256.w[3] = 0x0;
              }
            }
            // redundant
            is_inexact_lt_midpoint = false;
            is_inexact_gt_midpoint = false;
            is_midpoint_lt_even = false;
            is_midpoint_gt_even = false;
            // determine inexactness of the rounding of C2*
            // (cannot be followed by a second rounding)
            // if (0 < f2* - 1/2 < 10^(-x1)) then
            //   the result is exact
            // else (if f2* - 1/2 > T* then)
            //   the result of is inexact
            if ind <= 2 {
              if r256.w[1] > 0x8000000000000000 || (r256.w[1] == 0x8000000000000000 && r256.w[0] > 0x0) {
                // f2* > 1/2 and the result may be exact
                tmp64a = r256.w[1] - 0x8000000000000000; // f* - 1/2
                if tmp64a > bid_ten2mk128trunc(ind).w[1] || (tmp64a == bid_ten2mk128trunc(ind).w[1] && r256.w[0] >= bid_ten2mk128trunc(ind).w[0]) {
                  // set the inexact flag
                  *pfpsf |= BID_INEXACT_EXCEPTION;
                  // this rounding is applied to C2 only!
                  // x_sign != y_sign
                  is_inexact_gt_midpoint = true;
                } // else the result is exact
              // rounding down, unless a midpoint in [ODD, EVEN]
              } else {
                // the result is inexact; f2* <= 1/2
                // set the inexact flag
                *pfpsf |= BID_INEXACT_EXCEPTION;
                // this rounding is applied to C2 only!
                // x_sign != y_sign
                is_inexact_lt_midpoint = true;
              }
            } else if ind <= 21 {
              // if 3 <= ind <= 21
              if highf2star.w[1] > 0x0 || (highf2star.w[1] == 0x0 && highf2star.w[0] > bid_onehalf128(ind)) || (highf2star.w[1] == 0x0 && highf2star.w[0] == bid_onehalf128(ind) && (r256.w[1] > 0 || r256.w[0] > 0)) {
                // f2* > 1/2 and the result may be exact
                // Calculate f2* - 1/2
                tmp64a = highf2star.w[0] - bid_onehalf128(ind);
                tmp64b = highf2star.w[1];
                if tmp64a > highf2star.w[0] {
                  tmp64b = tmp64b.wrapping_sub(1);
                }
                if tmp64b > 0 || tmp64a > 0 || r256.w[1] > bid_ten2mk128trunc(ind).w[1] || (r256.w[1] == bid_ten2mk128trunc(ind).w[1] && r256.w[0] > bid_ten2mk128trunc(ind).w[0]) {
                  // set the inexact flag
                  *pfpsf |= BID_INEXACT_EXCEPTION;
                  // this rounding is applied to C2 only!
                  // x_sign != y_sign
                  is_inexact_gt_midpoint = true;
                } // else the result is exact
              } else {
                // the result is inexact; f2* <= 1/2
                // set the inexact flag
                *pfpsf |= BID_INEXACT_EXCEPTION;
                // this rounding is applied to C2 only!
                // x_sign != y_sign
                is_inexact_lt_midpoint = true;
              }
            } else {
              // if 22 <= ind <= 33
              if highf2star.w[1] > bid_onehalf128(ind) || (highf2star.w[1] == bid_onehalf128(ind) && (highf2star.w[0] > 0 || r256.w[1] > 0 || r256.w[0] > 0)) {
                // f2* > 1/2 and the result may be exact
                // Calculate f2* - 1/2
                // tmp64a = highf2star.w[0];
                tmp64b = highf2star.w[1] - bid_onehalf128(ind);
                if tmp64b > 0 || highf2star.w[0] > 0 || r256.w[1] > bid_ten2mk128trunc(ind).w[1] || (r256.w[1] == bid_ten2mk128trunc(ind).w[1] && r256.w[0] > bid_ten2mk128trunc(ind).w[0]) {
                  // set the inexact flag
                  *pfpsf |= BID_INEXACT_EXCEPTION;
                  // this rounding is applied to C2 only!
                  // x_sign != y_sign
                  is_inexact_gt_midpoint = true;
                } // else the result is exact
              } else {
                // the result is inexact; f2* <= 1/2
                // set the inexact flag
                *pfpsf |= BID_INEXACT_EXCEPTION;
                // this rounding is applied to C2 only!
                // x_sign != y_sign
                is_inexact_lt_midpoint = true;
              }
            }
            // check for midpoints after determining inexactness
            if (r256.w[1] > 0 || r256.w[0] > 0) && (highf2star.w[1] == 0) && (highf2star.w[0] == 0) && (r256.w[1] < bid_ten2mk128trunc(ind).w[1] || (r256.w[1] == bid_ten2mk128trunc(ind).w[1] && r256.w[0] <= bid_ten2mk128trunc(ind).w[0])) {
              // the result is a midpoint
              if ((tmp64 + r256.w[2]) & 0x01) > 0 {
                // MP in [EVEN, ODD]
                // if floor(C2*) is odd C = floor(C2*) - 1; the result may be 0
                r256.w[2] = r256.w[2].wrapping_sub(1);
                if r256.w[2] == 0xffffffffffffffff {
                  r256.w[3] = r256.w[3].wrapping_sub(1);
                }
                // this rounding is applied to C2 only!
                // x_sign != y_sign
                is_midpoint_lt_even = true;
                is_inexact_lt_midpoint = false;
                is_inexact_gt_midpoint = false;
              } else {
                // else MP in [ODD, EVEN]
                // this rounding is applied to C2 only!
                // x_sign != y_sign
                is_midpoint_gt_even = true;
                is_inexact_lt_midpoint = false;
                is_inexact_gt_midpoint = false;
              }
            }
          } else {
            // if (ind == -1) only when x1 = 0
            r256.w[2] = c2_lo;
            r256.w[3] = c2_hi;
            is_midpoint_lt_even = false;
            is_midpoint_gt_even = false;
            is_inexact_lt_midpoint = false;
            is_inexact_gt_midpoint = false;
          }
          // and now subtract C1 * 10^(e1-e2-x1) - (C2 * 10^(-x1))rnd,P34
          // because x_sign != y_sign this last operation is exact
          c1.w[0] = c1.w[0].wrapping_sub(r256.w[2]);
          c1.w[1] = c1.w[1].wrapping_sub(r256.w[3]);
          if c1.w[0] > tmp64 {
            c1.w[1] = c1.w[1].wrapping_sub(1); // borrow
          }
          if c1.w[1] >= 0x8000000000000000 {
            // negative coefficient!
            c1.w[0] = !c1.w[0];
            c1.w[0] = c1.w[0].wrapping_add(1);
            c1.w[1] = !c1.w[1];
            if c1.w[0] == 0x0 {
              c1.w[1] = c1.w[1].wrapping_add(1);
            }
            tmp_sign = y_sign; // the result will have the sign of y
          } else {
            tmp_sign = x_sign;
          }
          // the difference has exactly P34 digits
          x_sign = tmp_sign;
          if x1 >= 1 {
            y_exp = y_exp.wrapping_add((x1 as u64) << 49);
          }
          c1_hi = c1.w[1];
          c1_lo = c1.w[0];
          // general correction from RN to RA, RM, RP, RZ; result uses y_exp
          if rnd_mode != BID_ROUNDING_TO_NEAREST {
            if (x_sign == 0 && ((rnd_mode == BID_ROUNDING_UP && is_inexact_lt_midpoint) || ((rnd_mode == BID_ROUNDING_TIES_AWAY || rnd_mode == BID_ROUNDING_UP) && is_midpoint_gt_even)))
              || (x_sign > 0 && ((rnd_mode == BID_ROUNDING_DOWN && is_inexact_lt_midpoint) || ((rnd_mode == BID_ROUNDING_TIES_AWAY || rnd_mode == BID_ROUNDING_DOWN) && is_midpoint_gt_even)))
            {
              // C1 = C1 + 1
              c1_lo = c1_lo.wrapping_add(1);
              if c1_lo == 0 {
                // rounding overflow in the low 64 bits
                c1_hi = c1_hi.wrapping_add(1);
              }
              if c1_hi == 0x0001ed09bead87c0 && c1_lo == 0x378d8e6400000000 {
                // C1 = 10^34 => rounding overflow
                c1_hi = 0x0000314dc6448d93;
                c1_lo = 0x38c15b0a00000000; // 10^33
                y_exp = y_exp.wrapping_add(EXP_P1);
              }
            } else if (is_midpoint_lt_even || is_inexact_gt_midpoint)
              && ((x_sign > 0 && (rnd_mode == BID_ROUNDING_UP || rnd_mode == BID_ROUNDING_TO_ZERO)) || (x_sign == 0 && (rnd_mode == BID_ROUNDING_DOWN || rnd_mode == BID_ROUNDING_TO_ZERO)))
            {
              // C1 = C1 - 1
              c1_lo = c1_lo.wrapping_sub(1);
              if c1_lo == 0xffffffffffffffff {
                c1_hi = c1_hi.wrapping_sub(1);
              }
              // check if we crossed into the lower decade
              if c1_hi == 0x0000314dc6448d93 && c1_lo == 0x38c15b09ffffffff {
                // 10^33 - 1
                c1_hi = 0x0001ed09bead87c0; // 10^34 - 1
                c1_lo = 0x378d8e63ffffffff;
                y_exp = y_exp.wrapping_sub(EXP_P1);
                // no underflow, because delta + q2 >= P34 + 1
              }
            } else { // exact, the result is already correct
            }
          }
          // assemble the result
          res.w[1] = x_sign | y_exp | c1_hi;
          res.w[0] = c1_lo;
        }
      } // end delta = P34
    } else {
      // if (|delta| <= P34 - 1)
      if delta >= 0 {
        // if (0 <= delta <= P34 - 1)
        if delta <= P34 - 1 - q2 {
          // calculate C' directly; the result is exact
          // in this case 1<=q1<=P34-1, 1<=q2<=P34-1 and 0 <= e1-e2 <= P34-2
          // The coefficient of the result is C1 * 10^(e1-e2) + C2 and the
          // exponent is e2; either C1 or 10^(e1-e2) may not fit is 64 bits,
          // but their product fits with certainty in 128 bits (actually in 113)
          scale = delta - q1 + q2; // scale = (int)(e1 >> 49) - (int)(e2 >> 49)

          if scale >= 20 {
            // 10^(e1-e2) does not fit in 64 bits, but C1 does
            mul_128x64_to_128(&mut c1, c1_lo, bid_ten2k128(scale - 20));
            c1_hi = c1.w[1];
            c1_lo = c1.w[0];
          } else if scale >= 1 {
            // if 1 <= scale <= 19 then 10^(e1-e2) fits in 64 bits
            if q1 <= 19 {
              // C1 fits in 64 bits
              mul_64x64_to_128mach(&mut c1, c1_lo, bid_ten2k64!(scale));
            } else {
              // q1 >= 20
              c1.w[1] = c1_hi;
              c1.w[0] = c1_lo;
              let c1copy = c1;
              mul_128x64_to_128(&mut c1, bid_ten2k64!(scale), c1copy);
            }
            c1_hi = c1.w[1];
            c1_lo = c1.w[0];
          } else {
            // if (scale == 0) C1 is unchanged
            c1.w[0] = c1_lo; // c1.w[1] = c1_hi;
          }
          // now add C2
          if x_sign == y_sign {
            // the result cannot overflow
            c1_lo = c1_lo.wrapping_add(c2_lo);
            c1_hi = c1_hi.wrapping_add(c2_hi);
            if c1_lo < c1.w[0] {
              c1_hi = c1_hi.wrapping_add(1);
            }
          } else {
            // if x_sign != y_sign
            c1_lo = c1_lo.wrapping_sub(c2_lo);
            c1_hi = c1_hi.wrapping_sub(c2_hi);
            if c1_lo > c1.w[0] {
              c1_hi = c1_hi.wrapping_sub(1);
            }
            // the result can be zero, but it cannot overflow
            if c1_lo == 0 && c1_hi == 0 {
              // assemble the result
              if x_exp < y_exp {
                res.w[1] = x_exp;
              } else {
                res.w[1] = y_exp;
              }
              res.w[0] = 0;
              if rnd_mode == BID_ROUNDING_DOWN {
                res.w[1] |= 0x8000000000000000;
              }
              return res;
            }
            if c1_hi >= 0x8000000000000000 {
              // negative coefficient!
              c1_lo = !c1_lo;
              c1_lo = c1_lo.wrapping_add(1);
              c1_hi = !c1_hi;
              if c1_lo == 0x0 {
                c1_hi = c1_hi.wrapping_add(1);
              }
              x_sign = y_sign; // the result will have the sign of y
            }
          }
          // assemble the result
          res.w[1] = x_sign | y_exp | c1_hi;
          res.w[0] = c1_lo;
        } else if delta == P34 - q2 {
          // calculate C' directly; the result may be inexact if it requires
          // P34+1 decimal digits; in this case the 'cutoff' point for addition
          // is at the position of the lsb of C2, so 0 <= e1-e2 <= P34-1
          // The coefficient of the result is C1 * 10^(e1-e2) + C2 and the
          // exponent is e2; either C1 or 10^(e1-e2) may not fit is 64 bits,
          // but their product fits with certainty in 128 bits (actually in 113)
          scale = delta - q1 + q2; // scale = (int)(e1 >> 49) - (int)(e2 >> 49)
          if scale >= 20 {
            // 10^(e1-e2) does not fit in 64 bits, but C1 does
            mul_128x64_to_128(&mut c1, c1_lo, bid_ten2k128(scale - 20));
          } else if scale >= 1 {
            // if 1 <= scale <= 19 then 10^(e1-e2) fits in 64 bits
            if q1 <= 19 {
              // C1 fits in 64 bits
              mul_64x64_to_128mach(&mut c1, c1_lo, bid_ten2k64!(scale));
            } else {
              // q1 >= 20
              c1.w[1] = c1_hi;
              c1.w[0] = c1_lo;
              let c1copy = c1;
              mul_128x64_to_128(&mut c1, bid_ten2k64!(scale), c1copy);
            }
          } else {
            // if (scale == 0) C1 is unchanged
            c1.w[1] = c1_hi;
            c1.w[0] = c1_lo; // only the low part is necessary
          }
          c1_hi = c1.w[1];
          c1_lo = c1.w[0];
          // now add C2
          if x_sign == y_sign {
            // the result can overflow!
            c1_lo = c1_lo.wrapping_add(c2_lo);
            c1_hi = c1_hi.wrapping_add(c2_hi);
            if c1_lo < c1.w[0] {
              c1_hi = c1_hi.wrapping_add(1);
            }
            // test for overflow, possible only when C1 >= 10^34
            if c1_hi > 0x0001ed09bead87c0 || (c1_hi == 0x0001ed09bead87c0 && c1_lo >= 0x378d8e6400000000) {
              // C1 >= 10^34
              // in this case q = P34 + 1 and x = q - P34 = 1, so multiply
              // C'' = C'+ 5 = C1 + 5 by k1 ~ 10^(-1) calculated for P34 + 1
              // decimal digits
              // Calculate C'' = C' + 1/2 * 10^x
              if c1_lo >= 0xfffffffffffffffb {
                // low half add has carry
                c1_lo = c1_lo.wrapping_add(5);
                c1_hi = c1_hi.wrapping_add(1);
              } else {
                c1_lo = c1_lo.wrapping_add(5);
              }
              // the approximation of 10^(-1) was rounded up to 118 bits
              // 10^(-1) =~ 33333333333333333333333333333400 * 2^-129
              // 10^(-1) =~ 19999999999999999999999999999a00 * 2^-128
              c1.w[1] = c1_hi;
              c1.w[0] = c1_lo; // C''
              ten2m1.w[1] = 0x1999999999999999;
              ten2m1.w[0] = 0x9999999999999a00;
              mul_128x128_to_256(&mut p256, c1, ten2m1); // P256 = C*, f*
              // C* is actually floor(C*) in this case
              // the top Ex = 128 bits of 10^(-1) are
              // T* = 0x00199999999999999999999999999999
              // if (0 < f* < 10^(-x)) then
              //   if floor(C*) is even then C = floor(C*) - logical right
              //       shift; C has p decimal digits, correct by Prop. 1)
              //   else if floor(C*) is odd C = floor(C*) - 1 (logical right
              //       shift; C has p decimal digits, correct by Pr. 1)
              // else
              //   C = floor(C*) (logical right shift; C has p decimal digits,
              //       correct by Property 1)
              // n = C * 10^(e2+x)
              if (p256.w[1] > 0 || p256.w[0] > 0) && (p256.w[1] < 0x1999999999999999 || (p256.w[1] == 0x1999999999999999 && p256.w[0] <= 0x9999999999999999)) {
                // the result is a midpoint
                if (p256.w[2] & 0x01) > 0 {
                  is_midpoint_gt_even = true;
                  // if floor(C*) is odd C = floor(C*) - 1; the result is not 0
                  p256.w[2] = p256.w[2].wrapping_sub(1);
                  if p256.w[2] == 0xffffffffffffffff {
                    p256.w[3] = p256.w[3].wrapping_sub(1);
                  }
                } else {
                  is_midpoint_lt_even = true;
                }
              }
              // n = Cstar * 10^(e2+1)
              y_exp = y_exp.wrapping_add(EXP_P1);
              // C* != 10^P because C* has P34 digits
              // check for overflow
              if y_exp == EXP_MAX_P1 && (rnd_mode == BID_ROUNDING_TO_NEAREST || rnd_mode == BID_ROUNDING_TIES_AWAY) {
                // overflow for RN
                res.w[1] = x_sign | 0x7800000000000000; // +/-inf
                res.w[0] = 0x0;
                // set the inexact flag
                *pfpsf |= BID_INEXACT_EXCEPTION;
                // set the overflow flag
                *pfpsf |= BID_OVERFLOW_EXCEPTION;
                return res;
              }
              // if (0 < f* - 1/2 < 10^(-x)) then
              //   the result of the addition is exact
              // else
              //   the result of the addition is inexact
              if p256.w[1] > 0x8000000000000000 || (p256.w[1] == 0x8000000000000000 && p256.w[0] > 0x0) {
                // the result may be exact
                tmp64 = p256.w[1] - 0x8000000000000000; // f* - 1/2
                if tmp64 > 0x1999999999999999 || (tmp64 == 0x1999999999999999 && p256.w[0] >= 0x9999999999999999) {
                  // set the inexact flag
                  *pfpsf |= BID_INEXACT_EXCEPTION;
                  is_inexact = true;
                } // else the result is exact
              } else {
                // the result is inexact
                // set the inexact flag
                *pfpsf |= BID_INEXACT_EXCEPTION;
                is_inexact = true;
              }
              c1_hi = p256.w[3];
              c1_lo = p256.w[2];
              if !is_midpoint_gt_even && !is_midpoint_lt_even {
                is_inexact_lt_midpoint = is_inexact && (p256.w[1] & 0x8000000000000000) > 0;
                is_inexact_gt_midpoint = is_inexact && !(p256.w[1] & 0x8000000000000000) > 0;
              }
              // general correction from RN to RA, RM, RP, RZ;
              // result uses y_exp
              if rnd_mode != BID_ROUNDING_TO_NEAREST {
                if (x_sign == 0 && ((rnd_mode == BID_ROUNDING_UP && is_inexact_lt_midpoint) || ((rnd_mode == BID_ROUNDING_TIES_AWAY || rnd_mode == BID_ROUNDING_UP) && is_midpoint_gt_even)))
                  || (x_sign > 0 && ((rnd_mode == BID_ROUNDING_DOWN && is_inexact_lt_midpoint) || ((rnd_mode == BID_ROUNDING_TIES_AWAY || rnd_mode == BID_ROUNDING_DOWN) && is_midpoint_gt_even)))
                {
                  // C1 = C1 + 1
                  c1_lo = c1_lo.wrapping_add(1);
                  if c1_lo == 0 {
                    // rounding overflow in the low 64 bits
                    c1_hi = c1_hi.wrapping_add(1);
                  }
                  if c1_hi == 0x0001ed09bead87c0 && c1_lo == 0x378d8e6400000000 {
                    // C1 = 10^34 => rounding overflow
                    c1_hi = 0x0000314dc6448d93;
                    c1_lo = 0x38c15b0a00000000; // 10^33
                    y_exp = y_exp.wrapping_add(EXP_P1);
                  }
                } else if (is_midpoint_lt_even || is_inexact_gt_midpoint)
                  && ((x_sign > 0 && (rnd_mode == BID_ROUNDING_UP || rnd_mode == BID_ROUNDING_TO_ZERO)) || (x_sign == 0 && (rnd_mode == BID_ROUNDING_DOWN || rnd_mode == BID_ROUNDING_TO_ZERO)))
                {
                  // C1 = C1 - 1
                  c1_lo = c1_lo.wrapping_sub(1);
                  if c1_lo == 0xffffffffffffffff {
                    c1_hi = c1_hi.wrapping_sub(1);
                  }
                  // check if we crossed into the lower decade
                  if c1_hi == 0x0000314dc6448d93 && c1_lo == 0x38c15b09ffffffff {
                    // 10^33 - 1
                    c1_hi = 0x0001ed09bead87c0; // 10^34 - 1
                    c1_lo = 0x378d8e63ffffffff;
                    y_exp = y_exp.wrapping_sub(EXP_P1);
                    // no underflow, because delta + q2 >= P34 + 1
                  }
                } else { // exact, the result is already correct
                }
                // in all cases check for overflow (RN and RA solved already)
                if y_exp == EXP_MAX_P1 {
                  // overflow
                  if (rnd_mode == BID_ROUNDING_DOWN && x_sign > 0) || // RM and res < 0
                    (rnd_mode == BID_ROUNDING_UP && x_sign == 0)
                  {
                    // RP and res > 0
                    c1_hi = 0x7800000000000000; // +inf
                    c1_lo = 0x0;
                  } else {
                    // RM and res > 0, RP and res < 0, or RZ
                    c1_hi = 0x5fffed09bead87c0;
                    c1_lo = 0x378d8e63ffffffff;
                  }
                  y_exp = 0; // x_sign is preserved
                  // set the inexact flag (in case the exact addition was exact)
                  *pfpsf |= BID_INEXACT_EXCEPTION;
                  // set the overflow flag
                  *pfpsf |= BID_OVERFLOW_EXCEPTION;
                }
              }
            } // else if (C1 < 10^34) then C1 is the coeff.; the result is exact
          } else {
            // if x_sign != y_sign the result is exact
            c1_lo = c1_lo.wrapping_sub(c2_lo);
            c1_hi = c1_hi.wrapping_sub(c2_hi);
            if c1_lo > c1.w[0] {
              c1_hi = c1_hi.wrapping_sub(1);
            }
            // the result can be zero, but it cannot overflow
            if c1_lo == 0 && c1_hi == 0 {
              // assemble the result
              if x_exp < y_exp {
                res.w[1] = x_exp;
              } else {
                res.w[1] = y_exp;
              }
              res.w[0] = 0;
              if rnd_mode == BID_ROUNDING_DOWN {
                res.w[1] |= 0x8000000000000000;
              }
              return res;
            }
          }
          if c1_hi >= 0x8000000000000000 {
            // negative coefficient!
            c1_lo = !c1_lo;
            c1_lo = c1_lo.wrapping_add(1);
            c1_hi = !c1_hi;
            if c1_lo == 0x0 {
              c1_hi = c1_hi.wrapping_add(1);
            }
            x_sign = y_sign; // the result will have the sign of y
          }
          // assemble the result
          res.w[1] = x_sign | y_exp | c1_hi;
          res.w[0] = c1_lo;
        } else {
          // if (delta >= P34 + 1 - q2)
          // instead of C' = (C1 * 10^(e1-e2) + C2)rnd,P34
          // calculate C' = C1 * 10^(e1-e2-x1) + (C2 * 10^(-x1))rnd,P34
          // where x1 = q1 + e1 - e2 - P34, 1 <= x1 <= P34 - 1
          // In most cases C' will have P34 digits, and n = C' * 10^(e2+x1)
          // If the result has P34+1 digits, redo the steps above with x1+1
          // If the result has P34-1 digits or less, redo the steps above with
          // x1-1 but only if initially x1 >= 1
          // NOTE: these two steps can be improved, e.g. we could guess if
          // P34+1 or P34-1 digits will be obtained by adding/subtracting just
          // the top 64 bits of the two operands
          // The result cannot be zero, but it can overflow
          x1 = delta + q2 - P34; // 1 <= x1 <= P34-1

          'round_c2: loop {
            // Calculate C1 * 10^(e1-e2-x1) where 0 <= e1-e2-x1 <= P34 - 1
            // scale = (int)(e1 >> 49) - (int)(e2 >> 49) - x1; 0 <= scale <= P34-1
            scale = delta - q1 + q2 - x1; // scale = e1 - e2 - x1 = P34 - q1

            // either C1 or 10^(e1-e2-x1) may not fit is 64 bits,
            // but their product fits with certainty in 128 bits (actually in 113)
            if scale >= 20 {
              //10^(e1-e2-x1) doesn't fit in 64 bits, but C1 does
              mul_128x64_to_128(&mut c1, c1_lo, bid_ten2k128(scale - 20));
            } else if scale >= 1 {
              // if 1 <= scale <= 19 then 10^(e1-e2-x1) fits in 64 bits
              if q1 <= 19 {
                // C1 fits in 64 bits
                mul_64x64_to_128mach(&mut c1, c1_lo, bid_ten2k64!(scale));
              } else {
                // q1 >= 20
                c1.w[1] = c1_hi;
                c1.w[0] = c1_lo;
                let c1copy = c1;
                mul_128x64_to_128(&mut c1, bid_ten2k64!(scale), c1copy);
              }
            } else {
              // if (scale == 0) C1 is unchanged
              c1.w[1] = c1_hi;
              c1.w[0] = c1_lo;
            }
            tmp64 = c1.w[0]; // c1.w[1], c1.w[0] contains C1 * 10^(e1-e2-x1)

            // now round C2 to q2-x1 decimal digits, where 1<=x1<=q2-1<=P34-1
            // (but if we got here a second time after x1 = x1 - 1, then
            // x1 >= 0; note that for x1 = 0 C2 is unchanged)
            // C2' = C2 + 1/2 * 10^x1 = C2 + 5 * 10^(x1-1)
            ind = x1 - 1; // 0 <= ind <= q2-2<=P34-2=32; but note that if x1 = 0
            // during a second pass, then ind = -1
            if ind >= 0 {
              // if (x1 >= 1)
              c2.w[0] = c2_lo;
              c2.w[1] = c2_hi;
              if ind <= 18 {
                c2.w[0] = c2.w[0].wrapping_add(bid_midpoint64(ind));
                if c2.w[0] < c2_lo {
                  c2.w[1] = c2.w[1].wrapping_add(1);
                }
              } else {
                // 19 <= ind <= 32
                c2.w[0] = c2.w[0].wrapping_add(bid_midpoint128(ind - 19).w[0]);
                c2.w[1] = c2.w[1].wrapping_add(bid_midpoint128(ind - 19).w[1]);
                if c2.w[0] < c2_lo {
                  c2.w[1] = c2.w[1].wrapping_add(1);
                }
              }
              // the approximation of 10^(-x1) was rounded up to 118 bits
              mul_128x128_to_256(&mut r256, c2, bid_ten2mk128(ind)); // R256 = C2*, f2*
              // calculate C2* and f2*
              // C2* is actually floor(C2*) in this case
              // C2* and f2* need shifting and masking, as shown by
              // bid_shiftright128[] and bid_maskhigh128[]
              // the top Ex bits of 10^(-x1) are T* = bid_ten2mk128trunc(ind), e.g.
              // if x1=1, T*=bid_ten2mk128trunc[0]=0x19999999999999999999999999999999
              // if (0 < f2* < 10^(-x1)) then
              //   if floor(C1+C2*) is even then C2* = floor(C2*) - logical right
              //       shift; C2* has p decimal digits, correct by Prop. 1)
              //   else if floor(C1+C2*) is odd C2* = floor(C2*)-1 (logical right
              //       shift; C2* has p decimal digits, correct by Pr. 1)
              // else
              //   C2* = floor(C2*) (logical right shift; C has p decimal digits,
              //       correct by Property 1)
              // n = C2* * 10^(e2+x1)

              if ind <= 2 {
                highf2star.w[1] = 0;
                highf2star.w[0] = 0; // low f2* ok
              } else if ind <= 21 {
                highf2star.w[1] = 0;
                highf2star.w[0] = r256.w[2] & bid_maskhigh128(ind); // low f2* ok
              } else {
                highf2star.w[1] = r256.w[3] & bid_maskhigh128(ind);
                highf2star.w[0] = r256.w[2]; // low f2* is ok
              }
              // shift right C2* by Ex-128 = bid_shiftright128(ind)
              if ind >= 3 {
                shift = bid_shiftright128(ind);
                if shift < 64 {
                  // 3 <= shift <= 63
                  r256.w[2] = (r256.w[2] >> shift) | (r256.w[3] << (64 - shift));
                  r256.w[3] >>= shift;
                } else {
                  // 66 <= shift <= 102
                  r256.w[2] = r256.w[3] >> (shift - 64);
                  r256.w[3] = 0;
                }
              }
              if second_pass {
                is_inexact_lt_midpoint = false;
                is_inexact_gt_midpoint = false;
                is_midpoint_lt_even = false;
                is_midpoint_gt_even = false;
              }
              // determine inexactness of the rounding of C2* (this may be
              // followed by a second rounding only if we get P34+1
              // decimal digits)
              // if (0 < f2* - 1/2 < 10^(-x1)) then
              //   the result is exact
              // else (if f2* - 1/2 > T* then)
              //   the result of is inexact
              if ind <= 2 {
                if r256.w[1] > 0x8000000000000000 || (r256.w[1] == 0x8000000000000000 && r256.w[0] > 0x0) {
                  // f2* > 1/2 and the result may be exact
                  tmp64a = r256.w[1] - 0x8000000000000000; // f* - 1/2
                  if tmp64a > bid_ten2mk128trunc(ind).w[1] || (tmp64a == bid_ten2mk128trunc(ind).w[1] && r256.w[0] >= bid_ten2mk128trunc(ind).w[0]) {
                    // set the inexact flag
                    // *flags |= BID_INEXACT_EXCEPTION;
                    tmp_inexact = true; // may be set again during a second pass
                    // this rounding is applied to C2 only!
                    if x_sign == y_sign {
                      is_inexact_lt_midpoint = true;
                    } else {
                      is_inexact_gt_midpoint = true;
                    }
                  } // else the result is exact
                // rounding down, unless a midpoint in [ODD, EVEN]
                } else {
                  // the result is inexact; f2* <= 1/2
                  // set the inexact flag
                  // *flags |= BID_INEXACT_EXCEPTION;
                  tmp_inexact = true; // just in case we will round a second time
                  // rounding up, unless a midpoint in [EVEN, ODD]
                  // this rounding is applied to C2 only!
                  if x_sign == y_sign {
                    is_inexact_gt_midpoint = true;
                  } else {
                    is_inexact_lt_midpoint = true;
                  }
                }
              } else if ind <= 21 {
                // if 3 <= ind <= 21
                if highf2star.w[1] > 0x0 || (highf2star.w[1] == 0x0 && highf2star.w[0] > bid_onehalf128(ind)) || (highf2star.w[1] == 0x0 && highf2star.w[0] == bid_onehalf128(ind) && (r256.w[1] > 0 || r256.w[0] > 0)) {
                  // f2* > 1/2 and the result may be exact
                  // Calculate f2* - 1/2
                  tmp64a = highf2star.w[0] - bid_onehalf128(ind);
                  tmp64b = highf2star.w[1];
                  if tmp64a > highf2star.w[0] {
                    tmp64b = tmp64b.wrapping_sub(1);
                  }
                  if tmp64b > 0 || tmp64a > 0 || r256.w[1] > bid_ten2mk128trunc(ind).w[1] || (r256.w[1] == bid_ten2mk128trunc(ind).w[1] && r256.w[0] > bid_ten2mk128trunc(ind).w[0]) {
                    // set the inexact flag
                    // *flags |= BID_INEXACT_EXCEPTION;
                    tmp_inexact = true; // may be set again during a second pass
                    // this rounding is applied to C2 only!
                    if x_sign == y_sign {
                      is_inexact_lt_midpoint = true;
                    } else {
                      is_inexact_gt_midpoint = true;
                    }
                  } // else the result is exact
                } else {
                  // the result is inexact; f2* <= 1/2
                  // set the inexact flag
                  // *flags |= BID_INEXACT_EXCEPTION;
                  tmp_inexact = true; // may be set again during a second pass
                  // rounding up, unless a midpoint in [EVEN, ODD]
                  // this rounding is applied to C2 only!
                  if x_sign == y_sign {
                    is_inexact_gt_midpoint = true;
                  } else {
                    is_inexact_lt_midpoint = true;
                  }
                }
              } else {
                // if 22 <= ind <= 33
                if highf2star.w[1] > bid_onehalf128(ind) || (highf2star.w[1] == bid_onehalf128(ind) && (highf2star.w[0] > 0 || r256.w[1] > 0 || r256.w[0] > 0)) {
                  // f2* > 1/2 and the result may be exact
                  // Calculate f2* - 1/2
                  // tmp64a = highf2star.w[0];
                  tmp64b = highf2star.w[1] - bid_onehalf128(ind);
                  if tmp64b > 0 || highf2star.w[0] > 0 || r256.w[1] > bid_ten2mk128trunc(ind).w[1] || (r256.w[1] == bid_ten2mk128trunc(ind).w[1] && r256.w[0] > bid_ten2mk128trunc(ind).w[0]) {
                    // set the inexact flag
                    // *flags |= BID_INEXACT_EXCEPTION;
                    tmp_inexact = true; // may be set again during a second pass
                    // this rounding is applied to C2 only!
                    if x_sign == y_sign {
                      is_inexact_lt_midpoint = true;
                    } else {
                      is_inexact_gt_midpoint = true;
                    }
                  } // else the result is exact
                } else {
                  // the result is inexact; f2* <= 1/2
                  // set the inexact flag
                  // *flags |= BID_INEXACT_EXCEPTION;
                  tmp_inexact = true; // may be set again during a second pass
                  // rounding up, unless a midpoint in [EVEN, ODD]
                  // this rounding is applied to C2 only!
                  if x_sign == y_sign {
                    is_inexact_gt_midpoint = true;
                  } else {
                    is_inexact_lt_midpoint = true;
                  }
                }
              }
              // check for midpoints
              if (r256.w[1] > 0 || r256.w[0] > 0) && (highf2star.w[1] == 0) && (highf2star.w[0] == 0) && (r256.w[1] < bid_ten2mk128trunc(ind).w[1] || (r256.w[1] == bid_ten2mk128trunc(ind).w[1] && r256.w[0] <= bid_ten2mk128trunc(ind).w[0]))
              {
                // the result is a midpoint
                if (tmp64.wrapping_add(r256.w[2]) & 0x01) > 0 {
                  // MP in [EVEN, ODD]
                  // if floor(C2*) is odd C = floor(C2*) - 1; the result may be 0
                  r256.w[2] = r256.w[2].wrapping_sub(1);
                  if r256.w[2] == 0xffffffffffffffff {
                    r256.w[3] = r256.w[3].wrapping_sub(1);
                  }
                  // this rounding is applied to C2 only!
                  if x_sign == y_sign {
                    is_midpoint_gt_even = true;
                  } else {
                    is_midpoint_lt_even = true;
                  }
                  is_inexact_lt_midpoint = false;
                  is_inexact_gt_midpoint = false;
                } else {
                  // else MP in [ODD, EVEN]
                  // this rounding is applied to C2 only!
                  if x_sign == y_sign {
                    is_midpoint_lt_even = true;
                  } else {
                    is_midpoint_gt_even = true;
                  }
                  is_inexact_lt_midpoint = false;
                  is_inexact_gt_midpoint = false;
                }
              }
            // end if (ind >= 0)
            } else {
              // if (ind == -1); only during a 2nd pass, and when x1 = 0
              r256.w[2] = c2_lo;
              r256.w[3] = c2_hi;
              tmp_inexact = false;
              // to correct a possible setting to 1 from 1st pass
              if second_pass {
                is_midpoint_lt_even = false;
                is_midpoint_gt_even = false;
                is_inexact_lt_midpoint = false;
                is_inexact_gt_midpoint = false;
              }
            }
            // and now add/subtract C1 * 10^(e1-e2-x1) +/- (C2 * 10^(-x1))rnd,P34
            if x_sign == y_sign {
              // addition; could overflow
              // no second pass is possible this way (only for x_sign != y_sign)
              c1.w[0] = c1.w[0].wrapping_add(r256.w[2]);
              c1.w[1] = c1.w[1].wrapping_add(r256.w[3]);
              if c1.w[0] < tmp64 {
                c1.w[1] = c1.w[1].wrapping_add(1); // carry
              }
              // if the sum has P34+1 digits, i.e. C1>=10^34 redo the calculation
              // with x1=x1+1
              if c1.w[1] > 0x0001ed09bead87c0 || (c1.w[1] == 0x0001ed09bead87c0 && c1.w[0] >= 0x378d8e6400000000) {
                // C1 >= 10^34
                // chop off one more digit from the sum, but make sure there is
                // no double-rounding error (see table - double rounding logic)
                // now round C1 from P34+1 to P34 decimal digits
                // C1' = C1 + 1/2 * 10 = C1 + 5
                if c1.w[0] >= 0xfffffffffffffffb {
                  // low half add has carry
                  c1.w[0] = c1.w[0].wrapping_add(5);
                  c1.w[1] = c1.w[1].wrapping_add(1);
                } else {
                  c1.w[0] = c1.w[0].wrapping_add(5);
                }
                // the approximation of 10^(-1) was rounded up to 118 bits
                // q256 = C1*, f1*
                mul_128x128_to_256(&mut q256, c1, bid_ten2mk128(0));
                // C1* is actually floor(C1*) in this case
                // the top 128 bits of 10^(-1) are
                // T* = bid_ten2mk128trunc[0]=0x19999999999999999999999999999999
                // if (0 < f1* < 10^(-1)) then
                //   if floor(C1*) is even then C1* = floor(C1*) - logical right
                //       shift; C1* has p decimal digits, correct by Prop. 1)
                //   else if floor(C1*) is odd C1* = floor(C1*) - 1 (logical right
                //       shift; C1* has p decimal digits, correct by Pr. 1)
                // else
                //   C1* = floor(C1*) (logical right shift; C has p decimal digits
                //       correct by Property 1)
                // n = C1* * 10^(e2+x1+1)
                if (q256.w[1] > 0 || q256.w[0] > 0) && (q256.w[1] < bid_ten2mk128trunc(0).w[1] || (q256.w[1] == bid_ten2mk128trunc(0).w[1] && q256.w[0] <= bid_ten2mk128trunc(0).w[0])) {
                  // the result is a midpoint
                  if is_inexact_lt_midpoint {
                    // for the 1st rounding
                    is_inexact_gt_midpoint = true;
                    is_inexact_lt_midpoint = false;
                    is_midpoint_gt_even = false;
                    is_midpoint_lt_even = false;
                  } else if is_inexact_gt_midpoint {
                    // for the 1st rounding
                    q256.w[2] = q256.w[2].wrapping_sub(1);
                    if q256.w[2] == 0xffffffffffffffff {
                      q256.w[3] = q256.w[3].wrapping_sub(1);
                    }
                    is_inexact_gt_midpoint = false;
                    is_inexact_lt_midpoint = true;
                    is_midpoint_gt_even = false;
                    is_midpoint_lt_even = false;
                  } else if is_midpoint_gt_even {
                    // for the 1st rounding
                    // Note: cannot have is_midpoint_lt_even
                    is_inexact_gt_midpoint = false;
                    is_inexact_lt_midpoint = true;
                    is_midpoint_gt_even = false;
                    is_midpoint_lt_even = false;
                  } else {
                    // the first rounding must have been exact
                    if (q256.w[2] & 0x01) > 0 {
                      // MP in [EVEN, ODD]
                      // the truncated result is correct
                      q256.w[2] = q256.w[2].wrapping_sub(1);
                      if q256.w[2] == 0xffffffffffffffff {
                        q256.w[3] = q256.w[3].wrapping_sub(1);
                      }
                      is_inexact_gt_midpoint = false;
                      is_inexact_lt_midpoint = false;
                      is_midpoint_gt_even = true;
                      is_midpoint_lt_even = false;
                    } else {
                      // MP in [ODD, EVEN]
                      is_inexact_gt_midpoint = false;
                      is_inexact_lt_midpoint = false;
                      is_midpoint_gt_even = false;
                      is_midpoint_lt_even = true;
                    }
                  }
                  tmp_inexact = true; // in all cases
                } else {
                  // the result is not a midpoint
                  // determine inexactness of the rounding of C1 (the sum C1+C2*)
                  // if (0 < f1* - 1/2 < 10^(-1)) then
                  //   the result is exact
                  // else (if f1* - 1/2 > T* then)
                  //   the result of is inexact
                  // ind = 0
                  if q256.w[1] > 0x8000000000000000 || (q256.w[1] == 0x8000000000000000 && q256.w[0] > 0x0) {
                    // f1* > 1/2 and the result may be exact
                    q256.w[1] = q256.w[1].wrapping_sub(0x8000000000000000); // f1* - 1/2
                    if q256.w[1] > bid_ten2mk128trunc(0).w[1] || (q256.w[1] == bid_ten2mk128trunc(0).w[1] && q256.w[0] > bid_ten2mk128trunc(0).w[0]) {
                      is_inexact_gt_midpoint = false;
                      is_inexact_lt_midpoint = true;
                      is_midpoint_gt_even = false;
                      is_midpoint_lt_even = false;
                      // set the inexact flag
                      tmp_inexact = true;
                      // *flags |= BID_INEXACT_EXCEPTION;
                    } else {
                      // else the result is exact for the 2nd rounding
                      if tmp_inexact {
                        // if the previous rounding was inexact
                        if is_midpoint_lt_even {
                          is_inexact_gt_midpoint = true;
                          is_midpoint_lt_even = false;
                        } else if is_midpoint_gt_even {
                          is_inexact_lt_midpoint = true;
                          is_midpoint_gt_even = false;
                        } else { // no change
                        }
                      }
                    }
                    // rounding down, unless a midpoint in [ODD, EVEN]
                  } else {
                    // the result is inexact; f1* <= 1/2
                    is_inexact_gt_midpoint = true;
                    is_inexact_lt_midpoint = false;
                    is_midpoint_gt_even = false;
                    is_midpoint_lt_even = false;
                    // set the inexact flag
                    tmp_inexact = true;
                    // *flags |= BID_INEXACT_EXCEPTION;
                  }
                } // end 'the result is not a midpoint'
                // n = C1 * 10^(e2+x1)
                c1.w[1] = q256.w[3];
                c1.w[0] = q256.w[2];
                y_exp = y_exp.wrapping_add(((x1 + 1) as u64) << 49);
              } else {
                // C1 < 10^34
                // c1.w[1] and c1.w[0] already set
                // n = C1 * 10^(e2+x1)
                y_exp = y_exp.wrapping_add((x1 as u64) << 49);
              }
              // check for overflow
              if y_exp == EXP_MAX_P1 && (rnd_mode == BID_ROUNDING_TO_NEAREST || rnd_mode == BID_ROUNDING_TIES_AWAY) {
                res.w[1] = 0x7800000000000000 | x_sign; // +/-inf
                res.w[0] = 0x0;
                // set the inexact flag
                *pfpsf |= BID_INEXACT_EXCEPTION;
                // set the overflow flag
                *pfpsf |= BID_OVERFLOW_EXCEPTION;
                return res;
              } // else no overflow
            } else {
              // if x_sign != y_sign the result of this subtract. is exact
              c1.w[0] = c1.w[0].wrapping_sub(r256.w[2]);
              c1.w[1] = c1.w[1].wrapping_sub(r256.w[3]);
              if c1.w[0] > tmp64 {
                c1.w[1] = c1.w[1].wrapping_sub(1); // borrow
              }
              if c1.w[1] >= 0x8000000000000000 {
                // negative coefficient!
                c1.w[0] = !c1.w[0];
                c1.w[0] = c1.w[0].wrapping_add(1);
                c1.w[1] = !c1.w[1];
                if c1.w[0] == 0x0 {
                  c1.w[1] = c1.w[1].wrapping_add(1);
                }
                tmp_sign = y_sign;
                // the result will have the sign of y if last rnd
              } else {
                tmp_sign = x_sign;
              }
              // If the difference has P34-1 digits or less, i.e. C1 < 10^33 then redo the calculation with x1 = x1-1.
              // Redo the calculation also if C1 = 10^33 and (is_inexact_gt_midpoint or is_midpoint_lt_even);
              //   (the last part should have really been
              //   (is_inexact_lt_midpoint or is_midpoint_gt_even) from
              //    the rounding of C2, but the position flags have been reversed)
              // 10^33 = 0x0000314dc6448d93 0x38c15b0a00000000
              if (c1.w[1] < 0x0000314dc6448d93 || (c1.w[1] == 0x0000314dc6448d93 && c1.w[0] < 0x38c15b0a00000000)) || (c1.w[1] == 0x0000314dc6448d93 && c1.w[0] == 0x38c15b0a00000000 && (is_inexact_gt_midpoint || is_midpoint_lt_even)) {
                // C1=10^33
                x1 = x1.wrapping_sub(1); // x1 >= 0
                if x1 >= 0 {
                  // clear position flags and tmp_inexact
                  is_midpoint_lt_even = false;
                  is_midpoint_gt_even = false;
                  is_inexact_lt_midpoint = false;
                  is_inexact_gt_midpoint = false;
                  tmp_inexact = false;
                  second_pass = true;
                  // Round again, else the result has less than P34 digits.
                  continue 'round_c2;
                }
              }
              // if the coefficient of the result is 10^34 it means that this
              // must be the second pass, and we are done
              if c1.w[1] == 0x0001ed09bead87c0 && c1.w[0] == 0x378d8e6400000000 {
                // if  C1 = 10^34
                c1.w[1] = 0x0000314dc6448d93; // C1 = 10^33
                c1.w[0] = 0x38c15b0a00000000;
                y_exp = y_exp.wrapping_add(1 << 49);
              }
              x_sign = tmp_sign;
              if x1 >= 1 {
                y_exp = y_exp.wrapping_add((x1 as u64) << 49);
              }
              // x1 = -1 is possible at the end of a second pass when the
              // first pass started with x1 = 1
            }
            c1_hi = c1.w[1];
            c1_lo = c1.w[0];
            // general correction from RN to RA, RM, RP, RZ; result uses y_exp
            if rnd_mode != BID_ROUNDING_TO_NEAREST {
              if (x_sign == 0 && ((rnd_mode == BID_ROUNDING_UP && is_inexact_lt_midpoint) || ((rnd_mode == BID_ROUNDING_TIES_AWAY || rnd_mode == BID_ROUNDING_UP) && is_midpoint_gt_even)))
                || (x_sign > 0 && ((rnd_mode == BID_ROUNDING_DOWN && is_inexact_lt_midpoint) || ((rnd_mode == BID_ROUNDING_TIES_AWAY || rnd_mode == BID_ROUNDING_DOWN) && is_midpoint_gt_even)))
              {
                // C1 = C1 + 1
                c1_lo = c1_lo.wrapping_add(1);
                if c1_lo == 0 {
                  // rounding overflow in the low 64 bits
                  c1_hi = c1_hi.wrapping_add(1);
                }
                if c1_hi == 0x0001ed09bead87c0 && c1_lo == 0x378d8e6400000000 {
                  // C1 = 10^34 => rounding overflow
                  c1_hi = 0x0000314dc6448d93;
                  c1_lo = 0x38c15b0a00000000; // 10^33
                  y_exp = y_exp.wrapping_add(EXP_P1);
                }
              } else if (is_midpoint_lt_even || is_inexact_gt_midpoint)
                && ((x_sign > 0 && (rnd_mode == BID_ROUNDING_UP || rnd_mode == BID_ROUNDING_TO_ZERO)) || (x_sign == 0 && (rnd_mode == BID_ROUNDING_DOWN || rnd_mode == BID_ROUNDING_TO_ZERO)))
              {
                // C1 = C1 - 1
                c1_lo = c1_lo.wrapping_sub(1);
                if c1_lo == 0xffffffffffffffff {
                  c1_hi = c1_hi.wrapping_sub(1);
                }
                // check if we crossed into the lower decade
                if c1_hi == 0x0000314dc6448d93 && c1_lo == 0x38c15b09ffffffff {
                  // 10^33 - 1
                  c1_hi = 0x0001ed09bead87c0; // 10^34 - 1
                  c1_lo = 0x378d8e63ffffffff;
                  y_exp = y_exp.wrapping_sub(EXP_P1);
                  // no underflow, because delta + q2 >= P34 + 1
                }
              } else { // exact, the result is already correct
              }
              // in all cases check for overflow (RN and RA solved already)
              if y_exp == EXP_MAX_P1 {
                // overflow
                if (rnd_mode == BID_ROUNDING_DOWN && x_sign > 0) || // RM and res < 0
                (rnd_mode == BID_ROUNDING_UP && x_sign == 0)
                {
                  // RP and res > 0
                  c1_hi = 0x7800000000000000; // +inf
                  c1_lo = 0x0;
                } else {
                  // RM and res > 0, RP and res < 0, or RZ
                  c1_hi = 0x5fffed09bead87c0;
                  c1_lo = 0x378d8e63ffffffff;
                }
                y_exp = 0; // x_sign is preserved
                // Set the inexact flag (in case the exact addition was exact).
                *pfpsf |= BID_INEXACT_EXCEPTION;
                // Set the overflow flag.
                *pfpsf |= BID_OVERFLOW_EXCEPTION;
              }
            }
            // Assemble the result.
            res.w[1] = x_sign | y_exp | c1_hi;
            res.w[0] = c1_lo;
            if tmp_inexact {
              *pfpsf |= BID_INEXACT_EXCEPTION;
            }
            break 'round_c2;
          }
        }
      } else {
        // if (-P34 + 1 <= delta <= -1) <=> 1 <= -delta <= P34 - 1
        // NOTE: the following, up to "} else { // if x_sign != y_sign
        // the result is exact" is identical to "else if (delta == P34 - q2) {"
        // from above; also, the code is not symmetric: a+b and b+a may take
        // different paths (need to unify eventually!)
        // calculate C' = C2 + C1 * 10^(e1-e2) directly; the result may be
        // inexact if it requires P34 + 1 decimal digits; in either case the
        // 'cutoff' point for addition is at the position of the lsb of C2
        // The coefficient of the result is C1 * 10^(e1-e2) + C2 and the
        // exponent is e2; either C1 or 10^(e1-e2) may not fit is 64 bits,
        // but their product fits with certainty in 128 bits (actually in 113)
        // Note that 0 <= e1 - e2 <= P34 - 2
        //   -P34 + 1 <= delta <= -1 <=> -P34 + 1 <= delta <= -1 <=>
        //   -P34 + 1 <= q1 + e1 - q2 - e2 <= -1 <=>
        //   q2 - q1 - P34 + 1 <= e1 - e2 <= q2 - q1 - 1 <=>
        //   1 - P34 - P34 + 1 <= e1-e2 <= P34 - 1 - 1 => 0 <= e1-e2 <= P34 - 2
        scale = delta - q1 + q2; // scale = (int)(e1 >> 49) - (int)(e2 >> 49)
        if scale >= 20 {
          // 10^(e1-e2) does not fit in 64 bits, but C1 does
          mul_128x64_to_128(&mut c1, c1_lo, bid_ten2k128(scale - 20));
        } else if scale >= 1 {
          // if 1 <= scale <= 19 then 10^(e1-e2) fits in 64 bits
          if q1 <= 19 {
            // C1 fits in 64 bits
            mul_64x64_to_128mach(&mut c1, c1_lo, bid_ten2k64!(scale));
          } else {
            // q1 >= 20
            c1.w[1] = c1_hi;
            c1.w[0] = c1_lo;
            let c1copy = c1;
            mul_128x64_to_128(&mut c1, bid_ten2k64!(scale), c1copy);
          }
        } else {
          // if (scale == 0) C1 is unchanged
          c1.w[1] = c1_hi;
          c1.w[0] = c1_lo; // only the low part is necessary
        }
        c1_hi = c1.w[1];
        c1_lo = c1.w[0];
        // now add C2
        if x_sign == y_sign {
          // the result can overflow!
          c1_lo = c1_lo.wrapping_add(c2_lo);
          c1_hi = c1_hi.wrapping_add(c2_hi);
          if c1_lo < c1.w[0] {
            c1_hi = c1_hi.wrapping_add(1);
          }
          // test for overflow, possible only when C1 >= 10^34
          if c1_hi > 0x0001ed09bead87c0 || (c1_hi == 0x0001ed09bead87c0 && c1_lo >= 0x378d8e6400000000) {
            // C1 >= 10^34
            // in this case q = P34 + 1 and x = q - P34 = 1, so multiply
            // C'' = C'+ 5 = C1 + 5 by k1 ~ 10^(-1) calculated for P34 + 1
            // decimal digits
            // Calculate C'' = C' + 1/2 * 10^x
            if c1_lo >= 0xfffffffffffffffb {
              // low half add has carry
              c1_lo = c1_lo.wrapping_add(5);
              c1_hi = c1_hi.wrapping_add(1);
            } else {
              c1_lo = c1_lo.wrapping_add(5);
            }
            // the approximation of 10^(-1) was rounded up to 118 bits
            // 10^(-1) =~ 33333333333333333333333333333400 * 2^-129
            // 10^(-1) =~ 19999999999999999999999999999a00 * 2^-128
            c1.w[1] = c1_hi;
            c1.w[0] = c1_lo; // C''
            ten2m1.w[1] = 0x1999999999999999;
            ten2m1.w[0] = 0x9999999999999a00;
            mul_128x128_to_256(&mut p256, c1, ten2m1); // P256 = C*, f*
            // C* is actually floor(C*) in this case
            // the top Ex = 128 bits of 10^(-1) are
            // T* = 0x00199999999999999999999999999999
            // if (0 < f* < 10^(-x)) then
            //   if floor(C*) is even then C = floor(C*) - logical right
            //       shift; C has p decimal digits, correct by Prop. 1)
            //   else if floor(C*) is odd C = floor(C*) - 1 (logical right
            //       shift; C has p decimal digits, correct by Pr. 1)
            // else
            //   C = floor(C*) (logical right shift; C has p decimal digits,
            //       correct by Property 1)
            // n = C * 10^(e2+x)
            if (p256.w[1] > 0 || p256.w[0] > 0) && (p256.w[1] < 0x1999999999999999 || (p256.w[1] == 0x1999999999999999 && p256.w[0] <= 0x9999999999999999)) {
              // the result is a midpoint
              if (p256.w[2] & 0x01) > 0 {
                is_midpoint_gt_even = true;
                // if floor(C*) is odd C = floor(C*) - 1; the result is not 0
                p256.w[2] = p256.w[2].wrapping_sub(1);
                if p256.w[2] == 0xffffffffffffffff {
                  p256.w[3] = p256.w[3].wrapping_sub(1);
                }
              } else {
                is_midpoint_lt_even = true;
              }
            }
            // n = Cstar * 10^(e2+1)
            y_exp = y_exp.wrapping_add(EXP_P1);
            // C* != 10^P34 because C* has P34 digits
            // check for overflow
            if y_exp == EXP_MAX_P1 && (rnd_mode == BID_ROUNDING_TO_NEAREST || rnd_mode == BID_ROUNDING_TIES_AWAY) {
              // overflow for RN
              res.w[1] = x_sign | 0x7800000000000000; // +/-inf
              res.w[0] = 0x0;
              // set the inexact flag
              *pfpsf |= BID_INEXACT_EXCEPTION;
              // set the overflow flag
              *pfpsf |= BID_OVERFLOW_EXCEPTION;
              return res;
            }
            // if (0 < f* - 1/2 < 10^(-x)) then
            //   the result of the addition is exact
            // else
            //   the result of the addition is inexact
            if p256.w[1] > 0x8000000000000000 || (p256.w[1] == 0x8000000000000000 && p256.w[0] > 0x0) {
              // the result may be exact
              tmp64 = p256.w[1] - 0x8000000000000000; // f* - 1/2
              if tmp64 > 0x1999999999999999 || (tmp64 == 0x1999999999999999 && p256.w[0] >= 0x9999999999999999) {
                // set the inexact flag
                *pfpsf |= BID_INEXACT_EXCEPTION;
                is_inexact = false;
              } // else the result is exact
            } else {
              // the result is inexact
              // set the inexact flag
              *pfpsf |= BID_INEXACT_EXCEPTION;
              is_inexact = true;
            }
            c1_hi = p256.w[3];
            c1_lo = p256.w[2];
            if !is_midpoint_gt_even && !is_midpoint_lt_even {
              is_inexact_lt_midpoint = is_inexact && (p256.w[1] & 0x8000000000000000) > 0;
              is_inexact_gt_midpoint = is_inexact && !(p256.w[1] & 0x8000000000000000) > 0;
            }
            // general correction from RN to RA, RM, RP, RZ; result uses y_exp
            if rnd_mode != BID_ROUNDING_TO_NEAREST {
              if (x_sign == 0 && ((rnd_mode == BID_ROUNDING_UP && is_inexact_lt_midpoint) || ((rnd_mode == BID_ROUNDING_TIES_AWAY || rnd_mode == BID_ROUNDING_UP) && is_midpoint_gt_even)))
                || (x_sign > 0 && ((rnd_mode == BID_ROUNDING_DOWN && is_inexact_lt_midpoint) || ((rnd_mode == BID_ROUNDING_TIES_AWAY || rnd_mode == BID_ROUNDING_DOWN) && is_midpoint_gt_even)))
              {
                // C1 = C1 + 1
                c1_lo = c1_lo.wrapping_add(1);
                if c1_lo == 0 {
                  // rounding overflow in the low 64 bits
                  c1_hi = c1_hi.wrapping_add(1);
                }
                if c1_hi == 0x0001ed09bead87c0 && c1_lo == 0x378d8e6400000000 {
                  // C1 = 10^34 => rounding overflow
                  c1_hi = 0x0000314dc6448d93;
                  c1_lo = 0x38c15b0a00000000; // 10^33
                  y_exp = y_exp.wrapping_add(EXP_P1);
                }
              } else if (is_midpoint_lt_even || is_inexact_gt_midpoint)
                && ((x_sign > 0 && (rnd_mode == BID_ROUNDING_UP || rnd_mode == BID_ROUNDING_TO_ZERO)) || (x_sign == 0 && (rnd_mode == BID_ROUNDING_DOWN || rnd_mode == BID_ROUNDING_TO_ZERO)))
              {
                // C1 = C1 - 1
                c1_lo = c1_lo.wrapping_sub(1);
                if c1_lo == 0xffffffffffffffff {
                  c1_hi = c1_hi.wrapping_sub(1);
                }
                // check if we crossed into the lower decade
                if c1_hi == 0x0000314dc6448d93 && c1_lo == 0x38c15b09ffffffff {
                  // 10^33 - 1
                  c1_hi = 0x0001ed09bead87c0; // 10^34 - 1
                  c1_lo = 0x378d8e63ffffffff;
                  y_exp = y_exp.wrapping_sub(EXP_P1);
                  // no underflow, because delta + q2 >= P34 + 1
                }
              } else { // exact, the result is already correct
              }
              // in all cases check for overflow (RN and RA solved already)
              if y_exp == EXP_MAX_P1 {
                // overflow
                if (rnd_mode == BID_ROUNDING_DOWN && x_sign > 0) || // RM and res < 0
                  (rnd_mode == BID_ROUNDING_UP && x_sign == 0)
                {
                  // RP and res > 0
                  c1_hi = 0x7800000000000000; // +inf
                  c1_lo = 0x0;
                } else {
                  // RM and res > 0, RP and res < 0, or RZ
                  c1_hi = 0x5fffed09bead87c0;
                  c1_lo = 0x378d8e63ffffffff;
                }
                y_exp = 0; // x_sign is preserved
                // set the inexact flag (in case the exact addition was exact)
                *pfpsf |= BID_INEXACT_EXCEPTION;
                // set the overflow flag
                *pfpsf |= BID_OVERFLOW_EXCEPTION;
              }
            }
          } // else if (C1 < 10^34) then C1 is the coeff.; the result is exact
          // Assemble the result.
          res.w[1] = x_sign | y_exp | c1_hi;
          res.w[0] = c1_lo;
        } else {
          // If x_sign != y_sign the result is exact.
          c1_lo = c2_lo.wrapping_sub(c1_lo);
          c1_hi = c2_hi.wrapping_sub(c1_hi);
          if c1_lo > c2_lo {
            c1_hi = c1_hi.wrapping_sub(1);
          }
          if c1_hi >= 0x8000000000000000 {
            // Negative coefficient!
            c1_lo = !c1_lo;
            c1_lo = c1_lo.wrapping_add(1);
            c1_hi = !c1_hi;
            if c1_lo == 0 {
              c1_hi = c1_hi.wrapping_add(1);
            }
          }
          // The result can be zero, but it cannot overflow.
          if c1_lo == 0 && c1_hi == 0 {
            // assemble the result
            if x_exp < y_exp {
              res.w[1] = x_exp;
            } else {
              res.w[1] = y_exp;
            }
            res.w[0] = 0;
            if rnd_mode == BID_ROUNDING_DOWN {
              res.w[1] |= 0x8000000000000000;
            }
            return res;
          }
          // Assemble the result.
          res.w[1] = y_sign | y_exp | c1_hi;
          res.w[0] = c1_lo;
        }
      }
    }
    res
  }
}

#[repr(C)]
pub union BidU64double {
  pub u: u64,
  pub f: f64,
}

#[inline(always)]
fn bits(value: u64) -> usize {
  ((((unsafe { BidU64double { f: value as f64 }.u } >> 52) as u32) & 0x7ff) - 0x3ff) as usize
}
