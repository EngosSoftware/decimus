use super::*;
use decimus::{BidUint128, bid128_div, bid128_from_string};

macro_rules! check {
  ($rnd_mode:expr, $x:expr, $y:expr, $expected:expr, $expected_flags:expr) => {
    let mut actual_flags: IdecFlags = 0;
    let expected = BidUint128 { w: [$expected[1], $expected[0]] };
    let x = BidUint128 { w: [$x[1], $x[0]] };
    let y = BidUint128 { w: [$y[1], $y[0]] };
    let actual = bid128_div(x, y, $rnd_mode, &mut actual_flags);
    assert_eq!(expected, actual);
    assert_eq!($expected_flags, actual_flags, "Result flags error, expected = 0x{:02X}, actual = 0x{:02X}", $expected_flags, actual_flags);
  };
  ($rnd_mode:expr, $x_rnd_mode:expr, $y_rnd_mode:expr, $x:expr, $y:expr, $expected:expr, $expected_flags:expr, $x_flags:expr, $y_flags:expr) => {
    let mut actual_flags: IdecFlags = 0;
    let expected = BidUint128 { w: [$expected[1], $expected[0]] };
    let x = bid128_from_string($x, $x_rnd_mode, &mut actual_flags);
    assert_eq!($x_flags, actual_flags, "X flags error, expected = {:02X}, actual = {:02X}", $x_flags, actual_flags);
    let y = bid128_from_string($y, $y_rnd_mode, &mut actual_flags);
    assert_eq!($y_flags, actual_flags, "Y flags error, expected = {:02X}, actual = {:02X}", $y_flags, actual_flags);
    let actual = bid128_div(x, y, $rnd_mode, &mut actual_flags);
    assert_eq!(expected, actual);
    assert_eq!($expected_flags, actual_flags, "Result flags error, expected = 0x{:02X}, actual = 0x{:02X}", $expected_flags, actual_flags);
  };
}

#[test]
fn _0001() {
  check!(0, [0x3040000000000000, 0x0000000000000000], [0x3040000000000000, 0x0000000000000000], [0x7c00000000000000, 0x0000000000000000], F_01_00);
  //     to nearest even                          0                  /                      0                      =                  0   expected no exceptions
  //     ┬   ─┬────────────────────────────────────    ─┬────────────────────────────────────    ─┬────────────────────────────────────   ─┬─────
  //     │    └ argument x                              └ argument y                              └ expected result                        └ expected exception flags
  //     └ rounding mode
}

#[test]
fn _0002() {
  if cfg!(feature = "leave-trailing-zeros") {
    check!(0, [0x0000000000000000, 0xa4e0e3a5011dfdb3], [0x0000000000000000, 0x0000000000080000], [0x30186fb9db1af827, 0xa78b48ca1fae6286], F_00_00);
  } else {
    check!(0, [0x0000000000000000, 0xa4e0e3a5011dfdb3], [0x0000000000000000, 0x0000000000080000], [0x301a0b2c2f82b26a, 0x5d8dedadcff7d6a7], F_00_00);
  }
}

#[test]
fn _0003() {
  check!(0, [0x0000000000100000, 0x0000000010000000], [0x1f90612e201990ad, 0xffdfbfffffffd7ff], [0x105de3d7592c3b3a, 0x7670b2eea8801aad], F_20_00);
}

#[test]
fn _0004() {
  check!(0, [0xb0457561041f2a53, 0x8564461e0cc0c855], [0xb030000000000000, 0xdd77b6811f136c62], [0x302ee9f84615c4b2, 0xf831147e0365dac4], F_20_00);
}

#[test]
fn _0005() {
  check!(0, [0x05a98a9e27dde76e, 0xb75444501037a96c], [0x81da000000000000, 0xf13b929464b84377], [0xb3e8e304aa54ae4b, 0x0aef4174afa4aa19], F_20_00);
}

#[test]
fn _0006() {
  check!(0, [0x86bfaeda0f504aa9, 0xb1f3b568e25ce47d], [0x81da000000000000, 0xf13b929464b84377], [0x34fef7dcf7581919, 0xa4db66fcad2b4fc5], F_20_00);
}

#[test]
fn _0007() {
  check!(0, [0x3041bc61a052ace8, 0xa38ac28af05308d6], [0x3040000000000000, 0xf21b8a5c9e96e179], [0x301afeb904240fd9, 0x9273270a5e776dd7], F_20_00);
}

#[test]
fn _0008() {
  check!(0, [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x3040000000000000, 0x0000000000000001], F_00_00);
}

#[test]
fn _0009() {
  check!(0, [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x7800000000000000, 0x0000000000000000], F_04_00);
}

