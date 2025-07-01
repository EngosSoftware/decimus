//! # 128-bit decimal floating-point utilities

use crate::{BidUint64, BidUint128, BidUint192, BidUint256};

macro_rules! bid_kx64 {
  ($index:expr) => {
    BID_KX64[$index as usize]
  };
}
pub(crate) use bid_kx64;

/// Kx from 10^(-x) ~= Kx * 2^(-Ex); Kx rounded up to 64 bits, 1 <= x <= 17
pub const BID_KX64: [BidUint64; 17] = [
  0xcccccccccccccccd, // 10^-1 ~= cccccccccccccccd * 2^-67
  0xa3d70a3d70a3d70b, // 10^-2 ~= a3d70a3d70a3d70b * 2^-70
  0x83126e978d4fdf3c, // 10^-3 ~= 83126e978d4fdf3c * 2^-73
  0xd1b71758e219652c, // 10^-4 ~= d1b71758e219652c * 2^-77
  0xa7c5ac471b478424, // 10^-5 ~= a7c5ac471b478424 * 2^-80
  0x8637bd05af6c69b6, // 10^-6 ~= 8637bd05af6c69b6 * 2^-83
  0xd6bf94d5e57a42bd, // 10^-7 ~= d6bf94d5e57a42bd * 2^-87
  0xabcc77118461cefd, // 10^-8 ~= abcc77118461cefd * 2^-90
  0x89705f4136b4a598, // 10^-9 ~= 89705f4136b4a598 * 2^-93
  0xdbe6fecebdedd5bf, // 10^-10 ~= dbe6fecebdedd5bf * 2^-97
  0xafebff0bcb24aaff, // 10^-11 ~= afebff0bcb24aaff * 2^-100
  0x8cbccc096f5088cc, // 10^-12 ~= 8cbccc096f5088cc * 2^-103
  0xe12e13424bb40e14, // 10^-13 ~= e12e13424bb40e14 * 2^-107
  0xb424dc35095cd810, // 10^-14 ~= b424dc35095cd810 * 2^-110
  0x901d7cf73ab0acda, // 10^-15 ~= 901d7cf73ab0acda * 2^-113
  0xe69594bec44de15c, // 10^-16 ~= e69594bec44de15c * 2^-117
  0xb877aa3236a4b44a, // 10^-17 ~= b877aa3236a4b44a * 2^-120
];

macro_rules! bid_ex64m64 {
  ($index:expr) => {
    BID_EX64M64[$index as usize]
  };
}
pub(crate) use bid_ex64m64;

/// Ex-64 from 10^(-x) ~= Kx * 2^(-Ex); Kx rounded up to 64 bits, 1 <= x <= 17
pub const BID_EX64M64: [u32; 17] = [
  3,  // 67 - 64, Ex = 67
  6,  // 70 - 64, Ex = 70
  9,  // 73 - 64, Ex = 73
  13, // 77 - 64, Ex = 77
  16, // 80 - 64, Ex = 80
  19, // 83 - 64, Ex = 83
  23, // 87 - 64, Ex = 87
  26, // 90 - 64, Ex = 90
  29, // 93 - 64, Ex = 93
  33, // 97 - 64, Ex = 97
  36, // 100 - 64, Ex = 100
  39, // 103 - 64, Ex = 103
  43, // 107 - 64, Ex = 107
  46, // 110 - 64, Ex = 110
  49, // 113 - 64, Ex = 113
  53, // 117 - 64, Ex = 117
  56, // 120 - 64, Ex = 120
];

macro_rules! bid_half64 {
  ($index:expr) => {
    BID_HALF64[$index as usize]
  };
}
pub(crate) use bid_half64;

/// Values of 1/2 in the right position to be compared with the fraction from
/// C * kx, 1 <= x <= 17; the fraction consists of the low Ex bits in C * kx
/// (these values are aligned with the high 64 bits of the fraction)
pub const BID_HALF64: [BidUint64; 17] = [
  0x0000000000000004, // half / 2^64 = 4
  0x0000000000000020, // half / 2^64 = 20
  0x0000000000000100, // half / 2^64 = 100
  0x0000000000001000, // half / 2^64 = 1000
  0x0000000000008000, // half / 2^64 = 8000
  0x0000000000040000, // half / 2^64 = 40000
  0x0000000000400000, // half / 2^64 = 400000
  0x0000000002000000, // half / 2^64 = 2000000
  0x0000000010000000, // half / 2^64 = 10000000
  0x0000000100000000, // half / 2^64 = 100000000
  0x0000000800000000, // half / 2^64 = 800000000
  0x0000004000000000, // half / 2^64 = 4000000000
  0x0000040000000000, // half / 2^64 = 40000000000
  0x0000200000000000, // half / 2^64 = 200000000000
  0x0001000000000000, // half / 2^64 = 1000000000000
  0x0010000000000000, // half / 2^64 = 10000000000000
  0x0080000000000000, // half / 2^64 = 80000000000000
];

macro_rules! bid_mask64 {
  ($index:expr) => {
    BID_MASK64[$index as usize]
  };
}
pub(crate) use bid_mask64;

/// Values of mask in the right position to obtain the high Ex - 64 bits
/// of the fraction from C * kx, 1 <= x <= 17; the fraction consists of
/// the low Ex bits in C * kx
pub const BID_MASK64: [BidUint64; 17] = [
  0x0000000000000007, // mask / 2^64
  0x000000000000003f, // mask / 2^64
  0x00000000000001ff, // mask / 2^64
  0x0000000000001fff, // mask / 2^64
  0x000000000000ffff, // mask / 2^64
  0x000000000007ffff, // mask / 2^64
  0x00000000007fffff, // mask / 2^64
  0x0000000003ffffff, // mask / 2^64
  0x000000001fffffff, // mask / 2^64
  0x00000001ffffffff, // mask / 2^64
  0x0000000fffffffff, // mask / 2^64
  0x0000007fffffffff, // mask / 2^64
  0x000007ffffffffff, // mask / 2^64
  0x00003fffffffffff, // mask / 2^64
  0x0001ffffffffffff, // mask / 2^64
  0x001fffffffffffff, // mask / 2^64
  0x00ffffffffffffff, // mask / 2^64
];

macro_rules! bid_ten2mxtrunc64 {
  ($index:expr) => {
    BID_TEN2MXTRUNC64[$index as usize]
  };
}
pub(crate) use bid_ten2mxtrunc64;

/// Values of 10^(-x) trancated to Ex bits beyond the binary point, and
/// in the right position to be compared with the fraction from C * kx,
/// 1 <= x <= 17; the fraction consists of the low Ex bits in C * kx
/// (these values are aligned with the low 64 bits of the fraction)
pub const BID_TEN2MXTRUNC64: [BidUint64; 17] = [
  0xcccccccccccccccc, // (ten2mx >> 64) = cccccccccccccccc
  0xa3d70a3d70a3d70a, // (ten2mx >> 64) = a3d70a3d70a3d70a
  0x83126e978d4fdf3b, // (ten2mx >> 64) = 83126e978d4fdf3b
  0xd1b71758e219652b, // (ten2mx >> 64) = d1b71758e219652b
  0xa7c5ac471b478423, // (ten2mx >> 64) = a7c5ac471b478423
  0x8637bd05af6c69b5, // (ten2mx >> 64) = 8637bd05af6c69b5
  0xd6bf94d5e57a42bc, // (ten2mx >> 64) = d6bf94d5e57a42bc
  0xabcc77118461cefc, // (ten2mx >> 64) = abcc77118461cefc
  0x89705f4136b4a597, // (ten2mx >> 64) = 89705f4136b4a597
  0xdbe6fecebdedd5be, // (ten2mx >> 64) = dbe6fecebdedd5be
  0xafebff0bcb24aafe, // (ten2mx >> 64) = afebff0bcb24aafe
  0x8cbccc096f5088cb, // (ten2mx >> 64) = 8cbccc096f5088cb
  0xe12e13424bb40e13, // (ten2mx >> 64) = e12e13424bb40e13
  0xb424dc35095cd80f, // (ten2mx >> 64) = b424dc35095cd80f
  0x901d7cf73ab0acd9, // (ten2mx >> 64) = 901d7cf73ab0acd9
  0xe69594bec44de15b, // (ten2mx >> 64) = e69594bec44de15b
  0xb877aa3236a4b449, // (ten2mx >> 64) = b877aa3236a4b449
];

macro_rules! bid_kx128 {
  ($index:expr) => {
    BID_KX128[$index as usize]
  };
}
pub(crate) use bid_kx128;

/// Kx from 10^(-x) ~= Kx * 2^(-Ex); Kx rounded up to 128 bits, 1 <= x <= 37
#[rustfmt::skip]
pub const BID_KX128: [BidUint128; 37] = [
  BidUint128{ w: [0xcccccccccccccccd, 0xcccccccccccccccc] }, //  10^-1 ~= cccccccccccccccccccccccccccccccd * 2^-131
  BidUint128{ w: [0x3d70a3d70a3d70a4, 0xa3d70a3d70a3d70a] }, //  10^-2 ~= a3d70a3d70a3d70a3d70a3d70a3d70a4 * 2^-134
  BidUint128{ w: [0x645a1cac083126ea, 0x83126e978d4fdf3b] }, //  10^-3 ~= 83126e978d4fdf3b645a1cac083126ea * 2^-137
  BidUint128{ w: [0xd3c36113404ea4a9, 0xd1b71758e219652b] }, //  10^-4 ~= d1b71758e219652bd3c36113404ea4a9 * 2^-141
  BidUint128{ w: [0x0fcf80dc33721d54, 0xa7c5ac471b478423] }, //  10^-5 ~= a7c5ac471b4784230fcf80dc33721d54 * 2^-144
  BidUint128{ w: [0xa63f9a49c2c1b110, 0x8637bd05af6c69b5] }, //  10^-6 ~= 8637bd05af6c69b5a63f9a49c2c1b110 * 2^-147
  BidUint128{ w: [0x3d32907604691b4d, 0xd6bf94d5e57a42bc] }, //  10^-7 ~= d6bf94d5e57a42bc3d32907604691b4d * 2^-151
  BidUint128{ w: [0xfdc20d2b36ba7c3e, 0xabcc77118461cefc] }, //  10^-8 ~= abcc77118461cefcfdc20d2b36ba7c3e * 2^-154
  BidUint128{ w: [0x31680a88f8953031, 0x89705f4136b4a597] }, //  10^-9 ~= 89705f4136b4a59731680a88f8953031 * 2^-157
  BidUint128{ w: [0xb573440e5a884d1c, 0xdbe6fecebdedd5be] }, // 10^-10 ~= dbe6fecebdedd5beb573440e5a884d1c * 2^-161
  BidUint128{ w: [0xf78f69a51539d749, 0xafebff0bcb24aafe] }, // 10^-11 ~= afebff0bcb24aafef78f69a51539d749 * 2^-164
  BidUint128{ w: [0xf93f87b7442e45d4, 0x8cbccc096f5088cb] }, // 10^-12 ~= 8cbccc096f5088cbf93f87b7442e45d4 * 2^-167
  BidUint128{ w: [0x2865a5f206b06fba, 0xe12e13424bb40e13] }, // 10^-13 ~= e12e13424bb40e132865a5f206b06fba * 2^-171
  BidUint128{ w: [0x538484c19ef38c95, 0xb424dc35095cd80f] }, // 10^-14 ~= b424dc35095cd80f538484c19ef38c95 * 2^-174
  BidUint128{ w: [0x0f9d37014bf60a11, 0x901d7cf73ab0acd9] }, // 10^-15 ~= 901d7cf73ab0acd90f9d37014bf60a11 * 2^-177
  BidUint128{ w: [0x4c2ebe687989a9b4, 0xe69594bec44de15b] }, // 10^-16 ~= e69594bec44de15b4c2ebe687989a9b4 * 2^-181
  BidUint128{ w: [0x09befeb9fad487c3, 0xb877aa3236a4b449] }, // 10^-17 ~= b877aa3236a4b44909befeb9fad487c3 * 2^-184
  BidUint128{ w: [0x3aff322e62439fd0, 0x9392ee8e921d5d07] }, // 10^-18 ~= 9392ee8e921d5d073aff322e62439fd0 * 2^-187
  BidUint128{ w: [0x2b31e9e3d06c32e6, 0xec1e4a7db69561a5] }, // 10^-19 ~= ec1e4a7db69561a52b31e9e3d06c32e6 * 2^-191
  BidUint128{ w: [0x88f4bb1ca6bcf585, 0xbce5086492111aea] }, // 10^-20 ~= bce5086492111aea88f4bb1ca6bcf585 * 2^-194
  BidUint128{ w: [0xd3f6fc16ebca5e04, 0x971da05074da7bee] }, // 10^-21 ~= 971da05074da7beed3f6fc16ebca5e04 * 2^-197
  BidUint128{ w: [0x5324c68b12dd6339, 0xf1c90080baf72cb1] }, // 10^-22 ~= f1c90080baf72cb15324c68b12dd6339 * 2^-201
  BidUint128{ w: [0x75b7053c0f178294, 0xc16d9a0095928a27] }, // 10^-23 ~= c16d9a0095928a2775b7053c0f178294 * 2^-204
  BidUint128{ w: [0xc4926a9672793543, 0x9abe14cd44753b52] }, // 10^-24 ~= 9abe14cd44753b52c4926a9672793543 * 2^-207
  BidUint128{ w: [0x3a83ddbd83f52205, 0xf79687aed3eec551] }, // 10^-25 ~= f79687aed3eec5513a83ddbd83f52205 * 2^-211
  BidUint128{ w: [0x95364afe032a819e, 0xc612062576589dda] }, // 10^-26 ~= c612062576589dda95364afe032a819e * 2^-214
  BidUint128{ w: [0x775ea264cf55347e, 0x9e74d1b791e07e48] }, // 10^-27 ~= 9e74d1b791e07e48775ea264cf55347e * 2^-217
  BidUint128{ w: [0x8bca9d6e188853fd, 0xfd87b5f28300ca0d] }, // 10^-28 ~= fd87b5f28300ca0d8bca9d6e188853fd * 2^-221
  BidUint128{ w: [0x096ee45813a04331, 0xcad2f7f5359a3b3e] }, // 10^-29 ~= cad2f7f5359a3b3e096ee45813a04331 * 2^-224
  BidUint128{ w: [0xa1258379a94d028e, 0xa2425ff75e14fc31] }, // 10^-30 ~= a2425ff75e14fc31a1258379a94d028e * 2^-227
  BidUint128{ w: [0x80eacf948770ced8, 0x81ceb32c4b43fcf4] }, // 10^-31 ~= 81ceb32c4b43fcf480eacf948770ced8 * 2^-230
  BidUint128{ w: [0x67de18eda5814af3, 0xcfb11ead453994ba] }, // 10^-32 ~= cfb11ead453994ba67de18eda5814af3 * 2^-234
  BidUint128{ w: [0xecb1ad8aeacdd58f, 0xa6274bbdd0fadd61] }, // 10^-33 ~= a6274bbdd0fadd61ecb1ad8aeacdd58f * 2^-237
  BidUint128{ w: [0xbd5af13bef0b113f, 0x84ec3c97da624ab4] }, // 10^-34 ~= 84ec3c97da624ab4bd5af13bef0b113f * 2^-240
  BidUint128{ w: [0x955e4ec64b44e865, 0xd4ad2dbfc3d07787] }, // 10^-35 ~= d4ad2dbfc3d07787955e4ec64b44e865 * 2^-244
  BidUint128{ w: [0xdde50bd1d5d0b9ea, 0xaa242499697392d2] }, // 10^-36 ~= aa242499697392d2dde50bd1d5d0b9ea * 2^-247
  BidUint128{ w: [0x7e50d64177da2e55, 0x881cea14545c7575] }  // 10^-37 ~= 881cea14545c75757e50d64177da2e55 * 2^-250
];

macro_rules! bid_ex128m128 {
  ($index:expr) => {
    BID_EX128M128[$index as usize]
  };
}
pub(crate) use bid_ex128m128;

/// Ex-128 from 10^(-x) ~= Kx*2^(-Ex); Kx rounded up to 128 bits, 1 <= x <= 37
#[rustfmt::skip]
pub const BID_EX128M128:[u32;37] = [
   3, // 131 - 128, Ex = 131
   6, // 134 - 128, Ex = 134
   9, // 137 - 128, Ex = 137
  13, // 141 - 128, Ex = 141
  16, // 144 - 128, Ex = 144
  19, // 147 - 128, Ex = 147
  23, // 151 - 128, Ex = 151
  26, // 154 - 128, Ex = 154
  29, // 157 - 128, Ex = 157
  33, // 161 - 128, Ex = 161
  36, // 164 - 128, Ex = 164
  39, // 167 - 128, Ex = 167
  43, // 171 - 128, Ex = 171
  46, // 174 - 128, Ex = 174
  49, // 177 - 128, Ex = 177
  53, // 181 - 128, Ex = 181
  56, // 184 - 128, Ex = 184
  59, // 187 - 128, Ex = 187
  63, // 191 - 128, Ex = 191
   2, // 194 - 192, Ex = 194
   5, // 197 - 192, Ex = 197
   9, // 201 - 192, Ex = 201
  12, // 204 - 192, Ex = 204
  15, // 207 - 192, Ex = 207
  19, // 211 - 192, Ex = 211
  22, // 214 - 192, Ex = 214
  25, // 217 - 192, Ex = 217
  29, // 221 - 192, Ex = 221
  32, // 224 - 192, Ex = 224
  35, // 227 - 192, Ex = 227
  38, // 230 - 192, Ex = 230
  42, // 234 - 192, Ex = 234
  45, // 237 - 192, Ex = 237
  48, // 240 - 192, Ex = 240
  52, // 244 - 192, Ex = 244
  55, // 247 - 192, Ex = 247
  58  // 250 - 192, Ex = 250
];

macro_rules! bid_mask128 {
  ($index:expr) => {
    BID_MASK128[$index as usize]
  };
}
pub(crate) use bid_mask128;

/// Values of mask in the right position to obtain the high Ex - 128 or Ex - 192
/// bits of the fraction from C * kx, 1 <= x <= 37; the fraction consists of
/// the low Ex bits in C * kx.
#[rustfmt::skip]
pub const BID_MASK128: [BidUint64; 37] = [
  0x0000000000000007, // mask / 2^128
  0x000000000000003f, // mask / 2^128
  0x00000000000001ff, // mask / 2^128
  0x0000000000001fff, // mask / 2^128
  0x000000000000ffff, // mask / 2^128
  0x000000000007ffff, // mask / 2^128
  0x00000000007fffff, // mask / 2^128
  0x0000000003ffffff, // mask / 2^128
  0x000000001fffffff, // mask / 2^128
  0x00000001ffffffff, // mask / 2^128
  0x0000000fffffffff, // mask / 2^128
  0x0000007fffffffff, // mask / 2^128
  0x000007ffffffffff, // mask / 2^128
  0x00003fffffffffff, // mask / 2^128
  0x0001ffffffffffff, // mask / 2^128
  0x001fffffffffffff, // mask / 2^128
  0x00ffffffffffffff, // mask / 2^128
  0x07ffffffffffffff, // mask / 2^128
  0x7fffffffffffffff, // mask / 2^128
  0x0000000000000003, // mask / 2^192
  0x000000000000001f, // mask / 2^192
  0x00000000000001ff, // mask / 2^192
  0x0000000000000fff, // mask / 2^192
  0x0000000000007fff, // mask / 2^192
  0x000000000007ffff, // mask / 2^192
  0x00000000003fffff, // mask / 2^192
  0x0000000001ffffff, // mask / 2^192
  0x000000001fffffff, // mask / 2^192
  0x00000000ffffffff, // mask / 2^192
  0x00000007ffffffff, // mask / 2^192
  0x0000003fffffffff, // mask / 2^192
  0x000003ffffffffff, // mask / 2^192
  0x00001fffffffffff, // mask / 2^192
  0x0000ffffffffffff, // mask / 2^192
  0x000fffffffffffff, // mask / 2^192
  0x007fffffffffffff, // mask / 2^192
  0x03ffffffffffffff	// mask / 2^192
];

macro_rules! bid_half128 {
  ($index:expr) => {
    BID_HALF128[$index as usize]
  };
}
pub(crate) use bid_half128;

/// Values of 1/2 in the right position to be compared with the fraction from
/// C * kx, 1 <= x <= 37; the fraction consists of the low Ex bits in C * kx
/// (these values are aligned with the high 128 bits of the fraction)
#[rustfmt::skip]
pub const BID_HALF128: [BidUint64; 37] = [
  0x0000000000000004,	// half / 2^128 = 4
  0x0000000000000020,	// half / 2^128 = 20
  0x0000000000000100,	// half / 2^128 = 100
  0x0000000000001000,	// half / 2^128 = 1000
  0x0000000000008000,	// half / 2^128 = 8000
  0x0000000000040000,	// half / 2^128 = 40000
  0x0000000000400000,	// half / 2^128 = 400000
  0x0000000002000000,	// half / 2^128 = 2000000
  0x0000000010000000,	// half / 2^128 = 10000000
  0x0000000100000000,	// half / 2^128 = 100000000
  0x0000000800000000,	// half / 2^128 = 800000000
  0x0000004000000000,	// half / 2^128 = 4000000000
  0x0000040000000000,	// half / 2^128 = 40000000000
  0x0000200000000000,	// half / 2^128 = 200000000000
  0x0001000000000000,	// half / 2^128 = 1000000000000
  0x0010000000000000,	// half / 2^128 = 10000000000000
  0x0080000000000000,	// half / 2^128 = 80000000000000
  0x0400000000000000,	// half / 2^128 = 400000000000000
  0x4000000000000000,	// half / 2^128 = 4000000000000000
  0x0000000000000002,	// half / 2^192 = 2
  0x0000000000000010,	// half / 2^192 = 10
  0x0000000000000100,	// half / 2^192 = 100
  0x0000000000000800,	// half / 2^192 = 800
  0x0000000000004000,	// half / 2^192 = 4000
  0x0000000000040000,	// half / 2^192 = 40000
  0x0000000000200000,	// half / 2^192 = 200000
  0x0000000001000000,	// half / 2^192 = 1000000
  0x0000000010000000,	// half / 2^192 = 10000000
  0x0000000080000000,	// half / 2^192 = 80000000
  0x0000000400000000,	// half / 2^192 = 400000000
  0x0000002000000000,	// half / 2^192 = 2000000000
  0x0000020000000000,	// half / 2^192 = 20000000000
  0x0000100000000000,	// half / 2^192 = 100000000000
  0x0000800000000000,	// half / 2^192 = 800000000000
  0x0008000000000000,	// half / 2^192 = 8000000000000
  0x0040000000000000,	// half / 2^192 = 40000000000000
  0x0200000000000000	// half / 2^192 = 200000000000000
];

