#![allow(unused_macros)]

use crate::bid_decimal_data::*;
use crate::bid_functions::*;
use crate::{BidUint64, BidUint128, BidUint256, IdecFlags, IdecRound};

//pub const EXP_MIN: u64 = 0x0000000000000000;
// EXP_MIN = (-6176 + 6176) << 49
//pub const EXP_MAX: u64 = 0x5ffe000000000000;
// EXP_MAX = (6111 + 6176) << 49
pub const EXP_MAX_P1: u64 = 0x6000000000000000;
// EXP_MAX + 1 = (6111 + 6176 + 1) << 49
pub const EXP_P1: u64 = 0x0002000000000000;

pub const EXPMAX: i32 = 6111;

pub const EXPMIN: i32 = -6176;

pub const SIGNMASK32: u32 = 0x80000000;

pub const QUIET_MASK64: u64 = 0xfdffffffffffffff;

//pub const P7: i32 = 7;
//pub const P16: i32 = 16;
pub const P34: i32 = 34;

pub const DECIMAL_EXPONENT_BIAS_128: i32 = 6176;
pub const DECIMAL_MAX_EXPON_128: i32 = 12287;
pub const MAX_FORMAT_DIGITS_128: i32 = 34;
pub const _LARGEST_BID128_HIGH: u64 = 0x5fffed09bead87c0;
pub const _LARGEST_BID128_LOW: u64 = 0x378d8e63ffffffff;
pub const INFINITY_MASK64: u64 = 0x7800000000000000;

//======================================
// Utility macros
//======================================

macro_rules! inc {
  ($x:expr, $y:expr) => {
    $x = ($x).wrapping_add($y)
  };
  ($x:expr) => {
    $x = ($x).wrapping_add(1)
  };
}
pub(crate) use inc;

macro_rules! dec {
  ($x:expr, $y:expr) => {
    $x = ($x).wrapping_sub($y)
  };
  ($x:expr) => {
    $x = ($x).wrapping_sub(1)
  };
}
pub(crate) use dec;

//======================================
// Status flag handling
//======================================

macro_rules! set_status_flags {
  ($fpsc:expr, $status:expr) => {{ *$fpsc |= $status }};
}

pub(crate) use set_status_flags;

macro_rules! is_inexact {
  ($fpsc:expr) => {
    *$fpsc & BID_INEXACT_EXCEPTION == BID_INEXACT_EXCEPTION
  };
}

//======================================
// String macros
//======================================

macro_rules! tolower_macro {
  ($x:expr) => {
    if $x.is_ascii_uppercase() { $x - b'A' + b'a' } else { $x }
  };
}
pub(crate) use tolower_macro;

macro_rules! bid_nr_digits {
  ($index:expr) => {
    BID_NR_DIGITS[$index as usize]
  };
}
pub(crate) use bid_nr_digits;

pub struct DecDigits {
  pub digits: u32,
  pub threshold_hi: u64,
  pub threshold_lo: u64,
  pub digits1: u32,
}

