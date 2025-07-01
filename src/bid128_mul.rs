//! # bid128_mul

use crate::bid_conf::*;
use crate::bid128_common::*;
use crate::bid128_fma::bid128_fma;
use crate::{BidUint64, BidUint128};

/// Multiplies two 128-bit decimal floating-point values.
pub fn bid128_mul(x: BidUint128, y: BidUint128, _rnd_mode: IdecRound, _pfpsf: &mut IdecFlags) -> BidUint128 {
  let z = BidUint128 { w: [0x0000000000000000, 0x5ffe000000000000] };
  let mut res = BidUint128 { w: [0xbaddbaddbaddbadd, 0xbaddbaddbaddbadd] };
  let x_sign: BidUint64;
  let y_sign: BidUint64;
  let x_exp: BidUint64;
  let p_sign: BidUint64;
  let y_exp: BidUint64;
  let p_exp: BidUint64;
  let mut c1 = BidUint128::default();
  let mut c2 = BidUint128::default();
  let true_p_exp: i32;

  // Skip cases where at least one operand is NaN or infinity.
  if !(((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN) || ((x.w[1] & MASK_ANY_INF) == MASK_INF) || ((y.w[1] & MASK_ANY_INF) == MASK_INF)) {
    // x, y are 0 or f but not inf or NaN => unpack the arguments and check
    // for non-canonical values

    x_sign = x.w[1] & MASK_SIGN; // 0 for positive, MASK_SIGN for negative
    c1.w[1] = x.w[1] & MASK_COEFF;
    c1.w[0] = x.w[0];
    // check for non-canonical values - treated as zero
    if (x.w[1] & 0x6000000000000000) == 0x6000000000000000 {
      // G0_G1=11 => non-canonical
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
      }
    }
    y_sign = y.w[1] & MASK_SIGN; // 0 for positive, MASK_SIGN for negative
    c2.w[1] = y.w[1] & MASK_COEFF;
    c2.w[0] = y.w[0];
    // check for non-canonical values - treated as zero
    if (y.w[1] & 0x6000000000000000) == 0x6000000000000000 {
      // G0_G1=11 => non-canonical
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
      }
    }
    p_sign = x_sign ^ y_sign; // sign of the product

    true_p_exp = (x_exp >> 49) as i32 - 6176 + (y_exp >> 49) as i32 - 6176;
    // true_p_exp, p_exp are used only for 0 * 0, 0 * f, or f * 0
    if true_p_exp < -6176 {
      p_exp = 0; // cannot be less than EXP_MIN
    } else if true_p_exp > 6111 {
      p_exp = ((6111 + 6176) as u64) << 49; // cannot be more than EXP_MAX
    } else {
      p_exp = ((true_p_exp + 6176) as u64) << 49;
    }

    if (c1.w[1] == 0 && c1.w[0] == 0) || (c2.w[1] == 0 && c2.w[0] == 0) {
      // x = 0 or y = 0
      // the result is 0
      res.w[1] = p_sign | p_exp; // preferred exponent in [EXP_MIN, EXP_MAX]
      res.w[0] = 0;
      return res;
    }
  }

  bid128_fma(y, x, z, _rnd_mode, _pfpsf)
}