macro_rules! bid_half192 {
  ($index:expr) => {
    BID_HALF192[$index as usize]
  };
}
pub(crate) use bid_half192;

#[rustfmt::skip]
pub const BID_HALF192: [BidUint64; 56] = [
  0x0000000000000004, // half / 2^192 = 4
  0x0000000000000020, // half / 2^192 = 20
  0x0000000000000100, // half / 2^192 = 100
  0x0000000000001000, // half / 2^192 = 1000
  0x0000000000008000, // half / 2^192 = 8000
  0x0000000000040000, // half / 2^192 = 40000
  0x0000000000400000, // half / 2^192 = 400000
  0x0000000002000000, // half / 2^192 = 2000000
  0x0000000010000000, // half / 2^192 = 10000000
  0x0000000100000000, // half / 2^192 = 100000000
  0x0000000800000000, // half / 2^192 = 800000000
  0x0000004000000000, // half / 2^192 = 4000000000
  0x0000040000000000, // half / 2^192 = 40000000000
  0x0000200000000000, // half / 2^192 = 200000000000
  0x0001000000000000, // half / 2^192 = 1000000000000
  0x0010000000000000, // half / 2^192 = 10000000000000
  0x0080000000000000, // half / 2^192 = 80000000000000
  0x0400000000000000, // half / 2^192 = 400000000000000
  0x4000000000000000, // half / 2^192 = 4000000000000000
  0x0000000000000002, // half / 2^256 = 2
  0x0000000000000010, // half / 2^256 = 10
  0x0000000000000100, // half / 2^256 = 100
  0x0000000000000800, // half / 2^256 = 800
  0x0000000000004000, // half / 2^256 = 4000
  0x0000000000040000, // half / 2^256 = 40000
  0x0000000000200000, // half / 2^256 = 200000
  0x0000000001000000, // half / 2^256 = 1000000
  0x0000000010000000, // half / 2^256 = 10000000
  0x0000000080000000, // half / 2^256 = 80000000
  0x0000000400000000, // half / 2^256 = 400000000
  0x0000002000000000, // half / 2^256 = 2000000000
  0x0000020000000000, // half / 2^256 = 20000000000
  0x0000100000000000, // half / 2^256 = 100000000000
  0x0000800000000000, // half / 2^256 = 800000000000
  0x0008000000000000, // half / 2^256 = 8000000000000
  0x0040000000000000, // half / 2^256 = 40000000000000
  0x0200000000000000, // half / 2^256 = 200000000000000
  0x2000000000000000, // half / 2^256 = 2000000000000000
  0x0000000000000001, // half / 2^320 = 1
  0x0000000000000008, // half / 2^320 = 8
  0x0000000000000080, // half / 2^320 = 80
  0x0000000000000400, // half / 2^320 = 400
  0x0000000000002000, // half / 2^320 = 2000
  0x0000000000020000, // half / 2^320 = 20000
  0x0000000000100000, // half / 2^320 = 100000
  0x0000000000800000, // half / 2^320 = 800000
  0x0000000008000000, // half / 2^320 = 8000000
  0x0000000040000000, // half / 2^320 = 40000000
  0x0000000200000000, // half / 2^320 = 200000000
  0x0000002000000000, // half / 2^320 = 2000000000
  0x0000010000000000, // half / 2^320 = 10000000000
  0x0000080000000000, // half / 2^320 = 80000000000
  0x0000800000000000, // half / 2^320 = 800000000000
  0x0004000000000000, // half / 2^320 = 4000000000000
  0x0020000000000000, // half / 2^320 = 20000000000000
  0x0200000000000000	// half / 2^320 = 200000000000000
];

macro_rules! bid_mask192 {
  ($index:expr) => {
    BID_MASK192[$index as usize]
  };
}
pub(crate) use bid_mask192;

#[rustfmt::skip]
pub const BID_MASK192: [BidUint64; 56] = [
  0x0000000000000007,	// mask / 2^192
  0x000000000000003f,	// mask / 2^192
  0x00000000000001ff,	// mask / 2^192
  0x0000000000001fff,	// mask / 2^192
  0x000000000000ffff,	// mask / 2^192
  0x000000000007ffff,	// mask / 2^192
  0x00000000007fffff,	// mask / 2^192
  0x0000000003ffffff,	// mask / 2^192
  0x000000001fffffff,	// mask / 2^192
  0x00000001ffffffff,	// mask / 2^192
  0x0000000fffffffff,	// mask / 2^192
  0x0000007fffffffff,	// mask / 2^192
  0x000007ffffffffff,	// mask / 2^192
  0x00003fffffffffff,	// mask / 2^192
  0x0001ffffffffffff,	// mask / 2^192
  0x001fffffffffffff,	// mask / 2^192
  0x00ffffffffffffff,	// mask / 2^192
  0x07ffffffffffffff,	// mask / 2^192
  0x7fffffffffffffff,	// mask / 2^192
  0x0000000000000003,	// mask / 2^256
  0x000000000000001f,	// mask / 2^256
  0x00000000000001ff,	// mask / 2^256
  0x0000000000000fff,	// mask / 2^256
  0x0000000000007fff,	// mask / 2^256
  0x000000000007ffff,	// mask / 2^256
  0x00000000003fffff,	// mask / 2^256
  0x0000000001ffffff,	// mask / 2^256
  0x000000001fffffff,	// mask / 2^256
  0x00000000ffffffff,	// mask / 2^256
  0x00000007ffffffff,	// mask / 2^256
  0x0000003fffffffff,	// mask / 2^256
  0x000003ffffffffff,	// mask / 2^256
  0x00001fffffffffff,	// mask / 2^256
  0x0000ffffffffffff,	// mask / 2^256
  0x000fffffffffffff,	// mask / 2^256
  0x007fffffffffffff,	// mask / 2^256
  0x03ffffffffffffff,	// mask / 2^256
  0x3fffffffffffffff,	// mask / 2^256
  0x0000000000000001,	// mask / 2^320
  0x000000000000000f,	// mask / 2^320
  0x00000000000000ff,	// mask / 2^320
  0x00000000000007ff,	// mask / 2^320
  0x0000000000003fff,	// mask / 2^320
  0x000000000003ffff,	// mask / 2^320
  0x00000000001fffff,	// mask / 2^320
  0x0000000000ffffff,	// mask / 2^320
  0x000000000fffffff,	// mask / 2^320
  0x000000007fffffff,	// mask / 2^320
  0x00000003ffffffff,	// mask / 2^320
  0x0000003fffffffff,	// mask / 2^320
  0x000001ffffffffff,	// mask / 2^320
  0x00000fffffffffff,	// mask / 2^320
  0x0000ffffffffffff,	// mask / 2^320
  0x0007ffffffffffff,	// mask / 2^320
  0x003fffffffffffff,	// mask / 2^320
  0x03ffffffffffffff	// mask / 2^320
];

macro_rules! bid_ten2mxtrunc192 {
  ($index:expr) => {
    BID_TEN2MXTRUNC192[$index as usize]
  };
}
pub(crate) use bid_ten2mxtrunc192;

#[rustfmt::skip]
pub const BID_TEN2MXTRUNC192: [BidUint192; 56] = [
  BidUint192{ w: [0xcccccccccccccccc, 0xcccccccccccccccc,0xcccccccccccccccc ] }, // (ten2mx >> 192) = cccccccccccccccccccccccccccccccccccccccccccccccc
  BidUint192{ w: [0xd70a3d70a3d70a3d, 0x3d70a3d70a3d70a3,0xa3d70a3d70a3d70a ] }, // (ten2mx >> 192) = a3d70a3d70a3d70a3d70a3d70a3d70a3d70a3d70a3d70a3d
  BidUint192{ w: [0x78d4fdf3b645a1ca, 0x645a1cac083126e9,0x83126e978d4fdf3b ] }, // (ten2mx >> 192) = 83126e978d4fdf3b645a1cac083126e978d4fdf3b645a1ca
  BidUint192{ w: [0xc154c985f06f6944, 0xd3c36113404ea4a8,0xd1b71758e219652b ] }, // (ten2mx >> 192) = d1b71758e219652bd3c36113404ea4a8c154c985f06f6944
  BidUint192{ w: [0xcddd6e04c0592103, 0x0fcf80dc33721d53,0xa7c5ac471b478423 ] }, // (ten2mx >> 192) = a7c5ac471b4784230fcf80dc33721d53cddd6e04c0592103
  BidUint192{ w: [0xd7e45803cd141a69, 0xa63f9a49c2c1b10f,0x8637bd05af6c69b5 ] }, // (ten2mx >> 192) = 8637bd05af6c69b5a63f9a49c2c1b10fd7e45803cd141a69
  BidUint192{ w: [0x8ca08cd2e1b9c3db, 0x3d32907604691b4c,0xd6bf94d5e57a42bc ] }, // (ten2mx >> 192) = d6bf94d5e57a42bc3d32907604691b4c8ca08cd2e1b9c3db
  BidUint192{ w: [0x3d4d3d758161697c, 0xfdc20d2b36ba7c3d,0xabcc77118461cefc ] }, // (ten2mx >> 192) = abcc77118461cefcfdc20d2b36ba7c3d3d4d3d758161697c
  BidUint192{ w: [0xfdd7645e011abac9, 0x31680a88f8953030,0x89705f4136b4a597 ] }, // (ten2mx >> 192) = 89705f4136b4a59731680a88f8953030fdd7645e011abac9
  BidUint192{ w: [0x2fbf06fcce912adc, 0xb573440e5a884d1b,0xdbe6fecebdedd5be ] }, // (ten2mx >> 192) = dbe6fecebdedd5beb573440e5a884d1b2fbf06fcce912adc
  BidUint192{ w: [0xf2ff38ca3eda88b0, 0xf78f69a51539d748,0xafebff0bcb24aafe ] }, // (ten2mx >> 192) = afebff0bcb24aafef78f69a51539d748f2ff38ca3eda88b0
  BidUint192{ w: [0xf598fa3b657ba08d, 0xf93f87b7442e45d3,0x8cbccc096f5088cb ] }, // (ten2mx >> 192) = 8cbccc096f5088cbf93f87b7442e45d3f598fa3b657ba08d
  BidUint192{ w: [0x88f4c3923bf900e2, 0x2865a5f206b06fb9,0xe12e13424bb40e13 ] }, // (ten2mx >> 192) = e12e13424bb40e132865a5f206b06fb988f4c3923bf900e2
  BidUint192{ w: [0x6d909c74fcc733e8, 0x538484c19ef38c94,0xb424dc35095cd80f ] }, // (ten2mx >> 192) = b424dc35095cd80f538484c19ef38c946d909c74fcc733e8
  BidUint192{ w: [0x57a6e390ca38f653, 0x0f9d37014bf60a10,0x901d7cf73ab0acd9 ] }, // (ten2mx >> 192) = 901d7cf73ab0acd90f9d37014bf60a1057a6e390ca38f653
  BidUint192{ w: [0xbf716c1add27f085, 0x4c2ebe687989a9b3,0xe69594bec44de15b ] }, // (ten2mx >> 192) = e69594bec44de15b4c2ebe687989a9b3bf716c1add27f085
  BidUint192{ w: [0xff8df0157db98d37, 0x09befeb9fad487c2,0xb877aa3236a4b449 ] }, // (ten2mx >> 192) = b877aa3236a4b44909befeb9fad487c2ff8df0157db98d37
  BidUint192{ w: [0x32d7f344649470f9, 0x3aff322e62439fcf,0x9392ee8e921d5d07 ] }, // (ten2mx >> 192) = 9392ee8e921d5d073aff322e62439fcf32d7f344649470f9
  BidUint192{ w: [0x1e2652070753e7f4, 0x2b31e9e3d06c32e5,0xec1e4a7db69561a5 ] }, // (ten2mx >> 192) = ec1e4a7db69561a52b31e9e3d06c32e51e2652070753e7f4
  BidUint192{ w: [0x181ea8059f76532a, 0x88f4bb1ca6bcf584,0xbce5086492111aea ] }, // (ten2mx >> 192) = bce5086492111aea88f4bb1ca6bcf584181ea8059f76532a
  BidUint192{ w: [0x467eecd14c5ea8ee, 0xd3f6fc16ebca5e03,0x971da05074da7bee ] }, // (ten2mx >> 192) = 971da05074da7beed3f6fc16ebca5e03467eecd14c5ea8ee
  BidUint192{ w: [0x70cb148213caa7e4, 0x5324c68b12dd6338,0xf1c90080baf72cb1 ] }, // (ten2mx >> 192) = f1c90080baf72cb15324c68b12dd633870cb148213caa7e4
  BidUint192{ w: [0x8d6f439b43088650, 0x75b7053c0f178293,0xc16d9a0095928a27 ] }, // (ten2mx >> 192) = c16d9a0095928a2775b7053c0f1782938d6f439b43088650
  BidUint192{ w: [0xd78c3615cf3a050c, 0xc4926a9672793542,0x9abe14cd44753b52 ] }, // (ten2mx >> 192) = 9abe14cd44753b52c4926a9672793542d78c3615cf3a050c
  BidUint192{ w: [0x8c1389bc7ec33b47, 0x3a83ddbd83f52204,0xf79687aed3eec551 ] }, // (ten2mx >> 192) = f79687aed3eec5513a83ddbd83f522048c1389bc7ec33b47
  BidUint192{ w: [0x3cdc6e306568fc39, 0x95364afe032a819d,0xc612062576589dda ] }, // (ten2mx >> 192) = c612062576589dda95364afe032a819d3cdc6e306568fc39
  BidUint192{ w: [0xca49f1c05120c9c7, 0x775ea264cf55347d,0x9e74d1b791e07e48 ] }, // (ten2mx >> 192) = 9e74d1b791e07e48775ea264cf55347dca49f1c05120c9c7
  BidUint192{ w: [0x76dcb60081ce0fa5, 0x8bca9d6e188853fc,0xfd87b5f28300ca0d ] }, // (ten2mx >> 192) = fd87b5f28300ca0d8bca9d6e188853fc76dcb60081ce0fa5
  BidUint192{ w: [0x5f16f80067d80c84, 0x096ee45813a04330,0xcad2f7f5359a3b3e ] }, // (ten2mx >> 192) = cad2f7f5359a3b3e096ee45813a043305f16f80067d80c84
  BidUint192{ w: [0x18df2ccd1fe00a03, 0xa1258379a94d028d,0xa2425ff75e14fc31 ] }, // (ten2mx >> 192) = a2425ff75e14fc31a1258379a94d028d18df2ccd1fe00a03
  BidUint192{ w: [0x4718f0a419800802, 0x80eacf948770ced7,0x81ceb32c4b43fcf4 ] }, // (ten2mx >> 192) = 81ceb32c4b43fcf480eacf948770ced74718f0a419800802
  BidUint192{ w: [0x0b5b1aa028ccd99e, 0x67de18eda5814af2,0xcfb11ead453994ba ] }, // (ten2mx >> 192) = cfb11ead453994ba67de18eda5814af20b5b1aa028ccd99e
  BidUint192{ w: [0x6f7c154ced70ae18, 0xecb1ad8aeacdd58e,0xa6274bbdd0fadd61 ] }, // (ten2mx >> 192) = a6274bbdd0fadd61ecb1ad8aeacdd58e6f7c154ced70ae18
  BidUint192{ w: [0xbf967770bdf3be79, 0xbd5af13bef0b113e,0x84ec3c97da624ab4 ] }, // (ten2mx >> 192) = 84ec3c97da624ab4bd5af13bef0b113ebf967770bdf3be79
  BidUint192{ w: [0x65bd8be79652ca5c, 0x955e4ec64b44e864,0xd4ad2dbfc3d07787 ] }, // (ten2mx >> 192) = d4ad2dbfc3d07787955e4ec64b44e86465bd8be79652ca5c
  BidUint192{ w: [0xeafe098611dbd516, 0xdde50bd1d5d0b9e9,0xaa242499697392d2 ] }, // (ten2mx >> 192) = aa242499697392d2dde50bd1d5d0b9e9eafe098611dbd516
  BidUint192{ w: [0xbbfe6e04db164412, 0x7e50d64177da2e54,0x881cea14545c7575 ] }, // (ten2mx >> 192) = 881cea14545c75757e50d64177da2e54bbfe6e04db164412
  BidUint192{ w: [0x2cca49a15e8a0683, 0x96e7bd358c904a21,0xd9c7dced53c72255 ] }, // (ten2mx >> 192) = d9c7dced53c7225596e7bd358c904a212cca49a15e8a0683
  BidUint192{ w: [0x8a3b6e1ab2080536, 0xabec975e0a0d081a,0xae397d8aa96c1b77 ] }, // (ten2mx >> 192) = ae397d8aa96c1b77abec975e0a0d081a8a3b6e1ab2080536
  BidUint192{ w: [0x3b62be7bc1a0042b, 0x2323ac4b3b3da015,0x8b61313bbabce2c6 ] }, // (ten2mx >> 192) = 8b61313bbabce2c62323ac4b3b3da0153b62be7bc1a0042b
  BidUint192{ w: [0x5f0463f935ccd378, 0x6b6c46dec52f6688,0xdf01e85f912e37a3 ] }, // (ten2mx >> 192) = df01e85f912e37a36b6c46dec52f66885f0463f935ccd378
  BidUint192{ w: [0x7f36b660f7d70f93, 0x55f038b237591ed3,0xb267ed1940f1c61c ] }, // (ten2mx >> 192) = b267ed1940f1c61c55f038b237591ed37f36b660f7d70f93
  BidUint192{ w: [0xcc2bc51a5fdf3fa9, 0x77f3608e92adb242,0x8eb98a7a9a5b04e3 ] }, // (ten2mx >> 192) = 8eb98a7a9a5b04e377f3608e92adb242cc2bc51a5fdf3fa9
  BidUint192{ w: [0xe046082a32fecc41, 0x8cb89a7db77c506a,0xe45c10c42a2b3b05 ] }, // (ten2mx >> 192) = e45c10c42a2b3b058cb89a7db77c506ae046082a32fecc41
  BidUint192{ w: [0x4d04d354f598a367, 0x3d607b97c5fd0d22,0xb6b00d69bb55c8d1 ] }, // (ten2mx >> 192) = b6b00d69bb55c8d13d607b97c5fd0d224d04d354f598a367
  BidUint192{ w: [0x3d9d75dd9146e91f, 0xcab3961304ca70e8,0x9226712162ab070d ] }, // (ten2mx >> 192) = 9226712162ab070dcab3961304ca70e83d9d75dd9146e91f
  BidUint192{ w: [0xc8fbefc8e87174ff, 0xaab8f01e6e10b4a6,0xe9d71b689dde71af ] }, // (ten2mx >> 192) = e9d71b689dde71afaab8f01e6e10b4a6c8fbefc8e87174ff
  BidUint192{ w: [0x3a63263a538df733, 0x5560c018580d5d52,0xbb127c53b17ec159 ] }, // (ten2mx >> 192) = bb127c53b17ec1595560c018580d5d523a63263a538df733
  BidUint192{ w: [0x2eb5b82ea93e5f5c, 0xdde7001379a44aa8,0x95a8637627989aad ] }, // (ten2mx >> 192) = 95a8637627989aaddde7001379a44aa82eb5b82ea93e5f5c
  BidUint192{ w: [0x4abc59e441fd6560, 0x963e66858f6d4440,0xef73d256a5c0f77c ] }, // (ten2mx >> 192) = ef73d256a5c0f77c963e66858f6d44404abc59e441fd6560
  BidUint192{ w: [0x6efd14b69b311de6, 0xde98520472bdd033,0xbf8fdb78849a5f96 ] }, // (ten2mx >> 192) = bf8fdb78849a5f96de98520472bdd0336efd14b69b311de6
  BidUint192{ w: [0x259743c548f417eb, 0xe546a8038efe4029,0x993fe2c6d07b7fab ] }, // (ten2mx >> 192) = 993fe2c6d07b7fabe546a8038efe4029259743c548f417eb
  BidUint192{ w: [0x3c25393ba7ecf312, 0xd53dd99f4b3066a8,0xf53304714d9265df ] }, // (ten2mx >> 192) = f53304714d9265dfd53dd99f4b3066a83c25393ba7ecf312
  BidUint192{ w: [0x96842dc95323f5a8, 0xaa97e14c3c26b886,0xc428d05aa4751e4c ] }, // (ten2mx >> 192) = c428d05aa4751e4caa97e14c3c26b88696842dc95323f5a8
  BidUint192{ w: [0xab9cf16ddc1cc486, 0x55464dd69685606b,0x9ced737bb6c4183d ] }, // (ten2mx >> 192) = 9ced737bb6c4183d55464dd69685606bab9cf16ddc1cc486
  BidUint192{ w: [0xac2e4f162cfad40a, 0xeed6e2f0f0d56712, 0xfb158592be068d2 ] }  // (ten2mx >> 192) = fb158592be068d2eeed6e2f0f0d56712ac2e4f162cfad40a
];

macro_rules! bid_kx192 {
  ($index:expr) => {
    BID_KX192[$index as usize]
  };
}
pub(crate) use bid_kx192;