/// Table BID_NR_DIGITS
///
/// The first entry of BID_NR_DIGITS`[i - 1`] (where 1 <= i <= 113),
/// indicates the number of decimal digits needed to represent a binary number with `i` bits.
/// However, if a binary number of `i` bits may require either `k` or `k + 1` decimal digits,
/// then the first entry of BID_NR_DIGITS\[i - 1\] is 0. In this case if the number
/// is less than the value represented by the second and third entries concatenated,
/// then the number of decimal digits `k` is the fourth entry, else the number of decimal
/// digits is the fourth entry plus **1**.
#[rustfmt::skip]
pub const BID_NR_DIGITS: [DecDigits; 113] = [
  DecDigits{ digits:  1, threshold_hi: 0x0000000000000000, threshold_lo: 0x000000000000000a, digits1:  1 }, //  Only the first entry is used if it is not 0.
  DecDigits{ digits:  1, threshold_hi: 0x0000000000000000, threshold_lo: 0x000000000000000a, digits1:  1 }, //   1-bit n < 10^1
  DecDigits{ digits:  1, threshold_hi: 0x0000000000000000, threshold_lo: 0x000000000000000a, digits1:  1 }, //   2-bit n < 10^1
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000000, threshold_lo: 0x000000000000000a, digits1:  1 }, //   3-bit n < 10^1
  DecDigits{ digits:  2, threshold_hi: 0x0000000000000000, threshold_lo: 0x0000000000000064, digits1:  2 }, //   4-bit n ? 10^1
  DecDigits{ digits:  2, threshold_hi: 0x0000000000000000, threshold_lo: 0x0000000000000064, digits1:  2 }, //   5-bit n < 10^2
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000000, threshold_lo: 0x0000000000000064, digits1:  2 }, //   6-bit n < 10^2
  DecDigits{ digits:  3, threshold_hi: 0x0000000000000000, threshold_lo: 0x00000000000003e8, digits1:  3 }, //   7-bit n ? 10^2
  DecDigits{ digits:  3, threshold_hi: 0x0000000000000000, threshold_lo: 0x00000000000003e8, digits1:  3 }, //   8-bit n < 10^3
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000000, threshold_lo: 0x00000000000003e8, digits1:  3 }, //   9-bit n < 10^3
  DecDigits{ digits:  4, threshold_hi: 0x0000000000000000, threshold_lo: 0x0000000000002710, digits1:  4 }, //  10-bit n ? 10^3
  DecDigits{ digits:  4, threshold_hi: 0x0000000000000000, threshold_lo: 0x0000000000002710, digits1:  4 }, //  11-bit n < 10^4
  DecDigits{ digits:  4, threshold_hi: 0x0000000000000000, threshold_lo: 0x0000000000002710, digits1:  4 }, //  12-bit n < 10^4
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000000, threshold_lo: 0x0000000000002710, digits1:  4 }, //  13-bit n < 10^4
  DecDigits{ digits:  5, threshold_hi: 0x0000000000000000, threshold_lo: 0x00000000000186a0, digits1:  5 }, //  14-bit n ? 10^4
  DecDigits{ digits:  5, threshold_hi: 0x0000000000000000, threshold_lo: 0x00000000000186a0, digits1:  5 }, //  15-bit n < 10^5
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000000, threshold_lo: 0x00000000000186a0, digits1:  5 }, //  16-bit n < 10^5
  DecDigits{ digits:  6, threshold_hi: 0x0000000000000000, threshold_lo: 0x00000000000f4240, digits1:  6 }, //  17-bit n ? 10^5
  DecDigits{ digits:  6, threshold_hi: 0x0000000000000000, threshold_lo: 0x00000000000f4240, digits1:  6 }, //  18-bit n < 10^6
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000000, threshold_lo: 0x00000000000f4240, digits1:  6 }, //  19-bit n < 10^6
  DecDigits{ digits:  7, threshold_hi: 0x0000000000000000, threshold_lo: 0x0000000000989680, digits1:  7 }, //  20-bit n ? 10^6
  DecDigits{ digits:  7, threshold_hi: 0x0000000000000000, threshold_lo: 0x0000000000989680, digits1:  7 }, //  21-bit n < 10^7
  DecDigits{ digits:  7, threshold_hi: 0x0000000000000000, threshold_lo: 0x0000000000989680, digits1:  7 }, //  22-bit n < 10^7
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000000, threshold_lo: 0x0000000000989680, digits1:  7 }, //  23-bit n < 10^7
  DecDigits{ digits:  8, threshold_hi: 0x0000000000000000, threshold_lo: 0x0000000005f5e100, digits1:  8 }, //  24-bit n ? 10^7
  DecDigits{ digits:  8, threshold_hi: 0x0000000000000000, threshold_lo: 0x0000000005f5e100, digits1:  8 }, //  25-bit n < 10^8
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000000, threshold_lo: 0x0000000005f5e100, digits1:  8 }, //  26-bit n < 10^8
  DecDigits{ digits:  9, threshold_hi: 0x0000000000000000, threshold_lo: 0x000000003b9aca00, digits1:  9 }, //  27-bit n ? 10^8
  DecDigits{ digits:  9, threshold_hi: 0x0000000000000000, threshold_lo: 0x000000003b9aca00, digits1:  9 }, //  28-bit n < 10^9
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000000, threshold_lo: 0x000000003b9aca00, digits1:  9 }, //  29-bit n < 10^9
  DecDigits{ digits: 10, threshold_hi: 0x0000000000000000, threshold_lo: 0x00000002540be400, digits1: 10 },	//  30-bit n ? 10^9
  DecDigits{ digits: 10, threshold_hi: 0x0000000000000000, threshold_lo: 0x00000002540be400, digits1: 10 },	//  31-bit n < 10^10
  DecDigits{ digits: 10, threshold_hi: 0x0000000000000000, threshold_lo: 0x00000002540be400, digits1: 10 },	//  32-bit n < 10^10
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000000, threshold_lo: 0x00000002540be400, digits1: 10 },	//  33-bit n < 10^10
  DecDigits{ digits: 11, threshold_hi: 0x0000000000000000, threshold_lo: 0x000000174876e800, digits1: 11 },	//  34-bit n ? 10^10
  DecDigits{ digits: 11, threshold_hi: 0x0000000000000000, threshold_lo: 0x000000174876e800, digits1: 11 },	//  35-bit n < 10^11
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000000, threshold_lo: 0x000000174876e800, digits1: 11 },	//  36-bit n < 10^11
  DecDigits{ digits: 12, threshold_hi: 0x0000000000000000, threshold_lo: 0x000000e8d4a51000, digits1: 12 },	//  37-bit n ? 10^11
  DecDigits{ digits: 12, threshold_hi: 0x0000000000000000, threshold_lo: 0x000000e8d4a51000, digits1: 12 },	//  38-bit n < 10^12
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000000, threshold_lo: 0x000000e8d4a51000, digits1: 12 },	//  39-bit n < 10^12
  DecDigits{ digits: 13, threshold_hi: 0x0000000000000000, threshold_lo: 0x000009184e72a000, digits1: 13 },	//  40-bit n ? 10^12
  DecDigits{ digits: 13, threshold_hi: 0x0000000000000000, threshold_lo: 0x000009184e72a000, digits1: 13 },	//  41-bit n < 10^13
  DecDigits{ digits: 13, threshold_hi: 0x0000000000000000, threshold_lo: 0x000009184e72a000, digits1: 13 },	//  42-bit n < 10^13
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000000, threshold_lo: 0x000009184e72a000, digits1: 13 },	//  43-bit n < 10^13
  DecDigits{ digits: 14, threshold_hi: 0x0000000000000000, threshold_lo: 0x00005af3107a4000, digits1: 14 },	//  44-bit n ? 10^13
  DecDigits{ digits: 14, threshold_hi: 0x0000000000000000, threshold_lo: 0x00005af3107a4000, digits1: 14 },	//  45-bit n < 10^14
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000000, threshold_lo: 0x00005af3107a4000, digits1: 14 },	//  46-bit n < 10^14
  DecDigits{ digits: 15, threshold_hi: 0x0000000000000000, threshold_lo: 0x00038d7ea4c68000, digits1: 15 },	//  47-bit n ? 10^14
  DecDigits{ digits: 15, threshold_hi: 0x0000000000000000, threshold_lo: 0x00038d7ea4c68000, digits1: 15 },	//  48-bit n < 10^15
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000000, threshold_lo: 0x00038d7ea4c68000, digits1: 15 },	//  49-bit n < 10^15
  DecDigits{ digits: 16, threshold_hi: 0x0000000000000000, threshold_lo: 0x002386f26fc10000, digits1: 16 },	//  50-bit n ? 10^15
  DecDigits{ digits: 16, threshold_hi: 0x0000000000000000, threshold_lo: 0x002386f26fc10000, digits1: 16 },	//  51-bit n < 10^16
  DecDigits{ digits: 16, threshold_hi: 0x0000000000000000, threshold_lo: 0x002386f26fc10000, digits1: 16 },	//  52-bit n < 10^16
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000000, threshold_lo: 0x002386f26fc10000, digits1: 16 },	//  53-bit n < 10^16
  DecDigits{ digits: 17, threshold_hi: 0x0000000000000000, threshold_lo: 0x016345785d8a0000, digits1: 17 },	//  54-bit n ? 10^16
  DecDigits{ digits: 17, threshold_hi: 0x0000000000000000, threshold_lo: 0x016345785d8a0000, digits1: 17 },	//  55-bit n < 10^17
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000000, threshold_lo: 0x016345785d8a0000, digits1: 17 },	//  56-bit n < 10^17
  DecDigits{ digits: 18, threshold_hi: 0x0000000000000000, threshold_lo: 0x0de0b6b3a7640000, digits1: 18 },	//  57-bit n ? 10^17
  DecDigits{ digits: 18, threshold_hi: 0x0000000000000000, threshold_lo: 0x0de0b6b3a7640000, digits1: 18 },	//  58-bit n < 10^18
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000000, threshold_lo: 0x0de0b6b3a7640000, digits1: 18 },	//  59-bit n < 10^18
  DecDigits{ digits: 19, threshold_hi: 0x0000000000000000, threshold_lo: 0x8ac7230489e80000, digits1: 19 },	//  60-bit n ? 10^18
  DecDigits{ digits: 19, threshold_hi: 0x0000000000000000, threshold_lo: 0x8ac7230489e80000, digits1: 19 },	//  61-bit n < 10^19
  DecDigits{ digits: 19, threshold_hi: 0x0000000000000000, threshold_lo: 0x8ac7230489e80000, digits1: 19 },	//  62-bit n < 10^19
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000000, threshold_lo: 0x8ac7230489e80000, digits1: 19 },	//  63-bit n < 10^19
  DecDigits{ digits: 20, threshold_hi: 0x0000000000000005, threshold_lo: 0x6bc75e2d63100000, digits1: 20 },	//  64-bit n ? 10^19
  DecDigits{ digits: 20, threshold_hi: 0x0000000000000005, threshold_lo: 0x6bc75e2d63100000, digits1: 20 },	//  65-bit n < 10^20
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000005, threshold_lo: 0x6bc75e2d63100000, digits1: 20 },	//  66-bit n < 10^20
  DecDigits{ digits: 21, threshold_hi: 0x0000000000000036, threshold_lo: 0x35c9adc5dea00000, digits1: 21 },	//  67-bit n ? 10^20
  DecDigits{ digits: 21, threshold_hi: 0x0000000000000036, threshold_lo: 0x35c9adc5dea00000, digits1: 21 },	//  68-bit n < 10^21
  DecDigits{ digits:  0, threshold_hi: 0x0000000000000036, threshold_lo: 0x35c9adc5dea00000, digits1: 21 },	//  69-bit n < 10^21
  DecDigits{ digits: 22, threshold_hi: 0x000000000000021e, threshold_lo: 0x19e0c9bab2400000, digits1: 22 },	//  70-bit n ? 10^21
  DecDigits{ digits: 22, threshold_hi: 0x000000000000021e, threshold_lo: 0x19e0c9bab2400000, digits1: 22 },	//  71-bit n < 10^22
  DecDigits{ digits: 22, threshold_hi: 0x000000000000021e, threshold_lo: 0x19e0c9bab2400000, digits1: 22 },	//  72-bit n < 10^22
  DecDigits{ digits:  0, threshold_hi: 0x000000000000021e, threshold_lo: 0x19e0c9bab2400000, digits1: 22 },	//  73-bit n < 10^22
  DecDigits{ digits: 23, threshold_hi: 0x000000000000152d, threshold_lo: 0x02c7e14af6800000, digits1: 23 },	//  74-bit n ? 10^22
  DecDigits{ digits: 23, threshold_hi: 0x000000000000152d, threshold_lo: 0x02c7e14af6800000, digits1: 23 },	//  75-bit n < 10^23
  DecDigits{ digits:  0, threshold_hi: 0x000000000000152d, threshold_lo: 0x02c7e14af6800000, digits1: 23 },	//  76-bit n < 10^23
  DecDigits{ digits: 24, threshold_hi: 0x000000000000d3c2, threshold_lo: 0x1bcecceda1000000, digits1: 24 },	//  77-bit n ? 10^23
  DecDigits{ digits: 24, threshold_hi: 0x000000000000d3c2, threshold_lo: 0x1bcecceda1000000, digits1: 24 },	//  78-bit n < 10^24
  DecDigits{ digits:  0, threshold_hi: 0x000000000000d3c2, threshold_lo: 0x1bcecceda1000000, digits1: 24 },	//  79-bit n < 10^24
  DecDigits{ digits: 25, threshold_hi: 0x0000000000084595, threshold_lo: 0x161401484a000000, digits1: 25 },	//  80-bit n ? 10^24
  DecDigits{ digits: 25, threshold_hi: 0x0000000000084595, threshold_lo: 0x161401484a000000, digits1: 25 },	//  81-bit n < 10^25
  DecDigits{ digits: 25, threshold_hi: 0x0000000000084595, threshold_lo: 0x161401484a000000, digits1: 25 },	//  82-bit n < 10^25
  DecDigits{ digits:  0, threshold_hi: 0x0000000000084595, threshold_lo: 0x161401484a000000, digits1: 25 },	//  83-bit n < 10^25
  DecDigits{ digits: 26, threshold_hi: 0x000000000052b7d2, threshold_lo: 0xdcc80cd2e4000000, digits1: 26 },	//  84-bit n ? 10^25
  DecDigits{ digits: 26, threshold_hi: 0x000000000052b7d2, threshold_lo: 0xdcc80cd2e4000000, digits1: 26 },	//  85-bit n < 10^26
  DecDigits{ digits:  0, threshold_hi: 0x000000000052b7d2, threshold_lo: 0xdcc80cd2e4000000, digits1: 26 },	//  86-bit n < 10^26
  DecDigits{ digits: 27, threshold_hi: 0x00000000033b2e3c, threshold_lo: 0x9fd0803ce8000000, digits1: 27 },	//  87-bit n ? 10^26
  DecDigits{ digits: 27, threshold_hi: 0x00000000033b2e3c, threshold_lo: 0x9fd0803ce8000000, digits1: 27 },	//  88-bit n < 10^27
  DecDigits{ digits:  0, threshold_hi: 0x00000000033b2e3c, threshold_lo: 0x9fd0803ce8000000, digits1: 27 },	//  89-bit n < 10^27
  DecDigits{ digits: 28, threshold_hi: 0x00000000204fce5e, threshold_lo: 0x3e25026110000000, digits1: 28 },	//  90-bit n ? 10^27
  DecDigits{ digits: 28, threshold_hi: 0x00000000204fce5e, threshold_lo: 0x3e25026110000000, digits1: 28 },	//  91-bit n < 10^28
  DecDigits{ digits: 28, threshold_hi: 0x00000000204fce5e, threshold_lo: 0x3e25026110000000, digits1: 28 },	//  92-bit n < 10^28
  DecDigits{ digits:  0, threshold_hi: 0x00000000204fce5e, threshold_lo: 0x3e25026110000000, digits1: 28 },	//  93-bit n < 10^28
  DecDigits{ digits: 29, threshold_hi: 0x00000001431e0fae, threshold_lo: 0x6d7217caa0000000, digits1: 29 },	//  94-bit n ? 10^28
  DecDigits{ digits: 29, threshold_hi: 0x00000001431e0fae, threshold_lo: 0x6d7217caa0000000, digits1: 29 },	//  95-bit n < 10^29
  DecDigits{ digits:  0, threshold_hi: 0x00000001431e0fae, threshold_lo: 0x6d7217caa0000000, digits1: 29 },	//  96-bit n < 10^29
  DecDigits{ digits: 30, threshold_hi: 0x0000000c9f2c9cd0, threshold_lo: 0x4674edea40000000, digits1: 30 },	//  97-bit n ? 10^29
  DecDigits{ digits: 30, threshold_hi: 0x0000000c9f2c9cd0, threshold_lo: 0x4674edea40000000, digits1: 30 },	//  98-bit n < 10^30
  DecDigits{ digits:  0, threshold_hi: 0x0000000c9f2c9cd0, threshold_lo: 0x4674edea40000000, digits1: 30 },	//  99-bit n < 10^30
  DecDigits{ digits: 31, threshold_hi: 0x0000007e37be2022, threshold_lo: 0xc0914b2680000000, digits1: 31 },	// 100-bit n ? 10^30
  DecDigits{ digits: 31, threshold_hi: 0x0000007e37be2022, threshold_lo: 0xc0914b2680000000, digits1: 31 },	// 101-bit n < 10^31
  DecDigits{ digits:  0, threshold_hi: 0x0000007e37be2022, threshold_lo: 0xc0914b2680000000, digits1: 31 },	// 102-bit n < 10^31
  DecDigits{ digits: 32, threshold_hi: 0x000004ee2d6d415b, threshold_lo: 0x85acef8100000000, digits1: 32 },	// 103-bit n ? 10^31
  DecDigits{ digits: 32, threshold_hi: 0x000004ee2d6d415b, threshold_lo: 0x85acef8100000000, digits1: 32 },	// 104-bit n < 10^32
  DecDigits{ digits: 32, threshold_hi: 0x000004ee2d6d415b, threshold_lo: 0x85acef8100000000, digits1: 32 },	// 105-bit n < 10^32
  DecDigits{ digits:  0, threshold_hi: 0x000004ee2d6d415b, threshold_lo: 0x85acef8100000000, digits1: 32 },	// 106-bit n < 10^32
  DecDigits{ digits: 33, threshold_hi: 0x0000314dc6448d93, threshold_lo: 0x38c15b0a00000000, digits1: 33 },	// 107-bit n ? 10^32
  DecDigits{ digits: 33, threshold_hi: 0x0000314dc6448d93, threshold_lo: 0x38c15b0a00000000, digits1: 33 },	// 108-bit n < 10^33
  DecDigits{ digits:  0, threshold_hi: 0x0000314dc6448d93, threshold_lo: 0x38c15b0a00000000, digits1: 33 },	// 109-bit n < 10^33
  DecDigits{ digits: 34, threshold_hi: 0x0001ed09bead87c0, threshold_lo: 0x378d8e6400000000, digits1: 34 },	// 100-bit n ? 10^33
  DecDigits{ digits: 34, threshold_hi: 0x0001ed09bead87c0, threshold_lo: 0x378d8e6400000000, digits1: 34 },	// 111-bit n < 10^34
  DecDigits{ digits:  0, threshold_hi: 0x0001ed09bead87c0, threshold_lo: 0x378d8e6400000000, digits1: 34 }  // 112-bit n < 10^34
];

macro_rules! bid_ten2k64 {
  ($index:expr) => {
    BID_TEN2K64[$index as usize]
  };
}
pub(crate) use bid_ten2k64;

/// Table BID_TEN2K64
///
/// BID_TEN2K64\[i\] = 10^i (where 0 <= i <= 19)
#[rustfmt::skip]
pub const BID_TEN2K64: [u64; 20] = [
  0x0000000000000001, // 10^0
  0x000000000000000a, // 10^1
  0x0000000000000064, // 10^2
  0x00000000000003e8, // 10^3
  0x0000000000002710, // 10^4
  0x00000000000186a0, // 10^5
  0x00000000000f4240, // 10^6
  0x0000000000989680, // 10^7
  0x0000000005f5e100, // 10^8
  0x000000003b9aca00, // 10^9
  0x00000002540be400, // 10^10
  0x000000174876e800, // 10^11
  0x000000e8d4a51000, // 10^12
  0x000009184e72a000, // 10^13
  0x00005af3107a4000, // 10^14
  0x00038d7ea4c68000, // 10^15
  0x002386f26fc10000, // 10^16
  0x016345785d8a0000, // 10^17
  0x0de0b6b3a7640000, // 10^18
  0x8ac7230489e80000, // 10^19 (20 digits)
];

