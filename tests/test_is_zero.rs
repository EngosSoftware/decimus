use decimus::Bid128;

#[test]
fn _0001() {
  assert!(Bid128::zero().is_zero());
}

#[test]
fn _0002() {
  assert!(Bid128::minus_zero().is_zero());
}

#[test]
fn _0003() {
  assert!(!Bid128::inf().is_zero());
}

#[test]
fn _0004() {
  assert!(!Bid128::minus_inf().is_zero());
}

#[test]
fn _0005() {
  assert!(!Bid128::min().is_zero());
}

#[test]
fn _0006() {
  assert!(!Bid128::max().is_zero());
}

#[test]
fn _0007() {
  assert!(!Bid128::nan().is_zero());
}

#[test]
fn _0008() {
  assert!(!Bid128::snan().is_zero());
}

#[test]
fn _0009() {
  assert!(!Bid128::new(0x0001ed09bead87c0, 0x378d8e62ffffffff).is_zero());
}

#[test]
fn _0010() {
  assert!(Bid128::new(0x0001ed09bead87c0, 0x378d8e64ffffffff).is_zero());
}

#[test]
fn _0011() {
  assert!(Bid128::new(0x0ee8000000000000, 0x0000000000000000).is_zero());
}

#[test]
fn _0012() {
  assert!(!Bid128::new(0x13ee9ca2e80fd3a8, 0x07c0d8414c535392).is_zero());
}

#[test]
fn _0013() {
  assert!(!Bid128::new(0x31de9b1749a9038c, 0x04b0b67e429838c2).is_zero());
}

#[test]
fn _0014() {
  assert!(Bid128::new(0x3b6ffefffd79ddfd, 0x3304651402b7cb82).is_zero());
}

#[test]
fn _0015() {
  assert!(!Bid128::new(0x789b88be70d10384, 0xffffffffffffffff).is_zero());
}

#[test]
fn _0016() {
  assert!(!Bid128::new(0x7c003fffffffffff, 0x38c15b08ffffffff).is_zero());
}

#[test]
fn _0017() {
  assert!(!Bid128::new(0x7c003fffffffffff, 0x38c15b0affffffff).is_zero());
}

#[test]
fn _0018() {
  assert!(!Bid128::new(0x7e0028d5f55d1b90, 0xcd0683a16d4f6440).is_zero());
}

#[test]
fn _0019() {
  assert!(!Bid128::new(0x892747418097592c, 0x11a5167c09ca2055).is_zero());
}

#[test]
fn _0020() {
  assert!(!Bid128::new(0x8aa0c8dafc695d02, 0x42fb6071b7550296).is_zero());
}

#[test]
fn _0021() {
  assert!(!Bid128::new(0x9ee35adc537f2993, 0x21571042d581776a).is_zero());
}

#[test]
fn _0022() {
  assert!(Bid128::new(0xaf52000000000000, 0x0000000000000000).is_zero());
}

#[test]
fn _0023() {
  assert!(!Bid128::new(0xb37ef809e2d1f6a6, 0x2badece51a0eddd9).is_zero());
}

#[test]
fn _0024() {
  assert!(!Bid128::new(0xb8249c80a0002a5e, 0x9fc635c5912fb958).is_zero());
}

#[test]
fn _0025() {
  assert!(!Bid128::new(0xd8e96a50ff859c40, 0x1b0d91d7b39d89c8).is_zero());
}
