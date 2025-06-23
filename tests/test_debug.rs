use decimus::{BidUint32, BidUint64, BidUint128, BidUint192, BidUint256};

#[test]
fn _0001() {
  assert_eq!("0", format!("{:?}", BidUint32::default()));
}

#[test]
fn _0002() {
  assert_eq!("4294967295", format!("{:?}", 0xFFFFFFFF as BidUint32));
}

#[test]
fn _0003() {
  assert_eq!("0", format!("{:?}", BidUint64::default()));
}

#[test]
fn _0004() {
  assert_eq!("18446744073709551615", format!("{:?}", 0xFFFFFFFFFFFFFFFF as BidUint64));
}

#[test]
fn _0005() {
  assert_eq!("[0000000000000000 0000000000000000]", format!("{:?}", BidUint128::default()));
}

#[test]
fn _0006() {
  assert_eq!("[2000000000000000 1000000000000000]", format!("{:?}", BidUint128 { w: [0x1000000000000000, 0x2000000000000000] }));
}

#[test]
fn _0007() {
  assert_eq!("[0000000000000000 0000000000000000 0000000000000000]", format!("{:?}", BidUint192::default()));
}

#[test]
fn _0008() {
  assert_eq!(
    "[3000000000000000 2000000000000000 1000000000000000]",
    format!(
      "{:?}",
      BidUint192 {
        w: [0x1000000000000000, 0x2000000000000000, 0x3000000000000000]
      }
    )
  );
}

#[test]
fn _0009() {
  assert_eq!("[0000000000000000 0000000000000000 0000000000000000 0000000000000000]", format!("{:?}", BidUint256::default()));
}

#[test]
fn _0010() {
  assert_eq!(
    "[4000000000000000 3000000000000000 2000000000000000 1000000000000000]",
    format!(
      "{:?}",
      BidUint256 {
        w: [0x1000000000000000, 0x2000000000000000, 0x3000000000000000, 0x4000000000000000]
      }
    )
  );
}
