/// Rounding to nearest.
pub const BID_ROUNDING_TO_NEAREST: u32 = 0x0;

/// Rounding towards minus infinity.
pub const BID_ROUNDING_DOWN: u32 = 0x1;

/// Rounding towards plus infinity.
pub const BID_ROUNDING_UP: u32 = 0x2;

/// Rounding towards zero.
pub const BID_ROUNDING_TO_ZERO: u32 = 0x3;

/// Rounding away from zero.
pub const BID_ROUNDING_TIES_AWAY: u32 = 0x4;

/// Flag indicating no exceptions.
pub const BID_EXACT_STATUS: u32 = 0x00000000;

const DEC_FE_CLEAR: u32 = 0x00;
const DEC_FE_INVALID: u32 = 0x01;
const DEC_FE_UNNORMAL: u32 = 0x02;
const DEC_FE_DIVBYZERO: u32 = 0x04;
const DEC_FE_OVERFLOW: u32 = 0x08;
const DEC_FE_UNDERFLOW: u32 = 0x10;
const DEC_FE_INEXACT: u32 = 0x20;

/// No exception.
pub const BID_NO_EXCEPTION: u32 = DEC_FE_CLEAR;

/// Inexact exception.
pub const BID_INEXACT_EXCEPTION: u32 = DEC_FE_INEXACT;

/// Underflow exception.
pub const BID_UNDERFLOW_EXCEPTION: u32 = DEC_FE_UNDERFLOW;

/// Overflow exception.
pub const BID_OVERFLOW_EXCEPTION: u32 = DEC_FE_OVERFLOW;

/// Zero divide exception.
pub const BID_ZERO_DIVIDE_EXCEPTION: u32 = DEC_FE_DIVBYZERO;

/// Denormal exception.
pub const BID_DENORMAL_EXCEPTION: u32 = DEC_FE_UNNORMAL;

/// Invalid exception.
pub const BID_INVALID_EXCEPTION: u32 = DEC_FE_INVALID;

/// Underflow inexact exception.
pub const BID_UNDERFLOW_INEXACT_EXCEPTION: u32 = DEC_FE_UNDERFLOW | DEC_FE_INEXACT;

/// Overflow inexact exception.
pub const BID_OVERFLOW_INEXACT_EXCEPTION: u32 = DEC_FE_OVERFLOW | DEC_FE_INEXACT;
