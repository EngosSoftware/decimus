use super::*;
use decimus::{BidUint128, IdecFlags, bid128_from_string};

macro_rules! check {
  ($rnd_mode:expr, $x:expr, $expected:expr, $expected_flags:expr) => {
    let mut actual_flags: IdecFlags = 0;
    let expected = BidUint128 { w: [$expected[1], $expected[0]] };
    let actual = bid128_from_string($x, $rnd_mode, &mut actual_flags);
    assert_eq!(expected, actual);
    assert_eq!($expected_flags, actual_flags, "Flags error, expected = {:02X}, actual = {:02X}", $expected_flags, actual_flags);
  };
}

#[test]
fn _0001() {
  check!(0, "", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0002() {
  check!(0, "      ", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0003() {
  check!(0, " \t  \r   ", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0004() {
  check!(0, "Decimal", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0005() {
  check!(0, "-", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0006() {
  check!(0, "+", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0007() {
  check!(0, ".", [0x3040000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0008() {
  check!(0, "..", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0009() {
  check!(0, "0.000000000.0", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0010() {
  check!(0, "1.0", [0x303e000000000000, 0x000000000000000a], F_00_00);
}

#[test]
fn _0011() {
  check!(0, "1.0e0004", [0x3046000000000000, 0x000000000000000a], F_00_00);
}

#[test]
fn _0012() {
  check!(0, "1.0.0", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0013() {
  check!(0, "+.", [0x3040000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0014() {
  check!(0, "-.", [0xb040000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0015() {
  check!(0, "inf", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0016() {
  check!(0, "Inf", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0017() {
  check!(0, "INF", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0018() {
  check!(0, "iNf", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0019() {
  check!(0, "inF", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0020() {
  check!(0, " \t  inf", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0021() {
  check!(0, "infa", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0022() {
  check!(0, "+inf", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0023() {
  check!(0, "+Inf", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0024() {
  check!(0, "+INF", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0025() {
  check!(0, "+iNf", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0026() {
  check!(0, "+inF", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0027() {
  check!(0, "\t \t +inF", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0028() {
  check!(0, "+INFa", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0029() {
  check!(0, "-inf", [0xf800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0030() {
  check!(0, "-Inf", [0xf800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0031() {
  check!(0, "-INF", [0xf800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0032() {
  check!(0, "-iNf", [0xf800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0033() {
  check!(0, "-inF", [0xf800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0034() {
  check!(0, " \t \t -inF", [0xf800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0035() {
  check!(0, ".inf", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0036() {
  check!(0, "infinity", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0037() {
  check!(0, "Infinity", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0038() {
  check!(0, "INFINITY", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0039() {
  check!(0, "iNFINITy", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0040() {
  check!(0, "infinitY", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0041() {
  check!(0, "+infinity", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0042() {
  check!(0, "+Infinity", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0043() {
  check!(0, "+INFINITY", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0044() {
  check!(0, "+iNFINITy", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0045() {
  check!(0, "+infinitY", [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0046() {
  check!(0, "-infinity", [0xf800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0047() {
  check!(0, "-Infinity", [0xf800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0048() {
  check!(0, "-INFINITY", [0xf800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0049() {
  check!(0, "-iNFINITy", [0xf800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0050() {
  check!(0, "-infinitY", [0xf800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0051() {
  check!(0, "i", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0052() {
  check!(0, "in", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0053() {
  check!(0, "infa", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0054() {
  check!(0, "infi", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0055() {
  check!(0, "infin", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0056() {
  check!(0, "infini", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0057() {
  check!(0, "infinit", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0058() {
  check!(0, "infinitya", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0059() {
  check!(0, "+i", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0060() {
  check!(0, "+in", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0061() {
  check!(0, "+infa", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0062() {
  check!(0, "+infi", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0063() {
  check!(0, "+infin", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0064() {
  check!(0, "+infini", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0065() {
  check!(0, "+infinit", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0066() {
  check!(0, "+infinitya", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0067() {
  check!(0, "-i", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0068() {
  check!(0, "-in", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0069() {
  check!(0, "-infa", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0070() {
  check!(0, "-infi", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0071() {
  check!(0, "-infin", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0072() {
  check!(0, "-infini", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0073() {
  check!(0, "-infinit", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0074() {
  check!(0, "-infinitya", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0075() {
  check!(0, "NaN", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0076() {
  check!(0, "+NaN", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0077() {
  check!(0, "nan", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0078() {
  check!(0, "+nan", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0079() {
  check!(0, "nAn", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0080() {
  check!(0, "+nAn", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0081() {
  check!(0, "-NaN", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0082() {
  check!(0, "-NaN", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0083() {
  check!(0, "-nan", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0084() {
  check!(0, "-nan", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0085() {
  check!(0, "-nAn", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0086() {
  check!(0, "-nAn", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0087() {
  check!(0, "n", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0088() {
  check!(0, "na", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0089() {
  check!(0, "nana", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0090() {
  check!(0, "+n", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0091() {
  check!(0, "+na", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0092() {
  check!(0, "+nana", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0093() {
  check!(0, "-n", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0094() {
  check!(0, "-na", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0095() {
  check!(0, "-nana", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0096() {
  check!(0, "SNaN", [0x7e00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0097() {
  check!(0, "sNaN", [0x7e00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0098() {
  check!(0, "SNaNi", [0x7e00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0099() {
  check!(0, "+SNaN", [0x7e00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0100() {
  check!(0, "+sNaN", [0x7e00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0101() {
  check!(0, "+SNaNi", [0x7e00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0102() {
  check!(0, "snan", [0x7e00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0103() {
  check!(0, "+snan", [0x7e00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0104() {
  check!(0, "snAn", [0x7e00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0105() {
  check!(0, "+snAn", [0x7e00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0106() {
  check!(0, "-SNaN", [0xfe00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0107() {
  check!(0, "-sNaN", [0xfe00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0108() {
  check!(0, "-SNaN", [0xfe00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0109() {
  check!(0, "-SNaNi", [0xfe00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0110() {
  check!(0, "-sNaN", [0xfe00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0111() {
  check!(0, "-snan", [0xfe00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0112() {
  check!(0, "-snan", [0xfe00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0113() {
  check!(0, "-snAn", [0xfe00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0114() {
  check!(0, "-snAn", [0xfe00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0115() {
  check!(0, "s", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0116() {
  check!(0, "sn", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0117() {
  check!(0, "sna", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0118() {
  check!(0, "snana", [0x7e00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0119() {
  check!(0, "+s", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0120() {
  check!(0, "+sn", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0121() {
  check!(0, "+sna", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0122() {
  check!(0, "+snana", [0x7e00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0123() {
  check!(0, "-s", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0124() {
  check!(0, "-sn", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0125() {
  check!(0, "-sna", [0xfc00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0126() {
  check!(0, "-snana", [0xfe00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0127() {
  check!(0, "0", [0x3040000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0128() {
  check!(0, "00", [0x3040000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0129() {
  check!(0, "000", [0x3040000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0130() {
  check!(0, "+0", [0x3040000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0131() {
  check!(0, "+00", [0x3040000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0132() {
  check!(0, "+000", [0x3040000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0133() {
  check!(0, "-0", [0xb040000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0134() {
  check!(0, "-00", [0xb040000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0135() {
  check!(0, "-000", [0xb040000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0136() {
  check!(0, "0", [0x3040000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0137() {
  check!(0, "0e6176", [0x5ffe000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0138() {
  check!(0, "12345678901234567890123456789012345", [0x30423cde6fff9732, 0xde825cd07e96aff2], F_20_00);
}

#[test]
fn _0139() {
  check!(1, "12345678901234567890123456789012345", [0x30423cde6fff9732, 0xde825cd07e96aff2], F_20_00);
}

#[test]
fn _0140() {
  check!(2, "12345678901234567890123456789012345", [0x30423cde6fff9732, 0xde825cd07e96aff3], F_20_00);
}

#[test]
fn _0141() {
  check!(3, "12345678901234567890123456789012345", [0x30423cde6fff9732, 0xde825cd07e96aff2], F_20_00);
}

#[test]
fn _0142() {
  check!(4, "12345678901234567890123456789012345", [0x30423cde6fff9732, 0xde825cd07e96aff3], F_20_00);
}

#[test]
fn _0143() {
  check!(1, "-9.9999999999999999999999999999999995", [0xb000314dc6448d93, 0x38c15b0a00000000], F_20_00);
}

#[test]
fn _0144() {
  check!(2, "-9.9999999999999999999999999999999995", [0xafffed09bead87c0, 0x378d8e63ffffffff], F_20_00);
}

#[test]
fn _0145() {
  check!(0, "9.9999999999999999999999999999999995", [0x3000314dc6448d93, 0x38c15b0a00000000], F_20_00);
}

#[test]
fn _0146() {
  check!(1, "9.9999999999999999999999999999999995", [0x2fffed09bead87c0, 0x378d8e63ffffffff], F_20_00);
}

#[test]
fn _0147() {
  check!(2, "9.9999999999999999999999999999999995", [0x3000314dc6448d93, 0x38c15b0a00000000], F_20_00);
}

#[test]
fn _0148() {
  check!(3, "9.9999999999999999999999999999999995", [0x2fffed09bead87c0, 0x378d8e63ffffffff], F_20_00);
}

#[test]
fn _0149() {
  check!(4, "9.9999999999999999999999999999999995", [0x3000314dc6448d93, 0x38c15b0a00000000], F_20_00);
}

#[test]
fn _0150() {
  check!(0, "1.0000000000000000000000000000000015", [0x2ffe314dc6448d93, 0x38c15b0a00000002], F_20_00);
}

#[test]
fn _0151() {
  check!(1, "1.0000000000000000000000000000000015", [0x2ffe314dc6448d93, 0x38c15b0a00000001], F_20_00);
}

#[test]
fn _0152() {
  check!(2, "1.0000000000000000000000000000000015", [0x2ffe314dc6448d93, 0x38c15b0a00000002], F_20_00);
}

#[test]
fn _0153() {
  check!(3, "1.0000000000000000000000000000000015", [0x2ffe314dc6448d93, 0x38c15b0a00000001], F_20_00);
}

#[test]
fn _0154() {
  check!(4, "1.0000000000000000000000000000000015", [0x2ffe314dc6448d93, 0x38c15b0a00000002], F_20_00);
}

#[test]
fn _0155() {
  check!(1, "000.0", [0x303e000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0156() {
  check!(1, "0.0000000000000000000000000000000000000000000000000000000000000001001", [0x2fba000000000000, 0x00000000000003e9], F_00_00);
}

#[test]
fn _0157() {
  check!(1, "0.0000000000000000000000000000000000000000000000000000000000000001001", [0x2fba000000000000, 0x00000000000003e9], F_00_00);
}

#[test]
fn _0158() {
  check!(0, "0.", [0x3040000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0159() {
  check!(0, "1.", [0x3040000000000000, 0x0000000000000001], F_00_00);
}

#[test]
fn _0160() {
  check!(0, "1..", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0161() {
  check!(0, "1.0000000000000000000000000000000005", [0x2ffe314dc6448d93, 0x38c15b0a00000000], F_20_00);
}

#[test]
fn _0162() {
  check!(2, "1.0000000000000000000000000000000005", [0x2ffe314dc6448d93, 0x38c15b0a00000001], F_20_00);
}

#[test]
fn _0163() {
  check!(4, "1.0000000000000000000000000000000005", [0x2ffe314dc6448d93, 0x38c15b0a00000001], F_20_00);
}

#[test]
fn _0164() {
  check!(3, "1.0000000000000000000000000000000005", [0x2ffe314dc6448d93, 0x38c15b0a00000000], F_20_00);
}

#[test]
fn _0165() {
  check!(1, "1.0000000000000000000000000000000005", [0x2ffe314dc6448d93, 0x38c15b0a00000000], F_20_00);
}

#[test]
fn _0166() {
  check!(0, "1.00000000000000000000000000000000051", [0x2ffe314dc6448d93, 0x38c15b0a00000001], F_20_00);
}

#[test]
fn _0167() {
  check!(2, "1.00000000000000000000000000000000051", [0x2ffe314dc6448d93, 0x38c15b0a00000001], F_20_00);
}

#[test]
fn _0168() {
  check!(4, "1.00000000000000000000000000000000051", [0x2ffe314dc6448d93, 0x38c15b0a00000001], F_20_00);
}

#[test]
fn _0169() {
  check!(3, "1.00000000000000000000000000000000051", [0x2ffe314dc6448d93, 0x38c15b0a00000000], F_20_00);
}

#[test]
fn _0170() {
  check!(1, "1.00000000000000000000000000000000051", [0x2ffe314dc6448d93, 0x38c15b0a00000000], F_20_00);
}

#[test]
fn _0171() {
  check!(0, "1.9999999999999999999999999990000004999999999999999", [0x2ffe629b8c891b26, 0x7182b613fff0bdc0], F_20_00);
}

#[test]
fn _0172() {
  check!(2, "1.9999999999999999999999999990000004999999999999999", [0x2ffe629b8c891b26, 0x7182b613fff0bdc1], F_20_00);
}

#[test]
fn _0173() {
  check!(1, "1.9999999999999999999999999990000004999999999999999", [0x2ffe629b8c891b26, 0x7182b613fff0bdc0], F_20_00);
}

#[test]
fn _0174() {
  check!(4, "1.9999999999999999999999999990000004999999999999999", [0x2ffe629b8c891b26, 0x7182b613fff0bdc0], F_20_00);
}

#[test]
fn _0175() {
  check!(3, "1.9999999999999999999999999990000004999999999999999", [0x2ffe629b8c891b26, 0x7182b613fff0bdc0], F_20_00);
}

#[test]
fn _0176() {
  check!(0, "1.1E2", [0x3042000000000000, 0x000000000000000b], F_00_00);
}

#[test]
fn _0177() {
  check!(0, "1.1P2", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0178() {
  check!(0, "1.1EE", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0179() {
  check!(0, "1.1P-2", [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0180() {
  check!(0, "1.1E-2E", [0x303a000000000000, 0x000000000000000b], F_00_00);
}

#[test]
fn _0181() {
  check!(2, "+10000000000000000000000000000000000", [0x3042314DC6448D93, 0x38C15B0A00000000], F_00_00);
}

#[test]
fn _0182() {
  check!(0, "+6875897.879876979566658996675E6133", [0x5ffe0000de2c181d, 0x935716cd27b2e19e], F_00_00);
}

#[test]
fn _0183() {
  check!(0, "+89797785599559975.97E6141", [0x7800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0184() {
  check!(0, "-10011100011110110.1111010001E6132", [0xf800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0185() {
  check!(0, "-958896965.958776968777978E-6196", [0x8000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0186() {
  let input = format!("0.{}", "0".repeat(6177));
  check!(0, &input, [0x0000000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0187() {
  let input = "1".repeat(101);
  check!(0, &input, [0x30c636c831a180dc, 0x77f348b5c71c71c7], F_20_00);
}

#[test]
fn _0188() {
  let input = format!("{}0", "1".repeat(100));
  check!(0, &input, [0x30c636c831a180dc, 0x77f348b5c71c71c7], F_20_00);
}

#[test]
fn _0189() {
  let input = format!("0.{}", "1".repeat(100));
  check!(0, &input, [0x2FFC36C831A180DC, 0x77F348B5C71C71C7], F_20_00);
}

#[test]
fn _0190() {
  let input = format!("0.{}0", "1".repeat(99));
  check!(0, &input, [0x2FFC36C831A180DC, 0x77F348B5C71C71C7], F_20_00);
}
