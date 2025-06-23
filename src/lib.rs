//! # Decimal Floating-Point Math Library

#![no_std]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]

extern crate alloc;

mod bid128;
mod bid128_2_str_macros;
mod bid128_2_str_tables;
mod bid128_add;
mod bid128_common;
mod bid128_noncomp;
mod bid128_string;
mod bid_conf;
mod bid_decimal_data;
mod bid_from_int;
mod bid_functions;
mod bid_internal;
mod bid_types;

pub use bid_conf::{IdecFlags, IdecRound};
pub use bid_from_int::bid128_from_int32;
pub use bid_functions::{
  BID_DENORMAL_EXCEPTION, BID_EXACT_STATUS, BID_INEXACT_EXCEPTION, BID_INVALID_EXCEPTION, BID_NO_EXCEPTION, BID_OVERFLOW_EXCEPTION, BID_OVERFLOW_INEXACT_EXCEPTION, BID_ROUNDING_DOWN, BID_ROUNDING_TIES_AWAY, BID_ROUNDING_TO_NEAREST,
  BID_ROUNDING_TO_ZERO, BID_ROUNDING_UP, BID_UNDERFLOW_EXCEPTION, BID_UNDERFLOW_INEXACT_EXCEPTION, BID_ZERO_DIVIDE_EXCEPTION,
};
pub use bid_types::{BidUint32, BidUint64, BidUint128, BidUint192, BidUint256};
pub use bid128_add::{bid128_add, bid128_sub};
pub use bid128_noncomp::bid128_is_zero;
pub use bid128_string::{bid128_from_string, bid128_to_string};
