use crate::bid64::Bid64;

/// 256-bit decimal floating-point in binary format.
#[repr(C, align(16))]
#[derive(Default, Copy, Clone)]
pub struct Bid192 {
  pub(crate) w: [Bid64; 3],
}
