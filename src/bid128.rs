//! # 128-bit decimal floating-point

use crate::bid64::Bid64;
use crate::bid128_common::*;
use core::fmt;
use core::fmt::Debug;

/// 128-bit decimal floating-point in binary format.
#[repr(C, align(16))]
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct Bid128 {
  pub(crate) w: [Bid64; 2],
}

impl Debug for Bid128 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{:016X} {:016X}]", self.w[1], self.w[0])
  }
}

impl Bid128 {
  /// Creates a new 128-bit decimal floating-point value from HI and LO 64-bit words.
  pub fn new(hi: Bid64, lo: Bid64) -> Self {
    Self { w: [lo, hi] }
  }

  /// Returns a positive zero `+0` as 128-bit decimal floating-point value.
  pub const fn zero() -> Self {
    Self { w: BID128_ZERO }
  }

  /// Returns a negative zero `-0` as 128-bit decimal floating-point value.
  pub const fn minus_zero() -> Self {
    Self { w: BID128_MINUS_ZERO }
  }

  /// Returns a `+NaN` as 128-bit decimal floating-point value.
  pub const fn nan() -> Self {
    Self { w: BID128_NAN }
  }

  /// Returns a `-NaN` as 128-bit decimal floating-point value.
  pub const fn minus_nan() -> Self {
    Self { w: BID128_MINUS_NAN }
  }

  /// Returns a `+QNaN` as 128-bit decimal floating-point value.
  pub const fn qnan() -> Self {
    Self::nan()
  }

  /// Returns a `-QNaN` as 128-bit decimal floating-point value.
  pub const fn minus_qnan() -> Self {
    Self::minus_nan()
  }

  /// Returns a `+SNaN` as 128-bit decimal floating-point value.
  pub const fn snan() -> Self {
    Self { w: BID128_SNAN }
  }

  /// Returns a `-SNaN` as 128-bit decimal floating-point value.
  pub const fn minus_snan() -> Self {
    Self { w: BID128_MINUS_SNAN }
  }

  /// Returns a positive infinity `+inf` as 128-bit decimal floating-point value.
  pub const fn inf() -> Self {
    Self { w: BID128_INF }
  }

  /// Returns a negative infinity `-inf` as 128-bit decimal floating-point value.
  pub const fn minus_inf() -> Self {
    Self { w: BID128_MINUS_INF }
  }

  /// Returns a minimum possible 128-bit decimal floating-point value.
  pub const fn min() -> Self {
    Self { w: BID128_MIN }
  }

  /// Returns a maximum possible 128-bit decimal floating-point value.
  pub const fn max() -> Self {
    Self { w: BID128_MAX }
  }

  /// Returns `true` if this value is `zero` (+0 or -0).
  pub fn is_zero(&self) -> bool {
    if (self.w[1] & MASK_INF) == MASK_INF {
      return false;
    }
    let sig: [Bid64; 2] = [self.w[0], self.w[1] & 0x0001ffffffffffff];
    if (sig[1] > 0x0001ed09bead87c0) /* significand is non-canonical */ ||
      ((sig[1] == 0x0001ed09bead87c0) && (sig[0] > 0x378d8e63ffffffff)) /* significand is non-canonical */ ||
      ((self.w[1] & MASK_STEERING_BITS) == MASK_STEERING_BITS) /* value has steering bits set */ ||
      (sig[1] == 0 && sig[0] == 0 /* significand is 0 */)
    {
      return true;
    }
    false
  }

  /// Returns `true` if this is non-signaling non-value (`NaN`, `-NaN`, `QNaN`, `-QNaN`).
  pub fn is_nan(&self) -> bool {
    self.w[1] & MASK_NAN == MASK_NAN
  }

  /// Returns `true` if this is signaling non-value (`SNaN` or `-SNaN`).
  pub fn is_snan(&self) -> bool {
    self.w[1] & MASK_SNAN == MASK_SNAN
  }
}