macro_rules! bid_ten2k128 {
  ($index:expr) => {
    BID_TEN2K128[$index as usize]
  };
}
pub(crate) use bid_ten2k128;

/// Table BID_TEN2K128
///
/// BID_TEN2K128\[i - 20\] = 10^i (where 20 <= i <= 38)
///
/// The 64-bit word order is Lo, Hi.
#[rustfmt::skip]
pub const BID_TEN2K128: [BidUint128; 19] = [
  BidUint128 { w: [0x6bc75e2d63100000, 0x0000000000000005] }, // 10^20
  BidUint128 { w: [0x35c9adc5dea00000, 0x0000000000000036] }, // 10^21
  BidUint128 { w: [0x19e0c9bab2400000, 0x000000000000021e] }, // 10^22
  BidUint128 { w: [0x02c7e14af6800000, 0x000000000000152d] }, // 10^23
  BidUint128 { w: [0x1bcecceda1000000, 0x000000000000d3c2] }, // 10^24
  BidUint128 { w: [0x161401484a000000, 0x0000000000084595] }, // 10^25
  BidUint128 { w: [0xdcc80cd2e4000000, 0x000000000052b7d2] }, // 10^26
  BidUint128 { w: [0x9fd0803ce8000000, 0x00000000033b2e3c] }, // 10^27
  BidUint128 { w: [0x3e25026110000000, 0x00000000204fce5e] }, // 10^28
  BidUint128 { w: [0x6d7217caa0000000, 0x00000001431e0fae] }, // 10^29
  BidUint128 { w: [0x4674edea40000000, 0x0000000c9f2c9cd0] }, // 10^30
  BidUint128 { w: [0xc0914b2680000000, 0x0000007e37be2022] }, // 10^31
  BidUint128 { w: [0x85acef8100000000, 0x000004ee2d6d415b] }, // 10^32
  BidUint128 { w: [0x38c15b0a00000000, 0x0000314dc6448d93] }, // 10^33
  BidUint128 { w: [0x378d8e6400000000, 0x0001ed09bead87c0] }, // 10^34
  BidUint128 { w: [0x2b878fe800000000, 0x0013426172c74d82] }, // 10^35
  BidUint128 { w: [0xb34b9f1000000000, 0x00c097ce7bc90715] }, // 10^36
  BidUint128 { w: [0x00f436a000000000, 0x0785ee10d5da46d9] }, // 10^37
  BidUint128 { w: [0x098a224000000000, 0x4b3b4ca85a86c47a] }, // 10^38 (39 digits)
];

macro_rules! bid_midpoint64 {
  ($index:expr) => {
    BID_MIDPOINT64[$index as usize]
  };
}
pub(crate) use bid_midpoint64;

/// Table BID_MIDPOINT64
///
/// BID_MIDPOINT64\[i - 1\] = 1/2 * 10^i = 5 * 10^(i-1) (where 1 <= i <= 19)
#[rustfmt::skip]
pub const BID_MIDPOINT64: [u64; 19] = [
  0x0000000000000005, // 1/2 * 10^1 = 5 * 10^0
  0x0000000000000032, // 1/2 * 10^2 = 5 * 10^1
  0x00000000000001f4, // 1/2 * 10^3 = 5 * 10^2
  0x0000000000001388, // 1/2 * 10^4 = 5 * 10^3
  0x000000000000c350, // 1/2 * 10^5 = 5 * 10^4
  0x000000000007a120, // 1/2 * 10^6 = 5 * 10^5
  0x00000000004c4b40, // 1/2 * 10^7 = 5 * 10^6
  0x0000000002faf080, // 1/2 * 10^8 = 5 * 10^7
  0x000000001dcd6500, // 1/2 * 10^9 = 5 * 10^8
  0x000000012a05f200, // 1/2 * 10^10 = 5 * 10^9
  0x0000000ba43b7400, // 1/2 * 10^11 = 5 * 10^10
  0x000000746a528800, // 1/2 * 10^12 = 5 * 10^11
  0x0000048c27395000, // 1/2 * 10^13 = 5 * 10^12
  0x00002d79883d2000, // 1/2 * 10^14 = 5 * 10^13
  0x0001c6bf52634000, // 1/2 * 10^15 = 5 * 10^14
  0x0011c37937e08000, // 1/2 * 10^16 = 5 * 10^15
  0x00b1a2bc2ec50000, // 1/2 * 10^17 = 5 * 10^16
  0x06f05b59d3b20000, // 1/2 * 10^18 = 5 * 10^17
  0x4563918244f40000, // 1/2 * 10^19 = 5 * 10^18
];

macro_rules! bid_midpoint128 {
  ($index:expr) => {
    BID_MIDPOINT128[$index as usize]
  };
}
pub(crate) use bid_midpoint128;

/// Table BID_MIDPOINT128
///
/// BID_MIDPOINT128\[i - 20\] = 1/2 * 10^i = 5 * 10^(i-1) (where 20 <= i <= 38)
///
/// The 64-bit word order is L, H.
#[rustfmt::skip]
pub const BID_MIDPOINT128: [BidUint128; 19] = [
  BidUint128 { w: [0xb5e3af16b1880000, 0x0000000000000002] }, // 1/2 * 10^20 = 5 * 10^19
  BidUint128 { w: [0x1ae4d6e2ef500000, 0x000000000000001b] }, // 1/2 * 10^21 = 5 * 10^20
  BidUint128 { w: [0x0cf064dd59200000, 0x000000000000010f] }, // 1/2 * 10^22 = 5 * 10^21
  BidUint128 { w: [0x8163f0a57b400000, 0x0000000000000a96] }, // 1/2 * 10^23 = 5 * 10^22
  BidUint128 { w: [0x0de76676d0800000, 0x00000000000069e1] }, // 1/2 * 10^24 = 5 * 10^23
  BidUint128 { w: [0x8b0a00a425000000, 0x00000000000422ca] }, // 1/2 * 10^25 = 5 * 10^24
  BidUint128 { w: [0x6e64066972000000, 0x0000000000295be9] }, // 1/2 * 10^26 = 5 * 10^25
  BidUint128 { w: [0x4fe8401e74000000, 0x00000000019d971e] }, // 1/2 * 10^27 = 5 * 10^26
  BidUint128 { w: [0x1f12813088000000, 0x000000001027e72f] }, // 1/2 * 10^28 = 5 * 10^27
  BidUint128 { w: [0x36b90be550000000, 0x00000000a18f07d7] }, // 1/2 * 10^29 = 5 * 10^28
  BidUint128 { w: [0x233a76f520000000, 0x000000064f964e68] }, // 1/2 * 10^30 = 5 * 10^29
  BidUint128 { w: [0x6048a59340000000, 0x0000003f1bdf1011] }, // 1/2 * 10^31 = 5 * 10^30
  BidUint128 { w: [0xc2d677c080000000, 0x0000027716b6a0ad] }, // 1/2 * 10^32 = 5 * 10^31
  BidUint128 { w: [0x9c60ad8500000000, 0x000018a6e32246c9] }, // 1/2 * 10^33 = 5 * 10^32
  BidUint128 { w: [0x1bc6c73200000000, 0x0000f684df56c3e0] }, // 1/2 * 10^34 = 5 * 10^33
  BidUint128 { w: [0x15c3c7f400000000, 0x0009a130b963a6c1] }, // 1/2 * 10^35 = 5 * 10^34
  BidUint128 { w: [0xd9a5cf8800000000, 0x00604be73de4838a] }, // 1/2 * 10^36 = 5 * 10^35
  BidUint128 { w: [0x807a1b5000000000, 0x03c2f7086aed236c] }, // 1/2 * 10^37 = 5 * 10^36
  BidUint128 { w: [0x04c5112000000000, 0x259da6542d43623d] }, // 1/2 * 10^38 = 5 * 10^37
];

macro_rules! bid_ten2k256 {
  ($index:expr) => {
    BID_TEN2K256[$index as usize]
  };
}
pub(crate) use bid_ten2k256;