#[rustfmt::skip]
pub const BID_KX192: [ BidUint192; 56] = [
  BidUint192{ w: [0xcccccccccccccccd, 0xcccccccccccccccc, 0xcccccccccccccccc ] }, //  10^-1 ~= cccccccccccccccccccccccccccccccccccccccccccccccd * 2^-195
  BidUint192{ w: [0xd70a3d70a3d70a3e, 0x3d70a3d70a3d70a3, 0xa3d70a3d70a3d70a ] }, //  10^-2 ~= a3d70a3d70a3d70a3d70a3d70a3d70a3d70a3d70a3d70a3e * 2^-198
  BidUint192{ w: [0x78d4fdf3b645a1cb, 0x645a1cac083126e9, 0x83126e978d4fdf3b ] }, //  10^-3 ~= 83126e978d4fdf3b645a1cac083126e978d4fdf3b645a1cb * 2^-201
  BidUint192{ w: [0xc154c985f06f6945, 0xd3c36113404ea4a8, 0xd1b71758e219652b ] }, //  10^-4 ~= d1b71758e219652bd3c36113404ea4a8c154c985f06f6945 * 2^-205
  BidUint192{ w: [0xcddd6e04c0592104, 0x0fcf80dc33721d53, 0xa7c5ac471b478423 ] }, //  10^-5 ~= a7c5ac471b4784230fcf80dc33721d53cddd6e04c0592104 * 2^-208
  BidUint192{ w: [0xd7e45803cd141a6a, 0xa63f9a49c2c1b10f, 0x8637bd05af6c69b5 ] }, //  10^-6 ~= 8637bd05af6c69b5a63f9a49c2c1b10fd7e45803cd141a6a * 2^-211
  BidUint192{ w: [0x8ca08cd2e1b9c3dc, 0x3d32907604691b4c, 0xd6bf94d5e57a42bc ] }, //  10^-7 ~= d6bf94d5e57a42bc3d32907604691b4c8ca08cd2e1b9c3dc * 2^-215
  BidUint192{ w: [0x3d4d3d758161697d, 0xfdc20d2b36ba7c3d, 0xabcc77118461cefc ] }, //  10^-8 ~= abcc77118461cefcfdc20d2b36ba7c3d3d4d3d758161697d * 2^-218
  BidUint192{ w: [0xfdd7645e011abaca, 0x31680a88f8953030, 0x89705f4136b4a597 ] }, //  10^-9 ~= 89705f4136b4a59731680a88f8953030fdd7645e011abaca * 2^-221
  BidUint192{ w: [0x2fbf06fcce912add, 0xb573440e5a884d1b, 0xdbe6fecebdedd5be ] }, // 10^-10 ~= dbe6fecebdedd5beb573440e5a884d1b2fbf06fcce912add * 2^-225
  BidUint192{ w: [0xf2ff38ca3eda88b1, 0xf78f69a51539d748, 0xafebff0bcb24aafe ] }, // 10^-11 ~= afebff0bcb24aafef78f69a51539d748f2ff38ca3eda88b1 * 2^-228
  BidUint192{ w: [0xf598fa3b657ba08e, 0xf93f87b7442e45d3, 0x8cbccc096f5088cb ] }, // 10^-12 ~= 8cbccc096f5088cbf93f87b7442e45d3f598fa3b657ba08e * 2^-231
  BidUint192{ w: [0x88f4c3923bf900e3, 0x2865a5f206b06fb9, 0xe12e13424bb40e13 ] }, // 10^-13 ~= e12e13424bb40e132865a5f206b06fb988f4c3923bf900e3 * 2^-235
  BidUint192{ w: [0x6d909c74fcc733e9, 0x538484c19ef38c94, 0xb424dc35095cd80f ] }, // 10^-14 ~= b424dc35095cd80f538484c19ef38c946d909c74fcc733e9 * 2^-238
  BidUint192{ w: [0x57a6e390ca38f654, 0x0f9d37014bf60a10, 0x901d7cf73ab0acd9 ] }, // 10^-15 ~= 901d7cf73ab0acd90f9d37014bf60a1057a6e390ca38f654 * 2^-241
  BidUint192{ w: [0xbf716c1add27f086, 0x4c2ebe687989a9b3, 0xe69594bec44de15b ] }, // 10^-16 ~= e69594bec44de15b4c2ebe687989a9b3bf716c1add27f086 * 2^-245
  BidUint192{ w: [0xff8df0157db98d38, 0x09befeb9fad487c2, 0xb877aa3236a4b449 ] }, // 10^-17 ~= b877aa3236a4b44909befeb9fad487c2ff8df0157db98d38 * 2^-248
  BidUint192{ w: [0x32d7f344649470fa, 0x3aff322e62439fcf, 0x9392ee8e921d5d07 ] }, // 10^-18 ~= 9392ee8e921d5d073aff322e62439fcf32d7f344649470fa * 2^-251
  BidUint192{ w: [0x1e2652070753e7f5, 0x2b31e9e3d06c32e5, 0xec1e4a7db69561a5 ] }, // 10^-19 ~= ec1e4a7db69561a52b31e9e3d06c32e51e2652070753e7f5 * 2^-255
  BidUint192{ w: [0x181ea8059f76532b, 0x88f4bb1ca6bcf584, 0xbce5086492111aea ] }, // 10^-20 ~= bce5086492111aea88f4bb1ca6bcf584181ea8059f76532b * 2^-258
  BidUint192{ w: [0x467eecd14c5ea8ef, 0xd3f6fc16ebca5e03, 0x971da05074da7bee ] }, // 10^-21 ~= 971da05074da7beed3f6fc16ebca5e03467eecd14c5ea8ef * 2^-261
  BidUint192{ w: [0x70cb148213caa7e5, 0x5324c68b12dd6338, 0xf1c90080baf72cb1 ] }, // 10^-22 ~= f1c90080baf72cb15324c68b12dd633870cb148213caa7e5 * 2^-265
  BidUint192{ w: [0x8d6f439b43088651, 0x75b7053c0f178293, 0xc16d9a0095928a27 ] }, // 10^-23 ~= c16d9a0095928a2775b7053c0f1782938d6f439b43088651 * 2^-268
  BidUint192{ w: [0xd78c3615cf3a050d, 0xc4926a9672793542, 0x9abe14cd44753b52 ] }, // 10^-24 ~= 9abe14cd44753b52c4926a9672793542d78c3615cf3a050d * 2^-271
  BidUint192{ w: [0x8c1389bc7ec33b48, 0x3a83ddbd83f52204, 0xf79687aed3eec551 ] }, // 10^-25 ~= f79687aed3eec5513a83ddbd83f522048c1389bc7ec33b48 * 2^-275
  BidUint192{ w: [0x3cdc6e306568fc3a, 0x95364afe032a819d, 0xc612062576589dda ] }, // 10^-26 ~= c612062576589dda95364afe032a819d3cdc6e306568fc3a * 2^-278
  BidUint192{ w: [0xca49f1c05120c9c8, 0x775ea264cf55347d, 0x9e74d1b791e07e48 ] }, // 10^-27 ~= 9e74d1b791e07e48775ea264cf55347dca49f1c05120c9c8 * 2^-281
  BidUint192{ w: [0x76dcb60081ce0fa6, 0x8bca9d6e188853fc, 0xfd87b5f28300ca0d ] }, // 10^-28 ~= fd87b5f28300ca0d8bca9d6e188853fc76dcb60081ce0fa6 * 2^-285
  BidUint192{ w: [0x5f16f80067d80c85, 0x096ee45813a04330, 0xcad2f7f5359a3b3e ] }, // 10^-29 ~= cad2f7f5359a3b3e096ee45813a043305f16f80067d80c85 * 2^-288
  BidUint192{ w: [0x18df2ccd1fe00a04, 0xa1258379a94d028d, 0xa2425ff75e14fc31 ] }, // 10^-30 ~= a2425ff75e14fc31a1258379a94d028d18df2ccd1fe00a04 * 2^-291
  BidUint192{ w: [0x4718f0a419800803, 0x80eacf948770ced7, 0x81ceb32c4b43fcf4 ] }, // 10^-31 ~= 81ceb32c4b43fcf480eacf948770ced74718f0a419800803 * 2^-294
  BidUint192{ w: [0x0b5b1aa028ccd99f, 0x67de18eda5814af2, 0xcfb11ead453994ba ] }, // 10^-32 ~= cfb11ead453994ba67de18eda5814af20b5b1aa028ccd99f * 2^-298
  BidUint192{ w: [0x6f7c154ced70ae19, 0xecb1ad8aeacdd58e, 0xa6274bbdd0fadd61 ] }, // 10^-33 ~= a6274bbdd0fadd61ecb1ad8aeacdd58e6f7c154ced70ae19 * 2^-301
  BidUint192{ w: [0xbf967770bdf3be7a, 0xbd5af13bef0b113e, 0x84ec3c97da624ab4 ] }, // 10^-34 ~= 84ec3c97da624ab4bd5af13bef0b113ebf967770bdf3be7a * 2^-304
  BidUint192{ w: [0x65bd8be79652ca5d, 0x955e4ec64b44e864, 0xd4ad2dbfc3d07787 ] }, // 10^-35 ~= d4ad2dbfc3d07787955e4ec64b44e86465bd8be79652ca5d * 2^-308
  BidUint192{ w: [0xeafe098611dbd517, 0xdde50bd1d5d0b9e9, 0xaa242499697392d2 ] }, // 10^-36 ~= aa242499697392d2dde50bd1d5d0b9e9eafe098611dbd517 * 2^-311
  BidUint192{ w: [0xbbfe6e04db164413, 0x7e50d64177da2e54, 0x881cea14545c7575 ] }, // 10^-37 ~= 881cea14545c75757e50d64177da2e54bbfe6e04db164413 * 2^-314
  BidUint192{ w: [0x2cca49a15e8a0684, 0x96e7bd358c904a21, 0xd9c7dced53c72255 ] }, // 10^-38 ~= d9c7dced53c7225596e7bd358c904a212cca49a15e8a0684 * 2^-318
  BidUint192{ w: [0x8a3b6e1ab2080537, 0xabec975e0a0d081a, 0xae397d8aa96c1b77 ] }, // 10^-39 ~= ae397d8aa96c1b77abec975e0a0d081a8a3b6e1ab2080537 * 2^-321
  BidUint192{ w: [0x3b62be7bc1a0042c, 0x2323ac4b3b3da015, 0x8b61313bbabce2c6 ] }, // 10^-40 ~= 8b61313bbabce2c62323ac4b3b3da0153b62be7bc1a0042c * 2^-324
  BidUint192{ w: [0x5f0463f935ccd379, 0x6b6c46dec52f6688, 0xdf01e85f912e37a3 ] }, // 10^-41 ~= df01e85f912e37a36b6c46dec52f66885f0463f935ccd379 * 2^-328
  BidUint192{ w: [0x7f36b660f7d70f94, 0x55f038b237591ed3, 0xb267ed1940f1c61c ] }, // 10^-42 ~= b267ed1940f1c61c55f038b237591ed37f36b660f7d70f94 * 2^-331
  BidUint192{ w: [0xcc2bc51a5fdf3faa, 0x77f3608e92adb242, 0x8eb98a7a9a5b04e3 ] }, // 10^-43 ~= 8eb98a7a9a5b04e377f3608e92adb242cc2bc51a5fdf3faa * 2^-334
  BidUint192{ w: [0xe046082a32fecc42, 0x8cb89a7db77c506a, 0xe45c10c42a2b3b05 ] }, // 10^-44 ~= e45c10c42a2b3b058cb89a7db77c506ae046082a32fecc42 * 2^-338
  BidUint192{ w: [0x4d04d354f598a368, 0x3d607b97c5fd0d22, 0xb6b00d69bb55c8d1 ] }, // 10^-45 ~= b6b00d69bb55c8d13d607b97c5fd0d224d04d354f598a368 * 2^-341
  BidUint192{ w: [0x3d9d75dd9146e920, 0xcab3961304ca70e8, 0x9226712162ab070d ] }, // 10^-46 ~= 9226712162ab070dcab3961304ca70e83d9d75dd9146e920 * 2^-344
  BidUint192{ w: [0xc8fbefc8e8717500, 0xaab8f01e6e10b4a6, 0xe9d71b689dde71af ] }, // 10^-47 ~= e9d71b689dde71afaab8f01e6e10b4a6c8fbefc8e8717500 * 2^-348
  BidUint192{ w: [0x3a63263a538df734, 0x5560c018580d5d52, 0xbb127c53b17ec159 ] }, // 10^-48 ~= bb127c53b17ec1595560c018580d5d523a63263a538df734 * 2^-351
  BidUint192{ w: [0x2eb5b82ea93e5f5d, 0xdde7001379a44aa8, 0x95a8637627989aad ] }, // 10^-49 ~= 95a8637627989aaddde7001379a44aa82eb5b82ea93e5f5d * 2^-354
  BidUint192{ w: [0x4abc59e441fd6561, 0x963e66858f6d4440, 0xef73d256a5c0f77c ] }, // 10^-50 ~= ef73d256a5c0f77c963e66858f6d44404abc59e441fd6561 * 2^-358
  BidUint192{ w: [0x6efd14b69b311de7, 0xde98520472bdd033, 0xbf8fdb78849a5f96 ] }, // 10^-51 ~= bf8fdb78849a5f96de98520472bdd0336efd14b69b311de7 * 2^-361
  BidUint192{ w: [0x259743c548f417ec, 0xe546a8038efe4029, 0x993fe2c6d07b7fab ] }, // 10^-52 ~= 993fe2c6d07b7fabe546a8038efe4029259743c548f417ec * 2^-364
  BidUint192{ w: [0x3c25393ba7ecf313, 0xd53dd99f4b3066a8, 0xf53304714d9265df ] }, // 10^-53 ~= f53304714d9265dfd53dd99f4b3066a83c25393ba7ecf313 * 2^-368
  BidUint192{ w: [0x96842dc95323f5a9, 0xaa97e14c3c26b886, 0xc428d05aa4751e4c ] }, // 10^-54 ~= c428d05aa4751e4caa97e14c3c26b88696842dc95323f5a9 * 2^-371
  BidUint192{ w: [0xab9cf16ddc1cc487, 0x55464dd69685606b, 0x9ced737bb6c4183d ] }, // 10^-55 ~= 9ced737bb6c4183d55464dd69685606bab9cf16ddc1cc487 * 2^-374
  BidUint192{ w: [0xac2e4f162cfad40b, 0xeed6e2f0f0d56712, 0xfb158592be068d2e ] }  // 10^-56 ~= fb158592be068d2eeed6e2f0f0d56712ac2e4f162cfad40b * 2^-378
];

macro_rules! bid_ex192m192 {
  ($index:expr) => {
    BID_EX192M192[$index as usize]
  };
}
pub(crate) use bid_ex192m192;

#[rustfmt::skip]
pub const BID_EX192M192: [u32; 56] = [
   3,	// 195 - 192, Ex = 195
   6,	// 198 - 192, Ex = 198
   9,	// 201 - 192, Ex = 201
  13, // 205 - 192, Ex = 205
  16, // 208 - 192, Ex = 208
  19, // 211 - 192, Ex = 211
  23, // 215 - 192, Ex = 215
  26, // 218 - 192, Ex = 218
  29, // 221 - 192, Ex = 221
  33, // 225 - 192, Ex = 225
  36, // 228 - 192, Ex = 228
  39, // 231 - 192, Ex = 231
  43, // 235 - 192, Ex = 235
  46, // 238 - 192, Ex = 238
  49, // 241 - 192, Ex = 241
  53, // 245 - 192, Ex = 245
  56, // 248 - 192, Ex = 248
  59, // 251 - 192, Ex = 251
  63, // 255 - 192, Ex = 255
   2,	// 258 - 256, Ex = 258
   5,	// 261 - 256, Ex = 261
   9,	// 265 - 256, Ex = 265
  12, // 268 - 256, Ex = 268
  15, // 271 - 256, Ex = 271
  19, // 275 - 256, Ex = 275
  22, // 278 - 256, Ex = 278
  25, // 281 - 256, Ex = 281
  29, // 285 - 256, Ex = 285
  32, // 288 - 256, Ex = 288
  35, // 291 - 256, Ex = 291
  38, // 294 - 256, Ex = 294
  42, // 298 - 256, Ex = 298
  45, // 301 - 256, Ex = 301
  48, // 304 - 256, Ex = 304
  52, // 308 - 256, Ex = 308
  55, // 311 - 256, Ex = 311
  58, // 314 - 256, Ex = 314
  62, // 318 - 256, Ex = 318
   1,	// 321 - 320, Ex = 321
   4,	// 324 - 320, Ex = 324
   8,	// 328 - 320, Ex = 328
  11, // 331 - 320, Ex = 331
  14, // 334 - 320, Ex = 334
  18, // 338 - 320, Ex = 338
  21, // 341 - 320, Ex = 341
  24, // 344 - 320, Ex = 344
  28, // 348 - 320, Ex = 348
  31, // 351 - 320, Ex = 351
  34, // 354 - 320, Ex = 354
  38, // 358 - 320, Ex = 358
  41, // 361 - 320, Ex = 361
  44, // 364 - 320, Ex = 364
  48, // 368 - 320, Ex = 368
  51, // 371 - 320, Ex = 371
  54, // 374 - 320, Ex = 374
  58	// 378 - 320, Ex = 378
];

macro_rules! bid_kx256 {
  ($index:expr) => {
    BID_KX256[$index as usize]
  };
}
pub(crate) use bid_kx256;

