use core::num::Wrapping;
use std::ops::{Add, BitXor, Deref, Div, Mul, Sub};

pub type Bid64 = u64;

/// 64-bit unsigned integer value with always wrapping arithmetic.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct BidUint64(Wrapping<u64>);

impl BidUint64 {
  /// Creates a new [BidUint64] value.
  pub fn new(value: u64) -> Self {
    BidUint64(Wrapping(value))
  }
}

impl Add for BidUint64 {
  type Output = BidUint64;

  fn add(self, rhs: Self) -> Self::Output {
    BidUint64(self.0 + rhs.0)
  }
}

impl Add<u32> for BidUint64 {
  type Output = BidUint64;

  fn add(self, rhs: u32) -> Self::Output {
    BidUint64(self.0 + Wrapping(rhs as u64))
  }
}

impl Add<u64> for BidUint64 {
  type Output = BidUint64;

  fn add(self, rhs: u64) -> Self::Output {
    BidUint64(self.0 + Wrapping(rhs))
  }
}

impl Add<BidUint64> for u32 {
  type Output = BidUint64;

  fn add(self, rhs: BidUint64) -> Self::Output {
    BidUint64(Wrapping(self as u64) + rhs.0)
  }
}

impl Add<BidUint64> for u64 {
  type Output = BidUint64;

  fn add(self, rhs: BidUint64) -> Self::Output {
    BidUint64(Wrapping(self) + rhs.0)
  }
}

impl Sub for BidUint64 {
  type Output = BidUint64;

  fn sub(self, rhs: Self) -> Self::Output {
    BidUint64(self.0 - rhs.0)
  }
}

impl Mul for BidUint64 {
  type Output = BidUint64;

  fn mul(self, rhs: Self) -> Self::Output {
    BidUint64(self.0 * rhs.0)
  }
}

impl Div for BidUint64 {
  type Output = BidUint64;

  fn div(self, rhs: Self) -> Self::Output {
    BidUint64(self.0 / rhs.0)
  }
}

impl BitXor for BidUint64 {
  type Output = BidUint64;

  fn bitxor(self, rhs: Self) -> Self::Output {
    BidUint64(self.0 ^ rhs.0)
  }
}

impl Deref for BidUint64 {
  type Target = u64;

  fn deref(&self) -> &Self::Target {
    &self.0.0
  }
}
