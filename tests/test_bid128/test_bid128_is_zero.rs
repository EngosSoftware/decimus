use super::*;
use decimus::{BidUint128, bid128_from_string, bid128_is_zero};

macro_rules! check {
  ($rnd_mode:expr, $x:expr, $expected:expr, $expected_flags:expr) => {
    let mut actual_flags: IdecFlags = 0;
    let x = bid128_from_string($x, $rnd_mode, &mut actual_flags);
    assert_eq!($expected_flags, actual_flags, "Flags error, expected = {:02X}, actual = {:02X}", $expected_flags, actual_flags);
    assert_eq!($expected, bid128_is_zero(x));
  };
  ($x:expr, $expected:expr) => {
    let x = BidUint128 { w: [$x[1], $x[0]] };
    assert_eq!($expected, bid128_is_zero(x));
  };
}

#[test]
fn _0001() {
  check!([0x0001ed09bead87c0, 0x378d8e62ffffffff], false);
}

#[test]
fn _0002() {
  check!([0x0001ed09bead87c0, 0x378d8e64ffffffff], true);
}

#[test]
fn _0003() {
  check!([0x0ee8000000000000, 0x0000000000000000], true);
}

#[test]
fn _0004() {
  check!([0x13ee9ca2e80fd3a8, 0x07c0d8414c535392], false);
}

#[test]
fn _0005() {
  check!([0x31de9b1749a9038c, 0x04b0b67e429838c2], false);
}

#[test]
fn _0006() {
  check!([0x3b6ffefffd79ddfd, 0x3304651402b7cb82], true);
}

#[test]
fn _0007() {
  check!([0x789b88be70d10384, 0xffffffffffffffff], false);
}

#[test]
fn _0008() {
  check!([0x7c003fffffffffff, 0x38c15b08ffffffff], false);
}

#[test]
fn _0009() {
  check!([0x7c003fffffffffff, 0x38c15b0affffffff], false);
}

#[test]
fn _0010() {
  check!([0x7e0028d5f55d1b90, 0xcd0683a16d4f6440], false);
}

#[test]
fn _0011() {
  check!([0x892747418097592c, 0x11a5167c09ca2055], false);
}

#[test]
fn _0012() {
  check!([0x8aa0c8dafc695d02, 0x42fb6071b7550296], false);
}

#[test]
fn _0013() {
  check!([0x9ee35adc537f2993, 0x21571042d581776a], false);
}

#[test]
fn _0014() {
  check!([0xaf52000000000000, 0x0000000000000000], true);
}

#[test]
fn _0015() {
  check!([0xb37ef809e2d1f6a6, 0x2badece51a0eddd9], false);
}

#[test]
fn _0016() {
  check!([0xb8249c80a0002a5e, 0x9fc635c5912fb958], false);
}

#[test]
fn _0017() {
  check!([0xd8e96a50ff859c40, 0x1b0d91d7b39d89c8], false);
}

#[test]
fn _0018() {
  check!(0, "0", true, F_00_00);
}

#[test]
fn _0019() {
  check!(0, "-0", true, F_00_00);
}

#[test]
fn _0020() {
  check!(0, "Infinity", false, F_00_00);
}

#[test]
fn _0021() {
  check!(0, "-Infinity", false, F_00_00);
}

#[test]
fn _0022() {
  check!(0, "QNaN", false, F_00_00);
}

#[test]
fn _0023() {
  check!(0, "-QNaN", false, F_00_00);
}

#[test]
fn _0024() {
  check!(0, "SNaN", false, F_00_00);
}
#[test]
fn _0025() {
  check!(0, "-SNaN", false, F_00_00);
}