#[rustfmt::skip]
pub const BID_KX256: [BidUint256; 75] = [
  BidUint256{ w: [0xcccccccccccccccd, 0xcccccccccccccccc, 0xcccccccccccccccc, 0xcccccccccccccccc] },  //  10^-1 ~= cccccccccccccccc  cccccccccccccccc  |  cccccccccccccccccccccccccccccccd   * 2^-259
  BidUint256{ w: [0x70a3d70a3d70a3d8, 0xd70a3d70a3d70a3d, 0x3d70a3d70a3d70a3, 0xa3d70a3d70a3d70a] },  //  10^-2 ~= a3d70a3d70a3d70a  3d70a3d70a3d70a3  |  d70a3d70a3d70a3d70a3d70a3d70a3d8   * 2^-262
  BidUint256{ w: [0xc083126e978d4fe0, 0x78d4fdf3b645a1ca, 0x645a1cac083126e9, 0x83126e978d4fdf3b] },  //  10^-3 ~= 83126e978d4fdf3b  645a1cac083126e9  |  78d4fdf3b645a1cac083126e978d4fe0   * 2^-265
  BidUint256{ w: [0x67381d7dbf487fcc, 0xc154c985f06f6944, 0xd3c36113404ea4a8, 0xd1b71758e219652b] },  //  10^-4 ~= d1b71758e219652b  d3c36113404ea4a8  |  c154c985f06f694467381d7dbf487fcc   * 2^-269
  BidUint256{ w: [0x85c67dfe32a0663d, 0xcddd6e04c0592103, 0x0fcf80dc33721d53, 0xa7c5ac471b478423] },  //  10^-5 ~= a7c5ac471b478423  fcf80dc33721d53   |  cddd6e04c059210385c67dfe32a0663d   * 2^-272
  BidUint256{ w: [0x37d1fe64f54d1e97, 0xd7e45803cd141a69, 0xa63f9a49c2c1b10f, 0x8637bd05af6c69b5] },  //  10^-6 ~= 8637bd05af6c69b5  a63f9a49c2c1b10f  |  d7e45803cd141a6937d1fe64f54d1e97   * 2^-275
  BidUint256{ w: [0x8c8330a1887b6425, 0x8ca08cd2e1b9c3db, 0x3d32907604691b4c, 0xd6bf94d5e57a42bc] },  //  10^-7 ~= d6bf94d5e57a42bc  3d32907604691b4c  |  8ca08cd2e1b9c3db8c8330a1887b6425   * 2^-279
  BidUint256{ w: [0x7068f3b46d2f8351, 0x3d4d3d758161697c, 0xfdc20d2b36ba7c3d, 0xabcc77118461cefc] },  //  10^-8 ~= abcc77118461cefc  fdc20d2b36ba7c3d  |  3d4d3d758161697c7068f3b46d2f8351   * 2^-282
  BidUint256{ w: [0xf387295d242602a7, 0xfdd7645e011abac9, 0x31680a88f8953030, 0x89705f4136b4a597] },  //  10^-9 ~= 89705f4136b4a597  31680a88f8953030  |  fdd7645e011abac9f387295d242602a7   * 2^-285
  BidUint256{ w: [0xb8d8422ea03cd10b, 0x2fbf06fcce912adc, 0xb573440e5a884d1b, 0xdbe6fecebdedd5be] },  // 10^-10 ~= dbe6fecebdedd5be  b573440e5a884d1b  |  2fbf06fcce912adcb8d8422ea03cd10b   * 2^-289
  BidUint256{ w: [0x93e034f219ca40d6, 0xf2ff38ca3eda88b0, 0xf78f69a51539d748, 0xafebff0bcb24aafe] },  // 10^-11 ~= afebff0bcb24aafe  f78f69a51539d748  |  f2ff38ca3eda88b093e034f219ca40d6   * 2^-292
  BidUint256{ w: [0x4319c3f4e16e9a45, 0xf598fa3b657ba08d, 0xf93f87b7442e45d3, 0x8cbccc096f5088cb] },  // 10^-12 ~= 8cbccc096f5088cb  f93f87b7442e45d3  |  f598fa3b657ba08d4319c3f4e16e9a45   * 2^-295
  BidUint256{ w: [0x04f606549be42a07, 0x88f4c3923bf900e2, 0x2865a5f206b06fb9, 0xe12e13424bb40e13] },  // 10^-13 ~= e12e13424bb40e13  2865a5f206b06fb9  |  88f4c3923bf900e204f606549be42a07   * 2^-299
  BidUint256{ w: [0x03f805107cb68806, 0x6d909c74fcc733e8, 0x538484c19ef38c94, 0xb424dc35095cd80f] },  // 10^-14 ~= b424dc35095cd80f  538484c19ef38c94  |  6d909c74fcc733e803f805107cb68806   * 2^-302
  BidUint256{ w: [0x3660040d3092066b, 0x57a6e390ca38f653, 0x0f9d37014bf60a10, 0x901d7cf73ab0acd9] },  // 10^-15 ~= 901d7cf73ab0acd9  f9d37014bf60a10   |  57a6e390ca38f6533660040d3092066b   * 2^-305
  BidUint256{ w: [0x23ccd3484db670ab, 0xbf716c1add27f085, 0x4c2ebe687989a9b3, 0xe69594bec44de15b] },  // 10^-16 ~= e69594bec44de15b  4c2ebe687989a9b3  |  bf716c1add27f08523ccd3484db670ab   * 2^-309
  BidUint256{ w: [0x4fd70f6d0af85a23, 0xff8df0157db98d37, 0x09befeb9fad487c2, 0xb877aa3236a4b449] },  // 10^-17 ~= b877aa3236a4b449  9befeb9fad487c2   |  ff8df0157db98d374fd70f6d0af85a23   * 2^-312
  BidUint256{ w: [0x0cac0c573bf9e1b6, 0x32d7f344649470f9, 0x3aff322e62439fcf, 0x9392ee8e921d5d07] },  // 10^-18 ~= 9392ee8e921d5d07  3aff322e62439fcf  |  32d7f344649470f90cac0c573bf9e1b6   * 2^-315
  BidUint256{ w: [0xe11346f1f98fcf89, 0x1e2652070753e7f4, 0x2b31e9e3d06c32e5, 0xec1e4a7db69561a5] },  // 10^-19 ~= ec1e4a7db69561a5  2b31e9e3d06c32e5  |  1e2652070753e7f4e11346f1f98fcf89   * 2^-319
  BidUint256{ w: [0x4da9058e613fd93a, 0x181ea8059f76532a, 0x88f4bb1ca6bcf584, 0xbce5086492111aea] },  // 10^-20 ~= bce5086492111aea  88f4bb1ca6bcf584  |  181ea8059f76532a4da9058e613fd93a   * 2^-322
  BidUint256{ w: [0xa48737a51a997a95, 0x467eecd14c5ea8ee, 0xd3f6fc16ebca5e03, 0x971da05074da7bee] },  // 10^-21 ~= 971da05074da7bee  d3f6fc16ebca5e03  |  467eecd14c5ea8eea48737a51a997a95   * 2^-325
  BidUint256{ w: [0x3a71f2a1c428c421, 0x70cb148213caa7e4, 0x5324c68b12dd6338, 0xf1c90080baf72cb1] },  // 10^-22 ~= f1c90080baf72cb1  5324c68b12dd6338  |  70cb148213caa7e43a71f2a1c428c421   * 2^-329
  BidUint256{ w: [0x2ec18ee7d0209ce8, 0x8d6f439b43088650, 0x75b7053c0f178293, 0xc16d9a0095928a27] },  // 10^-23 ~= c16d9a0095928a27  75b7053c0f178293  |  8d6f439b430886502ec18ee7d0209ce8   * 2^-332
  BidUint256{ w: [0xf23472530ce6e3ed, 0xd78c3615cf3a050c, 0xc4926a9672793542, 0x9abe14cd44753b52] },  // 10^-24 ~= 9abe14cd44753b52  c4926a9672793542  |  d78c3615cf3a050cf23472530ce6e3ed   * 2^-335
  BidUint256{ w: [0xe9ed83b814a49fe1, 0x8c1389bc7ec33b47, 0x3a83ddbd83f52204, 0xf79687aed3eec551] },  // 10^-25 ~= f79687aed3eec551  3a83ddbd83f52204  |  8c1389bc7ec33b47e9ed83b814a49fe1   * 2^-339
  BidUint256{ w: [0x87f1362cdd507fe7, 0x3cdc6e306568fc39, 0x95364afe032a819d, 0xc612062576589dda] },  // 10^-26 ~= c612062576589dda  95364afe032a819d  |  3cdc6e306568fc3987f1362cdd507fe7   * 2^-342
  BidUint256{ w: [0x9ff42b5717739986, 0xca49f1c05120c9c7, 0x775ea264cf55347d, 0x9e74d1b791e07e48] },  // 10^-27 ~= 9e74d1b791e07e48  775ea264cf55347d  |  ca49f1c05120c9c79ff42b5717739986   * 2^-345
  BidUint256{ w: [0xccb9def1bf1f5c09, 0x76dcb60081ce0fa5, 0x8bca9d6e188853fc, 0xfd87b5f28300ca0d] },  // 10^-28 ~= fd87b5f28300ca0d  8bca9d6e188853fc  |  76dcb60081ce0fa5ccb9def1bf1f5c09   * 2^-349
  BidUint256{ w: [0xa3c7e58e327f7cd4, 0x5f16f80067d80c84, 0x096ee45813a04330, 0xcad2f7f5359a3b3e] },  // 10^-29 ~= cad2f7f5359a3b3e  96ee45813a04330   |  5f16f80067d80c84a3c7e58e327f7cd4   * 2^-352
  BidUint256{ w: [0xb6398471c1ff9710, 0x18df2ccd1fe00a03, 0xa1258379a94d028d, 0xa2425ff75e14fc31] },  // 10^-30 ~= a2425ff75e14fc31  a1258379a94d028d  |  18df2ccd1fe00a03b6398471c1ff9710   * 2^-355
  BidUint256{ w: [0xf82e038e34cc78da, 0x4718f0a419800802, 0x80eacf948770ced7, 0x81ceb32c4b43fcf4] },  // 10^-31 ~= 81ceb32c4b43fcf4  80eacf948770ced7  |  4718f0a419800802f82e038e34cc78da   * 2^-358
  BidUint256{ w: [0x59e338e387ad8e29, 0x0b5b1aa028ccd99e, 0x67de18eda5814af2, 0xcfb11ead453994ba] },  // 10^-32 ~= cfb11ead453994ba  67de18eda5814af2  |  b5b1aa028ccd99e59e338e387ad8e29    * 2^-362
  BidUint256{ w: [0x47e8fa4f9fbe0b54, 0x6f7c154ced70ae18, 0xecb1ad8aeacdd58e, 0xa6274bbdd0fadd61] },  // 10^-33 ~= a6274bbdd0fadd61  ecb1ad8aeacdd58e  |  6f7c154ced70ae1847e8fa4f9fbe0b54   * 2^-365
  BidUint256{ w: [0xd320c83fb2fe6f76, 0xbf967770bdf3be79, 0xbd5af13bef0b113e, 0x84ec3c97da624ab4] },  // 10^-34 ~= 84ec3c97da624ab4  bd5af13bef0b113e  |  bf967770bdf3be79d320c83fb2fe6f76   * 2^-368
  BidUint256{ w: [0x85014065eb30b257, 0x65bd8be79652ca5c, 0x955e4ec64b44e864, 0xd4ad2dbfc3d07787] },  // 10^-35 ~= d4ad2dbfc3d07787  955e4ec64b44e864  |  65bd8be79652ca5c85014065eb30b257   * 2^-372
  BidUint256{ w: [0xd0cdcd1e55c08eac, 0xeafe098611dbd516, 0xdde50bd1d5d0b9e9, 0xaa242499697392d2] },  // 10^-36 ~= aa242499697392d2  dde50bd1d5d0b9e9  |  eafe098611dbd516d0cdcd1e55c08eac   * 2^-375
  BidUint256{ w: [0x40a4a418449a0bbd, 0xbbfe6e04db164412, 0x7e50d64177da2e54, 0x881cea14545c7575] },  // 10^-37 ~= 881cea14545c7575  7e50d64177da2e54  |  bbfe6e04db16441240a4a418449a0bbd   * 2^-378
  BidUint256{ w: [0x9aa1068d3a9012c8, 0x2cca49a15e8a0683, 0x96e7bd358c904a21, 0xd9c7dced53c72255] },  // 10^-38 ~= d9c7dced53c72255  96e7bd358c904a21  |  2cca49a15e8a06839aa1068d3a9012c8   * 2^-382
  BidUint256{ w: [0x154d9ed7620cdbd3, 0x8a3b6e1ab2080536, 0xabec975e0a0d081a, 0xae397d8aa96c1b77] },  // 10^-39 ~= ae397d8aa96c1b77  abec975e0a0d081a  |  8a3b6e1ab2080536154d9ed7620cdbd3   * 2^-385
  BidUint256{ w: [0x443e18ac4e70afdc, 0x3b62be7bc1a0042b, 0x2323ac4b3b3da015, 0x8b61313bbabce2c6] },  // 10^-40 ~= 8b61313bbabce2c6  2323ac4b3b3da015  |  3b62be7bc1a0042b443e18ac4e70afdc   * 2^-388
  BidUint256{ w: [0x6d30277a171ab2f9, 0x5f0463f935ccd378, 0x6b6c46dec52f6688, 0xdf01e85f912e37a3] },  // 10^-41 ~= df01e85f912e37a3  6b6c46dec52f6688  |  5f0463f935ccd3786d30277a171ab2f9   * 2^-392
  BidUint256{ w: [0x8a8cec61ac155bfb, 0x7f36b660f7d70f93, 0x55f038b237591ed3, 0xb267ed1940f1c61c] },  // 10^-42 ~= b267ed1940f1c61c  55f038b237591ed3  |  7f36b660f7d70f938a8cec61ac155bfb   * 2^-395
  BidUint256{ w: [0x3ba3f04e23444996, 0xcc2bc51a5fdf3fa9, 0x77f3608e92adb242, 0x8eb98a7a9a5b04e3] },  // 10^-43 ~= 8eb98a7a9a5b04e3  77f3608e92adb242  |  cc2bc51a5fdf3fa93ba3f04e23444996   * 2^-398
  BidUint256{ w: [0xf9064d49d206dc22, 0xe046082a32fecc41, 0x8cb89a7db77c506a, 0xe45c10c42a2b3b05] },  // 10^-44 ~= e45c10c42a2b3b05  8cb89a7db77c506a  |  e046082a32fecc41f9064d49d206dc22   * 2^-402
  BidUint256{ w: [0xfa6b7107db38b01b, 0x4d04d354f598a367, 0x3d607b97c5fd0d22, 0xb6b00d69bb55c8d1] },  // 10^-45 ~= b6b00d69bb55c8d1  3d607b97c5fd0d22  |  4d04d354f598a367fa6b7107db38b01b   * 2^-405
  BidUint256{ w: [0xfb8927397c2d59b0, 0x3d9d75dd9146e91f, 0xcab3961304ca70e8, 0x9226712162ab070d] },  // 10^-46 ~= 9226712162ab070d  cab3961304ca70e8  |  3d9d75dd9146e91ffb8927397c2d59b0   * 2^-408
  BidUint256{ w: [0xf8db71f5937bc2b2, 0xc8fbefc8e87174ff, 0xaab8f01e6e10b4a6, 0xe9d71b689dde71af] },  // 10^-47 ~= e9d71b689dde71af  aab8f01e6e10b4a6  |  c8fbefc8e87174fff8db71f5937bc2b2   * 2^-412
  BidUint256{ w: [0x2d7c5b2adc630228, 0x3a63263a538df733, 0x5560c018580d5d52, 0xbb127c53b17ec159] },  // 10^-48 ~= bb127c53b17ec159  5560c018580d5d52  |  3a63263a538df7332d7c5b2adc630228   * 2^-415
  BidUint256{ w: [0x24637c2249e8ce87, 0x2eb5b82ea93e5f5c, 0xdde7001379a44aa8, 0x95a8637627989aad] },  // 10^-49 ~= 95a8637627989aad  dde7001379a44aa8  |  2eb5b82ea93e5f5c24637c2249e8ce87   * 2^-418
  BidUint256{ w: [0x3a38c69d430e173e, 0x4abc59e441fd6560, 0x963e66858f6d4440, 0xef73d256a5c0f77c] },  // 10^-50 ~= ef73d256a5c0f77c  963e66858f6d4440  |  4abc59e441fd65603a38c69d430e173e   * 2^-422
  BidUint256{ w: [0x94fa387dcf3e78fe, 0x6efd14b69b311de6, 0xde98520472bdd033, 0xbf8fdb78849a5f96] },  // 10^-51 ~= bf8fdb78849a5f96  de98520472bdd033  |  6efd14b69b311de694fa387dcf3e78fe   * 2^-425
  BidUint256{ w: [0xaa61c6cb0c31fa65, 0x259743c548f417eb, 0xe546a8038efe4029, 0x993fe2c6d07b7fab] },  // 10^-52 ~= 993fe2c6d07b7fab  e546a8038efe4029  |  259743c548f417ebaa61c6cb0c31fa65   * 2^-428
  BidUint256{ w: [0xaa360ade79e990a2, 0x3c25393ba7ecf312, 0xd53dd99f4b3066a8, 0xf53304714d9265df] },  // 10^-53 ~= f53304714d9265df  d53dd99f4b3066a8  |  3c25393ba7ecf312aa360ade79e990a2   * 2^-432
  BidUint256{ w: [0x882b3be52e5473b5, 0x96842dc95323f5a8, 0xaa97e14c3c26b886, 0xc428d05aa4751e4c] },  // 10^-54 ~= c428d05aa4751e4c  aa97e14c3c26b886  |  96842dc95323f5a8882b3be52e5473b5   * 2^-435
  BidUint256{ w: [0xd355c98425105c91, 0xab9cf16ddc1cc486, 0x55464dd69685606b, 0x9ced737bb6c4183d] },  // 10^-55 ~= 9ced737bb6c4183d  55464dd69685606b  |  ab9cf16ddc1cc486d355c98425105c91   * 2^-438
  BidUint256{ w: [0xebbc75a03b4d60e7, 0xac2e4f162cfad40a, 0xeed6e2f0f0d56712, 0xfb158592be068d2e] },  // 10^-56 ~= fb158592be068d2e  eed6e2f0f0d56712  |  ac2e4f162cfad40aebbc75a03b4d60e7   * 2^-442
  BidUint256{ w: [0x8963914cfc3de71f, 0x568b727823fbdcd5, 0xf245825a5a445275, 0xc8de047564d20a8b] },  // 10^-57 ~= c8de047564d20a8b  f245825a5a445275  |  568b727823fbdcd58963914cfc3de71f   * 2^-445
  BidUint256{ w: [0xd44fa770c9cb1f4c, 0x453c5b934ffcb0aa, 0x5b6aceaeae9d0ec4, 0xa0b19d2ab70e6ed6] },  // 10^-58 ~= a0b19d2ab70e6ed6  5b6aceaeae9d0ec4  |  453c5b934ffcb0aad44fa770c9cb1f4c   * 2^-448
  BidUint256{ w: [0xdd0c85f3d4a27f70, 0x37637c75d996f3bb, 0xe2bbd88bbee40bd0, 0x808e17555f3ebf11] },  // 10^-59 ~= 808e17555f3ebf11  e2bbd88bbee40bd0  |  37637c75d996f3bbdd0c85f3d4a27f70   * 2^-451
  BidUint256{ w: [0x61ada31fba9d98b3, 0x256bfa5628f185f9, 0x3792f412cb06794d, 0xcdb02555653131b6] },  // 10^-60 ~= cdb02555653131b6  3792f412cb06794d  |  256bfa5628f185f961ada31fba9d98b3   * 2^-455
  BidUint256{ w: [0xe7be1c196217ad5c, 0x51232eab53f46b2d, 0x5fa8c3423c052dd7, 0xa48ceaaab75a8e2b] },  // 10^-61 ~= a48ceaaab75a8e2b  5fa8c3423c052dd7  |  51232eab53f46b2de7be1c196217ad5c   * 2^-458
  BidUint256{ w: [0x52fe7ce11b46244a, 0x40e8f222a99055be, 0x1953cf68300424ac, 0x83a3eeeef9153e89] },  // 10^-62 ~= 83a3eeeef9153e89  1953cf68300424ac  |  40e8f222a99055be52fe7ce11b46244a   * 2^-461
  BidUint256{ w: [0x51972e34f8703a10, 0x34a7e9d10f4d55fd, 0x8eec7f0d19a03aad, 0xd29fe4b18e88640e] },  // 10^-63 ~= d29fe4b18e88640e  8eec7f0d19a03aad  |  34a7e9d10f4d55fd51972e34f8703a10   * 2^-465
  BidUint256{ w: [0x0e128b5d938cfb40, 0x2a1fee40d90aab31, 0x3f2398d747b36224, 0xa87fea27a539e9a5] },  // 10^-64 ~= a87fea27a539e9a5  3f2398d747b36224  |  2a1fee40d90aab310e128b5d938cfb40   * 2^-468
  BidUint256{ w: [0x3e753c4adc70c900, 0xbb4cbe9a473bbc27, 0x98e947129fc2b4e9, 0x86ccbb52ea94baea] },  // 10^-65 ~= 86ccbb52ea94baea  98e947129fc2b4e9  |  bb4cbe9a473bbc273e753c4adc70c900   * 2^-471
  BidUint256{ w: [0x30bb93aafa4e0e66, 0x9214642a0b92c6a5, 0x5b0ed81dcc6abb0f, 0xd7adf884aa879177] },  // 10^-66 ~= d7adf884aa879177  5b0ed81dcc6abb0f  |  9214642a0b92c6a530bb93aafa4e0e66   * 2^-475
  BidUint256{ w: [0xc0960fbbfb71a51f, 0xa8105021a2dbd21d, 0xe272467e3d222f3f, 0xac8b2d36eed2dac5] },  // 10^-67 ~= ac8b2d36eed2dac5  e272467e3d222f3f  |  a8105021a2dbd21dc0960fbbfb71a51f   * 2^-478
  BidUint256{ w: [0x66de72fcc927b74c, 0xb9a6a6814f1641b1, 0x1b8e9ecb641b58ff, 0x8a08f0f8bf0f156b] },  // 10^-68 ~= 8a08f0f8bf0f156b  1b8e9ecb641b58ff  |  b9a6a6814f1641b166de72fcc927b74c   * 2^-481
  BidUint256{ w: [0xd7ca5194750c5879, 0xf5d770cee4f0691b, 0xf8e431456cf88e65, 0xdcdb1b2798182244] },  // 10^-69 ~= dcdb1b2798182244  f8e431456cf88e65  |  f5d770cee4f0691bd7ca5194750c5879   * 2^-485
  BidUint256{ w: [0xdfd50e105da379fa, 0x9179270bea59edaf, 0x2d835a9df0c6d851, 0xb0af48ec79ace837] },  // 10^-70 ~= b0af48ec79ace837  2d835a9df0c6d851  |  9179270bea59edafdfd50e105da379fa   * 2^-488
  BidUint256{ w: [0x19773e737e1c6195, 0x0dfa85a321e18af3, 0x579c487e5a38ad0e, 0x8d590723948a535f] },  // 10^-71 ~= 8d590723948a535f  579c487e5a38ad0e  |  dfa85a321e18af319773e737e1c6195    * 2^-491
  BidUint256{ w: [0xf58b971f302d68ef, 0x165da29e9c9c1184, 0x25c6da63c38de1b0, 0xe2280b6c20dd5232] },  // 10^-72 ~= e2280b6c20dd5232  25c6da63c38de1b0  |  165da29e9c9c1184f58b971f302d68ef   * 2^-495
  BidUint256{ w: [0xc46fac18f3578725, 0x4517b54bb07cdad0, 0x1e38aeb6360b1af3, 0xb4ecd5f01a4aa828] },  // 10^-73 ~= b4ecd5f01a4aa828  1e38aeb6360b1af3  |  4517b54bb07cdad0c46fac18f3578725   * 2^-498
  BidUint256{ w: [0x36bfbce0c2ac6c1e, 0x9dac910959fd7bda, 0xb1c6f22b5e6f48c2, 0x90bd77f3483bb9b9] },  // 10^-74 ~= 90bd77f3483bb9b9  b1c6f22b5e6f48c2  |  9dac910959fd7bda36bfbce0c2ac6c1e   * 2^-501
  BidUint256{ w: [0x2465fb01377a4696, 0x2f7a81a88ffbf95d, 0xb60b1d1230b20e04, 0xe7958cb87392c2c2] }   // 10^-75 ~= e7958cb87392c2c2  b60b1d1230b20e04  |  2f7a81a88ffbf95d2465fb01377a4696   * 2^-505
];

macro_rules! bid_ex256m256 {
  ($index:expr) => {
    BID_EX256M256[$index as usize]
  };
}
pub(crate) use bid_ex256m256;

