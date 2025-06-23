use crate::BidUint128;
use crate::bid_internal::*;

/// Converts signed 32-bit integer into 128-bit decimal floting-point value.
pub fn bid128_from_int32(x: i32) -> BidUint128 {
  let mut res = BidUint128::default();
  // If integer is negative, use the absolute value.
  let u = x as u32;
  if (u & SIGNMASK32) == SIGNMASK32 {
    res.w[1] = 0xb040000000000000;
    res.w[0] = (!u + 1) as u64; // 2's complement of x
  } else {
    res.w[1] = 0x3040000000000000;
    res.w[0] = u as u64;
  }
  res
}
