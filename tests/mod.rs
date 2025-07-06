use decimus::IdecFlags;

mod test_bid128;
mod test_debug;

type BidArray = [u64; 2];

const F_00_00: IdecFlags = 0x00;
const F_01_00: IdecFlags = if cfg!(feature = "bid-set-status-flags") { 0x01 } else { 0x00 };
const F_01_01: IdecFlags = 0x01;
const F_04_00: IdecFlags = if cfg!(feature = "bid-set-status-flags") { 0x04 } else { 0x00 };

/// ```text
/// | bid-set-status-flags | Flags |
/// | ON                   | 0x20  |
/// | OFF                  | 0x00  |
/// ```
const F_20_00: IdecFlags = if cfg!(feature = "bid-set-status-flags") { 0x20 } else { 0x00 };
const F_20_20: IdecFlags = 0x20;
const F_28_00: IdecFlags = if cfg!(feature = "bid-set-status-flags") { 0x28 } else { 0x00 };
const F_28_28: IdecFlags = 0x28;
const F_29_01: IdecFlags = if cfg!(feature = "bid-set-status-flags") { 0x29 } else { 0x01 };
const F_30_00: IdecFlags = if cfg!(feature = "bid-set-status-flags") { 0x30 } else { 0x00 };
const F_30_20: IdecFlags = if cfg!(feature = "bid-set-status-flags") { 0x30 } else { 0x20 };
const F_30_30: IdecFlags = 0x30;

const TAB_1: IdecFlags = {
  match (cfg!(feature = "bid-set-status-flags"), cfg!(feature = "decimal-tiny-detection-after-rounding")) {
    (false, false) => 0x30,
    (true, false) => 0x30,
    (false, true) => 0x20,
    (true, true) => 0x20,
  }
};
