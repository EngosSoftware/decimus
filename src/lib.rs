//! # Decimal Floating-Point Math Library

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]
extern crate alloc;

mod bid128;
mod bid128_add;
mod bid128_common;
mod bid128_string;
mod bid192;
mod bid256;
mod bid64;
mod bid_conf;
mod bid_decimal_data;
mod bid_functions;
mod bid_internal;

pub use bid_conf::{IdecFlags, IdecRound};
pub use bid_functions::{
  BID_DENORMAL_EXCEPTION, BID_EXACT_STATUS, BID_INEXACT_EXCEPTION, BID_INVALID_EXCEPTION, BID_NO_EXCEPTION, BID_OVERFLOW_EXCEPTION, BID_OVERFLOW_INEXACT_EXCEPTION, BID_ROUNDING_DOWN, BID_ROUNDING_TIES_AWAY, BID_ROUNDING_TO_NEAREST,
  BID_ROUNDING_TO_ZERO, BID_ROUNDING_UP, BID_UNDERFLOW_EXCEPTION, BID_UNDERFLOW_INEXACT_EXCEPTION, BID_ZERO_DIVIDE_EXCEPTION,
};
pub use bid64::BidUint64;
pub use bid128::Bid128;
pub use bid128_add::bid128_add;
pub use bid128_string::bid128_from_string;