/// Table BID_TEN2K256
///
/// BID_TEN2K256\[i - 39\] = 10^i, 39 <= i <= 68
#[rustfmt::skip]
pub const BID_TEN2K256: [BidUint256;39] = [
  // the 64-bit word order is LL, LH, HL, HH
  BidUint256{ w: [0x5f65568000000000, 0xf050fe938943acc4, 0x0000000000000002, 0x0000000000000000]},	// 10^39
  BidUint256{ w: [0xb9f5610000000000, 0x6329f1c35ca4bfab, 0x000000000000001d, 0x0000000000000000]},	// 10^40
  BidUint256{ w: [0x4395ca0000000000, 0xdfa371a19e6f7cb5, 0x0000000000000125, 0x0000000000000000]},	// 10^41
  BidUint256{ w: [0xa3d9e40000000000, 0xbc627050305adf14, 0x0000000000000b7a, 0x0000000000000000]},	// 10^42
  BidUint256{ w: [0x6682e80000000000, 0x5bd86321e38cb6ce, 0x00000000000072cb, 0x0000000000000000]},	// 10^43
  BidUint256{ w: [0x011d100000000000, 0x9673df52e37f2410, 0x0000000000047bf1, 0x0000000000000000]},	// 10^44
  BidUint256{ w: [0x0b22a00000000000, 0xe086b93ce2f768a0, 0x00000000002cd76f, 0x0000000000000000]},	// 10^45
  BidUint256{ w: [0x6f5a400000000000, 0xc5433c60ddaa1640, 0x0000000001c06a5e, 0x0000000000000000]},	// 10^46
  BidUint256{ w: [0x5986800000000000, 0xb4a05bc8a8a4de84, 0x00000000118427b3, 0x0000000000000000]},	// 10^47
  BidUint256{ w: [0x7f41000000000000, 0x0e4395d69670b12b, 0x00000000af298d05, 0x0000000000000000]},	// 10^48
  BidUint256{ w: [0xf88a000000000000, 0x8ea3da61e066ebb2, 0x00000006d79f8232, 0x0000000000000000]},	// 10^49
  BidUint256{ w: [0xb564000000000000, 0x926687d2c40534fd, 0x000000446c3b15f9, 0x0000000000000000]},	// 10^50
  BidUint256{ w: [0x15e8000000000000, 0xb8014e3ba83411e9, 0x000002ac3a4edbbf, 0x0000000000000000]},	// 10^51
  BidUint256{ w: [0xdb10000000000000, 0x300d0e549208b31a, 0x00001aba4714957d, 0x0000000000000000]},	// 10^52
  BidUint256{ w: [0x8ea0000000000000, 0xe0828f4db456ff0c, 0x00010b46c6cdd6e3, 0x0000000000000000]},	// 10^53
  BidUint256{ w: [0x9240000000000000, 0xc51999090b65f67d, 0x000a70c3c40a64e6, 0x0000000000000000]},	// 10^54
  BidUint256{ w: [0xb680000000000000, 0xb2fffa5a71fba0e7, 0x006867a5a867f103, 0x0000000000000000]},	// 10^55
  BidUint256{ w: [0x2100000000000000, 0xfdffc78873d4490d, 0x04140c78940f6a24, 0x0000000000000000]},	// 10^56
  BidUint256{ w: [0x4a00000000000000, 0xebfdcb54864ada83, 0x28c87cb5c89a2571, 0x0000000000000000]},	// 10^57](58 digits)
  BidUint256{ w: [0xe400000000000000, 0x37e9f14d3eec8920, 0x97d4df19d6057673, 0x0000000000000001]},	// 10^58
  BidUint256{ w: [0xe800000000000000, 0x2f236d04753d5b48, 0xee50b7025c36a080, 0x000000000000000f]},	// 10^59
  BidUint256{ w: [0x1000000000000000, 0xd762422c946590d9, 0x4f2726179a224501, 0x000000000000009f]},	// 10^60
  BidUint256{ w: [0xa000000000000000, 0x69d695bdcbf7a87a, 0x17877cec0556b212, 0x0000000000000639]},	// 10^61
  BidUint256{ w: [0x4000000000000000, 0x2261d969f7ac94ca, 0xeb4ae1383562f4b8, 0x0000000000003e3a]},	// 10^62
  BidUint256{ w: [0x8000000000000000, 0x57d27e23acbdcfe6, 0x30eccc3215dd8f31, 0x0000000000026e4d]},	// 10^63
  BidUint256{ w: [0x0000000000000000, 0x6e38ed64bf6a1f01, 0xe93ff9f4daa797ed, 0x0000000000184f03]},	// 10^64
  BidUint256{ w: [0x0000000000000000, 0x4e3945ef7a25360a, 0x1c7fc3908a8bef46, 0x0000000000f31627]},	// 10^65
  BidUint256{ w: [0x0000000000000000, 0x0e3cbb5ac5741c64, 0x1cfda3a5697758bf, 0x00000000097edd87]},	// 10^66
  BidUint256{ w: [0x0000000000000000, 0x8e5f518bb6891be8, 0x21e864761ea97776, 0x000000005ef4a747]},	// 10^67
  BidUint256{ w: [0x0000000000000000, 0x8fb92f75215b1710, 0x5313ec9d329eaaa1, 0x00000003b58e88c7]},	// 10^68
  BidUint256{ w: [0x0000000000000000, 0x9d3bda934d8ee6a0, 0x3ec73e23fa32aa4f, 0x00000025179157c9]},	// 10^69
  BidUint256{ w: [0x0000000000000000, 0x245689c107950240, 0x73c86d67c5faa71c, 0x00000172ebad6ddc]},	// 10^70
  BidUint256{ w: [0x0000000000000000, 0x6b61618a4bd21680, 0x85d4460dbbca8719, 0x00000e7d34c64a9c]},	// 10^71
  BidUint256{ w: [0x0000000000000000, 0x31cdcf66f634e100, 0x3a4abc8955e946fe, 0x000090e40fbeea1d]},	// 10^72
  BidUint256{ w: [0x0000000000000000, 0xf20a1a059e10ca00, 0x46eb5d5d5b1cc5ed, 0x0005a8e89d752524]},	// 10^73
  BidUint256{ w: [0x0000000000000000, 0x746504382ca7e400, 0xc531a5a58f1fbb4b, 0x003899162693736a]},	// 10^74
  BidUint256{ w: [0x0000000000000000, 0x8bf22a31be8ee800, 0xb3f07877973d50f2, 0x0235fadd81c2822b]},	// 10^75
  BidUint256{ w: [0x0000000000000000, 0x7775a5f171951000, 0x0764b4abe8652979, 0x161bcca7119915b5]},	// 10^76
  BidUint256{ w: [0x0000000000000000, 0xaa987b6e6fd2a000, 0x49ef0eb713f39ebe, 0xdd15fe86affad912]}	// 10^77
];

macro_rules! bid_ten2mk128 {
  ($index:expr) => {
    BID_TEN2MK128[$index as usize]
  };
}
pub(crate) use bid_ten2mk128;

/// Table BID_TEN2MK128
///
/// BID_TEN2MK128\[k - 1\] = 10^(-k) * 2^exp (k) (where 1 <= k <= 34)
/// and exp(k) = bid_shiftright128\[k - 1\] + 128
#[rustfmt::skip]
pub const BID_TEN2MK128: [BidUint128; 34] = [
  BidUint128{ w: [0x999999999999999a, 0x1999999999999999] }, //  10^(-1) * 2^128
  BidUint128{ w: [0x28f5c28f5c28f5c3, 0x028f5c28f5c28f5c] }, //  10^(-2) * 2^128
  BidUint128{ w: [0x9db22d0e56041894, 0x004189374bc6a7ef] }, //  10^(-3) * 2^128
  BidUint128{ w: [0x4af4f0d844d013aa, 0x00346dc5d6388659] }, //  10^(-4) * 2^131
  BidUint128{ w: [0x08c3f3e0370cdc88, 0x0029f16b11c6d1e1] }, //  10^(-5) * 2^134
  BidUint128{ w: [0x6d698fe69270b06d, 0x00218def416bdb1a] }, //  10^(-6) * 2^137
  BidUint128{ w: [0xaf0f4ca41d811a47, 0x0035afe535795e90] }, //  10^(-7) * 2^141
  BidUint128{ w: [0xbf3f70834acdaea0, 0x002af31dc4611873] }, //  10^(-8) * 2^144
  BidUint128{ w: [0x65cc5a02a23e254d, 0x00225c17d04dad29] }, //  10^(-9) * 2^147
  BidUint128{ w: [0x6fad5cd10396a214, 0x0036f9bfb3af7b75] }, // 10^(-10) * 2^151
  BidUint128{ w: [0xbfbde3da69454e76, 0x002bfaffc2f2c92a] }, // 10^(-11) * 2^154
  BidUint128{ w: [0x32fe4fe1edd10b92, 0x00232f33025bd422] }, // 10^(-12) * 2^157
  BidUint128{ w: [0x84ca19697c81ac1c, 0x00384b84d092ed03] }, // 10^(-13) * 2^161
  BidUint128{ w: [0x03d4e1213067bce4, 0x002d09370d425736] }, // 10^(-14) * 2^164
  BidUint128{ w: [0x3643e74dc052fd83, 0x0024075f3dceac2b] }, // 10^(-15) * 2^167
  BidUint128{ w: [0x56d30baf9a1e626b, 0x0039a5652fb11378] }, // 10^(-16) * 2^171
  BidUint128{ w: [0x12426fbfae7eb522, 0x002e1dea8c8da92d] }, // 10^(-17) * 2^174
  BidUint128{ w: [0x41cebfcc8b9890e8, 0x0024e4bba3a48757] }, // 10^(-18) * 2^177
  BidUint128{ w: [0x694acc7a78f41b0d, 0x003b07929f6da558] }, // 10^(-19) * 2^181
  BidUint128{ w: [0xbaa23d2ec729af3e, 0x002f394219248446] }, // 10^(-20) * 2^184
  BidUint128{ w: [0xfbb4fdbf05baf298, 0x0025c768141d369e] }, // 10^(-21) * 2^187
  BidUint128{ w: [0x2c54c931a2c4b759, 0x003c7240202ebdcb] }, // 10^(-22) * 2^191
  BidUint128{ w: [0x89dd6dc14f03c5e1, 0x00305b66802564a2] }, // 10^(-23) * 2^194
  BidUint128{ w: [0xd4b1249aa59c9e4e, 0x0026af8533511d4e] }, // 10^(-24) * 2^197
  BidUint128{ w: [0x544ea0f76f60fd49, 0x003de5a1ebb4fbb1] }, // 10^(-25) * 2^201
  BidUint128{ w: [0x76a54d92bf80caa1, 0x00318481895d9627] }, // 10^(-26) * 2^204
  BidUint128{ w: [0x921dd7a89933d54e, 0x00279d346de4781f] }, // 10^(-27) * 2^207
  BidUint128{ w: [0x8362f2a75b862215, 0x003f61ed7ca0c032] }, // 10^(-28) * 2^211
  BidUint128{ w: [0xcf825bb91604e811, 0x0032b4bdfd4d668e] }, // 10^(-29) * 2^214
  BidUint128{ w: [0x0c684960de6a5341, 0x00289097fdd7853f] }, // 10^(-30) * 2^217
  BidUint128{ w: [0x3d203ab3e521dc34, 0x002073accb12d0ff] }, // 10^(-31) * 2^220
  BidUint128{ w: [0x2e99f7863b696053, 0x0033ec47ab514e65] }, // 10^(-32) * 2^224
  BidUint128{ w: [0x587b2c6b62bab376, 0x002989d2ef743eb7] }, // 10^(-33) * 2^227
  BidUint128{ w: [0xad2f56bc4efbc2c5, 0x00213b0f25f69892] }, // 10^(-34) * 2^230
];

macro_rules! bid_maskhigh128 {
  ($index:expr) => {
    BID_MASKHIGH128[$index as usize]
  };
}

pub(crate) use bid_maskhigh128;

/// Table BID_MASKHIGH128
///
/// BID_MASKHIGH128 contains the mask to apply to the top 128 bits of the 
/// 128x128-bit product in order to obtain the high bits of f2*.
#[rustfmt::skip]
pub const BID_MASKHIGH128: [BidUint64; 34] = [
  0x0000000000000000, //  0 = 128 - 128 bits
  0x0000000000000000, //  0 = 128 - 128 bits
  0x0000000000000000, //  0 = 128 - 128 bits
  0x0000000000000007, //  3 = 131 - 128 bits
  0x000000000000003f, //  6 = 134 - 128 bits
  0x00000000000001ff, //  9 = 137 - 128 bits
  0x0000000000001fff, // 13 = 141 - 128 bits
  0x000000000000ffff, // 16 = 144 - 128 bits
  0x000000000007ffff, // 19 = 147 - 128 bits
  0x00000000007fffff, // 23 = 151 - 128 bits
  0x0000000003ffffff, // 26 = 154 - 128 bits
  0x000000001fffffff, // 29 = 157 - 128 bits
  0x00000001ffffffff, // 33 = 161 - 128 bits
  0x0000000fffffffff, // 36 = 164 - 128 bits
  0x0000007fffffffff, // 39 = 167 - 128 bits
  0x000007ffffffffff, // 43 = 171 - 128 bits
  0x00003fffffffffff, // 46 = 174 - 128 bits
  0x0001ffffffffffff, // 49 = 177 - 128 bits
  0x001fffffffffffff, // 53 = 181 - 128 bits
  0x00ffffffffffffff, // 56 = 184 - 128 bits
  0x07ffffffffffffff, // 59 = 187 - 128 bits
  0x7fffffffffffffff, // 63 = 191 - 128 bits
  0x0000000000000003, //  2 = 194 - 192 bits
  0x000000000000001f, //  5 = 197 - 192 bits
  0x00000000000001ff, //  9 = 201 - 192 bits
  0x0000000000000fff, // 12 = 204 - 192 bits
  0x0000000000007fff, // 15 = 207 - 192 bits
  0x000000000007ffff, // 21 = 211 - 192 bits
  0x00000000003fffff, // 22 = 214 - 192 bits
  0x0000000001ffffff, // 25 = 217 - 192 bits
  0x000000000fffffff, // 28 = 220 - 192 bits
  0x00000000ffffffff, // 32 = 224 - 192 bits
  0x00000007ffffffff, // 35 = 227 - 192 bits
  0x0000003fffffffff  // 38 = 230 - 192 bits
];

macro_rules! bid_shiftright128 {
  ($index:expr) => {
    BID_SHIFTRIGHT128[$index as usize]
  };
}

pub(crate) use bid_shiftright128;