#[rustfmt::skip]
pub const BID_EX256M256: [u32; 75] = [
   3, // 259 - 256, Ex = 259
   6, // 262 - 256, Ex = 262
   9, // 265 - 256, Ex = 265
  13, // 269 - 256, Ex = 269
  16, // 272 - 256, Ex = 272
  19, // 275 - 256, Ex = 275
  23, // 279 - 256, Ex = 279
  26, // 282 - 256, Ex = 282
  29, // 285 - 256, Ex = 285
  33, // 289 - 256, Ex = 289
  36, // 292 - 256, Ex = 292
  39, // 295 - 256, Ex = 295
  43, // 299 - 256, Ex = 299
  46, // 302 - 256, Ex = 302
  49, // 305 - 256, Ex = 305
  53, // 309 - 256, Ex = 309
  56, // 312 - 256, Ex = 312
  59, // 315 - 256, Ex = 315
  63, // 319 - 256, Ex = 319
   2, // 322 - 320, Ex = 322
   5, // 325 - 320, Ex = 325
   9, // 329 - 320, Ex = 329
  12, // 332 - 320, Ex = 332
  15, // 335 - 320, Ex = 335
  19, // 339 - 320, Ex = 339
  22, // 342 - 320, Ex = 342
  25, // 345 - 320, Ex = 345
  29, // 349 - 320, Ex = 349
  32, // 352 - 320, Ex = 352
  35, // 355 - 320, Ex = 355
  38, // 358 - 320, Ex = 358
  42, // 362 - 320, Ex = 362
  45, // 365 - 320, Ex = 365
  48, // 368 - 320, Ex = 368
  52, // 372 - 320, Ex = 372
  55, // 375 - 320, Ex = 375
  58, // 378 - 320, Ex = 378
  62, // 382 - 320, Ex = 382
   1, // 385 - 384, Ex = 385
   4, // 388 - 384, Ex = 388
   8, // 392 - 384, Ex = 392
  11, // 395 - 384, Ex = 395
  14, // 398 - 384, Ex = 398
  18, // 402 - 384, Ex = 402
  21, // 405 - 384, Ex = 405
  24, // 408 - 384, Ex = 408
  28, // 412 - 384, Ex = 412
  31, // 415 - 384, Ex = 415
  34, // 418 - 384, Ex = 418
  38, // 422 - 384, Ex = 422
  41, // 425 - 384, Ex = 425
  44, // 428 - 384, Ex = 428
  48, // 432 - 384, Ex = 432
  51, // 435 - 384, Ex = 435
  54, // 438 - 384, Ex = 438
  58, // 442 - 384, Ex = 442
  61, // 445 - 384, Ex = 445
   0, // 448 - 448, Ex = 448
   3, // 451 - 448, Ex = 451
   7, // 455 - 448, Ex = 455
  10, // 458 - 448, Ex = 458
  13, // 461 - 448, Ex = 461
  17, // 465 - 448, Ex = 465
  20, // 468 - 448, Ex = 468
  23, // 471 - 448, Ex = 471
  27, // 475 - 448, Ex = 475
  30, // 478 - 448, Ex = 478
  33, // 481 - 448, Ex = 481
  37, // 485 - 448, Ex = 485
  40, // 488 - 448, Ex = 488
  43, // 491 - 448, Ex = 491
  47, // 495 - 448, Ex = 495
  50, // 498 - 448, Ex = 498
  53, // 501 - 448, Ex = 501
  57, // 505 - 448, Ex = 505
];

macro_rules! bid_ten2mxtrunc128 {
  ($index:expr) => {
    BID_TEN2MXTRUNC128[$index as usize]
  };
}
pub(crate) use bid_ten2mxtrunc128;

/// Values of 10^(-x) trancated to Ex bits beyond the binary point, and
/// in the right position to be compared with the fraction from C * kx,
/// 1 <= x <= 37; the fraction consists of the low Ex bits in C * kx
/// (these values are aligned with the low 128 bits of the fraction)
#[rustfmt::skip]
pub const BID_TEN2MXTRUNC128: [BidUint128; 37] = [
  BidUint128{ w: [0xcccccccccccccccc, 0xcccccccccccccccc] }, // (ten2mx >> 128) = cccccccccccccccccccccccccccccccc
  BidUint128{ w: [0x3d70a3d70a3d70a3, 0xa3d70a3d70a3d70a] }, // (ten2mx >> 128) = a3d70a3d70a3d70a3d70a3d70a3d70a3
  BidUint128{ w: [0x645a1cac083126e9, 0x83126e978d4fdf3b] }, // (ten2mx >> 128) = 83126e978d4fdf3b645a1cac083126e9
  BidUint128{ w: [0xd3c36113404ea4a8, 0xd1b71758e219652b] }, // (ten2mx >> 128) = d1b71758e219652bd3c36113404ea4a8
  BidUint128{ w: [0x0fcf80dc33721d53, 0xa7c5ac471b478423] }, // (ten2mx >> 128) = a7c5ac471b4784230fcf80dc33721d53
  BidUint128{ w: [0xa63f9a49c2c1b10f, 0x8637bd05af6c69b5] }, // (ten2mx >> 128) = 8637bd05af6c69b5a63f9a49c2c1b10f
  BidUint128{ w: [0x3d32907604691b4c, 0xd6bf94d5e57a42bc] }, // (ten2mx >> 128) = d6bf94d5e57a42bc3d32907604691b4c
  BidUint128{ w: [0xfdc20d2b36ba7c3d, 0xabcc77118461cefc] }, // (ten2mx >> 128) = abcc77118461cefcfdc20d2b36ba7c3d
  BidUint128{ w: [0x31680a88f8953030, 0x89705f4136b4a597] }, // (ten2mx >> 128) = 89705f4136b4a59731680a88f8953030
  BidUint128{ w: [0xb573440e5a884d1b, 0xdbe6fecebdedd5be] }, // (ten2mx >> 128) = dbe6fecebdedd5beb573440e5a884d1b
  BidUint128{ w: [0xf78f69a51539d748, 0xafebff0bcb24aafe] }, // (ten2mx >> 128) = afebff0bcb24aafef78f69a51539d748
  BidUint128{ w: [0xf93f87b7442e45d3, 0x8cbccc096f5088cb] }, // (ten2mx >> 128) = 8cbccc096f5088cbf93f87b7442e45d3
  BidUint128{ w: [0x2865a5f206b06fb9, 0xe12e13424bb40e13] }, // (ten2mx >> 128) = e12e13424bb40e132865a5f206b06fb9
  BidUint128{ w: [0x538484c19ef38c94, 0xb424dc35095cd80f] }, // (ten2mx >> 128) = b424dc35095cd80f538484c19ef38c94
  BidUint128{ w: [0x0f9d37014bf60a10, 0x901d7cf73ab0acd9] }, // (ten2mx >> 128) = 901d7cf73ab0acd90f9d37014bf60a10
  BidUint128{ w: [0x4c2ebe687989a9b3, 0xe69594bec44de15b] }, // (ten2mx >> 128) = e69594bec44de15b4c2ebe687989a9b3
  BidUint128{ w: [0x09befeb9fad487c2, 0xb877aa3236a4b449] }, // (ten2mx >> 128) = b877aa3236a4b44909befeb9fad487c2
  BidUint128{ w: [0x3aff322e62439fcf, 0x9392ee8e921d5d07] }, // (ten2mx >> 128) = 9392ee8e921d5d073aff322e62439fcf
  BidUint128{ w: [0x2b31e9e3d06c32e5, 0xec1e4a7db69561a5] }, // (ten2mx >> 128) = ec1e4a7db69561a52b31e9e3d06c32e5
  BidUint128{ w: [0x88f4bb1ca6bcf584, 0xbce5086492111aea] }, // (ten2mx >> 128) = bce5086492111aea88f4bb1ca6bcf584
  BidUint128{ w: [0xd3f6fc16ebca5e03, 0x971da05074da7bee] }, // (ten2mx >> 128) = 971da05074da7beed3f6fc16ebca5e03
  BidUint128{ w: [0x5324c68b12dd6338, 0xf1c90080baf72cb1] }, // (ten2mx >> 128) = f1c90080baf72cb15324c68b12dd6338
  BidUint128{ w: [0x75b7053c0f178293, 0xc16d9a0095928a27] }, // (ten2mx >> 128) = c16d9a0095928a2775b7053c0f178293
  BidUint128{ w: [0xc4926a9672793542, 0x9abe14cd44753b52] }, // (ten2mx >> 128) = 9abe14cd44753b52c4926a9672793542
  BidUint128{ w: [0x3a83ddbd83f52204, 0xf79687aed3eec551] }, // (ten2mx >> 128) = f79687aed3eec5513a83ddbd83f52204
  BidUint128{ w: [0x95364afe032a819d, 0xc612062576589dda] }, // (ten2mx >> 128) = c612062576589dda95364afe032a819d
  BidUint128{ w: [0x775ea264cf55347d, 0x9e74d1b791e07e48] }, // (ten2mx >> 128) = 9e74d1b791e07e48775ea264cf55347d
  BidUint128{ w: [0x8bca9d6e188853fc, 0xfd87b5f28300ca0d] }, // (ten2mx >> 128) = fd87b5f28300ca0d8bca9d6e188853fc
  BidUint128{ w: [0x096ee45813a04330, 0xcad2f7f5359a3b3e] }, // (ten2mx >> 128) = cad2f7f5359a3b3e096ee45813a04330
  BidUint128{ w: [0xa1258379a94d028d, 0xa2425ff75e14fc31] }, // (ten2mx >> 128) = a2425ff75e14fc31a1258379a94d028d
  BidUint128{ w: [0x80eacf948770ced7, 0x81ceb32c4b43fcf4] }, // (ten2mx >> 128) = 81ceb32c4b43fcf480eacf948770ced7
  BidUint128{ w: [0x67de18eda5814af2, 0xcfb11ead453994ba] }, // (ten2mx >> 128) = cfb11ead453994ba67de18eda5814af2
  BidUint128{ w: [0xecb1ad8aeacdd58e, 0xa6274bbdd0fadd61] }, // (ten2mx >> 128) = a6274bbdd0fadd61ecb1ad8aeacdd58e
  BidUint128{ w: [0xbd5af13bef0b113e, 0x84ec3c97da624ab4] }, // (ten2mx >> 128) = 84ec3c97da624ab4bd5af13bef0b113e
  BidUint128{ w: [0x955e4ec64b44e864, 0xd4ad2dbfc3d07787] }, // (ten2mx >> 128) = d4ad2dbfc3d07787955e4ec64b44e864
  BidUint128{ w: [0xdde50bd1d5d0b9e9, 0xaa242499697392d2] }, // (ten2mx >> 128) = aa242499697392d2dde50bd1d5d0b9e9
  BidUint128{ w: [0x7e50d64177da2e54, 0x881cea14545c7575] }  // (ten2mx >> 128) = 881cea14545c75757e50d64177da2e54
];

macro_rules! bid_midpoint192 {
  ($index:expr) => {
    BID_MIDPOINT192[$index as usize]
  };
}
pub(crate) use bid_midpoint192;

/// BID_MIDPOINT192\[i - 39\] = 1/2 * 10^i = 5 * 10^(i-1), 39 <= i <= 58
#[rustfmt::skip]
pub const BID_MIDPOINT192: [BidUint192; 20] = [
  // the 64-bit word order is L, M, H
  BidUint192{ w: [0x2fb2ab4000000000, 0x78287f49c4a1d662, 0x0000000000000001] }, // 1/2 * 10^39 = 5 * 10^38
  BidUint192{ w: [0xdcfab08000000000, 0xb194f8e1ae525fd5, 0x000000000000000e] }, // 1/2 * 10^40 = 5 * 10^39
  BidUint192{ w: [0xa1cae50000000000, 0xefd1b8d0cf37be5a, 0x0000000000000092] }, // 1/2 * 10^41 = 5 * 10^40
  BidUint192{ w: [0x51ecf20000000000, 0x5e313828182d6f8a, 0x00000000000005bd] }, // 1/2 * 10^42 = 5 * 10^41
  BidUint192{ w: [0x3341740000000000, 0xadec3190f1c65b67, 0x0000000000003965] }, // 1/2 * 10^43 = 5 * 10^42
  BidUint192{ w: [0x008e880000000000, 0xcb39efa971bf9208, 0x0000000000023df8] }, // 1/2 * 10^44 = 5 * 10^43
  BidUint192{ w: [0x0591500000000000, 0xf0435c9e717bb450, 0x0000000000166bb7] }, // 1/2 * 10^45 = 5 * 10^44
  BidUint192{ w: [0x37ad200000000000, 0x62a19e306ed50b20, 0x0000000000e0352f] }, // 1/2 * 10^46 = 5 * 10^45
  BidUint192{ w: [0x2cc3400000000000, 0xda502de454526f42, 0x0000000008c213d9] }, // 1/2 * 10^47 = 5 * 10^46
  BidUint192{ w: [0xbfa0800000000000, 0x8721caeb4b385895, 0x000000005794c682] }, // 1/2 * 10^48 = 5 * 10^47
  BidUint192{ w: [0x7c45000000000000, 0x4751ed30f03375d9, 0x000000036bcfc119] }, // 1/2 * 10^49 = 5 * 10^48
  BidUint192{ w: [0xdab2000000000000, 0xc93343e962029a7e, 0x00000022361d8afc] }, // 1/2 * 10^50 = 5 * 10^49
  BidUint192{ w: [0x8af4000000000000, 0xdc00a71dd41a08f4, 0x000001561d276ddf] }, // 1/2 * 10^51 = 5 * 10^50
  BidUint192{ w: [0x6d88000000000000, 0x9806872a4904598d, 0x00000d5d238a4abe] }, // 1/2 * 10^52 = 5 * 10^51
  BidUint192{ w: [0x4750000000000000, 0xf04147a6da2b7f86, 0x000085a36366eb71] }, // 1/2 * 10^53 = 5 * 10^52
  BidUint192{ w: [0xc920000000000000, 0x628ccc8485b2fb3e, 0x00053861e2053273] }, // 1/2 * 10^54 = 5 * 10^53
  BidUint192{ w: [0xdb40000000000000, 0xd97ffd2d38fdd073, 0x003433d2d433f881] }, // 1/2 * 10^55 = 5 * 10^54
  BidUint192{ w: [0x9080000000000000, 0x7effe3c439ea2486, 0x020a063c4a07b512] }, // 1/2 * 10^56 = 5 * 10^55
  BidUint192{ w: [0xa500000000000000, 0xf5fee5aa43256d41, 0x14643e5ae44d12b8] }, // 1/2 * 10^57 = 5 * 10^56
  BidUint192{ w: [0x7200000000000000, 0x9bf4f8a69f764490, 0xcbea6f8ceb02bb39] }  // 1/2 * 10^58 = 5 * 10^57
];

macro_rules! bid_midpoint256 {
  ($index:expr) => {
    BID_MIDPOINT256[$index as usize]
  };
}
pub(crate) use bid_midpoint256;

/// bid_midpoint256\[i - 59\] = 1/2 * 10^i = 5 * 10^(i-1), 59 <= i <= 68
#[rustfmt::skip]
pub const BID_MIDPOINT256: [BidUint256; 19] = [
  // the 64-bit word order is LL, LH, HL, HH
  BidUint256{ w: [0x7400000000000000, 0x1791b6823a9eada4, 0xf7285b812e1b5040, 0x0000000000000007 ] },	// 1/2 * 10^59 = 5 * 10^58
  BidUint256{ w: [0x8800000000000000, 0xebb121164a32c86c, 0xa793930bcd112280, 0x000000000000004f ] },	// 1/2 * 10^60 = 5 * 10^59
  BidUint256{ w: [0x5000000000000000, 0x34eb4adee5fbd43d, 0x8bc3be7602ab5909, 0x000000000000031c ] },	// 1/2 * 10^61 = 5 * 10^60
  BidUint256{ w: [0x2000000000000000, 0x1130ecb4fbd64a65, 0x75a5709c1ab17a5c, 0x0000000000001f1d ] },	// 1/2 * 10^62 = 5 * 10^61
  BidUint256{ w: [0x4000000000000000, 0xabe93f11d65ee7f3, 0x987666190aeec798, 0x0000000000013726 ] },	// 1/2 * 10^63 = 5 * 10^62
  BidUint256{ w: [0x8000000000000000, 0xb71c76b25fb50f80, 0xf49ffcfa6d53cbf6, 0x00000000000c2781 ] },	// 1/2 * 10^64 = 5 * 10^63
  BidUint256{ w: [0x0000000000000000, 0x271ca2f7bd129b05, 0x8e3fe1c84545f7a3, 0x0000000000798b13 ] },	// 1/2 * 10^65 = 5 * 10^64
  BidUint256{ w: [0x0000000000000000, 0x871e5dad62ba0e32, 0x8e7ed1d2b4bbac5f, 0x0000000004bf6ec3 ] },	// 1/2 * 10^66 = 5 * 10^65
  BidUint256{ w: [0x0000000000000000, 0x472fa8c5db448df4, 0x90f4323b0f54bbbb, 0x000000002f7a53a3 ] },	// 1/2 * 10^67 = 5 * 10^66
  BidUint256{ w: [0x0000000000000000, 0xc7dc97ba90ad8b88, 0xa989f64e994f5550, 0x00000001dac74463 ] },	// 1/2 * 10^68 = 5 * 10^67
  BidUint256{ w: [0x0000000000000000, 0xce9ded49a6c77350, 0x9f639f11fd195527, 0x000000128bc8abe4 ] },	// 1/2 * 10^69 = 5 * 10^68
  BidUint256{ w: [0x0000000000000000, 0x122b44e083ca8120, 0x39e436b3e2fd538e, 0x000000b975d6b6ee ] },	// 1/2 * 10^70 = 5 * 10^69
  BidUint256{ w: [0x0000000000000000, 0xb5b0b0c525e90b40, 0x42ea2306dde5438c, 0x0000073e9a63254e ] },	// 1/2 * 10^71 = 5 * 10^70
  BidUint256{ w: [0x0000000000000000, 0x18e6e7b37b1a7080, 0x9d255e44aaf4a37f, 0x0000487207df750e ] },	// 1/2 * 10^72 = 5 * 10^71
  BidUint256{ w: [0x0000000000000000, 0xf9050d02cf086500, 0x2375aeaead8e62f6, 0x0002d4744eba9292 ] },	// 1/2 * 10^73 = 5 * 10^72
  BidUint256{ w: [0x0000000000000000, 0xba32821c1653f200, 0x6298d2d2c78fdda5, 0x001c4c8b1349b9b5 ] },	// 1/2 * 10^74 = 5 * 10^73
  BidUint256{ w: [0x0000000000000000, 0x45f91518df477400, 0xd9f83c3bcb9ea879, 0x011afd6ec0e14115 ] },	// 1/2 * 10^75 = 5 * 10^74
  BidUint256{ w: [0x0000000000000000, 0xbbbad2f8b8ca8800, 0x83b25a55f43294bc, 0x0b0de65388cc8ada ] },	// 1/2 * 10^76 = 5 * 10^75
  BidUint256{ w: [0x0000000000000000, 0x554c3db737e95000, 0x24f7875b89f9cf5f, 0x6e8aff4357fd6c89 ] }  // 1/2 * 10^77 = 5 * 10^76
];

macro_rules! bid_half256 {
  ($index:expr) => {
    BID_HALF256[$index as usize]
  };
}
pub(crate) use bid_half256;

pub const BID_HALF256: [BidUint64; 75] = [
  0x0000000000000004, // half / 2^256 = 4
  0x0000000000000020, // half / 2^256 = 20
  0x0000000000000100, // half / 2^256 = 100
  0x0000000000001000, // half / 2^256 = 1000
  0x0000000000008000, // half / 2^256 = 8000
  0x0000000000040000, // half / 2^256 = 40000
  0x0000000000400000, // half / 2^256 = 400000
  0x0000000002000000, // half / 2^256 = 2000000
  0x0000000010000000, // half / 2^256 = 10000000
  0x0000000100000000, // half / 2^256 = 100000000
  0x0000000800000000, // half / 2^256 = 800000000
  0x0000004000000000, // half / 2^256 = 4000000000
  0x0000040000000000, // half / 2^256 = 40000000000
  0x0000200000000000, // half / 2^256 = 200000000000
  0x0001000000000000, // half / 2^256 = 1000000000000
  0x0010000000000000, // half / 2^256 = 10000000000000
  0x0080000000000000, // half / 2^256 = 80000000000000
  0x0400000000000000, // half / 2^256 = 400000000000000
  0x4000000000000000, // half / 2^256 = 4000000000000000
  0x0000000000000002, // half / 2^320 = 2
  0x0000000000000010, // half / 2^320 = 10
  0x0000000000000100, // half / 2^320 = 100
  0x0000000000000800, // half / 2^320 = 800
  0x0000000000004000, // half / 2^320 = 4000
  0x0000000000040000, // half / 2^320 = 40000
  0x0000000000200000, // half / 2^320 = 200000
  0x0000000001000000, // half / 2^320 = 1000000
  0x0000000010000000, // half / 2^320 = 10000000
  0x0000000080000000, // half / 2^320 = 80000000
  0x0000000400000000, // half / 2^320 = 400000000
  0x0000002000000000, // half / 2^320 = 2000000000
  0x0000020000000000, // half / 2^320 = 20000000000
  0x0000100000000000, // half / 2^320 = 100000000000
  0x0000800000000000, // half / 2^320 = 800000000000
  0x0008000000000000, // half / 2^320 = 8000000000000
  0x0040000000000000, // half / 2^320 = 40000000000000
  0x0200000000000000, // half / 2^320 = 200000000000000
  0x2000000000000000, // half / 2^320 = 2000000000000000
  0x0000000000000001, // half / 2^384 = 1
  0x0000000000000008, // half / 2^384 = 8
  0x0000000000000080, // half / 2^384 = 80
  0x0000000000000400, // half / 2^384 = 400
  0x0000000000002000, // half / 2^384 = 2000
  0x0000000000020000, // half / 2^384 = 20000
  0x0000000000100000, // half / 2^384 = 100000
  0x0000000000800000, // half / 2^384 = 800000
  0x0000000008000000, // half / 2^384 = 8000000
  0x0000000040000000, // half / 2^384 = 40000000
  0x0000000200000000, // half / 2^384 = 200000000
  0x0000002000000000, // half / 2^384 = 2000000000
  0x0000010000000000, // half / 2^384 = 10000000000
  0x0000080000000000, // half / 2^384 = 80000000000
  0x0000800000000000, // half / 2^384 = 800000000000
  0x0004000000000000, // half / 2^384 = 4000000000000
  0x0020000000000000, // half / 2^384 = 20000000000000
  0x0200000000000000, // half / 2^384 = 200000000000000
  0x1000000000000000, // half / 2^384 = 1000000000000000
  0x8000000000000000, // half / 2^384 = 8000000000000000
  0x0000000000000004, // half / 2^448 = 4
  0x0000000000000040, // half / 2^448 = 40
  0x0000000000000200, // half / 2^448 = 200
  0x0000000000001000, // half / 2^448 = 1000
  0x0000000000010000, // half / 2^448 = 10000
  0x0000000000080000, // half / 2^448 = 80000
  0x0000000000400000, // half / 2^448 = 400000
  0x0000000004000000, // half / 2^448 = 4000000
  0x0000000020000000, // half / 2^448 = 20000000
  0x0000000100000000, // half / 2^448 = 100000000
  0x0000001000000000, // half / 2^448 = 1000000000
  0x0000008000000000, // half / 2^448 = 8000000000
  0x0000040000000000, // half / 2^448 = 40000000000
  0x0000400000000000, // half / 2^448 = 400000000000
  0x0002000000000000, // half / 2^448 = 2000000000000
  0x0010000000000000, // half / 2^448 = 10000000000000
  0x0100000000000000, // half / 2^448 = 100000000000000
];

