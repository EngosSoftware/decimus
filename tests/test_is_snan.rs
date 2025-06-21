use decimus::Bid128;

#[test]
fn _0001() {
  assert!(!Bid128::nan().is_snan());
}

#[test]
fn _0002() {
  assert!(!Bid128::minus_nan().is_snan());
}

#[test]
fn _0003() {
  assert!(!Bid128::qnan().is_snan());
}

#[test]
fn _0004() {
  assert!(!Bid128::minus_qnan().is_snan());
}

#[test]
fn _0005() {
  assert!(Bid128::snan().is_snan());
}

#[test]
fn _0006() {
  assert!(Bid128::minus_snan().is_snan());
}

#[test]
fn _0007() {
  assert!(!Bid128::zero().is_snan());
}

#[test]
fn _0008() {
  assert!(!Bid128::minus_zero().is_snan());
}

#[test]
fn _0009() {
  assert!(!Bid128::inf().is_snan());
}

#[test]
fn _0010() {
  assert!(!Bid128::minus_inf().is_snan());
}

#[test]
fn _0011() {
  assert!(!Bid128::min().is_snan());
}

#[test]
fn _0012() {
  assert!(!Bid128::max().is_snan());
}