/// Table BID_SHIFTRIGHT128
///
/// BID_SHIFTRIGHT128 contains the right shift count to obtain C2*
/// from the top 128 bits of the 128x128-bit product C2 * Kx.
#[rustfmt::skip]
pub const BID_SHIFTRIGHT128: [i32; 34] = [
  0,   // 128 - 128
  0,   // 128 - 128
  0,   // 128 - 128
  3,   // 131 - 128
  6,   // 134 - 128
  9,   // 137 - 128
  13,  // 141 - 128
  16,  // 144 - 128
  19,  // 147 - 128
  23,  // 151 - 128
  26,  // 154 - 128
  29,  // 157 - 128
  33,  // 161 - 128
  36,  // 164 - 128
  39,  // 167 - 128
  43,  // 171 - 128
  46,  // 174 - 128
  49,  // 177 - 128
  53,  // 181 - 128
  56,  // 184 - 128
  59,  // 187 - 128
  63,  // 191 - 128
  66,  // 194 - 128
  69,  // 197 - 128
  73,  // 201 - 128
  76,  // 204 - 128
  79,  // 207 - 128
  83,  // 211 - 128
  86,  // 214 - 128
  89,  // 217 - 128
  92,  // 220 - 128
  96,  // 224 - 128
  99,  // 227 - 128
  102, // 230 - 128
];

macro_rules! bid_ten2mk128trunc {
  ($index:expr) => {
    BID_TEN2MK128TRUNC[$index as usize]
  };
}

pub(crate) use bid_ten2mk128trunc;

/// Table BID_TEN2MK128TRUNC 
///
/// BID_TEN2MK128TRUNC contains T*, the top Ex >= 128 bits of 10^(-k), for 1 <= k <= 34
///
/// The 64-bit word order is Lo, Hi.
#[rustfmt::skip]
pub const BID_TEN2MK128TRUNC:[BidUint128; 34] = [
  BidUint128{ w: [0x9999999999999999, 0x1999999999999999] },	//  10^(-1) * 2^128
  BidUint128{ w: [0x28f5c28f5c28f5c2, 0x028f5c28f5c28f5c] },	//  10^(-2) * 2^128
  BidUint128{ w: [0x9db22d0e56041893, 0x004189374bc6a7ef] },	//  10^(-3) * 2^128
  BidUint128{ w: [0x4af4f0d844d013a9, 0x00346dc5d6388659] },	//  10^(-4) * 2^131
  BidUint128{ w: [0x08c3f3e0370cdc87, 0x0029f16b11c6d1e1] },	//  10^(-5) * 2^134
  BidUint128{ w: [0x6d698fe69270b06c, 0x00218def416bdb1a] },	//  10^(-6) * 2^137
  BidUint128{ w: [0xaf0f4ca41d811a46, 0x0035afe535795e90] },	//  10^(-7) * 2^141
  BidUint128{ w: [0xbf3f70834acdae9f, 0x002af31dc4611873] },	//  10^(-8) * 2^144
  BidUint128{ w: [0x65cc5a02a23e254c, 0x00225c17d04dad29] },	//  10^(-9) * 2^147
  BidUint128{ w: [0x6fad5cd10396a213, 0x0036f9bfb3af7b75] },	// 10^(-10) * 2^151
  BidUint128{ w: [0xbfbde3da69454e75, 0x002bfaffc2f2c92a] },	// 10^(-11) * 2^154
  BidUint128{ w: [0x32fe4fe1edd10b91, 0x00232f33025bd422] },	// 10^(-12) * 2^157
  BidUint128{ w: [0x84ca19697c81ac1b, 0x00384b84d092ed03] },	// 10^(-13) * 2^161
  BidUint128{ w: [0x03d4e1213067bce3, 0x002d09370d425736] },	// 10^(-14) * 2^164
  BidUint128{ w: [0x3643e74dc052fd82, 0x0024075f3dceac2b] },	// 10^(-15) * 2^167
  BidUint128{ w: [0x56d30baf9a1e626a, 0x0039a5652fb11378] },	// 10^(-16) * 2^171
  BidUint128{ w: [0x12426fbfae7eb521, 0x002e1dea8c8da92d] },	// 10^(-17) * 2^174
  BidUint128{ w: [0x41cebfcc8b9890e7, 0x0024e4bba3a48757] },	// 10^(-18) * 2^177
  BidUint128{ w: [0x694acc7a78f41b0c, 0x003b07929f6da558] },	// 10^(-19) * 2^181
  BidUint128{ w: [0xbaa23d2ec729af3d, 0x002f394219248446] },	// 10^(-20) * 2^184
  BidUint128{ w: [0xfbb4fdbf05baf297, 0x0025c768141d369e] },	// 10^(-21) * 2^187
  BidUint128{ w: [0x2c54c931a2c4b758, 0x003c7240202ebdcb] },	// 10^(-22) * 2^191
  BidUint128{ w: [0x89dd6dc14f03c5e0, 0x00305b66802564a2] },	// 10^(-23) * 2^194
  BidUint128{ w: [0xd4b1249aa59c9e4d, 0x0026af8533511d4e] },	// 10^(-24) * 2^197
  BidUint128{ w: [0x544ea0f76f60fd48, 0x003de5a1ebb4fbb1] },	// 10^(-25) * 2^201
  BidUint128{ w: [0x76a54d92bf80caa0, 0x00318481895d9627] },	// 10^(-26) * 2^204
  BidUint128{ w: [0x921dd7a89933d54d, 0x00279d346de4781f] },	// 10^(-27) * 2^207
  BidUint128{ w: [0x8362f2a75b862214, 0x003f61ed7ca0c032] },	// 10^(-28) * 2^211
  BidUint128{ w: [0xcf825bb91604e810, 0x0032b4bdfd4d668e] },	// 10^(-29) * 2^214
  BidUint128{ w: [0x0c684960de6a5340, 0x00289097fdd7853f] },	// 10^(-30) * 2^217
  BidUint128{ w: [0x3d203ab3e521dc33, 0x002073accb12d0ff] },	// 10^(-31) * 2^220
  BidUint128{ w: [0x2e99f7863b696052, 0x0033ec47ab514e65] },	// 10^(-32) * 2^224
  BidUint128{ w: [0x587b2c6b62bab375, 0x002989d2ef743eb7] },	// 10^(-33) * 2^227
  BidUint128{ w: [0xad2f56bc4efbc2c4, 0x00213b0f25f69892] },	// 10^(-34) * 2^230
];

macro_rules! bid_onehalf128 {
  ($index:expr) => {
    BID_ONEHALF128[$index as usize]
  };
}

pub(crate) use bid_onehalf128;

/// Table BID_ONEHALF128
///
/// BID_ONEHALF128 contains the high bits of 1/2 positioned correctly
/// for comparison with the high bits of f2*.
///
/// The 64-bit word order is Lo, Hi.
#[rustfmt::skip]
pub const BID_ONEHALF128: [BidUint64; 34] = [
  0x0000000000000000, //  0 bits
  0x0000000000000000, //  0 bits
  0x0000000000000000, //  0 bits
  0x0000000000000004, //  3 bits
  0x0000000000000020, //  6 bits
  0x0000000000000100, //  9 bits
  0x0000000000001000, // 13 bits
  0x0000000000008000, // 16 bits
  0x0000000000040000, // 19 bits
  0x0000000000400000, // 23 bits
  0x0000000002000000, // 26 bits
  0x0000000010000000, // 29 bits
  0x0000000100000000, // 33 bits
  0x0000000800000000, // 36 bits
  0x0000004000000000, // 39 bits
  0x0000040000000000, // 43 bits
  0x0000200000000000, // 46 bits
  0x0001000000000000, // 49 bits
  0x0010000000000000, // 53 bits
  0x0080000000000000, // 56 bits
  0x0400000000000000, // 59 bits
  0x4000000000000000, // 63 bits
  0x0000000000000002, // 66 bits
  0x0000000000000010, // 69 bits
  0x0000000000000100, // 73 bits
  0x0000000000000800, // 76 bits
  0x0000000000004000, // 79 bits
  0x0000000000040000, // 83 bits
  0x0000000000200000, // 86 bits
  0x0000000001000000, // 89 bits
  0x0000000008000000, // 92 bits
  0x0000000080000000, // 96 bits
  0x0000000400000000, // 99 bits
  0x0000002000000000, // 102 bits
];

/// Adds 64-bit value to 128-bit value.
macro_rules! add_128_64 {
  ($r:expr,$x:expr,$y:expr) => {
    let mut rh: BidUint64 = $x.w[1];
    $r.w[0] = $y.wrapping_add($x.w[0]);
    if $r.w[0] < $y {
      rh = rh.wrapping_add(1);
    }
    $r.w[1] = rh;
  };
}

pub(crate) use add_128_64;

/// Adds 128-bit value to 128-bit value, assuming no carry-out.
macro_rules! add_128_128 {
  ($r128:expr, $a128:expr, $b128:expr) => {
    let mut q128: BidUint128 = BidUint128::default();
    q128.w[1] = $a128.w[1].wrapping_add($b128.w[1]);
    q128.w[0] = $b128.w[0].wrapping_add($a128.w[0]);
    if q128.w[0] < $b128.w[0] {
      q128.w[1] = q128.w[1].wrapping_add(1);
    }
    $r128.w[1] = q128.w[1];
    $r128.w[0] = q128.w[0];
  };
}
pub(crate) use add_128_128;

macro_rules! sub_128_128 {
  ($r128:expr, $a128:expr, $b128:expr) => {
    let mut q128: BidUint128 = Default::default();
    q128.w[1] = $a128.w[1].wrapping_sub($b128.w[1]);
    q128.w[0] = $a128.w[0].wrapping_sub($b128.w[0]);
    if $a128.w[0] < $b128.w[0] {
      q128.w[1] = q128.w[1].wrapping_sub(1);
    }
    $r128.w[1] = q128.w[1];
    $r128.w[0] = q128.w[0];
  };
}
pub(crate) use sub_128_128;

macro_rules! add_carry_out {
  ($r:expr, $co:expr, $x:expr, $y: expr) => {
    $r = $x.wrapping_add($y);
    $co = if $r < $x { 1 } else { 0 };
  };
}
pub(crate) use add_carry_out;

macro_rules! add_carry_in_out {
  ($r:expr, $co:expr, $x:expr, $y:expr, $ci:expr) => {
    let x1: BidUint64 = $x.wrapping_add($ci);
    $r = x1.wrapping_add($y);
    $co = if ($r < x1) || (x1 < $ci) { 1 } else { 0 }
  };
}
pub(crate) use add_carry_in_out;

macro_rules! mul_64x64_to_64 {
  ($p64:expr, $cx:expr, $cy:expr) => {
    $p64 = $cx.wrapping_mul($cy);
  };
}
pub(crate) use mul_64x64_to_64;

macro_rules! sub_borrow_out {
  ($s:expr, $cy:expr, $x:expr, $y:expr) => {
    let x1: BidUint64 = $x;
    $s = $x.wrapping_sub($y);
    $cy = if $s > x1 { 1 } else { 0 };
  };
}
pub(crate) use sub_borrow_out;

macro_rules! sub_borrow_in_out {
  ($s:expr, $cy:expr, $x:expr, $y:expr, $ci:expr) => {
    let x0: BidUint64 = $x;
    let x1: BidUint64 = $x.wrapping_sub($ci);
    $s = x1.wrapping_sub($y);
    $cy = if ($s > x1) || (x1 > x0) { 1 } else { 0 };
  };
}
pub(crate) use sub_borrow_in_out;

