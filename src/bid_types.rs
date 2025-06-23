use core::fmt;
use core::fmt::Debug;

/// A 32-bit decimal floating-point number represented in binary format.
pub type BidUint32 = u32;

/// A 64-bit decimal floating-point number represented in binary format.
pub type BidUint64 = u64;

/// A 128-bit decimal floating-point number represented in binary format.
///
/// This structure is used to represent a 128-bit decimal floating-point value
/// using two 64-bit unsigned integer words.
///
/// # Representation Details
///
/// The value is stored in a little-endian format:
/// - `w[0]` contains the **least significant** 64 bits.
/// - `w[1]` contains the **most significant** 64 bits.
///
/// # Examples
///
/// ```
/// use decimus::BidUint128;
///
/// let x = BidUint128 {
///     w: [0x0000000000000000, 0x3040000000000000],
/// };
///
/// // Represents a 128-bit binary value
/// // where the lower 64 bits are 0x0000000000000000
/// // and the upper 64 bits are 0x3040000000000000.
/// ```
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct BidUint128 {
  /// The underlying 64-bit words that make up the 128-bit decimal floating-point value.
  ///
  /// - `w[0]`: Least significant word (lower 64 bits).
  /// - `w[1]`: Most significant word (upper 64 bits).
  pub w: [BidUint64; 2],
}

impl Debug for BidUint128 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{:016X} {:016X}]", self.w[1], self.w[0])
  }
}

/// 256-bit decimal floating-point in binary format.
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct BidUint192 {
  /// The underlying 64-bit words that make up the 192-bit decimal floating-point value.
  ///
  /// - `w[0]`: Least significant word (lower 64 bits).
  /// - `w[1]`: Middle word (middle 64 bits).
  /// - `w[2]`: Most significant word (upper 64 bits).
  pub w: [BidUint64; 3],
}

impl Debug for BidUint192 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{:016X} {:016X} {:016X}]", self.w[2], self.w[1], self.w[0])
  }
}

/// 256-bit decimal floating-point in binary format.
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct BidUint256 {
  /// The underlying 64-bit words that make up the 256-bit decimal floating-point value.
  ///
  /// - `w[0]`: Least significant word (lower 64 bits).
  /// - `w[1]`: Least significant middle word (lower middle 64 bits).
  /// - `w[2]`: Most significant middle word (upper middle 64 bits).
  /// - `w[3]`: Most significant word (upper 64 bits).
  pub w: [BidUint64; 4],
}

impl Debug for BidUint256 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{:016X} {:016X} {:016X} {:016X}]", self.w[3], self.w[2], self.w[1], self.w[0])
  }
}
