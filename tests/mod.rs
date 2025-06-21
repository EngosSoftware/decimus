use decimus::IdecFlags;

mod test_bid128;
mod test_bid64;
mod test_debug;
mod test_is_nan;
mod test_is_snan;
mod test_is_zero;

/// Utility function that returns expected status flags based on feature flags.
#[allow(unused_variables)]
const fn flags(bid_set_status_flags: IdecFlags, no_bid_set_status_flags: IdecFlags) -> IdecFlags {
  #[cfg(feature = "bid-set-status-flags")]
  {
    bid_set_status_flags
  }
  #[cfg(not(feature = "bid-set-status-flags"))]
  {
    no_bid_set_status_flags
  }
}

const F_00_00: IdecFlags = flags(0x00, 0x00);
const F_01_01: IdecFlags = flags(0x01, 0x01);
const F_20_00: IdecFlags = flags(0x20, 0x00);
const F_20_20: IdecFlags = flags(0x20, 0x20);
const F_28_00: IdecFlags = flags(0x28, 0x00);
const F_28_28: IdecFlags = flags(0x28, 0x28);
const F_29_01: IdecFlags = flags(0x29, 0x01);
const F_30_00: IdecFlags = flags(0x30, 0x00);
const F_30_20: IdecFlags = flags(0x30, 0x20);