/// Returns 64 x 64 bit product.
macro_rules! mul_64x64_to_128mach {
  ($p:expr, $cx:expr, $cy:expr) => {
    let cxh: BidUint64 = (($cx >> 32) as u32) as u64;
    let cxl: BidUint64 = ($cx as u32) as u64;
    let cyh: BidUint64 = (($cy >> 32) as u32) as u64;
    let cyl: BidUint64 = ($cy as u32) as u64;
    let mut pm: BidUint64 = cxh.wrapping_mul(cyl);
    let mut ph: BidUint64 = cxh.wrapping_mul(cyh);
    let pl: BidUint64 = cxl.wrapping_mul(cyl);
    let pm2: BidUint64 = cxl.wrapping_mul(cyh);
    ph = ph.wrapping_add(((pm >> 32) as u32) as u64);
    pm = ((pm as u32) as u64).wrapping_add(pm2).wrapping_add(((pl >> 32) as u32) as u64);
    $p.w[1] = ph.wrapping_add(((pm >> 32) as u32) as u64);
    $p.w[0] = (pm << 32).wrapping_add((pl as u32) as u64);
  };
}
pub(crate) use mul_64x64_to_128mach;

/// Returns full 64 x 64 bit product.
macro_rules! mul_64x64_to_128 {
  ($p:expr,$cx:expr,$cy:expr) => {
    let cxh: BidUint64 = (($cx >> 32) as u32) as u64;
    let cxl: BidUint64 = ($cx as u32) as u64;
    let cyh: BidUint64 = (($cy >> 32) as u32) as u64;
    let cyl: BidUint64 = ($cy as u32) as u64;
    let mut pm = cxh.wrapping_mul(cyl);
    let mut ph = cxh.wrapping_mul(cyh);
    let pl = cxl.wrapping_mul(cyl);
    let pm2 = cxl.wrapping_mul(cyh);
    ph = ph.wrapping_add(((pm >> 32) as u32) as u64);
    pm = ((pm as u32) as u64).wrapping_add(pm2).wrapping_add(((pl >> 32) as u32) as u64);
    $p.w[1] = ph.wrapping_add(((pm >> 32) as u32) as u64);
    $p.w[0] = (pm << 32).wrapping_add((pl as u32) as u64);
  };
}
pub(crate) use mul_64x64_to_128;

macro_rules! mul_64x128_to_128 {
  ($ql:expr, $a:expr, $b:expr) => {
    let mut albl: BidUint128 = Default::default();
    let mut albh: BidUint128 = Default::default();
    let mut qm2: BidUint128 = Default::default();
    mul_64x64_to_128!(albh, $a, $b.w[1]);
    mul_64x64_to_128!(albl, $a, $b.w[0]);
    $ql.w[0] = albl.w[0];
    add_128_64!(qm2, albh, albl.w[1]);
    $ql.w[1] = qm2.w[0];
  };
}
pub(crate) use mul_64x128_to_128;

macro_rules! mul_64x128_short {
  ($ql:expr, $a:expr, $b:expr) => {
    let albh_l: BidUint64;
    mul_64x64_to_64!(albh_l, $a, $b.w[1]);
    mul_64x64_to_128!($ql, $a, $b.w[0]);
    $ql.w[1] = $ql.w[1].wrapping_add(albh_l);
  };
}
pub(crate) use mul_64x128_short;

macro_rules! mul_128x64_to_128 {
  ($q128:expr, $a64:expr, $b128:expr) => {
    let albh_l: BidUint64 = $a64.wrapping_mul($b128.w[1]);
    mul_64x64_to_128mach!($q128, $a64, $b128.w[0]);
    $q128.w[1] = $q128.w[1].wrapping_add(albh_l);
  };
}
pub(crate) use mul_128x64_to_128;

macro_rules! mul_64x256_to_320 {
  ($p:expr, $a:expr, $b:expr) => {
    let mut p0: BidUint128 = Default::default();
    let mut p1: BidUint128 = Default::default();
    let mut p2: BidUint128 = Default::default();
    let mut p3: BidUint128 = Default::default();
    let mut cy: BidUint64;
    mul_64x64_to_128!(p0, $a, $b.w[0]);
    mul_64x64_to_128!(p1, $a, $b.w[1]);
    mul_64x64_to_128!(p2, $a, $b.w[2]);
    mul_64x64_to_128!(p3, $a, $b.w[3]);
    $p.w[0] = p0.w[0];
    add_carry_out!($p.w[1], cy, p1.w[0], p0.w[1]);
    add_carry_in_out!($p.w[2], cy, p2.w[0], p1.w[1], cy);
    add_carry_in_out!($p.w[3], cy, p3.w[0], p2.w[1], cy);
    $p.w[4] = p3.w[1].wrapping_add(cy);
  };
}
pub(crate) use mul_64x256_to_320;

macro_rules! mul_64x128_full {
  ($ph:expr, $ql:expr, $x:expr, $y:expr) => {
    let mut albl = BidUint128::default();
    let mut albh = BidUint128::default();
    mul_64x64_to_128!(albh, $x, $y.w[1]);
    mul_64x64_to_128!(albl, $x, $y.w[0]);
    $ql.w[0] = albl.w[0];
    let mut qm2 = BidUint128::default();
    add_128_64!(qm2, albh, albl.w[1]);
    $ql.w[1] = qm2.w[0];
    $ph = qm2.w[1];
  };
}
pub(crate) use mul_64x128_full;

macro_rules! mul_64x128_to_192 {
  ($q:expr, $a:expr, $b:expr) => {
    let mut albl: BidUint128 = Default::default();
    let mut albh: BidUint128 = Default::default();
    let mut qm2: BidUint128 = Default::default();
    mul_64x64_to_128!(albh, $a, $b.w[1]);
    mul_64x64_to_128!(albl, $a, $b.w[0]);
    $q.w[0] = albl.w[0];
    add_128_64!(qm2, albh, albl.w[1]);
    $q.w[1] = qm2.w[0];
    $q.w[2] = qm2.w[1];
  };
}
pub(crate) use mul_64x128_to_192;

macro_rules! mul_64x192_to_256 {
  ($lP:expr, $lA:expr, $lB:expr) => {
    let mut p0: BidUint128 = Default::default();
    let mut p1: BidUint128 = Default::default();
    let mut p2: BidUint128 = Default::default();
    let mut cy: BidUint64;
    mul_64x64_to_128!(p0, $lA, $lB.w[0]);
    mul_64x64_to_128!(p1, $lA, $lB.w[1]);
    mul_64x64_to_128!(p2, $lA, $lB.w[2]);
    $lP.w[0] = p0.w[0];
    add_carry_out!($lP.w[1], cy, p1.w[0], p0.w[1]);
    add_carry_in_out!($lP.w[2], cy, p2.w[0], p1.w[1], cy);
    $lP.w[3] = p2.w[1].wrapping_add(cy);
  };
}
pub(crate) use mul_64x192_to_256;

macro_rules! mul_128x128_to_256 {
  ($r:expr, $x:expr, $y:expr) => {
    let mut qll = BidUint128::default();
    let mut qlh = BidUint128::default();
    let phl: BidUint64;
    let phh: BidUint64;
    let co1: BidUint64;
    let co2: BidUint64;
    mul_64x128_full!(phl, qll, $x.w[0], $y);
    mul_64x128_full!(phh, qlh, $x.w[1], $y);
    $r.w[0] = qll.w[0];
    add_carry_out!($r.w[1], co1, qlh.w[0], qll.w[1]);
    add_carry_in_out!($r.w[2], co2, qlh.w[1], phl, co1);
    $r.w[3] = phh.wrapping_add(co2);
  };
}
pub(crate) use mul_128x128_to_256;

macro_rules! mul_128x128_full {
  ($qh:expr, $ql:expr, $a:expr, $b:expr) => {
    let mut albl: BidUint128 = BidUint128::default();
    let mut albh: BidUint128 = BidUint128::default();
    let mut ahbl: BidUint128 = BidUint128::default();
    let mut ahbh: BidUint128 = BidUint128::default();
    let mut qm: BidUint128 = BidUint128::default();
    let mut qm2: BidUint128 = BidUint128::default();
    mul_64x64_to_128!(albh, $a.w[0], $b.w[1]);
    mul_64x64_to_128!(ahbl, $b.w[0], $a.w[1]);
    mul_64x64_to_128!(albl, $a.w[0], $b.w[0]);
    mul_64x64_to_128!(ahbh, $a.w[1], $b.w[1]);
    add_128_128!(qm, albh, ahbl);
    $ql.w[0] = albl.w[0];
    add_128_64!(qm2, qm, albl.w[1]);
    add_128_64!($qh, ahbh, qm2.w[1]);
    $ql.w[1] = qm2.w[0];
  };
}

macro_rules! mul_128x128_low {
  ($ql:expr, $a:expr, $b:expr) => {
    let mut albl: BidUint128 = Default::default();
    mul_64x64_to_128!(albl, $a.w[0], $b.w[0]);
    let qm64: BidUint64 = $b.w[0].wrapping_mul($a.w[1]).wrapping_add($a.w[0].wrapping_mul($b.w[1]));
    $ql.w[0] = albl.w[0];
    $ql.w[1] = qm64.wrapping_add(albl.w[1]);
  };
}
pub(crate) use mul_128x128_low;

macro_rules! mul_128x128_full {
  ($qh:expr, $ql:expr, $a:expr, $b:expr) => {
    let mut albl: BidUint128 = Default::default();
    let mut albh: BidUint128 = Default::default();
    let mut ahbl: BidUint128 = Default::default();
    let mut ahbh: BidUint128 = Default::default();
    let mut qm: BidUint128 = Default::default();
    let mut qm2: BidUint128 = Default::default();
    mul_64x64_to_128!(albh, ($a).w[0], ($b).w[1]);
    mul_64x64_to_128!(ahbl, ($b).w[0], ($a).w[1]);
    mul_64x64_to_128!(albl, ($a).w[0], ($b).w[0]);
    mul_64x64_to_128!(ahbh, ($a).w[1], ($b).w[1]);
    add_128_128!(qm, albh, ahbl);
    $ql.w[0] = albl.w[0];
    add_128_64!(qm2, qm, albl.w[1]);
    add_128_64!($qh, ahbh, qm2.w[1]);
    $ql.w[1] = qm2.w[0];
  };
}
pub(crate) use mul_128x128_full;

/// Get full 64x64-bit product.
/// Note that this macro is used for `CX < 2^61`, `CY < 2^61`.
macro_rules! mul_64x64_to_128_fast {
  ($p:expr, $cx:expr, $cy:expr) => {
    let cxh: BidUint64 = (($cx >> 32) as u32) as u64;
    let cxl: BidUint64 = ($cx as u32) as u64;
    let cyh: BidUint64 = (($cy >> 32) as u32) as u64;
    let cyl: BidUint64 = ($cy as u32) as u64;

    let mut pm = cxh * cyl;
    let pl: BidUint64 = cxl * cyl;
    let ph: BidUint64 = cxh * cyh;
    pm += cxl * cyh;
    pm += pl >> 32;

    $p.w[1] = ph.wrapping_add(((pm >> 32) as u32) as u64);
    $p.w[0] = (pm << 32).wrapping_add((pl as u32) as u64);
  };
}
pub(crate) use mul_64x64_to_128_fast;

macro_rules! mul_192x192_to_384 {
  ($p:expr, $a:expr, $b:expr) => {
    let mut p0: BidUint256 = Default::default();
    let mut p1: BidUint256 = Default::default();
    let mut p2: BidUint256 = Default::default();
    let mut cy: BidUint64;
    mul_64x192_to_256!(p0, $a.w[0], $b);
    mul_64x192_to_256!(p1, $a.w[1], $b);
    mul_64x192_to_256!(p2, $a.w[2], $b);
    $p.w[0] = p0.w[0];
    add_carry_out!($p.w[1], cy, p1.w[0], p0.w[1]);
    add_carry_in_out!($p.w[2], cy, p1.w[1], p0.w[2], cy);
    add_carry_in_out!($p.w[3], cy, p1.w[2], p0.w[3], cy);
    $p.w[4] = p1.w[3].wrapping_add(cy);
    add_carry_out!($p.w[2], cy, p2.w[0], $p.w[2]);
    add_carry_in_out!($p.w[3], cy, p2.w[1], $p.w[3], cy);
    add_carry_in_out!($p.w[4], cy, p2.w[2], $p.w[4], cy);
    $p.w[5] = p2.w[3].wrapping_add(cy);
  };
}
pub(crate) use mul_192x192_to_384;

