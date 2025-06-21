use decimus::BidUint64;

#[test]
fn _0001() {
  assert_eq!(BidUint64::new(0), BidUint64::default());
}

#[test]
fn _0002() {
  assert_eq!("BidUint64(0)", format!("{:?}", BidUint64::default()));
}

#[test]
fn _0003() {
  let ca = 10;
  let cb = 20;
  let a = BidUint64::new(ca);
  let b = BidUint64::new(cb);
  assert_eq!(ca.wrapping_add(cb), *(a + b));
}

#[test]
fn _0004() {
  let ca = 0xFFFFFFFFFFFFFFFF;
  let cb = 0x1;
  // let _ = ca + cb; // this would overflow without wrapping
  let a = BidUint64::new(ca);
  let b = BidUint64::new(cb);
  assert_eq!(ca.wrapping_add(cb), *(a + b));
}

#[test]
fn _0005() {
  let ca = 0x1;
  let cb = 0xFFFFFFFFFFFFFFFF;
  // let _ = ca - cb; // this would overflow without wrapping
  let a = BidUint64::new(ca);
  let b = BidUint64::new(cb);
  let c = a - b;
  assert_eq!(ca.wrapping_sub(cb), *c);
}

#[test]
fn _0006() {
  let ca = 0xFFFFFFF;
  let cb = 0xFFFFFFFFFFFFFFFF;
  // let _ = ca * cb; // this would overflow without wrapping
  let a = BidUint64::new(ca);
  let b = BidUint64::new(cb);
  let c = a * b;
  assert_eq!(ca.wrapping_mul(cb), *c);
}

#[test]
fn _0007() {
  let ca = 0xFFFFFFF;
  let cb = 0xFFFFFFFFFFFFFFFF;
  let a = BidUint64::new(ca);
  let b = BidUint64::new(cb);
  let c = a / b;
  assert_eq!(ca.wrapping_div(cb), *c);
}

#[test]
fn _0008() {
  let ca = 0xFFFFFFF;
  let cb = 0xFFFFFFFFFFFFFFFF;
  let a = BidUint64::new(ca);
  let b = BidUint64::new(cb);
  let c = a ^ b;
  assert_eq!(ca ^ cb, *c);
}

#[test]
fn _0009() {
  let ca = 0xFFFFFFFFFFFFFFFF_u64;
  let cb = 0x1_u32;
  let a = BidUint64::new(ca);
  assert_eq!(ca.wrapping_add(cb as u64), *(a + cb));
  assert_eq!((cb as u64).wrapping_add(ca), *(cb + a));
}

#[test]
fn _0010() {
  let ca = 0xFFFFFFFFFFFFFFFF_u64;
  let cb = 0x1_u64;
  let a = BidUint64::new(ca);
  assert_eq!(ca.wrapping_add(cb), *(a + cb));
  assert_eq!(cb.wrapping_add(ca), *(cb + a));
}