macro_rules! bid_mask256 {
  ($index:expr) => {
    BID_MASK256[$index as usize]
  };
}
pub(crate) use bid_mask256;

#[rustfmt::skip]
pub const BID_MASK256: [BidUint64; 75] = [
  0x0000000000000007, // mask / 2^256
  0x000000000000003f, // mask / 2^256
  0x00000000000001ff, // mask / 2^256
  0x0000000000001fff, // mask / 2^256
  0x000000000000ffff, // mask / 2^256
  0x000000000007ffff, // mask / 2^256
  0x00000000007fffff, // mask / 2^256
  0x0000000003ffffff, // mask / 2^256
  0x000000001fffffff, // mask / 2^256
  0x00000001ffffffff, // mask / 2^256
  0x0000000fffffffff, // mask / 2^256
  0x0000007fffffffff, // mask / 2^256
  0x000007ffffffffff, // mask / 2^256
  0x00003fffffffffff, // mask / 2^256
  0x0001ffffffffffff, // mask / 2^256
  0x001fffffffffffff, // mask / 2^256
  0x00ffffffffffffff, // mask / 2^256
  0x07ffffffffffffff, // mask / 2^256
  0x7fffffffffffffff, // mask / 2^256
  0x0000000000000003, // mask / 2^320
  0x000000000000001f, // mask / 2^320
  0x00000000000001ff, // mask / 2^320
  0x0000000000000fff, // mask / 2^320
  0x0000000000007fff, // mask / 2^320
  0x000000000007ffff, // mask / 2^320
  0x00000000003fffff, // mask / 2^320
  0x0000000001ffffff, // mask / 2^320
  0x000000001fffffff, // mask / 2^320
  0x00000000ffffffff, // mask / 2^320
  0x00000007ffffffff, // mask / 2^320
  0x0000003fffffffff, // mask / 2^320
  0x000003ffffffffff, // mask / 2^320
  0x00001fffffffffff, // mask / 2^320
  0x0000ffffffffffff, // mask / 2^320
  0x000fffffffffffff, // mask / 2^320
  0x007fffffffffffff, // mask / 2^320
  0x03ffffffffffffff, // mask / 2^320
  0x3fffffffffffffff, // mask / 2^320
  0x0000000000000001, // mask / 2^384
  0x000000000000000f, // mask / 2^384
  0x00000000000000ff, // mask / 2^384
  0x00000000000007ff, // mask / 2^384
  0x0000000000003fff, // mask / 2^384
  0x000000000003ffff, // mask / 2^384
  0x00000000001fffff, // mask / 2^384
  0x0000000000ffffff, // mask / 2^384
  0x000000000fffffff, // mask / 2^384
  0x000000007fffffff, // mask / 2^384
  0x00000003ffffffff, // mask / 2^384
  0x0000003fffffffff, // mask / 2^384
  0x000001ffffffffff, // mask / 2^384
  0x00000fffffffffff, // mask / 2^384
  0x0000ffffffffffff, // mask / 2^384
  0x0007ffffffffffff, // mask / 2^384
  0x003fffffffffffff, // mask / 2^384
  0x03ffffffffffffff, // mask / 2^384
  0x1fffffffffffffff, // mask / 2^384
  0xffffffffffffffff, // mask / 2^384
  0x0000000000000007, // mask / 2^448
  0x000000000000007f, // mask / 2^448
  0x00000000000003ff, // mask / 2^448
  0x0000000000001fff, // mask / 2^448
  0x000000000001ffff, // mask / 2^448
  0x00000000000fffff, // mask / 2^448
  0x00000000007fffff, // mask / 2^448
  0x0000000007ffffff, // mask / 2^448
  0x000000003fffffff, // mask / 2^448
  0x00000001ffffffff, // mask / 2^448
  0x0000001fffffffff, // mask / 2^448
  0x000000ffffffffff, // mask / 2^448
  0x000007ffffffffff, // mask / 2^448
  0x00007fffffffffff, // mask / 2^448
  0x0003ffffffffffff, // mask / 2^448
  0x001fffffffffffff, // mask / 2^448
  0x01ffffffffffffff, // mask / 2^448
];

macro_rules! bid_ten2mxtrunc256 {
  ($index:expr) => {
    BID_TEN2MXTRUNC256[$index as usize]
  };
}
pub(crate) use bid_ten2mxtrunc256;

#[rustfmt::skip]
pub const BID_TEN2MXTRUNC256: [BidUint256; 75] = [
  BidUint256{ w: [0xcccccccccccccccc, 0xcccccccccccccccc, 0xcccccccccccccccc, 0xcccccccccccccccc ] }, // (ten2mx >> 256) = cccccccccccccccc  cccccccccccccccc  |  cccccccccccccccccccccccccccccccc
  BidUint256{ w: [0x70a3d70a3d70a3d7, 0xd70a3d70a3d70a3d, 0x3d70a3d70a3d70a3, 0xa3d70a3d70a3d70a ] }, // (ten2mx >> 256) = a3d70a3d70a3d70a  3d70a3d70a3d70a3  |  d70a3d70a3d70a3d70a3d70a3d70a3d7
  BidUint256{ w: [0xc083126e978d4fdf, 0x78d4fdf3b645a1ca, 0x645a1cac083126e9, 0x83126e978d4fdf3b ] }, // (ten2mx >> 256) = 83126e978d4fdf3b  645a1cac083126e9  |  78d4fdf3b645a1cac083126e978d4fdf
  BidUint256{ w: [0x67381d7dbf487fcb, 0xc154c985f06f6944, 0xd3c36113404ea4a8, 0xd1b71758e219652b ] }, // (ten2mx >> 256) = d1b71758e219652b  d3c36113404ea4a8  |  c154c985f06f694467381d7dbf487fcb
  BidUint256{ w: [0x85c67dfe32a0663c, 0xcddd6e04c0592103, 0x0fcf80dc33721d53, 0xa7c5ac471b478423 ] }, // (ten2mx >> 256) = a7c5ac471b478423  fcf80dc33721d53   |  cddd6e04c059210385c67dfe32a0663c
  BidUint256{ w: [0x37d1fe64f54d1e96, 0xd7e45803cd141a69, 0xa63f9a49c2c1b10f, 0x8637bd05af6c69b5 ] }, // (ten2mx >> 256) = 8637bd05af6c69b5  a63f9a49c2c1b10f  |  d7e45803cd141a6937d1fe64f54d1e96
  BidUint256{ w: [0x8c8330a1887b6424, 0x8ca08cd2e1b9c3db, 0x3d32907604691b4c, 0xd6bf94d5e57a42bc ] }, // (ten2mx >> 256) = d6bf94d5e57a42bc  3d32907604691b4c  |  8ca08cd2e1b9c3db8c8330a1887b6424
  BidUint256{ w: [0x7068f3b46d2f8350, 0x3d4d3d758161697c, 0xfdc20d2b36ba7c3d, 0xabcc77118461cefc ] }, // (ten2mx >> 256) = abcc77118461cefc  fdc20d2b36ba7c3d  |  3d4d3d758161697c7068f3b46d2f8350
  BidUint256{ w: [0xf387295d242602a6, 0xfdd7645e011abac9, 0x31680a88f8953030, 0x89705f4136b4a597 ] }, // (ten2mx >> 256) = 89705f4136b4a597  31680a88f8953030  |  fdd7645e011abac9f387295d242602a6
  BidUint256{ w: [0xb8d8422ea03cd10a, 0x2fbf06fcce912adc, 0xb573440e5a884d1b, 0xdbe6fecebdedd5be ] }, // (ten2mx >> 256) = dbe6fecebdedd5be  b573440e5a884d1b  |  2fbf06fcce912adcb8d8422ea03cd10a
  BidUint256{ w: [0x93e034f219ca40d5, 0xf2ff38ca3eda88b0, 0xf78f69a51539d748, 0xafebff0bcb24aafe ] }, // (ten2mx >> 256) = afebff0bcb24aafe  f78f69a51539d748  |  f2ff38ca3eda88b093e034f219ca40d5
  BidUint256{ w: [0x4319c3f4e16e9a44, 0xf598fa3b657ba08d, 0xf93f87b7442e45d3, 0x8cbccc096f5088cb ] }, // (ten2mx >> 256) = 8cbccc096f5088cb  f93f87b7442e45d3  |  f598fa3b657ba08d4319c3f4e16e9a44
  BidUint256{ w: [0x04f606549be42a06, 0x88f4c3923bf900e2, 0x2865a5f206b06fb9, 0xe12e13424bb40e13 ] }, // (ten2mx >> 256) = e12e13424bb40e13  2865a5f206b06fb9  |  88f4c3923bf900e204f606549be42a06
  BidUint256{ w: [0x03f805107cb68805, 0x6d909c74fcc733e8, 0x538484c19ef38c94, 0xb424dc35095cd80f ] }, // (ten2mx >> 256) = b424dc35095cd80f  538484c19ef38c94  |  6d909c74fcc733e803f805107cb68805
  BidUint256{ w: [0x3660040d3092066a, 0x57a6e390ca38f653, 0x0f9d37014bf60a10, 0x901d7cf73ab0acd9 ] }, // (ten2mx >> 256) = 901d7cf73ab0acd9  f9d37014bf60a10   |  57a6e390ca38f6533660040d3092066a
  BidUint256{ w: [0x23ccd3484db670aa, 0xbf716c1add27f085, 0x4c2ebe687989a9b3, 0xe69594bec44de15b ] }, // (ten2mx >> 256) = e69594bec44de15b  4c2ebe687989a9b3  |  bf716c1add27f08523ccd3484db670aa
  BidUint256{ w: [0x4fd70f6d0af85a22, 0xff8df0157db98d37, 0x09befeb9fad487c2, 0xb877aa3236a4b449 ] }, // (ten2mx >> 256) = b877aa3236a4b449  9befeb9fad487c2   |  ff8df0157db98d374fd70f6d0af85a22
  BidUint256{ w: [0x0cac0c573bf9e1b5, 0x32d7f344649470f9, 0x3aff322e62439fcf, 0x9392ee8e921d5d07 ] }, // (ten2mx >> 256) = 9392ee8e921d5d07  3aff322e62439fcf  |  32d7f344649470f90cac0c573bf9e1b5
  BidUint256{ w: [0xe11346f1f98fcf88, 0x1e2652070753e7f4, 0x2b31e9e3d06c32e5, 0xec1e4a7db69561a5 ] }, // (ten2mx >> 256) = ec1e4a7db69561a5  2b31e9e3d06c32e5  |  1e2652070753e7f4e11346f1f98fcf88
  BidUint256{ w: [0x4da9058e613fd939, 0x181ea8059f76532a, 0x88f4bb1ca6bcf584, 0xbce5086492111aea ] }, // (ten2mx >> 256) = bce5086492111aea  88f4bb1ca6bcf584  |  181ea8059f76532a4da9058e613fd939
  BidUint256{ w: [0xa48737a51a997a94, 0x467eecd14c5ea8ee, 0xd3f6fc16ebca5e03, 0x971da05074da7bee ] }, // (ten2mx >> 256) = 971da05074da7bee  d3f6fc16ebca5e03  |  467eecd14c5ea8eea48737a51a997a94
  BidUint256{ w: [0x3a71f2a1c428c420, 0x70cb148213caa7e4, 0x5324c68b12dd6338, 0xf1c90080baf72cb1 ] }, // (ten2mx >> 256) = f1c90080baf72cb1  5324c68b12dd6338  |  70cb148213caa7e43a71f2a1c428c420
  BidUint256{ w: [0x2ec18ee7d0209ce7, 0x8d6f439b43088650, 0x75b7053c0f178293, 0xc16d9a0095928a27 ] }, // (ten2mx >> 256) = c16d9a0095928a27  75b7053c0f178293  |  8d6f439b430886502ec18ee7d0209ce7
  BidUint256{ w: [0xf23472530ce6e3ec, 0xd78c3615cf3a050c, 0xc4926a9672793542, 0x9abe14cd44753b52 ] }, // (ten2mx >> 256) = 9abe14cd44753b52  c4926a9672793542  |  d78c3615cf3a050cf23472530ce6e3ec
  BidUint256{ w: [0xe9ed83b814a49fe0, 0x8c1389bc7ec33b47, 0x3a83ddbd83f52204, 0xf79687aed3eec551 ] }, // (ten2mx >> 256) = f79687aed3eec551  3a83ddbd83f52204  |  8c1389bc7ec33b47e9ed83b814a49fe0
  BidUint256{ w: [0x87f1362cdd507fe6, 0x3cdc6e306568fc39, 0x95364afe032a819d, 0xc612062576589dda ] }, // (ten2mx >> 256) = c612062576589dda  95364afe032a819d  |  3cdc6e306568fc3987f1362cdd507fe6
  BidUint256{ w: [0x9ff42b5717739985, 0xca49f1c05120c9c7, 0x775ea264cf55347d, 0x9e74d1b791e07e48 ] }, // (ten2mx >> 256) = 9e74d1b791e07e48  775ea264cf55347d  |  ca49f1c05120c9c79ff42b5717739985
  BidUint256{ w: [0xccb9def1bf1f5c08, 0x76dcb60081ce0fa5, 0x8bca9d6e188853fc, 0xfd87b5f28300ca0d ] }, // (ten2mx >> 256) = fd87b5f28300ca0d  8bca9d6e188853fc  |  76dcb60081ce0fa5ccb9def1bf1f5c08
  BidUint256{ w: [0xa3c7e58e327f7cd3, 0x5f16f80067d80c84, 0x096ee45813a04330, 0xcad2f7f5359a3b3e ] }, // (ten2mx >> 256) = cad2f7f5359a3b3e  96ee45813a04330   |  5f16f80067d80c84a3c7e58e327f7cd3
  BidUint256{ w: [0xb6398471c1ff970f, 0x18df2ccd1fe00a03, 0xa1258379a94d028d, 0xa2425ff75e14fc31 ] }, // (ten2mx >> 256) = a2425ff75e14fc31  a1258379a94d028d  |  18df2ccd1fe00a03b6398471c1ff970f
  BidUint256{ w: [0xf82e038e34cc78d9, 0x4718f0a419800802, 0x80eacf948770ced7, 0x81ceb32c4b43fcf4 ] }, // (ten2mx >> 256) = 81ceb32c4b43fcf4  80eacf948770ced7  |  4718f0a419800802f82e038e34cc78d9
  BidUint256{ w: [0x59e338e387ad8e28, 0x0b5b1aa028ccd99e, 0x67de18eda5814af2, 0xcfb11ead453994ba ] }, // (ten2mx >> 256) = cfb11ead453994ba  67de18eda5814af2  |  b5b1aa028ccd99e59e338e387ad8e28
  BidUint256{ w: [0x47e8fa4f9fbe0b53, 0x6f7c154ced70ae18, 0xecb1ad8aeacdd58e, 0xa6274bbdd0fadd61 ] }, // (ten2mx >> 256) = a6274bbdd0fadd61  ecb1ad8aeacdd58e  |  6f7c154ced70ae1847e8fa4f9fbe0b53
  BidUint256{ w: [0xd320c83fb2fe6f75, 0xbf967770bdf3be79, 0xbd5af13bef0b113e, 0x84ec3c97da624ab4 ] }, // (ten2mx >> 256) = 84ec3c97da624ab4  bd5af13bef0b113e  |  bf967770bdf3be79d320c83fb2fe6f75
  BidUint256{ w: [0x85014065eb30b256, 0x65bd8be79652ca5c, 0x955e4ec64b44e864, 0xd4ad2dbfc3d07787 ] }, // (ten2mx >> 256) = d4ad2dbfc3d07787  955e4ec64b44e864  |  65bd8be79652ca5c85014065eb30b256
  BidUint256{ w: [0xd0cdcd1e55c08eab, 0xeafe098611dbd516, 0xdde50bd1d5d0b9e9, 0xaa242499697392d2 ] }, // (ten2mx >> 256) = aa242499697392d2  dde50bd1d5d0b9e9  |  eafe098611dbd516d0cdcd1e55c08eab
  BidUint256{ w: [0x40a4a418449a0bbc, 0xbbfe6e04db164412, 0x7e50d64177da2e54, 0x881cea14545c7575 ] }, // (ten2mx >> 256) = 881cea14545c7575  7e50d64177da2e54  |  bbfe6e04db16441240a4a418449a0bbc
  BidUint256{ w: [0x9aa1068d3a9012c7, 0x2cca49a15e8a0683, 0x96e7bd358c904a21, 0xd9c7dced53c72255 ] }, // (ten2mx >> 256) = d9c7dced53c72255  96e7bd358c904a21  |  2cca49a15e8a06839aa1068d3a9012c7
  BidUint256{ w: [0x154d9ed7620cdbd2, 0x8a3b6e1ab2080536, 0xabec975e0a0d081a, 0xae397d8aa96c1b77 ] }, // (ten2mx >> 256) = ae397d8aa96c1b77  abec975e0a0d081a  |  8a3b6e1ab2080536154d9ed7620cdbd2
  BidUint256{ w: [0x443e18ac4e70afdb, 0x3b62be7bc1a0042b, 0x2323ac4b3b3da015, 0x8b61313bbabce2c6 ] }, // (ten2mx >> 256) = 8b61313bbabce2c6  2323ac4b3b3da015  |  3b62be7bc1a0042b443e18ac4e70afdb
  BidUint256{ w: [0x6d30277a171ab2f8, 0x5f0463f935ccd378, 0x6b6c46dec52f6688, 0xdf01e85f912e37a3 ] }, // (ten2mx >> 256) = df01e85f912e37a3  6b6c46dec52f6688  |  5f0463f935ccd3786d30277a171ab2f8
  BidUint256{ w: [0x8a8cec61ac155bfa, 0x7f36b660f7d70f93, 0x55f038b237591ed3, 0xb267ed1940f1c61c ] }, // (ten2mx >> 256) = b267ed1940f1c61c  55f038b237591ed3  |  7f36b660f7d70f938a8cec61ac155bfa
  BidUint256{ w: [0x3ba3f04e23444995, 0xcc2bc51a5fdf3fa9, 0x77f3608e92adb242, 0x8eb98a7a9a5b04e3 ] }, // (ten2mx >> 256) = 8eb98a7a9a5b04e3  77f3608e92adb242  |  cc2bc51a5fdf3fa93ba3f04e23444995
  BidUint256{ w: [0xf9064d49d206dc21, 0xe046082a32fecc41, 0x8cb89a7db77c506a, 0xe45c10c42a2b3b05 ] }, // (ten2mx >> 256) = e45c10c42a2b3b05  8cb89a7db77c506a  |  e046082a32fecc41f9064d49d206dc21
  BidUint256{ w: [0xfa6b7107db38b01a, 0x4d04d354f598a367, 0x3d607b97c5fd0d22, 0xb6b00d69bb55c8d1 ] }, // (ten2mx >> 256) = b6b00d69bb55c8d1  3d607b97c5fd0d22  |  4d04d354f598a367fa6b7107db38b01a
  BidUint256{ w: [0xfb8927397c2d59af, 0x3d9d75dd9146e91f, 0xcab3961304ca70e8, 0x9226712162ab070d ] }, // (ten2mx >> 256) = 9226712162ab070d  cab3961304ca70e8  |  3d9d75dd9146e91ffb8927397c2d59af
  BidUint256{ w: [0xf8db71f5937bc2b1, 0xc8fbefc8e87174ff, 0xaab8f01e6e10b4a6, 0xe9d71b689dde71af ] }, // (ten2mx >> 256) = e9d71b689dde71af  aab8f01e6e10b4a6  |  c8fbefc8e87174fff8db71f5937bc2b1
  BidUint256{ w: [0x2d7c5b2adc630227, 0x3a63263a538df733, 0x5560c018580d5d52, 0xbb127c53b17ec159 ] }, // (ten2mx >> 256) = bb127c53b17ec159  5560c018580d5d52  |  3a63263a538df7332d7c5b2adc630227
  BidUint256{ w: [0x24637c2249e8ce86, 0x2eb5b82ea93e5f5c, 0xdde7001379a44aa8, 0x95a8637627989aad ] }, // (ten2mx >> 256) = 95a8637627989aad  dde7001379a44aa8  |  2eb5b82ea93e5f5c24637c2249e8ce86
  BidUint256{ w: [0x3a38c69d430e173d, 0x4abc59e441fd6560, 0x963e66858f6d4440, 0xef73d256a5c0f77c ] }, // (ten2mx >> 256) = ef73d256a5c0f77c  963e66858f6d4440  |  4abc59e441fd65603a38c69d430e173d
  BidUint256{ w: [0x94fa387dcf3e78fd, 0x6efd14b69b311de6, 0xde98520472bdd033, 0xbf8fdb78849a5f96 ] }, // (ten2mx >> 256) = bf8fdb78849a5f96  de98520472bdd033  |  6efd14b69b311de694fa387dcf3e78fd
  BidUint256{ w: [0xaa61c6cb0c31fa64, 0x259743c548f417eb, 0xe546a8038efe4029, 0x993fe2c6d07b7fab ] }, // (ten2mx >> 256) = 993fe2c6d07b7fab  e546a8038efe4029  |  259743c548f417ebaa61c6cb0c31fa64
  BidUint256{ w: [0xaa360ade79e990a1, 0x3c25393ba7ecf312, 0xd53dd99f4b3066a8, 0xf53304714d9265df ] }, // (ten2mx >> 256) = f53304714d9265df  d53dd99f4b3066a8  |  3c25393ba7ecf312aa360ade79e990a1
  BidUint256{ w: [0x882b3be52e5473b4, 0x96842dc95323f5a8, 0xaa97e14c3c26b886, 0xc428d05aa4751e4c ] }, // (ten2mx >> 256) = c428d05aa4751e4c  aa97e14c3c26b886  |  96842dc95323f5a8882b3be52e5473b4
  BidUint256{ w: [0xd355c98425105c90, 0xab9cf16ddc1cc486, 0x55464dd69685606b, 0x9ced737bb6c4183d ] }, // (ten2mx >> 256) = 9ced737bb6c4183d  55464dd69685606b  |  ab9cf16ddc1cc486d355c98425105c90
  BidUint256{ w: [0xebbc75a03b4d60e6, 0xac2e4f162cfad40a, 0xeed6e2f0f0d56712, 0xfb158592be068d2e ] }, // (ten2mx >> 256) = fb158592be068d2e  eed6e2f0f0d56712  |  ac2e4f162cfad40aebbc75a03b4d60e6
  BidUint256{ w: [0x8963914cfc3de71e, 0x568b727823fbdcd5, 0xf245825a5a445275, 0xc8de047564d20a8b ] }, // (ten2mx >> 256) = c8de047564d20a8b  f245825a5a445275  |  568b727823fbdcd58963914cfc3de71e
  BidUint256{ w: [0xd44fa770c9cb1f4b, 0x453c5b934ffcb0aa, 0x5b6aceaeae9d0ec4, 0xa0b19d2ab70e6ed6 ] }, // (ten2mx >> 256) = a0b19d2ab70e6ed6  5b6aceaeae9d0ec4  |  453c5b934ffcb0aad44fa770c9cb1f4b
  BidUint256{ w: [0xdd0c85f3d4a27f6f, 0x37637c75d996f3bb, 0xe2bbd88bbee40bd0, 0x808e17555f3ebf11 ] }, // (ten2mx >> 256) = 808e17555f3ebf11  e2bbd88bbee40bd0  |  37637c75d996f3bbdd0c85f3d4a27f6f
  BidUint256{ w: [0x61ada31fba9d98b2, 0x256bfa5628f185f9, 0x3792f412cb06794d, 0xcdb02555653131b6 ] }, // (ten2mx >> 256) = cdb02555653131b6  3792f412cb06794d  |  256bfa5628f185f961ada31fba9d98b2
  BidUint256{ w: [0xe7be1c196217ad5b, 0x51232eab53f46b2d, 0x5fa8c3423c052dd7, 0xa48ceaaab75a8e2b ] }, // (ten2mx >> 256) = a48ceaaab75a8e2b  5fa8c3423c052dd7  |  51232eab53f46b2de7be1c196217ad5b
  BidUint256{ w: [0x52fe7ce11b462449, 0x40e8f222a99055be, 0x1953cf68300424ac, 0x83a3eeeef9153e89 ] }, // (ten2mx >> 256) = 83a3eeeef9153e89  1953cf68300424ac  |  40e8f222a99055be52fe7ce11b462449
  BidUint256{ w: [0x51972e34f8703a0f, 0x34a7e9d10f4d55fd, 0x8eec7f0d19a03aad, 0xd29fe4b18e88640e ] }, // (ten2mx >> 256) = d29fe4b18e88640e  8eec7f0d19a03aad  |  34a7e9d10f4d55fd51972e34f8703a0f
  BidUint256{ w: [0x0e128b5d938cfb3f, 0x2a1fee40d90aab31, 0x3f2398d747b36224, 0xa87fea27a539e9a5 ] }, // (ten2mx >> 256) = a87fea27a539e9a5  3f2398d747b36224  |  2a1fee40d90aab310e128b5d938cfb3f
  BidUint256{ w: [0x3e753c4adc70c8ff, 0xbb4cbe9a473bbc27, 0x98e947129fc2b4e9, 0x86ccbb52ea94baea ] }, // (ten2mx >> 256) = 86ccbb52ea94baea  98e947129fc2b4e9  |  bb4cbe9a473bbc273e753c4adc70c8ff
  BidUint256{ w: [0x30bb93aafa4e0e65, 0x9214642a0b92c6a5, 0x5b0ed81dcc6abb0f, 0xd7adf884aa879177 ] }, // (ten2mx >> 256) = d7adf884aa879177  5b0ed81dcc6abb0f  |  9214642a0b92c6a530bb93aafa4e0e65
  BidUint256{ w: [0xc0960fbbfb71a51e, 0xa8105021a2dbd21d, 0xe272467e3d222f3f, 0xac8b2d36eed2dac5 ] }, // (ten2mx >> 256) = ac8b2d36eed2dac5  e272467e3d222f3f  |  a8105021a2dbd21dc0960fbbfb71a51e
  BidUint256{ w: [0x66de72fcc927b74b, 0xb9a6a6814f1641b1, 0x1b8e9ecb641b58ff, 0x8a08f0f8bf0f156b ] }, // (ten2mx >> 256) = 8a08f0f8bf0f156b  1b8e9ecb641b58ff  |  b9a6a6814f1641b166de72fcc927b74b
  BidUint256{ w: [0xd7ca5194750c5878, 0xf5d770cee4f0691b, 0xf8e431456cf88e65, 0xdcdb1b2798182244 ] }, // (ten2mx >> 256) = dcdb1b2798182244  f8e431456cf88e65  |  f5d770cee4f0691bd7ca5194750c5878
  BidUint256{ w: [0xdfd50e105da379f9, 0x9179270bea59edaf, 0x2d835a9df0c6d851, 0xb0af48ec79ace837 ] }, // (ten2mx >> 256) = b0af48ec79ace837  2d835a9df0c6d851  |  9179270bea59edafdfd50e105da379f9
  BidUint256{ w: [0x19773e737e1c6194, 0x0dfa85a321e18af3, 0x579c487e5a38ad0e, 0x8d590723948a535f ] }, // (ten2mx >> 256) = 8d590723948a535f  579c487e5a38ad0e  |  dfa85a321e18af319773e737e1c6194
  BidUint256{ w: [0xf58b971f302d68ee, 0x165da29e9c9c1184, 0x25c6da63c38de1b0, 0xe2280b6c20dd5232 ] }, // (ten2mx >> 256) = e2280b6c20dd5232  25c6da63c38de1b0  |  165da29e9c9c1184f58b971f302d68ee
  BidUint256{ w: [0xc46fac18f3578724, 0x4517b54bb07cdad0, 0x1e38aeb6360b1af3, 0xb4ecd5f01a4aa828 ] }, // (ten2mx >> 256) = b4ecd5f01a4aa828  1e38aeb6360b1af3  |  4517b54bb07cdad0c46fac18f3578724
  BidUint256{ w: [0x36bfbce0c2ac6c1d, 0x9dac910959fd7bda, 0xb1c6f22b5e6f48c2, 0x90bd77f3483bb9b9 ] }, // (ten2mx >> 256) = 90bd77f3483bb9b9  b1c6f22b5e6f48c2  |  9dac910959fd7bda36bfbce0c2ac6c1d
  BidUint256{ w: [0x2465fb01377a4695, 0x2f7a81a88ffbf95d, 0xb60b1d1230b20e04, 0xe7958cb87392c2c2 ] }  // (ten2mx >> 256) = e7958cb87392c2c2  b60b1d1230b20e0   | 2f7a81a88ffbf95d2465fb01377a4695
];