macro_rules! mul_256x256_to_512 {
  ($p:expr, $a:expr, $b:expr) => {
    let mut p0: BidUint512 = Default::default();
    let mut p1: BidUint512 = Default::default();
    let mut p2: BidUint512 = Default::default();
    let mut p3: BidUint512 = Default::default();
    let mut cy: BidUint64;
    mul_64x256_to_320!(p0, $a.w[0], $b);
    mul_64x256_to_320!(p1, $a.w[1], $b);
    mul_64x256_to_320!(p2, $a.w[2], $b);
    mul_64x256_to_320!(p3, $a.w[3], $b);
    $p.w[0] = p0.w[0];
    add_carry_out!($p.w[1], cy, p1.w[0], p0.w[1]);
    add_carry_in_out!($p.w[2], cy, p1.w[1], p0.w[2], cy);
    add_carry_in_out!($p.w[3], cy, p1.w[2], p0.w[3], cy);
    add_carry_in_out!($p.w[4], cy, p1.w[3], p0.w[4], cy);
    $p.w[5] = p1.w[4].wrapping_add(cy);
    add_carry_out!($p.w[2], cy, p2.w[0], $p.w[2]);
    add_carry_in_out!($p.w[3], cy, p2.w[1], $p.w[3], cy);
    add_carry_in_out!($p.w[4], cy, p2.w[2], $p.w[4], cy);
    add_carry_in_out!($p.w[5], cy, p2.w[3], $p.w[5], cy);
    $p.w[6] = p2.w[4].wrapping_add(cy);
    add_carry_out!($p.w[3], cy, p3.w[0], $p.w[3]);
    add_carry_in_out!($p.w[4], cy, p3.w[1], $p.w[4], cy);
    add_carry_in_out!($p.w[5], cy, p3.w[2], $p.w[5], cy);
    add_carry_in_out!($p.w[6], cy, p3.w[3], $p.w[6], cy);
    $p.w[7] = p3.w[4].wrapping_add(cy);
  };
}
pub(crate) use mul_256x256_to_512;

macro_rules! shl_128_long {
  ($q:expr, $a:expr, $k:expr) => {
    if $k < 64 {
      $q.w[1] = $a.w[1] << $k;
      $q.w[1] |= $a.w[0] >> (64 - $k);
      $q.w[0] = $a.w[0] << $k;
    } else {
      $q.w[1] = $a.w[0] << (($k) - 64);
      $q.w[0] = 0;
    }
  };
}

macro_rules! shr_128_long {
  ($q:expr, $a:expr, $k:expr) => {
    if $k < 64 {
      $q.w[0] = $a.w[0] >> $k;
      $q.w[0] |= $a.w[1] << (64 - $k);
      $q.w[1] = $a.w[1] >> $k;
    } else {
      $q.w[0] = $a.w[1] >> (($k) - 64);
      $q.w[1] = 0;
    }
  };
}
pub(crate) use shr_128_long;

/// Greater than.
macro_rules! unsigned_compare_gt_128 {
  ($a:expr, $b:expr) => {
    ($a.w[1] > $b.w[1]) || (($a.w[1] == $b.w[1]) && ($a.w[0] > $b.w[0]))
  };
}
pub(crate) use unsigned_compare_gt_128;

/// Greater or equal.
macro_rules! unsigned_compare_ge_128 {
  ($a:expr, $b:expr) => {
    ($a.w[1] > $b.w[1]) || (($a.w[1] == $b.w[1]) && ($a.w[0] >= $b.w[0]))
  };
}
pub(crate) use unsigned_compare_ge_128;

macro_rules! shr_128 {
  ($q:expr, $a:expr, $k:expr) => {
    $q.w[0] = $a.w[0] >> $k;
    $q.w[0] |= $a.w[1] << (64 - $k);
    $q.w[1] = $a.w[1] >> $k;
  };
}
pub(crate) use shr_128;

/// General BID128 pack function.
#[inline(always)]
pub fn bid_get_bid128(pres: &mut BidUint128, sgn: BidUint64, mut expon: i32, mut coeff: BidUint128, rnd_mode: IdecRound, pfpsf: &mut IdecFlags) -> BidUint128 {
  // Is coeff == 10^34 ?
  if coeff.w[1] == 0x0001ed09bead87c0 && coeff.w[0] == 0x378d8e6400000000 {
    expon += 1;
    // Set coefficient to 10^33
    coeff.w[1] = 0x0000314dc6448d93;
    coeff.w[0] = 0x38c15b0a00000000;
  }
  // Check overflow or ubderflow.
  if !(0..=DECIMAL_MAX_EXPON_128).contains(&expon) {
    // Check underflow.
    if expon < 0 {
      return handle_uf_128(pres, sgn, &mut expon, coeff, rnd_mode, pfpsf);
    }
    if expon - MAX_FORMAT_DIGITS_128 <= DECIMAL_MAX_EXPON_128 {
      let t = bid_power10_table_128!(MAX_FORMAT_DIGITS_128 - 1);
      while unsigned_compare_gt_128!(t, coeff) && expon > DECIMAL_MAX_EXPON_128 {
        coeff.w[1] = (coeff.w[1] << 3).wrapping_add(coeff.w[1] << 1).wrapping_add(coeff.w[0] >> 61).wrapping_add(coeff.w[0] >> 63);
        let tmp2 = coeff.w[0] << 3;
        coeff.w[0] = (coeff.w[0] << 1).wrapping_add(tmp2);
        if coeff.w[0] < tmp2 {
          coeff.w[1] = coeff.w[1].wrapping_add(1);
        }
        expon = expon.wrapping_sub(1);
      }
    }
    if expon > DECIMAL_MAX_EXPON_128 {
      if coeff.w[1] | coeff.w[0] == 0 {
        pres.w[1] = sgn | ((DECIMAL_MAX_EXPON_128 as u64) << 49);
        pres.w[0] = 0;
        return *pres;
      }
      // Check overflow.
      if cfg!(feature = "bid-set-status-flags") {
        set_status_flags!(pfpsf, BID_OVERFLOW_EXCEPTION | BID_INEXACT_EXCEPTION);
      }
      if cfg!(not(feature = "ieee-round-nearest-ties-away")) && cfg!(not(feature = "ieee-round-nearest")) {
        if rnd_mode == BID_ROUNDING_TO_ZERO || (sgn > 0 && rnd_mode == BID_ROUNDING_UP) || (sgn == 0 && rnd_mode == BID_ROUNDING_DOWN) {
          pres.w[1] = sgn | _LARGEST_BID128_HIGH;
          pres.w[0] = _LARGEST_BID128_LOW;
        } else {
          pres.w[1] = sgn | INFINITY_MASK64;
          pres.w[0] = 0;
        }
      } else {
        pres.w[1] = sgn | INFINITY_MASK64;
        pres.w[0] = 0;
      }
      return *pres;
    }
  }
  pres.w[0] = coeff.w[0];
  let mut tmp = expon as u64;
  tmp <<= 49;
  pres.w[1] = sgn | tmp | coeff.w[1];
  *pres
}

/// Macro for handling BID128 underflow sticky bit given as additional argument.
#[inline(always)]
pub fn bid_handle_uf_128_rem(pres: &mut BidUint128, sgn: BidUint64, expon: i32, mut cq: BidUint128, r: BidUint64, prounding_mode: IdecRound, _fpsc: &mut IdecFlags) -> BidUint128 {
  let mut qh: BidUint128 = Default::default();
  let mut ql: BidUint128 = Default::default();
  let mut qh1: BidUint128 = Default::default();
  let mut stemp: BidUint128 = Default::default();
  let mut tmp: BidUint128 = Default::default();
  let mut _tmp1: BidUint128 = Default::default();
  let mut cq2: BidUint128 = Default::default();
  let mut cq8: BidUint128 = Default::default();
  let mut carry: BidUint64;
  let _cy: BidUint64;
  let mut rmode: u32;
  let mut _status: u32;

  // UF occurs
  if expon + MAX_FORMAT_DIGITS_128 < 0 {
    if cfg!(feature = "bid-set-status-flags") {
      set_status_flags!(_fpsc, BID_UNDERFLOW_EXCEPTION | BID_INEXACT_EXCEPTION);
    }
    pres.w[1] = sgn;
    pres.w[0] = 0;
    if cfg!(not(feature = "ieee-round-nearest-ties-away")) && cfg!(not(feature = "ieee-round-nearest")) {
      // Round to 1 at least significant position
      if (sgn > 0 && prounding_mode == BID_ROUNDING_DOWN) || (sgn == 0 && prounding_mode == BID_ROUNDING_UP) {
        pres.w[0] = 1;
      }
    }
    return *pres;
  }

  // cq *= 10
  cq2.w[1] = (cq.w[1] << 1) | (cq.w[0] >> 63);
  cq2.w[0] = cq.w[0] << 1;
  cq8.w[1] = (cq.w[1] << 3) | (cq.w[0] >> 61);
  cq8.w[0] = cq.w[0] << 3;
  add_128_128!(cq, cq2, cq8);

  // add remainder
  if r > 0 {
    cq.w[0] |= 1;
  }

  let ed2: i32 = 1 - expon;
  // add rounding constant to cq
  if cfg!(not(feature = "ieee-round-nearest-ties-away")) {
    if cfg!(not(feature = "ieee-round-nearest")) {
      rmode = prounding_mode;
      if sgn > 0 && rmode.wrapping_sub(1) < 2 {
        rmode = 3 - rmode;
      }
    } else {
      rmode = 0;
    }
  } else {
    rmode = 0;
  }

  let t128: BidUint128 = bid_round_const_table_128![rmode, ed2];
  add_carry_out!(cq.w[0], carry, t128.w[0], cq.w[0]);
  cq.w[1] = cq.w[1] + t128.w[1] + carry;

  let tp128: BidUint128 = bid_reciprocals10_128![ed2];
  mul_128x128_full!(qh, ql, cq, tp128);
  let amount: i32 = bid_recip_scale![ed2];

  if amount >= 64 {
    cq.w[0] = qh.w[1] >> (amount - 64);
    cq.w[1] = 0;
  } else {
    shr_128!(cq, qh, amount);
  }

  if cfg!(not(feature = "ieee-round-nearest-ties-away")) {
    let feature_gate = if cfg!(feature = "ieee-round-nearest") { true } else { prounding_mode == 0 };
    if feature_gate && (cq.w[0] & 1) > 0 {
      // check whether fractional part of initial_P/10^ed1 is exactly .5
      // get remainder
      shl_128_long!(qh1, qh, (128 - amount));
      if qh1.w[1] == 0 && qh1.w[0] == 0 && (ql.w[1] < bid_reciprocals10_128![ed2].w[1] || (ql.w[1] == bid_reciprocals10_128![ed2].w[1] && ql.w[0] < bid_reciprocals10_128![ed2].w[0])) {
        dec!(cq.w[0]);
      }
    }
  }

  if cfg!(feature = "bid-set-status-flags") {
    if is_inexact!(_fpsc) {
      set_status_flags!(_fpsc, BID_UNDERFLOW_EXCEPTION);
    } else {
      _status = BID_INEXACT_EXCEPTION;
      // get remainder
      shl_128_long!(qh1, qh, (128 - amount));

      match rmode {
        BID_ROUNDING_TO_NEAREST | BID_ROUNDING_TIES_AWAY => {
          // test whether fractional part is 0
          if qh1.w[1] == 0x8000000000000000 && (qh1.w[0] == 0) && (ql.w[1] < bid_reciprocals10_128![ed2].w[1] || (ql.w[1] == bid_reciprocals10_128![ed2].w[1] && ql.w[0] < bid_reciprocals10_128![ed2].w[0])) {
            _status = BID_EXACT_STATUS;
          }
        }
        BID_ROUNDING_DOWN | BID_ROUNDING_TO_ZERO => {
          if (qh1.w[1] == 0) && (qh1.w[0] == 0) && (ql.w[1] < bid_reciprocals10_128![ed2].w[1] || (ql.w[1] == bid_reciprocals10_128![ed2].w[1] && ql.w[0] < bid_reciprocals10_128![ed2].w[0])) {
            _status = BID_EXACT_STATUS;
          }
        }
        _ => {
          // round up
          add_carry_out!(stemp.w[0], _cy, ql.w[0], bid_reciprocals10_128![ed2].w[0]);
          add_carry_in_out!(stemp.w[1], carry, ql.w[1], bid_reciprocals10_128![ed2].w[1], _cy);
          shr_128_long!(qh, qh1, (128 - amount));
          tmp.w[0] = 1;
          tmp.w[1] = 0;
          shl_128_long!(_tmp1, tmp, amount);
          inc!(qh.w[0], carry);
          if qh.w[0] < carry {
            inc!(qh.w[1]);
          }
          if unsigned_compare_ge_128!(qh, _tmp1) {
            _status = BID_EXACT_STATUS;
          }
        }
      }

      if _status != BID_EXACT_STATUS {
        set_status_flags!(_fpsc, BID_UNDERFLOW_EXCEPTION | _status);
      }
    }
  }
  pres.w[1] = sgn | cq.w[1];
  pres.w[0] = cq.w[0];
  *pres
}

