use decimus::Bid128;

#[test]
fn _0001() {
  assert_eq!("[0000000000000000 0000000000000000]", format!("{:?}", Bid128::default()))
}

#[test]
fn _0002() {
  assert_eq!("[3040000000000000 0000000000000000]", format!("{:?}", Bid128::zero()))
}

#[test]
fn _0003() {
  assert_eq!("[B040000000000000 0000000000000000]", format!("{:?}", Bid128::minus_zero()))
}

#[test]
fn _0004() {
  assert_eq!("[7800000000000000 0000000000000000]", format!("{:?}", Bid128::inf()))
}

#[test]
fn _0005() {
  assert_eq!("[F800000000000000 0000000000000000]", format!("{:?}", Bid128::minus_inf()))
}

#[test]
fn _0006() {
  assert_eq!("[DFFFED09BEAD87C0 378D8E63FFFFFFFF]", format!("{:?}", Bid128::min()))
}

#[test]
fn _0007() {
  assert_eq!("[5FFFED09BEAD87C0 378D8E63FFFFFFFF]", format!("{:?}", Bid128::max()))
}
