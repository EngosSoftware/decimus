use decimus::{BidUint128, bid128_to_string};

macro_rules! check {
  ($x:expr, $expected:expr) => {
    let x = BidUint128 { w: [$x[1], $x[0]] };
    assert_eq!($expected, bid128_to_string(x));
  };
}

#[test]
fn _0001() {
  check!([0x0001ed09bead87c0, 0x378d8e62ffffffff], "+9999999999999999999999995705032703E-6176");
}

#[test]
fn _0002() {
  check!([0x0000000000000000, 0x00000000004ccaff], "+5032703E-6176");
}

#[test]
fn _0003() {
  check!([0x3136000000000000, 0x000000000000007b], "+123E+123");
}

#[test]
fn _0004() {
  check!([0x8001ed09bead87c0, 0x378d8e62ffffffff], "-9999999999999999999999995705032703E-6176");
}

#[test]
fn _0005() {
  check!([0x0001ed09bead87c0, 0x378d8e64ffffffff], "+0E-6176");
}

#[test]
fn _0006() {
  check!([0x8001ed09bead87c0, 0x378d8e64ffffffff], "-0E-6176");
}

#[test]
fn _0007() {
  check!([0x3040000000000000, 0x0000000000000010], "+16E+0");
}

#[test]
fn _0008() {
  check!([0xB040000000000000, 0x0000000000000010], "-16E+0");
}

#[test]
fn _0009() {
  check!([0x6000000000000000, 0x0000000000000000], "+0E-6176");
}

#[test]
fn _0010() {
  check!([0xE000000000000000, 0x0000000000000000], "-0E-6176");
}

#[test]
fn _0011() {
  check!([0x69dbb75d7734cd9e, 0x1234567890123456], "+0E-1129");
}

#[test]
fn _0012() {
  check!([0xe9dbb75d7734cd9e, 0x1234567890123456], "-0E-1129");
}

#[test]
fn _0013() {
  check!([0x7910000000000000, 0x0000000000000000], "+Inf");
}

#[test]
fn _0014() {
  check!([0xf910000000000000, 0x0000000000000000], "-Inf");
}

#[test]
fn _0015() {
  check!([0x7c003fffffffffff, 0x38c15b08ffffffff], "+NaN");
}

#[test]
fn _0016() {
  check!([0xfc003fffffffffff, 0x38c15b08ffffffff], "-NaN");
}

#[test]
fn _0017() {
  check!([0x7c003fffffffffff, 0x38c15b0affffffff], "+NaN");
}

#[test]
fn _0018() {
  check!([0xfc003fffffffffff, 0x38c15b0affffffff], "-NaN");
}
#[test]
fn _0019() {
  check!([0x7e00000000000000, 0x0000000000000000], "+SNaN");
}

#[test]
fn _0020() {
  check!([0x7eff3fffffffffff, 0xffffffffffffffff], "+SNaN");
}

#[test]
fn _0021() {
  check!([0xb0fa000000000000, 0x0000000001312d00], "-20000000E+93");
}

#[test]
fn _0022() {
  check!([0xe000000000000000, 0x0000000000000001], "-0E-6176");
}

#[test]
fn _0023() {
  check!([0xf9003fffffffffff, 0x38c15b08ffffffff], "-Inf");
}

#[test]
fn _0024() {
  check!([0x0001ed09bead87c0, 0x378d8e62ffffffff], "+9999999999999999999999995705032703E-6176");
}

#[test]
fn _0025() {
  check!([0x0001ed09bead87c0, 0x378d8e64ffffffff], "+0E-6176");
}

#[test]
fn _0026() {
  check!([0x7c003fffffffffff, 0x38c15b08ffffffff], "+NaN");
}

#[test]
fn _0027() {
  check!([0x7c003fffffffffff, 0x38c15b0affffffff], "+NaN");
}

#[test]
fn _0028() {
  check!([0x0001ed09bead87c0, 0x378d8e62ffffffff], "+9999999999999999999999995705032703E-6176");
}

#[test]
fn _0029() {
  check!([0x0001ed09bead87c0, 0x378d8e64ffffffff], "+0E-6176");
}

#[test]
fn _0030() {
  check!([0x7c003fffffffffff, 0x38c15b08ffffffff], "+NaN");
}

#[test]
fn _0031() {
  check!([0xfc003fffffffffff, 0x38c15b08ffffffff], "-NaN");
}

#[test]
fn _0032() {
  check!([0x7c003fffffffffff, 0x38c15b0affffffff], "+NaN");
}

#[test]
fn _0033() {
  check!([0xfc003fffffffffff, 0x38c15b0affffffff], "-NaN");
}

#[test]
fn _0034() {
  check!([0x0001ed09bead87c0, 0x378d8e62ffffffff], "+9999999999999999999999995705032703E-6176");
}

#[test]
fn _0035() {
  check!([0x0001ed09bead87c0, 0x378d8e64ffffffff], "+0E-6176");
}

#[test]
fn _0036() {
  check!([0x7c003fffffffffff, 0x38c15b08ffffffff], "+NaN");
}

#[test]
fn _0037() {
  check!([0xfc003fffffffffff, 0x38c15b08ffffffff], "-NaN");
}

#[test]
fn _0038() {
  check!([0x7c003fffffffffff, 0x38c15b0affffffff], "+NaN");
}

#[test]
fn _0039() {
  check!([0xfc003fffffffffff, 0x38c15b0affffffff], "-NaN");
}

#[test]
fn _0040() {
  check!([0xfe003fffffffffff, 0xffffffffffffffff], "-SNaN");
}

#[test]
fn _0041() {
  check!([0x0001ed09bead87c0, 0x378d8e62ffffffff], "+9999999999999999999999995705032703E-6176");
}

#[test]
fn _0042() {
  check!([0x0001ed09bead87c0, 0x378d8e64ffffffff], "+0E-6176");
}

#[test]
fn _0043() {
  check!([0x7c003fffffffffff, 0x38c15b08ffffffff], "+NaN");
}

#[test]
fn _0044() {
  check!([0x7c003fffffffffff, 0x38c15b0affffffff], "+NaN");
}

#[test]
fn _0045() {
  check!([0x3810000000000000, 0x0000000000000000], "+0E+1000");
}

#[test]
fn _0046() {
  check!([0x380e000000000000, 0x0000000000000000], "+0E+999");
}

#[test]
fn _0047() {
  check!([0x3108000000000000, 0x0000000000000000], "+0E+100");
}

#[test]
fn _0048() {
  check!([0x3106000000000000, 0x0000000000000000], "+0E+99");
}

#[test]
fn _0049() {
  check!([0x3054000000000000, 0x0000000000000000], "+0E+10");
}

#[test]
fn _0050() {
  check!([0x3052000000000000, 0x0000000000000000], "+0E+9");
}

#[test]
fn _0051() {
  check!([0x2870000000000000, 0x0000000000000000], "+0E-1000");
}

#[test]
fn _0052() {
  check!([0x2872000000000000, 0x0000000000000000], "+0E-999");
}

#[test]
fn _0053() {
  check!([0x2f78000000000000, 0x0000000000000000], "+0E-100");
}

#[test]
fn _0054() {
  check!([0x2f7a000000000000, 0x0000000000000000], "+0E-99");
}

#[test]
fn _0055() {
  check!([0x302c000000000000, 0x0000000000000000], "+0E-10");
}

#[test]
fn _0056() {
  check!([0x302e000000000000, 0x0000000000000000], "+0E-9");
}