/// Handling BID128 underflow.
#[inline(always)]
pub fn handle_uf_128(pres: &mut BidUint128, sgn: BidUint64, expon: &mut i32, mut cq: BidUint128, rnd_mode: IdecRound, flags: &mut IdecFlags) -> BidUint128 {
  let mut stemp: BidUint128 = Default::default();
  let mut tmp: BidUint128 = Default::default();
  let mut tmp1: BidUint128 = Default::default();
  let mut qh: BidUint128 = BidUint128::default();
  let mut qh1: BidUint128 = BidUint128::default();
  let mut ql: BidUint128 = Default::default();
  let mut carry: BidUint64;
  let cy: BidUint64;
  let rmode: u32;
  let mut status: u32;

  // Underlow occurs.
  if *expon + MAX_FORMAT_DIGITS_128 < 0 {
    #[cfg(feature = "bid-set-status-flags")]
    set_status_flags!(flags, BID_UNDERFLOW_EXCEPTION | BID_INEXACT_EXCEPTION);
    pres.w[1] = sgn;
    pres.w[0] = 0;
    #[cfg(all(not(feature = "ieee-round-nearest-ties-away"), not(feature = "ieee-round-nearest")))]
    {
      use crate::bid_functions::{BID_ROUNDING_DOWN, BID_ROUNDING_UP};
      if (sgn > 0 && rnd_mode == BID_ROUNDING_DOWN) || (sgn == 0 && rnd_mode == BID_ROUNDING_UP) {
        pres.w[0] = 1;
      }
    }
    return *pres;
  }

  let ed2 = 0 - *expon;
  // Add rounding constant to 'cq'.
  #[cfg(not(feature = "ieee-round-nearest-ties-away"))]
  {
    #[cfg(not(feature = "ieee-round-nearest"))]
    {
      rmode = if sgn > 0 && (rnd_mode.wrapping_sub(1)) < 2 { 3 - rnd_mode } else { rnd_mode }
    }
    #[cfg(feature = "ieee-round-nearest")]
    {
      rmode = 0;
    }
  }
  #[cfg(feature = "ieee-round-nearest-ties-away")]
  {
    rmode = 0;
  }

  let t128 = bid_round_const_table_128!(rmode, ed2);
  add_carry_out!(cq.w[0], carry, t128.w[0], cq.w[0]);
  cq.w[1] = cq.w[1] + t128.w[1] + carry;

  let tp128 = bid_reciprocals10_128!(ed2);
  mul_128x128_full!(qh, ql, cq, tp128);
  let amount = bid_recip_scale!(ed2);

  if amount >= 64 {
    cq.w[0] = qh.w[1] >> (amount - 64);
    cq.w[1] = 0;
  } else {
    shr_128!(cq, qh, amount);
  }

  *expon = 0;

  if !cfg!(feature = "ieee-round-nearest-ties-away") {
    let feature_gate = { if cfg!(feature = "ieee-round-nearest") { true } else { rnd_mode == 0 } };
    if feature_gate && cq.w[0] & 1 > 0 {
      // Check whether fractional part of initial_P / 10 ^ ed1 is exactly .5
      // Get the remainder.
      shl_128_long!(qh1, qh, 128 - amount);
      if qh1.w[1] == 0 && qh1.w[0] == 0 && (ql.w[1] < bid_reciprocals10_128!(ed2).w[1] || (ql.w[1] == bid_reciprocals10_128!(ed2).w[1] && ql.w[0] < bid_reciprocals10_128!(ed2).w[0])) {
        cq.w[0] -= 1;
      }
    }
  }

  if cfg!(feature = "bid-set-status-flags") {
    use crate::bid_functions::*;
    if is_inexact!(flags) {
      set_status_flags!(flags, BID_UNDERFLOW_EXCEPTION);
    } else {
      status = BID_INEXACT_EXCEPTION;
      // get remainder
      shl_128_long!(qh1, qh, 128 - amount);
      match rmode {
        BID_ROUNDING_TO_NEAREST | BID_ROUNDING_TIES_AWAY => {
          // test whether fractional part is 0
          if qh1.w[1] == 0x8000000000000000 && qh1.w[0] == 0 && (ql.w[1] < bid_reciprocals10_128!(ed2).w[1] || (ql.w[1] == bid_reciprocals10_128!(ed2).w[1] && ql.w[0] < bid_reciprocals10_128!(ed2).w[0])) {
            status = BID_EXACT_STATUS;
          }
        }
        BID_ROUNDING_DOWN | BID_ROUNDING_TO_ZERO => {
          if (qh1.w[1] == 0) && (qh1.w[0] == 0) && (ql.w[1] < bid_reciprocals10_128!(ed2).w[1] || (ql.w[1] == bid_reciprocals10_128!(ed2).w[1] && ql.w[0] < bid_reciprocals10_128!(ed2).w[0])) {
            status = BID_EXACT_STATUS;
          }
        }
        _ => {
          // round up
          add_carry_out!(stemp.w[0], cy, ql.w[0], bid_reciprocals10_128!(ed2).w[0]);
          add_carry_in_out!(stemp.w[1], carry, ql.w[1], bid_reciprocals10_128!(ed2).w[1], cy);
          shr_128_long!(qh, qh1, 128 - amount);
          tmp.w[0] = 1;
          tmp.w[1] = 0;
          shl_128_long!(tmp1, tmp, amount);
          inc!(qh.w[0], carry);
          if qh.w[0] < carry {
            inc!(qh.w[1], 1);
          }
          if unsigned_compare_ge_128!(qh, tmp1) {
            status = BID_EXACT_STATUS;
          }
        }
      }

      if status != BID_EXACT_STATUS {
        set_status_flags!(flags, BID_UNDERFLOW_EXCEPTION | status);
      }
    }
  }

  pres.w[1] = sgn | cq.w[1];
  pres.w[0] = cq.w[0];

  *pres
}

/*********************************************************************
 *
 *      BID Pack/Unpack Macros
 *
 *********************************************************************/

const SPECIAL_ENCODING_MASK64: u64 = 0x6000000000000000;
const SINFINITY_MASK64: u64 = 0xf800000000000000;
const EXPONENT_MASK128: i32 = 0x3fff;
const NAN_MASK64: u64 = 0x7c00000000000000;
const SMALL_COEFF_MASK128: u64 = 0x0001ffffffffffff;

/// BID128 unpack, input passed by value.
#[inline(always)]
pub fn unpack_bid128_value(psign_x: &mut BidUint64, pexponent_x: &mut i32, pcoefficient_x: &mut BidUint128, x: BidUint128) -> BidUint64 {
  let mut coeff: BidUint128 = Default::default();
  let t33: BidUint128;
  let ex: BidUint64;

  *psign_x = (x.w[1]) & 0x8000000000000000;

  // special encodings
  if (x.w[1] & INFINITY_MASK64) >= SPECIAL_ENCODING_MASK64 {
    if (x.w[1] & INFINITY_MASK64) < INFINITY_MASK64 {
      // non-canonical input
      pcoefficient_x.w[0] = 0;
      pcoefficient_x.w[1] = 0;
      ex = (x.w[1]) >> 47;
      *pexponent_x = (ex as i32) & EXPONENT_MASK128;
      return 0;
    }
    // 10^33
    t33 = bid_power10_table_128![33];
    /*
    coeff.w[0] = x.w[0];
    coeff.w[1] = (x.w[1]) & LARGE_COEFF_MASK128;
    pcoefficient_x->w[0] = x.w[0];
    pcoefficient_x->w[1] = x.w[1];
    if (__unsigned_compare_ge_128 (coeff, t33)) // non-canonical
    pcoefficient_x->w[1] &= (~LARGE_COEFF_MASK128);
    */

    pcoefficient_x.w[0] = x.w[0];
    pcoefficient_x.w[1] = (x.w[1]) & 0x00003fffffffffff;
    if unsigned_compare_ge_128!(*pcoefficient_x, t33)
    // non-canonical
    {
      pcoefficient_x.w[1] = (x.w[1]) & 0xfe00000000000000;
      pcoefficient_x.w[0] = 0;
    } else {
      pcoefficient_x.w[1] = (x.w[1]) & 0xfe003fffffffffff;
    }
    if (x.w[1] & NAN_MASK64) == INFINITY_MASK64 {
      pcoefficient_x.w[0] = 0;
      pcoefficient_x.w[1] = x.w[1] & SINFINITY_MASK64;
    }
    *pexponent_x = 0;
    return 0; // NaN or Infinity
  }

  coeff.w[0] = x.w[0];
  coeff.w[1] = (x.w[1]) & SMALL_COEFF_MASK128;

  // 10^34
  let t34: BidUint128 = bid_power10_table_128![34];
  // check for non-canonical values
  if unsigned_compare_ge_128!(coeff, t34) {
    coeff.w[0] = 0;
    coeff.w[1] = 0;
  }

  pcoefficient_x.w[0] = coeff.w[0];
  pcoefficient_x.w[1] = coeff.w[1];

  ex = (x.w[1]) >> 49;
  *pexponent_x = (ex as i32) & EXPONENT_MASK128;

  coeff.w[0] | coeff.w[1]
}

#[repr(C)]
pub union U64Double {
  pub u: u64,
  pub f: f64,
}

#[repr(C)]
pub union IntDouble {
  pub i: u64,
  pub d: f64,
}

impl Default for IntDouble {
  fn default() -> Self {
    Self { i: 0 }
  }
}

#[repr(C)]
pub union IntFloat {
  pub i: u32,
  pub d: f32,
}

impl Default for IntFloat {
  fn default() -> Self {
    Self { i: 0 }
  }
}

macro_rules! bits {
  ($value:expr) => {
    ((((unsafe { U64Double { f: $value as f64 }.u } >> 52) as u32) & 0x7ff) - 0x3ff) as i32
  };
}
pub(crate) use bits;
