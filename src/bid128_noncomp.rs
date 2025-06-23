use crate::BidUint128;
use crate::bid128_common::*;

/// Returns `true` if value is `zero` (+0 or -0).
pub fn bid128_is_zero(x: BidUint128) -> bool {
  if (x.w[1] & MASK_INF) == MASK_INF {
    return false;
  }
  let sig_x = BidUint128 { w: [x.w[0], x.w[1] & 0x0001ffffffffffff] };
  if (sig_x.w[1] > 0x0001ed09bead87c0) /* significand is non-canonical */ ||
    ((sig_x.w[1] == 0x0001ed09bead87c0) && (sig_x.w[0] > 0x378d8e63ffffffff)) /* significand is non-canonical */ ||
    ((x.w[1] & MASK_STEERING_BITS) == MASK_STEERING_BITS) /* value has steering bits set */ ||
    (sig_x.w[1] == 0 && sig_x.w[0] == 0 /* significand is 0 */)
  {
    return true;
  }
  false
}