#[test]
fn _0010() {
  check!(0, [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x7c003fffffffffff, 0x38c15b08ffffffff], [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0011() {
  check!(0, [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x7c003fffffffffff, 0x38c15b0affffffff], [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0012() {
  check!(0, [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x3040000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0013() {
  check!(0, [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x7c00000000000000, 0x0000000000000000], F_01_00);
}

#[test]
fn _0014() {
  check!(0, [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x7c003fffffffffff, 0x38c15b08ffffffff], [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0015() {
  check!(0, [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x7c003fffffffffff, 0x38c15b0affffffff], [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0016() {
  check!(0, [0x0010000000000000, 0xe8f7eb63bf9ecd77], [0x1e71dbf7bdf3dbff, 0x8001200800040092], [0x118055bc43bd2e16, 0x858f61d773205eac], F_20_00);
}

#[test]
fn _0017() {
  check!(0, [0x0080000000004800, 0x0000000000000000], [0x828a1476c39f4cc1, 0xaf91720c11be3cfc], [0xade193e41a550ae7, 0x5f4e85982b7fd37d], F_20_00);
}

#[test]
fn _0018() {
  check!(0, [0x00849d8003d89321, 0x89a15d978953e007], [0xe0845520d80fd057, 0xe9b7df670ff7f7dd], [0xf800000000000000, 0x0000000000000000], F_04_00);
}

#[test]
fn _0019() {
  check!(0, [0x014287d9cebc1612, 0xe4fa290e7bfce8c7], [0x48c02c55900a6802, 0x12ae5ac68bf7cd7c], [0x0000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0020() {
  check!(0, [0x03d3f6a1a2be98fd, 0x05675c2721e98b1e], [0xdbe7fffb3bfe86cf, 0xffffeefbffffffef], [0x7c00000000000000, 0x0000000000000000], F_01_00);
}

#[test]
fn _0021() {
  check!(0, [0x0800001001200102, 0xfb7cbd9f6f7bbdfd], [0xbf9f287fd669e354, 0x4854bfb25e3c91df], [0x8000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0022() {
  check!(0, [0x10d580b2040866df, 0x5418d5cf1ad87f60], [0x47fc8a037c585432, 0xc56d954eeae84dbc], [0x0000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0023() {
  check!(0, [0x10e032c129c24730, 0x70dcf63765e91997], [0x410aa14c60018618, 0xffbdfefdf5efe79f], [0x0000000000000000, 0x000000075388ea15], F_30_00);
}

#[test]
fn _0024() {
  check!(0, [0x1aa6b19d7ae65af9, 0xff18372ff19f691f], [0xff3f733f8d177efa, 0xa73400501a380a30], [0xfc00000000000000, 0x0000000000000000], F_01_00);
}

#[test]
fn _0025() {
  check!(0, [0x1ea47d723a258782, 0x56018cb55e2192ef], [0xdaf1c0016d4dff25, 0xd69a2d819e52b625], [0x8000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0026() {
  check!(0, [0x2260210004208482, 0x38b7407aa02489a0], [0x0000000000000000, 0x0000000000000001], [0x52a0210004208482, 0x38b7407aa02489a0], F_00_00);
}

#[test]
fn _0027() {
  check!(0, [0x22a44b71ac3da011, 0x6eb5bd44422dea44], [0x1e4e000000000000, 0x0000000000000000], [0x7800000000000000, 0x0000000000000000], F_04_00);
}

#[test]
fn _0028() {
  let expected: BidArray = if cfg!(feature = "leave-trailing-zeros") {
    [0x58e0de49fa5bc685, 0xebe796a49d3b2000]
  } else {
    [0x58fa000000000018, 0x70e0e1d68b980c35]
  };
  check!(0, [0x28c44012080c0700, 0x0020000000000000], [0x0000000000000000, 0x0400000000000000], expected, F_00_00);
}

#[test]
fn _0029() {
  check!(0, [0x333f35ef131e4d4e, 0x28976a21359307a5], [0x9d5e1082c7650142, 0x4409851f173e99aa], [0xc5e05c8cfe06e9a5, 0x05fe87d0b754ee76], F_20_00);
}

#[test]
fn _0030() {
  check!(0, [0x33e63513719d9e30, 0x16fb2087e309f4e1], [0x93fec56dcf2fa39b, 0x009abaa41270f373], [0xcfe4848bd6adddef, 0x6e5040816616a3a0], F_20_00);
}

#[test]
fn _0031() {
  check!(0, [0x3a4c000000000000, 0x0000000000000000], [0xfc001e36a363a2ca, 0xfc5630c241fad77b], [0xfc001e36a363a2ca, 0xfc5630c241fad77b], F_00_00);
}

#[test]
fn _0032() {
  check!(0, [0x4000010040082800, 0x0400844998405000], [0x0000000000000000, 0x0000000000200000], [0x7800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0033() {
  check!(0, [0x40a46a3aae3793cf, 0xb072d3a233765dd6], [0xdfdab50a18d577e9, 0xd23d5a48965dd11f], [0x90c7214d4793b5e3, 0xf7dc4bf5d1b4c8a5], F_20_00);
}

#[test]
fn _0034() {
  check!(0, [0x4601b16cefe9effd, 0x31c9d1fb957f827d], [0xb4ece6c0fecdbce3, 0xd82f9af606824e86], [0xc1125c9b81bd7b51, 0xe01e5f389b29900f], F_20_00);
}

#[test]
fn _0035() {
  check!(0, [0x4f0b131761a5ebfe, 0xe17445ef98b870b9], [0x1c7fb95118314088, 0x0316f0c8237c5127], [0x7800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0036() {
  check!(0, [0x52b36974100f4928, 0x0f1bae6a614be06f], [0x1e304bbb94d4425c, 0x1b1a50112a2e1cb1], [0x7800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0037() {
  check!(0, [0x5530164e902e8437, 0xb288c0ee62a18589], [0x8ffac0e4688b1011, 0x8f918b0e12fb95cb], [0xf800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0038() {
  check!(0, [0x597e000000000000, 0x0000000000000000], [0x5bbc9676a11218e4, 0xc767d7f288836625], [0x2e02000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0039() {
  check!(0, [0x5f8cd8f1e56c6b3d, 0x541da77d81887813], [0x1c99d16b5a33a985, 0x2809b8660d9d69f0], [0x7800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0040() {
  check!(0, [0x7a3ffbef7ffbecfb, 0xbeebbdd55b51dadb], [0xfbffc997efebfdff, 0xd5bef7f7f6dffffe], [0x7c00000000000000, 0x0000000000000000], F_01_00);
}

#[test]
fn _0041() {
  check!(0, [0x7c003fffffffffff, 0x38c15b08ffffffff], [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0042() {
  check!(0, [0x7c003fffffffffff, 0x38c15b08ffffffff], [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0043() {
  check!(0, [0x7c003fffffffffff, 0x38c15b08ffffffff], [0x7c003fffffffffff, 0x38c15b08ffffffff], [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0044() {
  check!(0, [0x7c003fffffffffff, 0x38c15b08ffffffff], [0x7c003fffffffffff, 0x38c15b0affffffff], [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0045() {
  check!(0, [0x7c003fffffffffff, 0x38c15b0affffffff], [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0046() {
  check!(0, [0x7c003fffffffffff, 0x38c15b0affffffff], [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0047() {
  check!(0, [0x7c003fffffffffff, 0x38c15b0affffffff], [0x7c003fffffffffff, 0x38c15b08ffffffff], [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0048() {
  check!(0, [0x7c003fffffffffff, 0x38c15b0affffffff], [0x7c003fffffffffff, 0x38c15b0affffffff], [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0049() {
  check!(0, [0x806032960a142709, 0xd1a9eae619261fe6], [0xd747094621e87bdf, 0x42930ddffd2d2bb6], [0x0000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0050() {
  check!(0, [0x89859621903d1880, 0x43873b4ee0099f0d], [0x02861af92aa295f0, 0x4201fb87adf991d8], [0xb6fe4a3c5389a9e4, 0x31158e360d84b937], F_20_00);
}

#[test]
fn _0051() {
  check!(0, [0x89cf2ff12ec78d42, 0x42674691aeda0f44], [0x11d0582141400402, 0xfffffffffffffe7f], [0xa7fcaa09ef582623, 0xbe20e2e1d16ad69c], F_20_00);
}

#[test]
fn _0052() {
  check!(0, [0x8c40c590549957ea, 0x4a9a7cb1e9380813], [0x41153cd932ac2b46, 0x7ac11d6b17ae4462], [0x8000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0053() {
  check!(0, [0x8c750c6d27ff163d, 0xf024d4457485fa3c], [0x3cc5d0621a99f027, 0xeadaa3acfbc6eb50], [0x8000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0054() {
  check!(0, [0x8ccc407e66eb7261, 0xbf95795f91b67333], [0x2bfcea63f6e985b9, 0x385025a162db63fc], [0x90cc87a97793653f, 0x41c5b0843e759c4d], F_20_00);
}

#[test]
fn _0055() {
  check!(0, [0x8e2403cdcba75263, 0x0629b6f7a7010602], [0x5a3980ee7baa631d, 0x3708928990f473e4], [0x8000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0056() {
  check!(0, [0x9346134cb5d53576, 0xd5519b525d5ffa30], [0x88509674eb409001, 0xca9b988529db58b0], [0x3af23f3e6cf2fe9a, 0x91863d9c1fe83531], F_20_00);
}

#[test]
fn _0057() {
  check!(0, [0x9356239fd6f61f54, 0x3c82a573dd68c8f7], [0xd007306e51b05889, 0x7ada8064b6679c41], [0x0000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0058() {
  check!(0, [0x97e3407e35fe75ad, 0x38edea703e91a616], [0x4646000000000000, 0x0000000000000000], [0xf800000000000000, 0x0000000000000000], F_04_00);
}

#[test]
fn _0059() {
  check!(0, [0x9b5b494c1b0b648a, 0x37a1bb32e5a95fa9], [0xa93a0d7850c02072, 0xd381713d3707026a], [0x2220788822fcbd2b, 0xc4d2d1bab2fdc4f7], F_20_00);
}

#[test]
fn _0060() {
  check!(0, [0x9dac6392650e5f32, 0x6dc29520591ed6d2], [0x7e00000000000000, 0x0000000000000000], [0x7c00000000000000, 0x0000000000000000], F_01_00);
}

#[test]
fn _0061() {
  check!(0, [0xa6357f5a31dad8e5, 0xfb5417f4c2f2fba7], [0x9f3550d8712c4821, 0x2a40189484310e29], [0x36fe381c68fc8cc1, 0x098f25da42a2279e], F_20_00);
}

#[test]
fn _0062() {
  check!(0, [0xb01f00aa099e0920, 0x310b6ff9a1cf209c], [0x000360208d000480, 0xb164912945ad492e], [0xf800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0063() {
  check!(0, [0xb6b3957ffb4642e6, 0x113058313664c274], [0xfbdc6fdfbbe7df5b, 0x7ed77fb5f7778b7f], [0x0000000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0064() {
  check!(0, [0xca315abb5eb7928a, 0xf36c452c38ae05b0], [0x926e4674bba1c31c, 0xd8256ba008e56abd], [0x7800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0065() {
  check!(0, [0xce4eaaae52c725db, 0xa236e46671e9e4d1], [0x1f5d2bf1866d7bd9, 0xa8bed133ede1421e], [0xdeef188f6b4e4710, 0x5d84efb1a381eb09], F_20_00);
}

#[test]
fn _0066() {
  check!(0, [0xcf9fe06cec065c37, 0x1baf1e0586ff927d], [0x80c9b1ce835e3cb1, 0x9b9600b33d3c5041], [0x7800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0067() {
  check!(0, [0xd45a2d71f2fc5187, 0x26c22326f2005835], [0x102b679d08833f6d, 0x870cf5e9f920c0d5], [0xf800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0068() {
  check!(0, [0xdba033e002d88475, 0x7fde7d86b7ed34d2], [0x04673668347f5053, 0xac5aac9ee0505f1b], [0xf800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0069() {
  check!(0, [0xdd4fbf6532b84f73, 0x3c4c5763f08dca6b], [0xc8893309fd8b3667, 0x8a3434e6ff1c8ed9], [0x44c447d78c0580aa, 0x85f7db30b7388987], F_20_00);
}

#[test]
fn _0070() {
  check!(0, [0xe2bc9eff4d110e5d, 0xeed2aae2bc824d5f], [0x5b039e69d0c3c0b9, 0x0017bffdccdb1201], [0x8000000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0071() {
  check!(0, [0xf379c3f2a31b84ca, 0xafc0b172cc4a7017], [0x1670661017a53275, 0xd5b29af6ea63df75], [0xdffe000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0072() {
  check!(0, [0xf9fb5feffc7fffff, 0x01c92dcc0a4b9f23], [0xa545b0fe616ae5e9, 0x2a87318139631a90], [0x7800000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0073() {
  check!(0, [0xfe001c9bc3525d6a, 0xc0939fd3447b27a7], [0x8edc000000000000, 0x0000000000000000], [0xfc001c9bc3525d6a, 0xc0939fd3447b27a7], F_01_00);
}

#[test]
fn _0074() {
  check!(0, [0xfff7ffffffffffbf, 0x124ad21c0b932e7e], [0xc3a009ac4e75e090, 0xf4f576b2a2392f27], [0xfc00000000000000, 0x0000000000000000], F_01_00);
}

#[test]
fn _0075() {
  check!(0, 0, 0, "-0", "-0", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0076() {
  check!(0, 0, 0, "-0", "0", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0077() {
  check!(0, 0, 0, "0", "-0", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0078() {
  check!(0, 0, 0, "0", "SNaN", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0079() {
  let expected: BidArray = if cfg!(feature = "leave-trailing-zeros") {
    [0x4959d114e7d6ddad, 0x3852729436f20000]
  } else {
    [0x497a000000000000, 0x014f206c8d4f8525]
  };
  check!(0, 0, 0, "-9432976779742544.5E156", "-0.10E-3073", expected, F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0080() {
  let expected: BidArray = if cfg!(feature = "leave-trailing-zeros") {
    [0x5586314dc6448d93, 0x38c15b0a00000000]
  } else {
    [0x55c8000000000000, 0x0000000000000001]
  };
  check!(0, 0, 0, "+1000.E6069", "+0.010000000E1270", expected, F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0081() {
  check!(0, 0, 0, "-1.010E0", "+734.657E0", [0xaff843c84e0b05df, 0x35d5dd63a34a7f51], F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0082() {
  let expected: BidArray = if cfg!(feature = "leave-trailing-zeros") {
    [0xc0b431da00f0b67c, 0xcc47d93fb72c0000]
  } else {
    [0xc0d8000000000000, 0x000397996a664a43]
  };
  check!(0, 0, 0, "-1011110101011.011E4826", "+1000000000000.000E2687", expected, F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0083() {
  check!(0, 0, 0, "+1100101.01100E0", "-52.45339558946E0", [0xb00667678cd085a5, 0xdc88e14678177cab], F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0084() {
  check!(0, 0, 0, "-1101.0101100100011100E-4834", "+10000000.0E4733", [0x8000000000000000, 0x0000000000000000], F_30_00, F_00_00, F_00_00);
}

#[test]
fn _0085() {
  let expected: BidArray = if cfg!(feature = "leave-trailing-zeros") {
    [0x067c43caf09e42aa, 0x6e09dd2dc0000000]
  } else {
    [0x06b8000000000000, 0x000000000000055f]
  };
  check!(0, 0, 0, "+1.10E-4040", "+8.E1272", expected, F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0086() {
  check!(0, 0, 0, "-11101.0110010011100E0", "-0", [0x7800000000000000, 0x0000000000000000], F_04_00, F_00_00, F_00_00);
}

#[test]
fn _0087() {
  let expected: BidArray = if cfg!(feature = "leave-trailing-zeros") {
    [0x5fbe629b8c891b26, 0x7182b61400000000]
  } else {
    [0x5ffe000000000000, 0x0000000000000014]
  };
  check!(0, 0, 0, "1E+6109", "0.0005", expected, F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0088() {
  let expected: BidArray = if cfg!(feature = "leave-trailing-zeros") {
    [0x5ff8629b8c891b26, 0x7182b61400000000]
  } else {
    [0x5ffe00193e5939a0, 0x8ce9dbd480000000]
  };
  check!(0, 0, 0, "1E+6109", "5E-33", expected, F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0089() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") {
    [0x0000000000000000, 0x0000000000000001]
  } else {
    [0x0000000000000000, 0x0000000000000000]
  };
  check!(0, 0, 0, "1E-6176", "2", expected, F_30_00, F_00_00, F_00_00);
}

#[test]
fn _0090() {
  check!(0, 0, 0, "1E-6176", "8388608", [0x0000000000000000, 0x0000000000000000], F_30_00, F_00_00, F_00_00);
}

#[test]
fn _0091() {
  let expected: BidArray = if cfg!(feature = "leave-trailing-zeros") {
    [0x182b74a06d3f3d27, 0xd544374645b00000]
  } else {
    [0x1852000000000000, 0x000044bccbfce44b]
  };
  check!(0, 0, 0, "-755776668887.79E-3752", "-1.00E-691", expected, F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0092() {
  check!(0, 0, 0, "+8.888999998E-5822", "-10000000.000E5554", [0x8000000000000000, 0x0000000000000000], F_30_00, F_00_00, F_00_00);
}

#[test]
fn _0093() {
  let expected: BidArray = if cfg!(feature = "leave-trailing-zeros") {
    [0xbc97bb30e0dea07d, 0x7b7bf58720000000]
  } else {
    [0xbcce000000000000, 0x00000000000db752]
  };
  check!(0, 0, 0, "-898898.E5127", "+0.000000000000001000000E3535", expected, F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0094() {
  let expected: BidArray = if cfg!(feature = "leave-trailing-zeros") {
    [0x80c38a6e32246c99, 0xc60ad85000000000]
  } else {
    [0x8104000000000000, 0x0000000000000008]
  };
  check!(0, 0, 0, "+8.E-360", "-0.010E5688", expected, F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0095() {
  check!(0, 0, 0, "+9878987.5679766895E0", "0", [0x7800000000000000, 0x0000000000000000], F_04_00, F_00_00, F_00_00);
}

#[test]
fn _0096() {
  check!(0, 0, 0, "+98858.678996557986769E0", "-989.888889E0", [0xb001ec63b7d685d4, 0xe49c39a1b82c7906], F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0097() {
  check!(0, 0, 0, "+99.8998888898E0", "-55993.8675252E0", [0xaff857f6c8fae298, 0xe46f125bb81b162d], F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0098() {
  check!(0, 0, 0, "-0", "Infinity", [0x8000000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0099() {
  check!(0, 0, 0, "-Infinity", "-0", [0x7800000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0100() {
  check!(0, 0, 0, "-Infinity", "0", [0xf800000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0101() {
  check!(0, 0, 0, "-Infinity", "-Infinity", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0102() {
  check!(0, 0, 0, "-Infinity", "Infinity", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0103() {
  check!(0, 0, 0, "Infinity", "Infinity", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0104() {
  check!(0, 0, 0, "Infinity", "NaN", [0x7c00000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0105() {
  check!(0, 0, 0, "Infinity", "QNaN", [0x7c00000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0106() {
  check!(0, 0, 0, "Infinity", "SNaN", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0107() {
  check!(0, 0, 0, "QNaN", "-0", [0x7c00000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0108() {
  check!(0, 0, 0, "QNaN", "QNaN", [0x7c00000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0109() {
  check!(0, 0, 0, "NaN", "NaN", [0x7c00000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0110() {
  check!(0, 0, 0, "SNaN", "SNaN", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0111() {
  check!(1, [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x3040000000000000, 0x0000000000000001], F_00_00);
}

#[test]
fn _0112() {
  check!(1, [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x7800000000000000, 0x0000000000000000], F_04_00);
}

#[test]
fn _0113() {
  check!(1, [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x3040000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0114() {
  check!(1, [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x7c00000000000000, 0x0000000000000000], F_01_00);
}

#[test]
fn _0115() {
  check!(1, [0x10d580b2040866df, 0x5418d5cf1ad87f60], [0x47fc8a037c585432, 0xc56d954eeae84dbc], [0x0000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0116() {
  if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    check!(1, [0x1ea47d723a258782, 0x56018cb55e2192ef], [0xdaf1c0016d4dff25, 0xd69a2d819e52b625], [0x8000000000000000, 0x0000000000000000], F_30_00);
  } else {
    check!(1, [0x1ea47d723a258782, 0x56018cb55e2192ef], [0xdaf1c0016d4dff25, 0xd69a2d819e52b625], [0x8000000000000000, 0x0000000000000001], F_30_00);
  }
}

#[test]
fn _0117() {
  check!(1, [0x22a44b71ac3da011, 0x6eb5bd44422dea44], [0x1e4e000000000000, 0x0000000000000000], [0x7800000000000000, 0x0000000000000000], F_04_00);
}

#[test]
fn _0118() {
  check!(1, [0x333f35ef131e4d4e, 0x28976a21359307a5], [0x9d5e1082c7650142, 0x4409851f173e99aa], [0xc5e05c8cfe06e9a5, 0x05fe87d0b754ee76], F_20_00);
}

#[test]
fn _0119() {
  check!(1, [0x33e63513719d9e30, 0x16fb2087e309f4e1], [0x93fec56dcf2fa39b, 0x009abaa41270f373], [0xcfe4848bd6adddef, 0x6e5040816616a3a0], F_20_00);
}

#[test]
fn _0120() {
  check!(1, [0x3a4c000000000000, 0x0000000000000000], [0xfc001e36a363a2ca, 0xfc5630c241fad77b], [0xfc001e36a363a2ca, 0xfc5630c241fad77b], F_00_00);
}

#[test]
fn _0121() {
  if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    check!(1, [0x40a46a3aae3793cf, 0xb072d3a233765dd6], [0xdfdab50a18d577e9, 0xd23d5a48965dd11f], [0x90c7214d4793b5e3, 0xf7dc4bf5d1b4c8a5], F_20_00);
  } else {
    check!(1, [0x40a46a3aae3793cf, 0xb072d3a233765dd6], [0xdfdab50a18d577e9, 0xd23d5a48965dd11f], [0x90c7214d4793b5e3, 0xf7dc4bf5d1b4c8a6], F_20_00);
  }
}

#[test]
fn _0122() {
  check!(1, [0x4601b16cefe9effd, 0x31c9d1fb957f827d], [0xb4ece6c0fecdbce3, 0xd82f9af606824e86], [0xc1125c9b81bd7b51, 0xe01e5f389b29900f], F_20_00);
}

#[test]
fn _0123() {
  if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    check!(1, [0x4f0b131761a5ebfe, 0xe17445ef98b870b9], [0x1c7fb95118314088, 0x0316f0c8237c5127], [0x7800000000000000, 0x0000000000000000], F_28_00);
  } else {
    check!(1, [0x4f0b131761a5ebfe, 0xe17445ef98b870b9], [0x1c7fb95118314088, 0x0316f0c8237c5127], [0x5fffed09bead87c0, 0x378d8e63ffffffff], F_28_00);
  }
}

#[test]
fn _0124() {
  if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    check!(1, [0x52b36974100f4928, 0x0f1bae6a614be06f], [0x1e304bbb94d4425c, 0x1b1a50112a2e1cb1], [0x7800000000000000, 0x0000000000000000], F_28_00);
  } else {
    check!(1, [0x52b36974100f4928, 0x0f1bae6a614be06f], [0x1e304bbb94d4425c, 0x1b1a50112a2e1cb1], [0x5fffed09bead87c0, 0x378d8e63ffffffff], F_28_00);
  }
}

#[test]
fn _0125() {
  check!(1, [0x5530164e902e8437, 0xb288c0ee62a18589], [0x8ffac0e4688b1011, 0x8f918b0e12fb95cb], [0xf800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0126() {
  check!(1, [0x597e000000000000, 0x0000000000000000], [0x5bbc9676a11218e4, 0xc767d7f288836625], [0x2e02000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0127() {
  if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    check!(1, [0x5f8cd8f1e56c6b3d, 0x541da77d81887813], [0x1c99d16b5a33a985, 0x2809b8660d9d69f0], [0x7800000000000000, 0x0000000000000000], F_28_00);
  } else {
    check!(1, [0x5f8cd8f1e56c6b3d, 0x541da77d81887813], [0x1c99d16b5a33a985, 0x2809b8660d9d69f0], [0x5fffed09bead87c0, 0x378d8e63ffffffff], F_28_00);
  }
}

#[test]
fn _0128() {
  check!(1, [0x806032960a142709, 0xd1a9eae619261fe6], [0xd747094621e87bdf, 0x42930ddffd2d2bb6], [0x0000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0129() {
  check!(1, [0x89cf2ff12ec78d42, 0x42674691aeda0f44], [0x11d0582141400402, 0xfffffffffffffe7f], [0xa7fcaa09ef582623, 0xbe20e2e1d16ad69c], F_20_00);
}

#[test]
fn _0130() {
  if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    check!(1, [0x8c40c590549957ea, 0x4a9a7cb1e9380813], [0x41153cd932ac2b46, 0x7ac11d6b17ae4462], [0x8000000000000000, 0x0000000000000000], F_30_00);
  } else {
    check!(1, [0x8c40c590549957ea, 0x4a9a7cb1e9380813], [0x41153cd932ac2b46, 0x7ac11d6b17ae4462], [0x8000000000000000, 0x0000000000000001], F_30_00);
  }
}

#[test]
fn _0131() {
  if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    check!(1, [0x8c750c6d27ff163d, 0xf024d4457485fa3c], [0x3cc5d0621a99f027, 0xeadaa3acfbc6eb50], [0x8000000000000000, 0x0000000000000000], F_30_00);
  } else {
    check!(1, [0x8c750c6d27ff163d, 0xf024d4457485fa3c], [0x3cc5d0621a99f027, 0xeadaa3acfbc6eb50], [0x8000000000000000, 0x0000000000000001], F_30_00);
  }
}

#[test]
fn _0132() {
  check!(1, [0x8ccc407e66eb7261, 0xbf95795f91b67333], [0x2bfcea63f6e985b9, 0x385025a162db63fc], [0x90cc87a97793653f, 0x41c5b0843e759c4d], F_20_00);
}

#[test]
fn _0133() {
  if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    check!(1, [0x8e2403cdcba75263, 0x0629b6f7a7010602], [0x5a3980ee7baa631d, 0x3708928990f473e4], [0x8000000000000000, 0x0000000000000000], F_30_00);
  } else {
    check!(1, [0x8e2403cdcba75263, 0x0629b6f7a7010602], [0x5a3980ee7baa631d, 0x3708928990f473e4], [0x8000000000000000, 0x0000000000000001], F_30_00);
  }
}

#[test]
fn _0134() {
  check!(1, [0x9346134cb5d53576, 0xd5519b525d5ffa30], [0x88509674eb409001, 0xca9b988529db58b0], [0x3af23f3e6cf2fe9a, 0x91863d9c1fe83531], F_20_00);
}

#[test]
fn _0135() {
  check!(1, [0x9356239fd6f61f54, 0x3c82a573dd68c8f7], [0xd007306e51b05889, 0x7ada8064b6679c41], [0x0000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0136() {
  check!(1, [0x97e3407e35fe75ad, 0x38edea703e91a616], [0x4646000000000000, 0x0000000000000000], [0xf800000000000000, 0x0000000000000000], F_04_00);
}

#[test]
fn _0137() {
  if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    check!(1, [0x9b5b494c1b0b648a, 0x37a1bb32e5a95fa9], [0xa93a0d7850c02072, 0xd381713d3707026a], [0x2220788822fcbd2b, 0xc4d2d1bab2fdc4f7], F_20_00);
  } else {
    check!(1, [0x9b5b494c1b0b648a, 0x37a1bb32e5a95fa9], [0xa93a0d7850c02072, 0xd381713d3707026a], [0x2220788822fcbd2b, 0xc4d2d1bab2fdc4f6], F_20_00);
  }
}

#[test]
fn _0138() {
  check!(1, [0x9dac6392650e5f32, 0x6dc29520591ed6d2], [0x7c00000000000000, 0x0000000000000000], [0x7c00000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0139() {
  if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    check!(1, [0xca315abb5eb7928a, 0xf36c452c38ae05b0], [0x926e4674bba1c31c, 0xd8256ba008e56abd], [0x7800000000000000, 0x0000000000000000], F_28_00);
  } else {
    check!(1, [0xca315abb5eb7928a, 0xf36c452c38ae05b0], [0x926e4674bba1c31c, 0xd8256ba008e56abd], [0x5fffed09bead87c0, 0x378d8e63ffffffff], F_28_00);
  }
}

#[test]
fn _0140() {
  if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    check!(1, [0xce4eaaae52c725db, 0xa236e46671e9e4d1], [0x1f5d2bf1866d7bd9, 0xa8bed133ede1421e], [0xdeef188f6b4e4710, 0x5d84efb1a381eb09], F_20_00);
  } else {
    check!(1, [0xce4eaaae52c725db, 0xa236e46671e9e4d1], [0x1f5d2bf1866d7bd9, 0xa8bed133ede1421e], [0xdeef188f6b4e4710, 0x5d84efb1a381eb0a], F_20_00);
  }
}

#[test]
fn _0141() {
  if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    check!(1, [0xcf9fe06cec065c37, 0x1baf1e0586ff927d], [0x80c9b1ce835e3cb1, 0x9b9600b33d3c5041], [0x7800000000000000, 0x0000000000000000], F_28_00);
  } else {
    check!(1, [0xcf9fe06cec065c37, 0x1baf1e0586ff927d], [0x80c9b1ce835e3cb1, 0x9b9600b33d3c5041], [0x5fffed09bead87c0, 0x378d8e63ffffffff], F_28_00);
  }
}

#[test]
fn _0142() {
  check!(1, [0xd45a2d71f2fc5187, 0x26c22326f2005835], [0x102b679d08833f6d, 0x870cf5e9f920c0d5], [0xf800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0143() {
  check!(1, [0xdba033e002d88475, 0x7fde7d86b7ed34d2], [0x04673668347f5053, 0xac5aac9ee0505f1b], [0xf800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0144() {
  check!(1, [0xdd4fbf6532b84f73, 0x3c4c5763f08dca6b], [0xc8893309fd8b3667, 0x8a3434e6ff1c8ed9], [0x44c447d78c0580aa, 0x85f7db30b7388987], F_20_00);
}

#[test]
fn _0145() {
  check!(1, [0xfe001c9bc3525d6a, 0xc0939fd3447b27a7], [0x8edc000000000000, 0x0000000000000000], [0xfc001c9bc3525d6a, 0xc0939fd3447b27a7], F_01_00);
}

#[test]
fn _0146() {
  check!(1, 0, 0, "+99.8998888898E0", "-55993.8675252E0", [0xaff857f6c8fae298, 0xe46f125bb81b162d], F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0147() {
  check!(1, 0, 0, "-0", "-0", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0148() {
  check!(1, 0, 0, "-0", "0", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0149() {
  check!(1, 0, 0, "0", "-0", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0150() {
  check!(1, 0, 0, "-0", "Infinity", [0x8000000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0151() {
  check!(1, 0, 0, "0", "SNaN", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0152() {
  check!(1, 0, 0, "-1.010E0", "+734.657E0", [0xaff843c84e0b05df, 0x35d5dd63a34a7f51], F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0153() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xb00667678cd085a5, 0xdc88e14678177cab]
  } else {
    [0xb00667678cd085a5, 0xdc88e14678177cac]
  };
  check!(1, 0, 0, "+1100101.01100E0", "-52.45339558946E0", expected, F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0154() {
  check!(1, 0, 0, "-11101.0110010011100E0", "-0", [0x7800000000000000, 0x0000000000000000], F_04_00, F_00_00, F_00_00);
}

#[test]
fn _0155() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") {
    [0x0000000000000000, 0x0000000000000001]
  } else {
    [0x0000000000000000, 0x0000000000000000]
  };
  check!(1, 0, 0, "1E-6176", "2", expected, F_30_00, F_00_00, F_00_00);
}

#[test]
fn _0156() {
  check!(1, 0, 0, "+9878987.5679766895E0", "0", [0x7800000000000000, 0x0000000000000000], F_04_00, F_00_00, F_00_00);
}

#[test]
fn _0157() {
  check!(1, 0, 0, "+98858.678996557986769E0", "-989.888889E0", [0xb001ec63b7d685d4, 0xe49c39a1b82c7906], F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0158() {
  check!(1, 0, 0, "-Infinity", "-0", [0x7800000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0159() {
  check!(1, 0, 0, "-Infinity", "0", [0xf800000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0160() {
  check!(1, 0, 0, "-Infinity", "-Infinity", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0161() {
  check!(1, 0, 0, "-Infinity", "Infinity", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0162() {
  check!(1, 0, 0, "Infinity", "Infinity", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0163() {
  check!(1, 0, 0, "QNaN", "-0", [0x7c00000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0164() {
  check!(1, 0, 0, "QNaN", "QNaN", [0x7c00000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0165() {
  check!(2, [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x3040000000000000, 0x0000000000000001], F_00_00);
}

#[test]
fn _0166() {
  check!(2, [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x7800000000000000, 0x0000000000000000], F_04_00);
}

#[test]
fn _0167() {
  check!(2, [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x3040000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0168() {
  check!(2, [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x7c00000000000000, 0x0000000000000000], F_01_00);
}

#[test]
fn _0169() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0x0000000000000000, 0x0000000000000000]
  } else {
    [0x0000000000000000, 0x0000000000000001]
  };
  check!(2, [0x10d580b2040866df, 0x5418d5cf1ad87f60], [0x47fc8a037c585432, 0xc56d954eeae84dbc], expected, F_30_00);
}

#[test]
fn _0170() {
  check!(2, [0x1ea47d723a258782, 0x56018cb55e2192ef], [0xdaf1c0016d4dff25, 0xd69a2d819e52b625], [0x8000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0171() {
  check!(2, [0x22a44b71ac3da011, 0x6eb5bd44422dea44], [0x1e4e000000000000, 0x0000000000000000], [0x7800000000000000, 0x0000000000000000], F_04_00);
}

#[test]
fn _0172() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xc5e05c8cfe06e9a5, 0x05fe87d0b754ee76]
  } else {
    [0xc5e05c8cfe06e9a5, 0x05fe87d0b754ee75]
  };
  check!(2, [0x333f35ef131e4d4e, 0x28976a21359307a5], [0x9d5e1082c7650142, 0x4409851f173e99aa], expected, F_20_00);
}

#[test]
fn _0173() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xcfe4848bd6adddef, 0x6e5040816616a3a0]
  } else {
    [0xcfe4848bd6adddef, 0x6e5040816616a39f]
  };
  check!(2, [0x33e63513719d9e30, 0x16fb2087e309f4e1], [0x93fec56dcf2fa39b, 0x009abaa41270f373], expected, F_20_00);
}

#[test]
fn _0174() {
  check!(2, [0x3a4c000000000000, 0x0000000000000000], [0xfe001e36a363a2ca, 0xfc5630c241fad77b], [0xfc001e36a363a2ca, 0xfc5630c241fad77b], F_01_00);
}

#[test]
fn _0175() {
  check!(2, [0x40a46a3aae3793cf, 0xb072d3a233765dd6], [0xdfdab50a18d577e9, 0xd23d5a48965dd11f], [0x90c7214d4793b5e3, 0xf7dc4bf5d1b4c8a5], F_20_00);
}

#[test]
fn _0176() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xc1125c9b81bd7b51, 0xe01e5f389b29900f]
  } else {
    [0xc1125c9b81bd7b51, 0xe01e5f389b29900e]
  };
  check!(2, [0x4601b16cefe9effd, 0x31c9d1fb957f827d], [0xb4ece6c0fecdbce3, 0xd82f9af606824e86], expected, F_20_00);
}

#[test]
fn _0177() {
  check!(2, [0x4f0b131761a5ebfe, 0xe17445ef98b870b9], [0x1c7fb95118314088, 0x0316f0c8237c5127], [0x7800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0178() {
  check!(2, [0x52b36974100f4928, 0x0f1bae6a614be06f], [0x1e304bbb94d4425c, 0x1b1a50112a2e1cb1], [0x7800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0179() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xF800000000000000, 0x0000000000000000]
  } else {
    [0xdfffed09bead87c0, 0x378d8e63ffffffff]
  };
  check!(2, [0x5530164e902e8437, 0xb288c0ee62a18589], [0x8ffac0e4688b1011, 0x8f918b0e12fb95cb], expected, F_28_00);
}

#[test]
fn _0180() {
  check!(2, [0x597e000000000000, 0x0000000000000000], [0x5bbc9676a11218e4, 0xc767d7f288836625], [0x2e02000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0181() {
  check!(2, [0x5f8cd8f1e56c6b3d, 0x541da77d81887813], [0x1c99d16b5a33a985, 0x2809b8660d9d69f0], [0x7800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0182() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0x0000000000000000, 0x0000000000000000]
  } else {
    [0x0000000000000000, 0x0000000000000001]
  };
  check!(2, [0x806032960a142709, 0xd1a9eae619261fe6], [0xd747094621e87bdf, 0x42930ddffd2d2bb6], expected, F_30_00);
}

#[test]
fn _0183() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0x8000000000000000, 0x000000005cb31723]
  } else {
    [0x8000000000000000, 0x000000005cb31722]
  };
  check!(2, [0x8210008002958108, 0x0000000000000000], [0x32394184dc68c8c9, 0xdf7fe7fcf7ffbba7], expected, F_30_00);
}

#[test]
fn _0184() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xa7fcaa09ef582623, 0xbe20e2e1d16ad69c]
  } else {
    [0xa7fcaa09ef582623, 0xbe20e2e1d16ad69b]
  };
  check!(2, [0x89cf2ff12ec78d42, 0x42674691aeda0f44], [0x11d0582141400402, 0xfffffffffffffe7f], expected, F_20_00);
}

#[test]
fn _0185() {
  check!(2, [0x8c40c590549957ea, 0x4a9a7cb1e9380813], [0x41153cd932ac2b46, 0x7ac11d6b17ae4462], [0x8000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0186() {
  check!(2, [0x8c750c6d27ff163d, 0xf024d4457485fa3c], [0x3cc5d0621a99f027, 0xeadaa3acfbc6eb50], [0x8000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0187() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0x90cc87a97793653f, 0x41c5b0843e759c4d]
  } else {
    [0x90cc87a97793653f, 0x41c5b0843e759c4c]
  };
  check!(2, [0x8ccc407e66eb7261, 0xbf95795f91b67333], [0x2bfcea63f6e985b9, 0x385025a162db63fc], expected, F_20_00);
}

#[test]
fn _0188() {
  check!(2, [0x8e2403cdcba75263, 0x0629b6f7a7010602], [0x5a3980ee7baa631d, 0x3708928990f473e4], [0x8000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0189() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0x3af23f3e6cf2fe9a, 0x91863d9c1fe83531]
  } else {
    [0x3af23f3e6cf2fe9a, 0x91863d9c1fe83532]
  };
  check!(2, [0x9346134cb5d53576, 0xd5519b525d5ffa30], [0x88509674eb409001, 0xca9b988529db58b0], expected, F_20_00);
}

#[test]
fn _0190() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0x0000000000000000, 0x0000000000000000]
  } else {
    [0x0000000000000000, 0x0000000000000001]
  };
  check!(2, [0x9356239fd6f61f54, 0x3c82a573dd68c8f7], [0xd007306e51b05889, 0x7ada8064b6679c41], expected, F_30_00);
}

#[test]
fn _0191() {
  check!(2, [0x97e3407e35fe75ad, 0x38edea703e91a616], [0x4646000000000000, 0x0000000000000000], [0xf800000000000000, 0x0000000000000000], F_04_00);
}

#[test]
fn _0192() {
  check!(2, [0x9b5b494c1b0b648a, 0x37a1bb32e5a95fa9], [0xa93a0d7850c02072, 0xd381713d3707026a], [0x2220788822fcbd2b, 0xc4d2d1bab2fdc4f7], F_20_00);
}

#[test]
fn _0193() {
  check!(2, [0x9dac6392650e5f32, 0x6dc29520591ed6d2], [0x7e00000000000000, 0x0000000000000000], [0x7c00000000000000, 0x0000000000000000], F_01_00);
}

#[test]
fn _0194() {
  check!(2, [0xca315abb5eb7928a, 0xf36c452c38ae05b0], [0x926e4674bba1c31c, 0xd8256ba008e56abd], [0x7800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0195() {
  check!(2, [0xce4eaaae52c725db, 0xa236e46671e9e4d1], [0x1f5d2bf1866d7bd9, 0xa8bed133ede1421e], [0xdeef188f6b4e4710, 0x5d84efb1a381eb09], F_20_00);
}

#[test]
fn _0196() {
  check!(2, [0xcf9fe06cec065c37, 0x1baf1e0586ff927d], [0x80c9b1ce835e3cb1, 0x9b9600b33d3c5041], [0x7800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0197() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xF800000000000000, 0x0000000000000000]
  } else {
    [0xdfffed09bead87c0, 0x378d8e63ffffffff]
  };
  check!(2, [0xd45a2d71f2fc5187, 0x26c22326f2005835], [0x102b679d08833f6d, 0x870cf5e9f920c0d5], expected, F_28_00);
}

#[test]
fn _0198() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xF800000000000000, 0x0000000000000000]
  } else {
    [0xdfffed09bead87c0, 0x378d8e63ffffffff]
  };
  check!(2, [0xdba033e002d88475, 0x7fde7d86b7ed34d2], [0x04673668347f5053, 0xac5aac9ee0505f1b], expected, F_28_00);
}

#[test]
fn _0199() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0x44c447d78c0580aa, 0x85f7db30b7388987]
  } else {
    [0x44c447d78c0580aa, 0x85f7db30b7388988]
  };
  check!(2, [0xdd4fbf6532b84f73, 0x3c4c5763f08dca6b], [0xc8893309fd8b3667, 0x8a3434e6ff1c8ed9], expected, F_20_00);
}

#[test]
fn _0200() {
  check!(2, [0xfe001c9bc3525d6a, 0xc0939fd3447b27a7], [0x8edc000000000000, 0x0000000000000000], [0xfc001c9bc3525d6a, 0xc0939fd3447b27a7], F_01_00);
}

#[test]
fn _0201() {
  check!(2, 0, 0, "-0", "-0", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0202() {
  check!(2, 0, 0, "-0", "0", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0203() {
  check!(2, 0, 0, "0", "-0", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0204() {
  check!(2, 0, 0, "-0", "Infinity", [0x8000000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0205() {
  check!(2, 0, 0, "0", "SNaN", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0206() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xaff843c84e0b05df, 0x35d5dd63a34a7f51]
  } else {
    [0xaff843c84e0b05df, 0x35d5dd63a34a7f50]
  };
  check!(2, 0, 0, "-1.010E0", "+734.657E0", expected, F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0207() {
  check!(2, 0, 0, "+1100101.01100E0", "-52.45339558946E0", [0xb00667678cd085a5, 0xdc88e14678177cab], F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0208() {
  check!(2, 0, 0, "-11101.0110010011100E0", "-0", [0x7800000000000000, 0x0000000000000000], F_04_00, F_00_00, F_00_00);
}

#[test]
fn _0209() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest") && !cfg!(feature = "ieee-round-nearest-ties-away") {
    [0x0000000000000000, 0x0000000000000000]
  } else {
    [0x0000000000000000, 0x0000000000000001]
  };
  check!(2, 0, 0, "1E-6176", "2", expected, F_30_00, F_00_00, F_00_00);
}

#[test]
fn _0210() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") {
    [0x8000000000000000, 0x0000000000000001]
  } else {
    [0x8000000000000000, 0x0000000000000000]
  };
  check!(2, 0, 0, "1E-6176", "-2", expected, F_30_00, F_00_00, F_00_00);
}

#[test]
fn _0211() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0x0000000000000000, 0x0000000000000000]
  } else {
    [0x0000000000000000, 0x0000000000000001]
  };
  check!(2, 0, 0, "1E-6176", "4294967296", expected, F_30_00, F_00_00, F_00_00);
}

#[test]
fn _0212() {
  check!(2, 0, 0, "+9878987.5679766895E0", "0", [0x7800000000000000, 0x0000000000000000], F_04_00, F_00_00, F_00_00);
}

#[test]
fn _0213() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xb001ec63b7d685d4, 0xe49c39a1b82c7906]
  } else {
    [0xb001ec63b7d685d4, 0xe49c39a1b82c7905]
  };
  check!(2, 0, 0, "+98858.678996557986769E0", "-989.888889E0", expected, F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0214() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xaff857f6c8fae298, 0xe46f125bb81b162d]
  } else {
    [0xaff857f6c8fae298, 0xe46f125bb81b162c]
  };
  check!(2, 0, 0, "+99.8998888898E0", "-55993.8675252E0", expected, F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0215() {
  check!(2, 0, 0, "-Infinity", "-0", [0x7800000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0216() {
  check!(2, 0, 0, "-Infinity", "0", [0xf800000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0217() {
  check!(2, 0, 0, "-Infinity", "-Infinity", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0218() {
  check!(2, 0, 0, "-Infinity", "Infinity", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0219() {
  check!(2, 0, 0, "Infinity", "Infinity", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0220() {
  check!(2, 0, 0, "QNaN", "-0", [0x7c00000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0221() {
  check!(2, 0, 0, "QNaN", "QNaN", [0x7c00000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0222() {
  check!(3, [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x3040000000000000, 0x0000000000000001], F_00_00);
}

#[test]
fn _0223() {
  check!(3, [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x7800000000000000, 0x0000000000000000], F_04_00);
}

#[test]
fn _0224() {
  check!(3, [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x3040000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0225() {
  check!(3, [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x7c00000000000000, 0x0000000000000000], F_01_00);
}

#[test]
fn _0226() {
  check!(3, [0x10d580b2040866df, 0x5418d5cf1ad87f60], [0x47fc8a037c585432, 0xc56d954eeae84dbc], [0x0000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0227() {
  check!(3, [0x1ea47d723a258782, 0x56018cb55e2192ef], [0xdaf1c0016d4dff25, 0xd69a2d819e52b625], [0x8000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0228() {
  check!(3, [0x22a44b71ac3da011, 0x6eb5bd44422dea44], [0x1e4e000000000000, 0x0000000000000000], [0x7800000000000000, 0x0000000000000000], F_04_00);
}

#[test]
fn _0229() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xc5e05c8cfe06e9a5, 0x05fe87d0b754ee76]
  } else {
    [0xc5e05c8cfe06e9a5, 0x05fe87d0b754ee75]
  };
  check!(3, [0x333f35ef131e4d4e, 0x28976a21359307a5], [0x9d5e1082c7650142, 0x4409851f173e99aa], expected, F_20_00);
}

#[test]
fn _0230() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xcfe4848bd6adddef, 0x6e5040816616a3a0]
  } else {
    [0xcfe4848bd6adddef, 0x6e5040816616a39f]
  };
  check!(3, [0x33e63513719d9e30, 0x16fb2087e309f4e1], [0x93fec56dcf2fa39b, 0x009abaa41270f373], expected, F_20_00);
}

#[test]
fn _0231() {
  check!(3, [0x3a4c000000000000, 0x0000000000000000], [0xfe001e36a363a2ca, 0xfc5630c241fad77b], [0xfc001e36a363a2ca, 0xfc5630c241fad77b], F_01_00);
}

#[test]
fn _0232() {
  check!(3, [0x40a46a3aae3793cf, 0xb072d3a233765dd6], [0xdfdab50a18d577e9, 0xd23d5a48965dd11f], [0x90c7214d4793b5e3, 0xf7dc4bf5d1b4c8a5], F_20_00);
}

#[test]
fn _0233() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xc1125c9b81bd7b51, 0xe01e5f389b29900f]
  } else {
    [0xc1125c9b81bd7b51, 0xe01e5f389b29900e]
  };
  check!(3, [0x4601b16cefe9effd, 0x31c9d1fb957f827d], [0xb4ece6c0fecdbce3, 0xd82f9af606824e86], expected, F_20_00);
}

#[test]
fn _0234() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0x7800000000000000, 0x0000000000000000]
  } else {
    [0x5fffed09bead87c0, 0x378d8e63ffffffff]
  };
  check!(3, [0x4f0b131761a5ebfe, 0xe17445ef98b870b9], [0x1c7fb95118314088, 0x0316f0c8237c5127], expected, F_28_00);
}

#[test]
fn _0235() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0x7800000000000000, 0x0000000000000000]
  } else {
    [0x5fffed09bead87c0, 0x378d8e63ffffffff]
  };
  check!(3, [0x52b36974100f4928, 0x0f1bae6a614be06f], [0x1e304bbb94d4425c, 0x1b1a50112a2e1cb1], expected, F_28_00);
}

#[test]
fn _0236() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xF800000000000000, 0x0000000000000000]
  } else {
    [0xdfffed09bead87c0, 0x378d8e63ffffffff]
  };
  check!(3, [0x5530164e902e8437, 0xb288c0ee62a18589], [0x8ffac0e4688b1011, 0x8f918b0e12fb95cb], expected, F_28_00);
}

#[test]
fn _0237() {
  check!(3, [0x597e000000000000, 0x0000000000000000], [0x5bbc9676a11218e4, 0xc767d7f288836625], [0x2e02000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0238() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0x7800000000000000, 0x0000000000000000]
  } else {
    [0x5fffed09bead87c0, 0x378d8e63ffffffff]
  };
  check!(3, [0x5f8cd8f1e56c6b3d, 0x541da77d81887813], [0x1c99d16b5a33a985, 0x2809b8660d9d69f0], expected, F_28_00);
}

#[test]
fn _0239() {
  check!(3, [0x806032960a142709, 0xd1a9eae619261fe6], [0xd747094621e87bdf, 0x42930ddffd2d2bb6], [0x0000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0240() {
  check!(3, [0x8c40c590549957ea, 0x4a9a7cb1e9380813], [0x41153cd932ac2b46, 0x7ac11d6b17ae4462], [0x8000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0241() {
  check!(3, [0x8c750c6d27ff163d, 0xf024d4457485fa3c], [0x3cc5d0621a99f027, 0xeadaa3acfbc6eb50], [0x8000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0242() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0x90cc87a97793653f, 0x41c5b0843e759c4d]
  } else {
    [0x90cc87a97793653f, 0x41c5b0843e759c4c]
  };
  check!(3, [0x8ccc407e66eb7261, 0xbf95795f91b67333], [0x2bfcea63f6e985b9, 0x385025a162db63fc], expected, F_20_00);
}

#[test]
fn _0243() {
  check!(3, [0x8e2403cdcba75263, 0x0629b6f7a7010602], [0x5a3980ee7baa631d, 0x3708928990f473e4], [0x8000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0244() {
  check!(3, [0x9346134cb5d53576, 0xd5519b525d5ffa30], [0x88509674eb409001, 0xca9b988529db58b0], [0x3af23f3e6cf2fe9a, 0x91863d9c1fe83531], F_20_00);
}

#[test]
fn _0245() {
  check!(3, [0x9356239fd6f61f54, 0x3c82a573dd68c8f7], [0xd007306e51b05889, 0x7ada8064b6679c41], [0x0000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0246() {
  check!(3, [0x97e3407e35fe75ad, 0x38edea703e91a616], [0x4646000000000000, 0x0000000000000000], [0xf800000000000000, 0x0000000000000000], F_04_00);
}

#[test]
fn _0247() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0x2220788822fcbd2b, 0xc4d2d1bab2fdc4f7]
  } else {
    [0x2220788822fcbd2b, 0xc4d2d1bab2fdc4f6]
  };
  check!(3, [0x9b5b494c1b0b648a, 0x37a1bb32e5a95fa9], [0xa93a0d7850c02072, 0xd381713d3707026a], expected, F_20_00);
}

#[test]
fn _0248() {
  check!(3, [0x9dac6392650e5f32, 0x6dc29520591ed6d2], [0x7e00000000000000, 0x0000000000000000], [0x7c00000000000000, 0x0000000000000000], F_01_00);
}

#[test]
fn _0249() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0x7800000000000000, 0x0000000000000000]
  } else {
    [0x5fffed09bead87c0, 0x378d8e63ffffffff]
  };
  check!(3, [0xca315abb5eb7928a, 0xf36c452c38ae05b0], [0x926e4674bba1c31c, 0xd8256ba008e56abd], expected, F_28_00);
}

#[test]
fn _0250() {
  check!(3, [0xce4eaaae52c725db, 0xa236e46671e9e4d1], [0x1f5d2bf1866d7bd9, 0xa8bed133ede1421e], [0xdeef188f6b4e4710, 0x5d84efb1a381eb09], F_20_00);
}

#[test]
fn _0251() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0x7800000000000000, 0x0000000000000000]
  } else {
    [0x5fffed09bead87c0, 0x378d8e63ffffffff]
  };
  check!(3, [0xcf9fe06cec065c37, 0x1baf1e0586ff927d], [0x80c9b1ce835e3cb1, 0x9b9600b33d3c5041], expected, F_28_00);
}

#[test]
fn _0252() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xF800000000000000, 0x0000000000000000]
  } else {
    [0xdfffed09bead87c0, 0x378d8e63ffffffff]
  };
  check!(3, [0xd45a2d71f2fc5187, 0x26c22326f2005835], [0x102b679d08833f6d, 0x870cf5e9f920c0d5], expected, F_28_00);
}

#[test]
fn _0253() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xF800000000000000, 0x0000000000000000]
  } else {
    [0xdfffed09bead87c0, 0x378d8e63ffffffff]
  };
  check!(3, [0xdba033e002d88475, 0x7fde7d86b7ed34d2], [0x04673668347f5053, 0xac5aac9ee0505f1b], expected, F_28_00);
}

#[test]
fn _0254() {
  check!(3, [0xdd4fbf6532b84f73, 0x3c4c5763f08dca6b], [0xc8893309fd8b3667, 0x8a3434e6ff1c8ed9], [0x44c447d78c0580aa, 0x85f7db30b7388987], F_20_00);
}

#[test]
fn _0255() {
  check!(3, [0xfe001c9bc3525d6a, 0xc0939fd3447b27a7], [0x8edc000000000000, 0x0000000000000000], [0xfc001c9bc3525d6a, 0xc0939fd3447b27a7], F_01_00);
}

#[test]
fn _0256() {
  check!(3, 0, 0, "-0", "-0", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0257() {
  check!(3, 0, 0, "-0", "0", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0258() {
  check!(3, 0, 0, "0", "-0", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0259() {
  check!(3, 0, 0, "-0", "Infinity", [0x8000000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0260() {
  check!(3, 0, 0, "0", "SNaN", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0261() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xaff843c84e0b05df, 0x35d5dd63a34a7f51]
  } else {
    [0xaff843c84e0b05df, 0x35d5dd63a34a7f50]
  };
  check!(3, 0, 0, "-1.010E0", "+734.657E0", expected, F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0262() {
  check!(3, 0, 0, "+1100101.01100E0", "-52.45339558946E0", [0xb00667678cd085a5, 0xdc88e14678177cab], F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0263() {
  check!(3, 0, 0, "-11101.0110010011100E0", "-0", [0x7800000000000000, 0x0000000000000000], F_04_00, F_00_00, F_00_00);
}

#[test]
fn _0264() {
  check!(3, 0, 0, "+9878987.5679766895E0", "0", [0x7800000000000000, 0x0000000000000000], F_04_00, F_00_00, F_00_00);
}

#[test]
fn _0265() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xb001ec63b7d685d4, 0xe49c39a1b82c7906]
  } else {
    [0xb001ec63b7d685d4, 0xe49c39a1b82c7905]
  };
  check!(3, 0, 0, "+98858.678996557986769E0", "-989.888889E0", expected, F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0266() {
  let expected: BidArray = if cfg!(feature = "ieee-round-nearest-ties-away") || cfg!(feature = "ieee-round-nearest") {
    [0xaff857f6c8fae298, 0xe46f125bb81b162d]
  } else {
    [0xaff857f6c8fae298, 0xe46f125bb81b162c]
  };
  check!(3, 0, 0, "+99.8998888898E0", "-55993.8675252E0", expected, F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0267() {
  check!(3, 0, 0, "-Infinity", "-0", [0x7800000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0268() {
  check!(3, 0, 0, "-Infinity", "0", [0xf800000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0269() {
  check!(3, 0, 0, "-Infinity", "-Infinity", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0270() {
  check!(3, 0, 0, "-Infinity", "Infinity", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0271() {
  check!(3, 0, 0, "Infinity", "Infinity", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0272() {
  check!(3, 0, 0, "QNaN", "-0", [0x7c00000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0273() {
  check!(3, 0, 0, "QNaN", "QNaN", [0x7c00000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0274() {
  check!(4, [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x3040000000000000, 0x0000000000000001], F_00_00);
}

#[test]
fn _0275() {
  check!(4, [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x7800000000000000, 0x0000000000000000], F_04_00);
}

#[test]
fn _0276() {
  check!(4, [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x0001ed09bead87c0, 0x378d8e62ffffffff], [0x3040000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0277() {
  check!(4, [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x0001ed09bead87c0, 0x378d8e64ffffffff], [0x7c00000000000000, 0x0000000000000000], F_01_00);
}

#[test]
fn _0278() {
  check!(4, [0x10d580b2040866df, 0x5418d5cf1ad87f60], [0x47fc8a037c585432, 0xc56d954eeae84dbc], [0x0000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0279() {
  check!(4, [0x1ea47d723a258782, 0x56018cb55e2192ef], [0xdaf1c0016d4dff25, 0xd69a2d819e52b625], [0x8000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0280() {
  check!(4, [0x22a44b71ac3da011, 0x6eb5bd44422dea44], [0x1e4e000000000000, 0x0000000000000000], [0x7800000000000000, 0x0000000000000000], F_04_00);
}

#[test]
fn _0281() {
  check!(4, [0x333f35ef131e4d4e, 0x28976a21359307a5], [0x9d5e1082c7650142, 0x4409851f173e99aa], [0xc5e05c8cfe06e9a5, 0x05fe87d0b754ee76], F_20_00);
}

#[test]
fn _0282() {
  check!(4, [0x33e63513719d9e30, 0x16fb2087e309f4e1], [0x93fec56dcf2fa39b, 0x009abaa41270f373], [0xcfe4848bd6adddef, 0x6e5040816616a3a0], F_20_00);
}

#[test]
fn _0283() {
  check!(4, [0x3a4c000000000000, 0x0000000000000000], [0xfe001e36a363a2ca, 0xfc5630c241fad77b], [0xfc001e36a363a2ca, 0xfc5630c241fad77b], F_01_00);
}

#[test]
fn _0284() {
  check!(4, [0x40a46a3aae3793cf, 0xb072d3a233765dd6], [0xdfdab50a18d577e9, 0xd23d5a48965dd11f], [0x90c7214d4793b5e3, 0xf7dc4bf5d1b4c8a5], F_20_00);
}

#[test]
fn _0285() {
  check!(4, [0x4601b16cefe9effd, 0x31c9d1fb957f827d], [0xb4ece6c0fecdbce3, 0xd82f9af606824e86], [0xc1125c9b81bd7b51, 0xe01e5f389b29900f], F_20_00);
}

#[test]
fn _0286() {
  check!(4, [0x4f0b131761a5ebfe, 0xe17445ef98b870b9], [0x1c7fb95118314088, 0x0316f0c8237c5127], [0x7800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0287() {
  check!(4, [0x52b36974100f4928, 0x0f1bae6a614be06f], [0x1e304bbb94d4425c, 0x1b1a50112a2e1cb1], [0x7800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0288() {
  check!(4, [0x5530164e902e8437, 0xb288c0ee62a18589], [0x8ffac0e4688b1011, 0x8f918b0e12fb95cb], [0xf800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0289() {
  check!(4, [0x597e000000000000, 0x0000000000000000], [0x5bbc9676a11218e4, 0xc767d7f288836625], [0x2e02000000000000, 0x0000000000000000], F_00_00);
}

#[test]
fn _0290() {
  check!(4, [0x5f8cd8f1e56c6b3d, 0x541da77d81887813], [0x1c99d16b5a33a985, 0x2809b8660d9d69f0], [0x7800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0291() {
  check!(4, [0x806032960a142709, 0xd1a9eae619261fe6], [0xd747094621e87bdf, 0x42930ddffd2d2bb6], [0x0000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0292() {
  check!(4, [0x89cf2ff12ec78d42, 0x42674691aeda0f44], [0x11d0582141400402, 0xfffffffffffffe7f], [0xa7fcaa09ef582623, 0xbe20e2e1d16ad69c], F_20_00);
}

#[test]
fn _0293() {
  check!(4, [0x8c40c590549957ea, 0x4a9a7cb1e9380813], [0x41153cd932ac2b46, 0x7ac11d6b17ae4462], [0x8000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0294() {
  check!(4, [0x8c750c6d27ff163d, 0xf024d4457485fa3c], [0x3cc5d0621a99f027, 0xeadaa3acfbc6eb50], [0x8000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0295() {
  check!(4, [0x8ccc407e66eb7261, 0xbf95795f91b67333], [0x2bfcea63f6e985b9, 0x385025a162db63fc], [0x90cc87a97793653f, 0x41c5b0843e759c4d], F_20_00);
}

#[test]
fn _0296() {
  check!(4, [0x8e2403cdcba75263, 0x0629b6f7a7010602], [0x5a3980ee7baa631d, 0x3708928990f473e4], [0x8000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0297() {
  check!(4, [0x9346134cb5d53576, 0xd5519b525d5ffa30], [0x88509674eb409001, 0xca9b988529db58b0], [0x3af23f3e6cf2fe9a, 0x91863d9c1fe83531], F_20_00);
}

#[test]
fn _0298() {
  check!(4, [0x9356239fd6f61f54, 0x3c82a573dd68c8f7], [0xd007306e51b05889, 0x7ada8064b6679c41], [0x0000000000000000, 0x0000000000000000], F_30_00);
}

#[test]
fn _0299() {
  check!(4, [0x97e3407e35fe75ad, 0x38edea703e91a616], [0x4646000000000000, 0x0000000000000000], [0xf800000000000000, 0x0000000000000000], F_04_00);
}

#[test]
fn _0300() {
  check!(4, [0x9b5b494c1b0b648a, 0x37a1bb32e5a95fa9], [0xa93a0d7850c02072, 0xd381713d3707026a], [0x2220788822fcbd2b, 0xc4d2d1bab2fdc4f7], F_20_00);
}

#[test]
fn _0301() {
  check!(4, [0x9dac6392650e5f32, 0x6dc29520591ed6d2], [0x7e00000000000000, 0x0000000000000000], [0x7c00000000000000, 0x0000000000000000], F_01_00);
}

#[test]
fn _0302() {
  check!(4, [0xca315abb5eb7928a, 0xf36c452c38ae05b0], [0x926e4674bba1c31c, 0xd8256ba008e56abd], [0x7800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0303() {
  check!(4, [0xce4eaaae52c725db, 0xa236e46671e9e4d1], [0x1f5d2bf1866d7bd9, 0xa8bed133ede1421e], [0xdeef188f6b4e4710, 0x5d84efb1a381eb09], F_20_00);
}

#[test]
fn _0304() {
  check!(4, [0xcf9fe06cec065c37, 0x1baf1e0586ff927d], [0x80c9b1ce835e3cb1, 0x9b9600b33d3c5041], [0x7800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0305() {
  check!(4, [0xd45a2d71f2fc5187, 0x26c22326f2005835], [0x102b679d08833f6d, 0x870cf5e9f920c0d5], [0xf800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0306() {
  check!(4, [0xdba033e002d88475, 0x7fde7d86b7ed34d2], [0x04673668347f5053, 0xac5aac9ee0505f1b], [0xf800000000000000, 0x0000000000000000], F_28_00);
}

#[test]
fn _0307() {
  check!(4, [0xdd4fbf6532b84f73, 0x3c4c5763f08dca6b], [0xc8893309fd8b3667, 0x8a3434e6ff1c8ed9], [0x44c447d78c0580aa, 0x85f7db30b7388987], F_20_00);
}

#[test]
fn _0308() {
  check!(4, [0xfe001c9bc3525d6a, 0xc0939fd3447b27a7], [0x8edc000000000000, 0x0000000000000000], [0xfc001c9bc3525d6a, 0xc0939fd3447b27a7], F_01_00);
}

#[test]
fn _0309() {
  check!(4, 0, 0, "-0", "-0", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0310() {
  check!(4, 0, 0, "-0", "0", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0311() {
  check!(4, 0, 0, "0", "-0", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0312() {
  check!(4, 0, 0, "-0", "Infinity", [0x8000000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0313() {
  check!(4, 0, 0, "0", "SNaN", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0314() {
  check!(4, 0, 0, "-1.010E0", "+734.657E0", [0xaff843c84e0b05df, 0x35d5dd63a34a7f51], F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0315() {
  check!(4, 0, 0, "+1100101.01100E0", "-52.45339558946E0", [0xb00667678cd085a5, 0xdc88e14678177cab], F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0316() {
  check!(4, 0, 0, "-11101.0110010011100E0", "-0", [0x7800000000000000, 0x0000000000000000], F_04_00, F_00_00, F_00_00);
}

#[test]
fn _0317() {
  check!(4, 0, 0, "+9878987.5679766895E0", "0", [0x7800000000000000, 0x0000000000000000], F_04_00, F_00_00, F_00_00);
}

#[test]
fn _0318() {
  check!(4, 0, 0, "+98858.678996557986769E0", "-989.888889E0", [0xb001ec63b7d685d4, 0xe49c39a1b82c7906], F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0319() {
  check!(4, 0, 0, "+99.8998888898E0", "-55993.8675252E0", [0xaff857f6c8fae298, 0xe46f125bb81b162d], F_20_00, F_00_00, F_00_00);
}

#[test]
fn _0320() {
  check!(4, 0, 0, "-Infinity", "-0", [0x7800000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0321() {
  check!(4, 0, 0, "-Infinity", "0", [0xf800000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0322() {
  check!(4, 0, 0, "-Infinity", "-Infinity", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0323() {
  check!(4, 0, 0, "-Infinity", "Infinity", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0324() {
  check!(4, 0, 0, "Infinity", "Infinity", [0x7c00000000000000, 0x0000000000000000], F_01_00, F_00_00, F_00_00);
}

#[test]
fn _0325() {
  check!(4, 0, 0, "QNaN", "-0", [0x7c00000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}

#[test]
fn _0326() {
  check!(4, 0, 0, "QNaN", "QNaN", [0x7c00000000000000, 0x0000000000000000], F_00_00, F_00_00, F_00_00);
}