/// Table used to convert `n` to string, where **10** <= `n` <= **99**.
#[rustfmt::skip]
pub const BID_CHAR_TABLE2: [u8; 180] = [
  b'1', b'0',
  b'1', b'1',
  b'1', b'2',
  b'1', b'3',
  b'1', b'4',
  b'1', b'5',
  b'1', b'6',
  b'1', b'7',
  b'1', b'8',
  b'1', b'9',
  b'2', b'0',
  b'2', b'1',
  b'2', b'2',
  b'2', b'3',
  b'2', b'4',
  b'2', b'5',
  b'2', b'6',
  b'2', b'7',
  b'2', b'8',
  b'2', b'9',
  b'3', b'0',
  b'3', b'1',
  b'3', b'2',
  b'3', b'3',
  b'3', b'4',
  b'3', b'5',
  b'3', b'6',
  b'3', b'7',
  b'3', b'8',
  b'3', b'9',
  b'4', b'0',
  b'4', b'1',
  b'4', b'2',
  b'4', b'3',
  b'4', b'4',
  b'4', b'5',
  b'4', b'6',
  b'4', b'7',
  b'4', b'8',
  b'4', b'9',
  b'5', b'0',
  b'5', b'1',
  b'5', b'2',
  b'5', b'3',
  b'5', b'4',
  b'5', b'5',
  b'5', b'6',
  b'5', b'7',
  b'5', b'8',
  b'5', b'9',
  b'6', b'0',
  b'6', b'1',
  b'6', b'2',
  b'6', b'3',
  b'6', b'4',
  b'6', b'5',
  b'6', b'6',
  b'6', b'7',
  b'6', b'8',
  b'6', b'9',
  b'7', b'0',
  b'7', b'1',
  b'7', b'2',
  b'7', b'3',
  b'7', b'4',
  b'7', b'5',
  b'7', b'6',
  b'7', b'7',
  b'7', b'8',
  b'7', b'9',
  b'8', b'0',
  b'8', b'1',
  b'8', b'2',
  b'8', b'3',
  b'8', b'4',
  b'8', b'5',
  b'8', b'6',
  b'8', b'7',
  b'8', b'8',
  b'8', b'9',
  b'9', b'0',
  b'9', b'1',
  b'9', b'2',
  b'9', b'3',
  b'9', b'4',
  b'9', b'5',
  b'9', b'6',
  b'9', b'7',
  b'9', b'8',
  b'9', b'9'
];

/// Table used to convert `n` to string, where **000** <= `n` <= **999**.
#[rustfmt::skip]
pub const BID_CHAR_TABLE3: [u8; 3000] = [
  b'0', b'0', b'0',
  b'0', b'0', b'1',
  b'0', b'0', b'2',
  b'0', b'0', b'3',
  b'0', b'0', b'4',
  b'0', b'0', b'5',
  b'0', b'0', b'6',
  b'0', b'0', b'7',
  b'0', b'0', b'8',
  b'0', b'0', b'9',
  b'0', b'1', b'0',
  b'0', b'1', b'1',
  b'0', b'1', b'2',
  b'0', b'1', b'3',
  b'0', b'1', b'4',
  b'0', b'1', b'5',
  b'0', b'1', b'6',
  b'0', b'1', b'7',
  b'0', b'1', b'8',
  b'0', b'1', b'9',
  b'0', b'2', b'0',
  b'0', b'2', b'1',
  b'0', b'2', b'2',
  b'0', b'2', b'3',
  b'0', b'2', b'4',
  b'0', b'2', b'5',
  b'0', b'2', b'6',
  b'0', b'2', b'7',
  b'0', b'2', b'8',
  b'0', b'2', b'9',
  b'0', b'3', b'0',
  b'0', b'3', b'1',
  b'0', b'3', b'2',
  b'0', b'3', b'3',
  b'0', b'3', b'4',
  b'0', b'3', b'5',
  b'0', b'3', b'6',
  b'0', b'3', b'7',
  b'0', b'3', b'8',
  b'0', b'3', b'9',
  b'0', b'4', b'0',
  b'0', b'4', b'1',
  b'0', b'4', b'2',
  b'0', b'4', b'3',
  b'0', b'4', b'4',
  b'0', b'4', b'5',
  b'0', b'4', b'6',
  b'0', b'4', b'7',
  b'0', b'4', b'8',
  b'0', b'4', b'9',
  b'0', b'5', b'0',
  b'0', b'5', b'1',
  b'0', b'5', b'2',
  b'0', b'5', b'3',
  b'0', b'5', b'4',
  b'0', b'5', b'5',
  b'0', b'5', b'6',
  b'0', b'5', b'7',
  b'0', b'5', b'8',
  b'0', b'5', b'9',
  b'0', b'6', b'0',
  b'0', b'6', b'1',
  b'0', b'6', b'2',
  b'0', b'6', b'3',
  b'0', b'6', b'4',
  b'0', b'6', b'5',
  b'0', b'6', b'6',
  b'0', b'6', b'7',
  b'0', b'6', b'8',
  b'0', b'6', b'9',
  b'0', b'7', b'0',
  b'0', b'7', b'1',
  b'0', b'7', b'2',
  b'0', b'7', b'3',
  b'0', b'7', b'4',
  b'0', b'7', b'5',
  b'0', b'7', b'6',
  b'0', b'7', b'7',
  b'0', b'7', b'8',
  b'0', b'7', b'9',
  b'0', b'8', b'0',
  b'0', b'8', b'1',
  b'0', b'8', b'2',
  b'0', b'8', b'3',
  b'0', b'8', b'4',
  b'0', b'8', b'5',
  b'0', b'8', b'6',
  b'0', b'8', b'7',
  b'0', b'8', b'8',
  b'0', b'8', b'9',
  b'0', b'9', b'0',
  b'0', b'9', b'1',
  b'0', b'9', b'2',
  b'0', b'9', b'3',
  b'0', b'9', b'4',
  b'0', b'9', b'5',
  b'0', b'9', b'6',
  b'0', b'9', b'7',
  b'0', b'9', b'8',
  b'0', b'9', b'9',
  b'1', b'0', b'0',
  b'1', b'0', b'1',
  b'1', b'0', b'2',
  b'1', b'0', b'3',
  b'1', b'0', b'4',
  b'1', b'0', b'5',
  b'1', b'0', b'6',
  b'1', b'0', b'7',
  b'1', b'0', b'8',
  b'1', b'0', b'9',
  b'1', b'1', b'0',
  b'1', b'1', b'1',
  b'1', b'1', b'2',
  b'1', b'1', b'3',
  b'1', b'1', b'4',
  b'1', b'1', b'5',
  b'1', b'1', b'6',
  b'1', b'1', b'7',
  b'1', b'1', b'8',
  b'1', b'1', b'9',
  b'1', b'2', b'0',
  b'1', b'2', b'1',
  b'1', b'2', b'2',
  b'1', b'2', b'3',
  b'1', b'2', b'4',
  b'1', b'2', b'5',
  b'1', b'2', b'6',
  b'1', b'2', b'7',
  b'1', b'2', b'8',
  b'1', b'2', b'9',
  b'1', b'3', b'0',
  b'1', b'3', b'1',
  b'1', b'3', b'2',
  b'1', b'3', b'3',
  b'1', b'3', b'4',
  b'1', b'3', b'5',
  b'1', b'3', b'6',
  b'1', b'3', b'7',
  b'1', b'3', b'8',
  b'1', b'3', b'9',
  b'1', b'4', b'0',
  b'1', b'4', b'1',
  b'1', b'4', b'2',
  b'1', b'4', b'3',
  b'1', b'4', b'4',
  b'1', b'4', b'5',
  b'1', b'4', b'6',
  b'1', b'4', b'7',
  b'1', b'4', b'8',
  b'1', b'4', b'9',
  b'1', b'5', b'0',
  b'1', b'5', b'1',
  b'1', b'5', b'2',
  b'1', b'5', b'3',
  b'1', b'5', b'4',
  b'1', b'5', b'5',
  b'1', b'5', b'6',
  b'1', b'5', b'7',
  b'1', b'5', b'8',
  b'1', b'5', b'9',
  b'1', b'6', b'0',
  b'1', b'6', b'1',
  b'1', b'6', b'2',
  b'1', b'6', b'3',
  b'1', b'6', b'4',
  b'1', b'6', b'5',
  b'1', b'6', b'6',
  b'1', b'6', b'7',
  b'1', b'6', b'8',
  b'1', b'6', b'9',
  b'1', b'7', b'0',
  b'1', b'7', b'1',
  b'1', b'7', b'2',
  b'1', b'7', b'3',
  b'1', b'7', b'4',
  b'1', b'7', b'5',
  b'1', b'7', b'6',
  b'1', b'7', b'7',
  b'1', b'7', b'8',
  b'1', b'7', b'9',
  b'1', b'8', b'0',
  b'1', b'8', b'1',
  b'1', b'8', b'2',
  b'1', b'8', b'3',
  b'1', b'8', b'4',
  b'1', b'8', b'5',
  b'1', b'8', b'6',
  b'1', b'8', b'7',
  b'1', b'8', b'8',
  b'1', b'8', b'9',
  b'1', b'9', b'0',
  b'1', b'9', b'1',
  b'1', b'9', b'2',
  b'1', b'9', b'3',
  b'1', b'9', b'4',
  b'1', b'9', b'5',
  b'1', b'9', b'6',
  b'1', b'9', b'7',
  b'1', b'9', b'8',
  b'1', b'9', b'9',
  b'2', b'0', b'0',
  b'2', b'0', b'1',
  b'2', b'0', b'2',
  b'2', b'0', b'3',
  b'2', b'0', b'4',
  b'2', b'0', b'5',
  b'2', b'0', b'6',
  b'2', b'0', b'7',
  b'2', b'0', b'8',
  b'2', b'0', b'9',
  b'2', b'1', b'0',
  b'2', b'1', b'1',
  b'2', b'1', b'2',
  b'2', b'1', b'3',
  b'2', b'1', b'4',
  b'2', b'1', b'5',
  b'2', b'1', b'6',
  b'2', b'1', b'7',
  b'2', b'1', b'8',
  b'2', b'1', b'9',
  b'2', b'2', b'0',
  b'2', b'2', b'1',
  b'2', b'2', b'2',
  b'2', b'2', b'3',
  b'2', b'2', b'4',
  b'2', b'2', b'5',
  b'2', b'2', b'6',
  b'2', b'2', b'7',
  b'2', b'2', b'8',
  b'2', b'2', b'9',
  b'2', b'3', b'0',
  b'2', b'3', b'1',
  b'2', b'3', b'2',
  b'2', b'3', b'3',
  b'2', b'3', b'4',
  b'2', b'3', b'5',
  b'2', b'3', b'6',
  b'2', b'3', b'7',
  b'2', b'3', b'8',
  b'2', b'3', b'9',
  b'2', b'4', b'0',
  b'2', b'4', b'1',
  b'2', b'4', b'2',
  b'2', b'4', b'3',
  b'2', b'4', b'4',
  b'2', b'4', b'5',
  b'2', b'4', b'6',
  b'2', b'4', b'7',
  b'2', b'4', b'8',
  b'2', b'4', b'9',
  b'2', b'5', b'0',
  b'2', b'5', b'1',
  b'2', b'5', b'2',
  b'2', b'5', b'3',
  b'2', b'5', b'4',
  b'2', b'5', b'5',
  b'2', b'5', b'6',
  b'2', b'5', b'7',
  b'2', b'5', b'8',
  b'2', b'5', b'9',
  b'2', b'6', b'0',
  b'2', b'6', b'1',
  b'2', b'6', b'2',
  b'2', b'6', b'3',
  b'2', b'6', b'4',
  b'2', b'6', b'5',
  b'2', b'6', b'6',
  b'2', b'6', b'7',
  b'2', b'6', b'8',
  b'2', b'6', b'9',
  b'2', b'7', b'0',
  b'2', b'7', b'1',
  b'2', b'7', b'2',
  b'2', b'7', b'3',
  b'2', b'7', b'4',
  b'2', b'7', b'5',
  b'2', b'7', b'6',
  b'2', b'7', b'7',
  b'2', b'7', b'8',
  b'2', b'7', b'9',
  b'2', b'8', b'0',
  b'2', b'8', b'1',
  b'2', b'8', b'2',
  b'2', b'8', b'3',
  b'2', b'8', b'4',
  b'2', b'8', b'5',
  b'2', b'8', b'6',
  b'2', b'8', b'7',
  b'2', b'8', b'8',
  b'2', b'8', b'9',
  b'2', b'9', b'0',
  b'2', b'9', b'1',
  b'2', b'9', b'2',
  b'2', b'9', b'3',
  b'2', b'9', b'4',
  b'2', b'9', b'5',
  b'2', b'9', b'6',
  b'2', b'9', b'7',
  b'2', b'9', b'8',
  b'2', b'9', b'9',
  b'3', b'0', b'0',
  b'3', b'0', b'1',
  b'3', b'0', b'2',
  b'3', b'0', b'3',
  b'3', b'0', b'4',
  b'3', b'0', b'5',
  b'3', b'0', b'6',
  b'3', b'0', b'7',
  b'3', b'0', b'8',
  b'3', b'0', b'9',
  b'3', b'1', b'0',
  b'3', b'1', b'1',
  b'3', b'1', b'2',
  b'3', b'1', b'3',
  b'3', b'1', b'4',
  b'3', b'1', b'5',
  b'3', b'1', b'6',
  b'3', b'1', b'7',
  b'3', b'1', b'8',
  b'3', b'1', b'9',
  b'3', b'2', b'0',
  b'3', b'2', b'1',
  b'3', b'2', b'2',
  b'3', b'2', b'3',
  b'3', b'2', b'4',
  b'3', b'2', b'5',
  b'3', b'2', b'6',
  b'3', b'2', b'7',
  b'3', b'2', b'8',
  b'3', b'2', b'9',
  b'3', b'3', b'0',
  b'3', b'3', b'1',
  b'3', b'3', b'2',
  b'3', b'3', b'3',
  b'3', b'3', b'4',
  b'3', b'3', b'5',
  b'3', b'3', b'6',
  b'3', b'3', b'7',
  b'3', b'3', b'8',
  b'3', b'3', b'9',
  b'3', b'4', b'0',
  b'3', b'4', b'1',
  b'3', b'4', b'2',
  b'3', b'4', b'3',
  b'3', b'4', b'4',
  b'3', b'4', b'5',
  b'3', b'4', b'6',
  b'3', b'4', b'7',
  b'3', b'4', b'8',
  b'3', b'4', b'9',
  b'3', b'5', b'0',
  b'3', b'5', b'1',
  b'3', b'5', b'2',
  b'3', b'5', b'3',
  b'3', b'5', b'4',
  b'3', b'5', b'5',
  b'3', b'5', b'6',
  b'3', b'5', b'7',
  b'3', b'5', b'8',
  b'3', b'5', b'9',
  b'3', b'6', b'0',
  b'3', b'6', b'1',
  b'3', b'6', b'2',
  b'3', b'6', b'3',
  b'3', b'6', b'4',
  b'3', b'6', b'5',
  b'3', b'6', b'6',
  b'3', b'6', b'7',
  b'3', b'6', b'8',
  b'3', b'6', b'9',
  b'3', b'7', b'0',
  b'3', b'7', b'1',
  b'3', b'7', b'2',
  b'3', b'7', b'3',
  b'3', b'7', b'4',
  b'3', b'7', b'5',
  b'3', b'7', b'6',
  b'3', b'7', b'7',
  b'3', b'7', b'8',
  b'3', b'7', b'9',
  b'3', b'8', b'0',
  b'3', b'8', b'1',
  b'3', b'8', b'2',
  b'3', b'8', b'3',
  b'3', b'8', b'4',
  b'3', b'8', b'5',
  b'3', b'8', b'6',
  b'3', b'8', b'7',
  b'3', b'8', b'8',
  b'3', b'8', b'9',
  b'3', b'9', b'0',
  b'3', b'9', b'1',
  b'3', b'9', b'2',
  b'3', b'9', b'3',
  b'3', b'9', b'4',
  b'3', b'9', b'5',
  b'3', b'9', b'6',
  b'3', b'9', b'7',
  b'3', b'9', b'8',
  b'3', b'9', b'9',
  b'4', b'0', b'0',
  b'4', b'0', b'1',
  b'4', b'0', b'2',
  b'4', b'0', b'3',
  b'4', b'0', b'4',
  b'4', b'0', b'5',
  b'4', b'0', b'6',
  b'4', b'0', b'7',
  b'4', b'0', b'8',
  b'4', b'0', b'9',
  b'4', b'1', b'0',
  b'4', b'1', b'1',
  b'4', b'1', b'2',
  b'4', b'1', b'3',
  b'4', b'1', b'4',
  b'4', b'1', b'5',
  b'4', b'1', b'6',
  b'4', b'1', b'7',
  b'4', b'1', b'8',
  b'4', b'1', b'9',
  b'4', b'2', b'0',
  b'4', b'2', b'1',
  b'4', b'2', b'2',
  b'4', b'2', b'3',
  b'4', b'2', b'4',
  b'4', b'2', b'5',
  b'4', b'2', b'6',
  b'4', b'2', b'7',
  b'4', b'2', b'8',
  b'4', b'2', b'9',
  b'4', b'3', b'0',
  b'4', b'3', b'1',
  b'4', b'3', b'2',
  b'4', b'3', b'3',
  b'4', b'3', b'4',
  b'4', b'3', b'5',
  b'4', b'3', b'6',
  b'4', b'3', b'7',
  b'4', b'3', b'8',
  b'4', b'3', b'9',
  b'4', b'4', b'0',
  b'4', b'4', b'1',
  b'4', b'4', b'2',
  b'4', b'4', b'3',
  b'4', b'4', b'4',
  b'4', b'4', b'5',
  b'4', b'4', b'6',
  b'4', b'4', b'7',
  b'4', b'4', b'8',
  b'4', b'4', b'9',
  b'4', b'5', b'0',
  b'4', b'5', b'1',
  b'4', b'5', b'2',
  b'4', b'5', b'3',
  b'4', b'5', b'4',
  b'4', b'5', b'5',
  b'4', b'5', b'6',
  b'4', b'5', b'7',
  b'4', b'5', b'8',
  b'4', b'5', b'9',
  b'4', b'6', b'0',
  b'4', b'6', b'1',
  b'4', b'6', b'2',
  b'4', b'6', b'3',
  b'4', b'6', b'4',
  b'4', b'6', b'5',
  b'4', b'6', b'6',
  b'4', b'6', b'7',
  b'4', b'6', b'8',
  b'4', b'6', b'9',
  b'4', b'7', b'0',
  b'4', b'7', b'1',
  b'4', b'7', b'2',
  b'4', b'7', b'3',
  b'4', b'7', b'4',
  b'4', b'7', b'5',
  b'4', b'7', b'6',
  b'4', b'7', b'7',
  b'4', b'7', b'8',
  b'4', b'7', b'9',
  b'4', b'8', b'0',
  b'4', b'8', b'1',
  b'4', b'8', b'2',
  b'4', b'8', b'3',
  b'4', b'8', b'4',
  b'4', b'8', b'5',
  b'4', b'8', b'6',
  b'4', b'8', b'7',
  b'4', b'8', b'8',
  b'4', b'8', b'9',
  b'4', b'9', b'0',
  b'4', b'9', b'1',
  b'4', b'9', b'2',
  b'4', b'9', b'3',
  b'4', b'9', b'4',
  b'4', b'9', b'5',
  b'4', b'9', b'6',
  b'4', b'9', b'7',
  b'4', b'9', b'8',
  b'4', b'9', b'9',
  b'5', b'0', b'0',
  b'5', b'0', b'1',
  b'5', b'0', b'2',
  b'5', b'0', b'3',
  b'5', b'0', b'4',
  b'5', b'0', b'5',
  b'5', b'0', b'6',
  b'5', b'0', b'7',
  b'5', b'0', b'8',
  b'5', b'0', b'9',
  b'5', b'1', b'0',
  b'5', b'1', b'1',
  b'5', b'1', b'2',
  b'5', b'1', b'3',
  b'5', b'1', b'4',
  b'5', b'1', b'5',
  b'5', b'1', b'6',
  b'5', b'1', b'7',
  b'5', b'1', b'8',
  b'5', b'1', b'9',
  b'5', b'2', b'0',
  b'5', b'2', b'1',
  b'5', b'2', b'2',
  b'5', b'2', b'3',
  b'5', b'2', b'4',
  b'5', b'2', b'5',
  b'5', b'2', b'6',
  b'5', b'2', b'7',
  b'5', b'2', b'8',
  b'5', b'2', b'9',
  b'5', b'3', b'0',
  b'5', b'3', b'1',
  b'5', b'3', b'2',
  b'5', b'3', b'3',
  b'5', b'3', b'4',
  b'5', b'3', b'5',
  b'5', b'3', b'6',
  b'5', b'3', b'7',
  b'5', b'3', b'8',
  b'5', b'3', b'9',
  b'5', b'4', b'0',
  b'5', b'4', b'1',
  b'5', b'4', b'2',
  b'5', b'4', b'3',
  b'5', b'4', b'4',
  b'5', b'4', b'5',
  b'5', b'4', b'6',
  b'5', b'4', b'7',
  b'5', b'4', b'8',
  b'5', b'4', b'9',
  b'5', b'5', b'0',
  b'5', b'5', b'1',
  b'5', b'5', b'2',
  b'5', b'5', b'3',
  b'5', b'5', b'4',
  b'5', b'5', b'5',
  b'5', b'5', b'6',
  b'5', b'5', b'7',
  b'5', b'5', b'8',
  b'5', b'5', b'9',
  b'5', b'6', b'0',
  b'5', b'6', b'1',
  b'5', b'6', b'2',
  b'5', b'6', b'3',
  b'5', b'6', b'4',
  b'5', b'6', b'5',
  b'5', b'6', b'6',
  b'5', b'6', b'7',
  b'5', b'6', b'8',
  b'5', b'6', b'9',
  b'5', b'7', b'0',
  b'5', b'7', b'1',
  b'5', b'7', b'2',
  b'5', b'7', b'3',
  b'5', b'7', b'4',
  b'5', b'7', b'5',
  b'5', b'7', b'6',
  b'5', b'7', b'7',
  b'5', b'7', b'8',
  b'5', b'7', b'9',
  b'5', b'8', b'0',
  b'5', b'8', b'1',
  b'5', b'8', b'2',
  b'5', b'8', b'3',
  b'5', b'8', b'4',
  b'5', b'8', b'5',
  b'5', b'8', b'6',
  b'5', b'8', b'7',
  b'5', b'8', b'8',
  b'5', b'8', b'9',
  b'5', b'9', b'0',
  b'5', b'9', b'1',
  b'5', b'9', b'2',
  b'5', b'9', b'3',
  b'5', b'9', b'4',
  b'5', b'9', b'5',
  b'5', b'9', b'6',
  b'5', b'9', b'7',
  b'5', b'9', b'8',
  b'5', b'9', b'9',
  b'6', b'0', b'0',
  b'6', b'0', b'1',
  b'6', b'0', b'2',
  b'6', b'0', b'3',
  b'6', b'0', b'4',
  b'6', b'0', b'5',
  b'6', b'0', b'6',
  b'6', b'0', b'7',
  b'6', b'0', b'8',
  b'6', b'0', b'9',
  b'6', b'1', b'0',
  b'6', b'1', b'1',
  b'6', b'1', b'2',
  b'6', b'1', b'3',
  b'6', b'1', b'4',
  b'6', b'1', b'5',
  b'6', b'1', b'6',
  b'6', b'1', b'7',
  b'6', b'1', b'8',
  b'6', b'1', b'9',
  b'6', b'2', b'0',
  b'6', b'2', b'1',
  b'6', b'2', b'2',
  b'6', b'2', b'3',
  b'6', b'2', b'4',
  b'6', b'2', b'5',
  b'6', b'2', b'6',
  b'6', b'2', b'7',
  b'6', b'2', b'8',
  b'6', b'2', b'9',
  b'6', b'3', b'0',
  b'6', b'3', b'1',
  b'6', b'3', b'2',
  b'6', b'3', b'3',
  b'6', b'3', b'4',
  b'6', b'3', b'5',
  b'6', b'3', b'6',
  b'6', b'3', b'7',
  b'6', b'3', b'8',
  b'6', b'3', b'9',
  b'6', b'4', b'0',
  b'6', b'4', b'1',
  b'6', b'4', b'2',
  b'6', b'4', b'3',
  b'6', b'4', b'4',
  b'6', b'4', b'5',
  b'6', b'4', b'6',
  b'6', b'4', b'7',
  b'6', b'4', b'8',
  b'6', b'4', b'9',
  b'6', b'5', b'0',
  b'6', b'5', b'1',
  b'6', b'5', b'2',
  b'6', b'5', b'3',
  b'6', b'5', b'4',
  b'6', b'5', b'5',
  b'6', b'5', b'6',
  b'6', b'5', b'7',
  b'6', b'5', b'8',
  b'6', b'5', b'9',
  b'6', b'6', b'0',
  b'6', b'6', b'1',
  b'6', b'6', b'2',
  b'6', b'6', b'3',
  b'6', b'6', b'4',
  b'6', b'6', b'5',
  b'6', b'6', b'6',
  b'6', b'6', b'7',
  b'6', b'6', b'8',
  b'6', b'6', b'9',
  b'6', b'7', b'0',
  b'6', b'7', b'1',
  b'6', b'7', b'2',
  b'6', b'7', b'3',
  b'6', b'7', b'4',
  b'6', b'7', b'5',
  b'6', b'7', b'6',
  b'6', b'7', b'7',
  b'6', b'7', b'8',
  b'6', b'7', b'9',
  b'6', b'8', b'0',
  b'6', b'8', b'1',
  b'6', b'8', b'2',
  b'6', b'8', b'3',
  b'6', b'8', b'4',
  b'6', b'8', b'5',
  b'6', b'8', b'6',
  b'6', b'8', b'7',
  b'6', b'8', b'8',
  b'6', b'8', b'9',
  b'6', b'9', b'0',
  b'6', b'9', b'1',
  b'6', b'9', b'2',
  b'6', b'9', b'3',
  b'6', b'9', b'4',
  b'6', b'9', b'5',
  b'6', b'9', b'6',
  b'6', b'9', b'7',
  b'6', b'9', b'8',
  b'6', b'9', b'9',
  b'7', b'0', b'0',
  b'7', b'0', b'1',
  b'7', b'0', b'2',
  b'7', b'0', b'3',
  b'7', b'0', b'4',
  b'7', b'0', b'5',
  b'7', b'0', b'6',
  b'7', b'0', b'7',
  b'7', b'0', b'8',
  b'7', b'0', b'9',
  b'7', b'1', b'0',
  b'7', b'1', b'1',
  b'7', b'1', b'2',
  b'7', b'1', b'3',
  b'7', b'1', b'4',
  b'7', b'1', b'5',
  b'7', b'1', b'6',
  b'7', b'1', b'7',
  b'7', b'1', b'8',
  b'7', b'1', b'9',
  b'7', b'2', b'0',
  b'7', b'2', b'1',
  b'7', b'2', b'2',
  b'7', b'2', b'3',
  b'7', b'2', b'4',
  b'7', b'2', b'5',
  b'7', b'2', b'6',
  b'7', b'2', b'7',
  b'7', b'2', b'8',
  b'7', b'2', b'9',
  b'7', b'3', b'0',
  b'7', b'3', b'1',
  b'7', b'3', b'2',
  b'7', b'3', b'3',
  b'7', b'3', b'4',
  b'7', b'3', b'5',
  b'7', b'3', b'6',
  b'7', b'3', b'7',
  b'7', b'3', b'8',
  b'7', b'3', b'9',
  b'7', b'4', b'0',
  b'7', b'4', b'1',
  b'7', b'4', b'2',
  b'7', b'4', b'3',
  b'7', b'4', b'4',
  b'7', b'4', b'5',
  b'7', b'4', b'6',
  b'7', b'4', b'7',
  b'7', b'4', b'8',
  b'7', b'4', b'9',
  b'7', b'5', b'0',
  b'7', b'5', b'1',
  b'7', b'5', b'2',
  b'7', b'5', b'3',
  b'7', b'5', b'4',
  b'7', b'5', b'5',
  b'7', b'5', b'6',
  b'7', b'5', b'7',
  b'7', b'5', b'8',
  b'7', b'5', b'9',
  b'7', b'6', b'0',
  b'7', b'6', b'1',
  b'7', b'6', b'2',
  b'7', b'6', b'3',
  b'7', b'6', b'4',
  b'7', b'6', b'5',
  b'7', b'6', b'6',
  b'7', b'6', b'7',
  b'7', b'6', b'8',
  b'7', b'6', b'9',
  b'7', b'7', b'0',
  b'7', b'7', b'1',
  b'7', b'7', b'2',
  b'7', b'7', b'3',
  b'7', b'7', b'4',
  b'7', b'7', b'5',
  b'7', b'7', b'6',
  b'7', b'7', b'7',
  b'7', b'7', b'8',
  b'7', b'7', b'9',
  b'7', b'8', b'0',
  b'7', b'8', b'1',
  b'7', b'8', b'2',
  b'7', b'8', b'3',
  b'7', b'8', b'4',
  b'7', b'8', b'5',
  b'7', b'8', b'6',
  b'7', b'8', b'7',
  b'7', b'8', b'8',
  b'7', b'8', b'9',
  b'7', b'9', b'0',
  b'7', b'9', b'1',
  b'7', b'9', b'2',
  b'7', b'9', b'3',
  b'7', b'9', b'4',
  b'7', b'9', b'5',
  b'7', b'9', b'6',
  b'7', b'9', b'7',
  b'7', b'9', b'8',
  b'7', b'9', b'9',
  b'8', b'0', b'0',
  b'8', b'0', b'1',
  b'8', b'0', b'2',
  b'8', b'0', b'3',
  b'8', b'0', b'4',
  b'8', b'0', b'5',
  b'8', b'0', b'6',
  b'8', b'0', b'7',
  b'8', b'0', b'8',
  b'8', b'0', b'9',
  b'8', b'1', b'0',
  b'8', b'1', b'1',
  b'8', b'1', b'2',
  b'8', b'1', b'3',
  b'8', b'1', b'4',
  b'8', b'1', b'5',
  b'8', b'1', b'6',
  b'8', b'1', b'7',
  b'8', b'1', b'8',
  b'8', b'1', b'9',
  b'8', b'2', b'0',
  b'8', b'2', b'1',
  b'8', b'2', b'2',
  b'8', b'2', b'3',
  b'8', b'2', b'4',
  b'8', b'2', b'5',
  b'8', b'2', b'6',
  b'8', b'2', b'7',
  b'8', b'2', b'8',
  b'8', b'2', b'9',
  b'8', b'3', b'0',
  b'8', b'3', b'1',
  b'8', b'3', b'2',
  b'8', b'3', b'3',
  b'8', b'3', b'4',
  b'8', b'3', b'5',
  b'8', b'3', b'6',
  b'8', b'3', b'7',
  b'8', b'3', b'8',
  b'8', b'3', b'9',
  b'8', b'4', b'0',
  b'8', b'4', b'1',
  b'8', b'4', b'2',
  b'8', b'4', b'3',
  b'8', b'4', b'4',
  b'8', b'4', b'5',
  b'8', b'4', b'6',
  b'8', b'4', b'7',
  b'8', b'4', b'8',
  b'8', b'4', b'9',
  b'8', b'5', b'0',
  b'8', b'5', b'1',
  b'8', b'5', b'2',
  b'8', b'5', b'3',
  b'8', b'5', b'4',
  b'8', b'5', b'5',
  b'8', b'5', b'6',
  b'8', b'5', b'7',
  b'8', b'5', b'8',
  b'8', b'5', b'9',
  b'8', b'6', b'0',
  b'8', b'6', b'1',
  b'8', b'6', b'2',
  b'8', b'6', b'3',
  b'8', b'6', b'4',
  b'8', b'6', b'5',
  b'8', b'6', b'6',
  b'8', b'6', b'7',
  b'8', b'6', b'8',
  b'8', b'6', b'9',
  b'8', b'7', b'0',
  b'8', b'7', b'1',
  b'8', b'7', b'2',
  b'8', b'7', b'3',
  b'8', b'7', b'4',
  b'8', b'7', b'5',
  b'8', b'7', b'6',
  b'8', b'7', b'7',
  b'8', b'7', b'8',
  b'8', b'7', b'9',
  b'8', b'8', b'0',
  b'8', b'8', b'1',
  b'8', b'8', b'2',
  b'8', b'8', b'3',
  b'8', b'8', b'4',
  b'8', b'8', b'5',
  b'8', b'8', b'6',
  b'8', b'8', b'7',
  b'8', b'8', b'8',
  b'8', b'8', b'9',
  b'8', b'9', b'0',
  b'8', b'9', b'1',
  b'8', b'9', b'2',
  b'8', b'9', b'3',
  b'8', b'9', b'4',
  b'8', b'9', b'5',
  b'8', b'9', b'6',
  b'8', b'9', b'7',
  b'8', b'9', b'8',
  b'8', b'9', b'9',
  b'9', b'0', b'0',
  b'9', b'0', b'1',
  b'9', b'0', b'2',
  b'9', b'0', b'3',
  b'9', b'0', b'4',
  b'9', b'0', b'5',
  b'9', b'0', b'6',
  b'9', b'0', b'7',
  b'9', b'0', b'8',
  b'9', b'0', b'9',
  b'9', b'1', b'0',
  b'9', b'1', b'1',
  b'9', b'1', b'2',
  b'9', b'1', b'3',
  b'9', b'1', b'4',
  b'9', b'1', b'5',
  b'9', b'1', b'6',
  b'9', b'1', b'7',
  b'9', b'1', b'8',
  b'9', b'1', b'9',
  b'9', b'2', b'0',
  b'9', b'2', b'1',
  b'9', b'2', b'2',
  b'9', b'2', b'3',
  b'9', b'2', b'4',
  b'9', b'2', b'5',
  b'9', b'2', b'6',
  b'9', b'2', b'7',
  b'9', b'2', b'8',
  b'9', b'2', b'9',
  b'9', b'3', b'0',
  b'9', b'3', b'1',
  b'9', b'3', b'2',
  b'9', b'3', b'3',
  b'9', b'3', b'4',
  b'9', b'3', b'5',
  b'9', b'3', b'6',
  b'9', b'3', b'7',
  b'9', b'3', b'8',
  b'9', b'3', b'9',
  b'9', b'4', b'0',
  b'9', b'4', b'1',
  b'9', b'4', b'2',
  b'9', b'4', b'3',
  b'9', b'4', b'4',
  b'9', b'4', b'5',
  b'9', b'4', b'6',
  b'9', b'4', b'7',
  b'9', b'4', b'8',
  b'9', b'4', b'9',
  b'9', b'5', b'0',
  b'9', b'5', b'1',
  b'9', b'5', b'2',
  b'9', b'5', b'3',
  b'9', b'5', b'4',
  b'9', b'5', b'5',
  b'9', b'5', b'6',
  b'9', b'5', b'7',
  b'9', b'5', b'8',
  b'9', b'5', b'9',
  b'9', b'6', b'0',
  b'9', b'6', b'1',
  b'9', b'6', b'2',
  b'9', b'6', b'3',
  b'9', b'6', b'4',
  b'9', b'6', b'5',
  b'9', b'6', b'6',
  b'9', b'6', b'7',
  b'9', b'6', b'8',
  b'9', b'6', b'9',
  b'9', b'7', b'0',
  b'9', b'7', b'1',
  b'9', b'7', b'2',
  b'9', b'7', b'3',
  b'9', b'7', b'4',
  b'9', b'7', b'5',
  b'9', b'7', b'6',
  b'9', b'7', b'7',
  b'9', b'7', b'8',
  b'9', b'7', b'9',
  b'9', b'8', b'0',
  b'9', b'8', b'1',
  b'9', b'8', b'2',
  b'9', b'8', b'3',
  b'9', b'8', b'4',
  b'9', b'8', b'5',
  b'9', b'8', b'6',
  b'9', b'8', b'7',
  b'9', b'8', b'8',
  b'9', b'8', b'9',
  b'9', b'9', b'0',
  b'9', b'9', b'1',
  b'9', b'9', b'2',
  b'9', b'9', b'3',
  b'9', b'9', b'4',
  b'9', b'9', b'5',
  b'9', b'9', b'6',
  b'9', b'9', b'7',
  b'9', b'9', b'8',
  b'9', b'9', b'9'
];
