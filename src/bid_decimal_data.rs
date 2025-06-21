use crate::Bid128;

pub fn bid_round_const_table_128(rnd_mode: u32, index: i32) -> Bid128 {
  BID_ROUND_CONST_TABLE_128[rnd_mode as usize][index as usize]
}

#[rustfmt::skip]
const BID_ROUND_CONST_TABLE_128: [[Bid128;36]; 5] = [
  [	// Rounding to nearest
    Bid128{ w: [0, 0] }, // 0 extra digits
    Bid128{ w: [5, 0] }, // 1 extra digits
    Bid128{ w: [50, 0] }, // 2 extra digits
    Bid128{ w: [500, 0] }, // 3 extra digits
    Bid128{ w: [5000, 0] }, // 4 extra digits
    Bid128{ w: [50000, 0] }, // 5 extra digits
    Bid128{ w: [500000, 0] }, // 6 extra digits
    Bid128{ w: [5000000, 0] }, // 7 extra digits
    Bid128{ w: [50000000, 0] }, // 8 extra digits
    Bid128{ w: [500000000, 0] }, // 9 extra digits
    Bid128{ w: [5000000000, 0] }, // 10 extra digits
    Bid128{ w: [50000000000, 0] }, // 11 extra digits
    Bid128{ w: [500000000000, 0] }, // 12 extra digits
    Bid128{ w: [5000000000000, 0] }, // 13 extra digits
    Bid128{ w: [50000000000000, 0] }, // 14 extra digits
    Bid128{ w: [500000000000000, 0] }, // 15 extra digits
    Bid128{ w: [5000000000000000, 0] }, // 16 extra digits
    Bid128{ w: [50000000000000000, 0] }, // 17 extra digits
    Bid128{ w: [500000000000000000, 0] }, // 18 extra digits
    Bid128{ w: [5000000000000000000, 0] }, // 19 extra digits
    Bid128{ w: [0xb5e3af16b1880000, 2] }, // 20 extra digits
    Bid128{ w: [0x1ae4d6e2ef500000, 27] }, // 21 extra digits
    Bid128{ w: [0xcf064dd59200000, 271] }, // 22 extra digits
    Bid128{ w: [0x8163f0a57b400000, 2710] }, // 23 extra digits
    Bid128{ w: [0xde76676d0800000, 27105] }, // 24 extra digits
    Bid128{ w: [0x8b0a00a425000000, 0x422ca] }, // 25 extra digits
    Bid128{ w: [0x6e64066972000000, 0x295be9] }, // 26 extra digits
    Bid128{ w: [0x4fe8401e74000000, 0x19d971e] }, // 27 extra digits
    Bid128{ w: [0x1f12813088000000, 0x1027e72f] }, // 28 extra digits
    Bid128{ w: [0x36b90be550000000, 0xa18f07d7] }, // 29 extra digits
    Bid128{ w: [0x233a76f520000000, 0x64f964e68] }, // 30 extra digits
    Bid128{ w: [0x6048a59340000000, 0x3f1bdf1011] }, // 31 extra digits
    Bid128{ w: [0xc2d677c080000000, 0x27716b6a0ad] }, // 32 extra digits
    Bid128{ w: [0x9c60ad8500000000, 0x18a6e32246c9] }, // 33 extra digits
    Bid128{ w: [0x1bc6c73200000000, 0xf684df56c3e0] }, // 34 extra digits
    Bid128{ w: [0x15c3c7f400000000, 0x9a130b963a6c1] }, // 35 extra digits
  ],
  [	// Rounding down
    Bid128{ w: [0, 0] }, // 0 extra digits
    Bid128{ w: [0, 0] }, // 1 extra digits
    Bid128{ w: [0, 0] }, // 2 extra digits
    Bid128{ w: [0, 0] }, // 3 extra digits
    Bid128{ w: [0, 0] }, // 4 extra digits
    Bid128{ w: [0, 0] }, // 5 extra digits
    Bid128{ w: [0, 0] }, // 6 extra digits
    Bid128{ w: [0, 0] }, // 7 extra digits
    Bid128{ w: [0, 0] }, // 8 extra digits
    Bid128{ w: [0, 0] }, // 9 extra digits
    Bid128{ w: [0, 0] }, // 10 extra digits
    Bid128{ w: [0, 0] }, // 11 extra digits
    Bid128{ w: [0, 0] }, // 12 extra digits
    Bid128{ w: [0, 0] }, // 13 extra digits
    Bid128{ w: [0, 0] }, // 14 extra digits
    Bid128{ w: [0, 0] }, // 15 extra digits
    Bid128{ w: [0, 0] }, // 16 extra digits
    Bid128{ w: [0, 0] }, // 17 extra digits
    Bid128{ w: [0, 0] }, // 18 extra digits
    Bid128{ w: [0, 0] }, // 19 extra digits
    Bid128{ w: [0, 0] }, // 20 extra digits
    Bid128{ w: [0, 0] }, // 21 extra digits
    Bid128{ w: [0, 0] }, // 22 extra digits
    Bid128{ w: [0, 0] }, // 23 extra digits
    Bid128{ w: [0, 0] }, // 24 extra digits
    Bid128{ w: [0, 0] }, // 25 extra digits
    Bid128{ w: [0, 0] }, // 26 extra digits
    Bid128{ w: [0, 0] }, // 27 extra digits
    Bid128{ w: [0, 0] }, // 28 extra digits
    Bid128{ w: [0, 0] }, // 29 extra digits
    Bid128{ w: [0, 0] }, // 30 extra digits
    Bid128{ w: [0, 0] }, // 31 extra digits
    Bid128{ w: [0, 0] }, // 32 extra digits
    Bid128{ w: [0, 0] }, // 33 extra digits
    Bid128{ w: [0, 0] }, // 34 extra digits
    Bid128{ w: [0, 0] }, // 35 extra digits
  ],
  [	// Rounding up
    Bid128{ w: [0, 0] }, // 0 extra digits
    Bid128{ w: [9, 0] }, // 1 extra digits
    Bid128{ w: [99, 0] }, // 2 extra digits
    Bid128{ w: [999, 0] }, // 3 extra digits
    Bid128{ w: [9999, 0] }, // 4 extra digits
    Bid128{ w: [99999, 0] }, // 5 extra digits
    Bid128{ w: [999999, 0] }, // 6 extra digits
    Bid128{ w: [9999999, 0] }, // 7 extra digits
    Bid128{ w: [99999999, 0] }, // 8 extra digits
    Bid128{ w: [999999999, 0] }, // 9 extra digits
    Bid128{ w: [9999999999, 0] }, // 10 extra digits
    Bid128{ w: [99999999999, 0] }, // 11 extra digits
    Bid128{ w: [999999999999, 0] }, // 12 extra digits
    Bid128{ w: [9999999999999, 0] }, // 13 extra digits
    Bid128{ w: [99999999999999, 0] }, // 14 extra digits
    Bid128{ w: [999999999999999, 0] }, // 15 extra digits
    Bid128{ w: [9999999999999999, 0] }, // 16 extra digits
    Bid128{ w: [99999999999999999, 0] }, // 17 extra digits
    Bid128{ w: [999999999999999999, 0] }, // 18 extra digits
    Bid128{ w: [9999999999999999999, 0] }, // 19 extra digits
    Bid128{ w: [0x6BC75E2D630FFFFF, 0x5] }, // 20 extra digits
    Bid128{ w: [0x35C9ADC5DE9FFFFF, 0x36] }, // 21 extra digits
    Bid128{ w: [0x19E0C9BAB23FFFFF, 0x21e] }, // 22 extra digits
    Bid128{ w: [0x2C7E14AF67FFFFF, 0x152d] }, // 23 extra digits
    Bid128{ w: [0x1BCECCEDA0FFFFFF, 0xd3c2] }, // 24 extra digits
    Bid128{ w: [0x1614014849FFFFFF, 0x84595] }, // 25 extra digits
    Bid128{ w: [0xDCC80CD2E3FFFFFF, 0x52b7d2] }, // 26 extra digits
    Bid128{ w: [0x9FD0803CE7FFFFFF, 0x33B2E3C] }, // 27 extra digits
    Bid128{ w: [0x3E2502610FFFFFFF, 0x204FCE5E] }, // 28 extra digits
    Bid128{ w: [0x6D7217CA9FFFFFFF, 0x1431E0FAE] }, // 29 extra digits
    Bid128{ w: [0x4674EDEA3FFFFFFF, 0xC9F2C9CD0] }, // 30 extra digits
    Bid128{ w: [0xC0914B267FFFFFFF, 0x7E37BE2022] }, // 31 extra digits
    Bid128{ w: [0x85ACEF80FFFFFFFF, 0x4EE2D6D415B] }, // 32 extra digits
    Bid128{ w: [0x38c15b09ffffffff, 0x314dc6448d93] }, // 33 extra digits
    Bid128{ w: [0x378d8e63ffffffff, 0x1ed09bead87c0] }, // 34 extra digits
    Bid128{ w: [0x2b878fe7ffffffff, 0x13426172c74d82] }, // 35 extra digits
  ],
  [	// Rounding to zero
    Bid128{ w: [0, 0] }, // 0 extra digits
    Bid128{ w: [0, 0] }, // 1 extra digits
    Bid128{ w: [0, 0] }, // 2 extra digits
    Bid128{ w: [0, 0] }, // 3 extra digits
    Bid128{ w: [0, 0] }, // 4 extra digits
    Bid128{ w: [0, 0] }, // 5 extra digits
    Bid128{ w: [0, 0] }, // 6 extra digits
    Bid128{ w: [0, 0] }, // 7 extra digits
    Bid128{ w: [0, 0] }, // 8 extra digits
    Bid128{ w: [0, 0] }, // 9 extra digits
    Bid128{ w: [0, 0] }, // 10 extra digits
    Bid128{ w: [0, 0] }, // 11 extra digits
    Bid128{ w: [0, 0] }, // 12 extra digits
    Bid128{ w: [0, 0] }, // 13 extra digits
    Bid128{ w: [0, 0] }, // 14 extra digits
    Bid128{ w: [0, 0] }, // 15 extra digits
    Bid128{ w: [0, 0] }, // 16 extra digits
    Bid128{ w: [0, 0] }, // 17 extra digits
    Bid128{ w: [0, 0] }, // 18 extra digits
    Bid128{ w: [0, 0] }, // 19 extra digits
    Bid128{ w: [0, 0] }, // 20 extra digits
    Bid128{ w: [0, 0] }, // 21 extra digits
    Bid128{ w: [0, 0] }, // 22 extra digits
    Bid128{ w: [0, 0] }, // 23 extra digits
    Bid128{ w: [0, 0] }, // 24 extra digits
    Bid128{ w: [0, 0] }, // 25 extra digits
    Bid128{ w: [0, 0] }, // 26 extra digits
    Bid128{ w: [0, 0] }, // 27 extra digits
    Bid128{ w: [0, 0] }, // 28 extra digits
    Bid128{ w: [0, 0] }, // 29 extra digits
    Bid128{ w: [0, 0] }, // 30 extra digits
    Bid128{ w: [0, 0] }, // 31 extra digits
    Bid128{ w: [0, 0] }, // 32 extra digits
    Bid128{ w: [0, 0] }, // 33 extra digits
    Bid128{ w: [0, 0] }, // 34 extra digits
    Bid128{ w: [0, 0] }, // 35 extra digits
  ],
  [	// Rounding ties away
    Bid128{ w: [0, 0] }, // 0 extra digits
    Bid128{ w: [5, 0] }, // 1 extra digits
    Bid128{ w: [50, 0] }, // 2 extra digits
    Bid128{ w: [500, 0] }, // 3 extra digits
    Bid128{ w: [5000, 0] }, // 4 extra digits
    Bid128{ w: [50000, 0] }, // 5 extra digits
    Bid128{ w: [500000, 0] }, // 6 extra digits
    Bid128{ w: [5000000, 0] }, // 7 extra digits
    Bid128{ w: [50000000, 0] }, // 8 extra digits
    Bid128{ w: [500000000, 0] }, // 9 extra digits
    Bid128{ w: [5000000000, 0] }, // 10 extra digits
    Bid128{ w: [50000000000, 0] }, // 11 extra digits
    Bid128{ w: [500000000000, 0] }, // 12 extra digits
    Bid128{ w: [5000000000000, 0] }, // 13 extra digits
    Bid128{ w: [50000000000000, 0] }, // 14 extra digits
    Bid128{ w: [500000000000000, 0] }, // 15 extra digits
    Bid128{ w: [5000000000000000, 0] }, // 16 extra digits
    Bid128{ w: [50000000000000000, 0] }, // 17 extra digits
    Bid128{ w: [500000000000000000, 0] }, // 18 extra digits
    Bid128{ w: [5000000000000000000, 0] }, // 19 extra digits
    Bid128{ w: [0xb5e3af16b1880000, 2] }, // 20 extra didits
    Bid128{ w: [0x1ae4d6e2ef500000, 27] }, // 21 extra didits
    Bid128{ w: [0xcf064dd59200000, 271] }, // 22 extra didits
    Bid128{ w: [0x8163f0a57b400000, 2710] }, // 23 extra didits
    Bid128{ w: [0xde76676d0800000, 27105] }, // 24 extra didits
    Bid128{ w: [0x8b0a00a425000000, 0x422ca] }, // 25 extra didits
    Bid128{ w: [0x6e64066972000000, 0x295be9] }, // 26 extra didits
    Bid128{ w: [0x4fe8401e74000000, 0x19d971e] }, // 27 extra didits
    Bid128{ w: [0x1f12813088000000, 0x1027e72f] }, // 28 extra didits
    Bid128{ w: [0x36b90be550000000, 0xa18f07d7] }, // 29 extra didits
    Bid128{ w: [0x233a76f520000000, 0x64f964e68] }, // 30 extra didits
    Bid128{ w: [0x6048a59340000000, 0x3f1bdf1011] }, // 31 extra didits
    Bid128{ w: [0xc2d677c080000000, 0x27716b6a0ad] }, // 32 extra didits
    Bid128{ w: [0x9c60ad8500000000, 0x18a6e32246c9] }, // 33 extra didits
    Bid128{ w: [0x1bc6c73200000000, 0xf684df56c3e0] }, // 34 extra didits
    Bid128{ w: [0x15c3c7f400000000, 0x9a130b963a6c1] }, // 35 extra didits
  ]
];

pub fn bid_reciprocals10_128(index: i32) -> Bid128 {
  BID_RECIPROCALS10_128[index as usize]
}

#[rustfmt::skip]
const BID_RECIPROCALS10_128: [Bid128;36] = [
  Bid128{ w: [0, 0] },	// 0 extra digits
  Bid128{ w: [0x3333333333333334, 0x3333333333333333] },	// 1 extra digit
  Bid128{ w: [0x51eb851eb851eb86, 0x051eb851eb851eb8] },	// 2 extra digits
  Bid128{ w: [0x3b645a1cac083127, 0x0083126e978d4fdf] },	// 3 extra digits
  Bid128{ w: [0x4af4f0d844d013aa, 0x00346dc5d6388659] },	// 10^(-4) * 2^131
  Bid128{ w: [0x08c3f3e0370cdc88, 0x0029f16b11c6d1e1] },	// 10^(-5) * 2^134
  Bid128{ w: [0x6d698fe69270b06d, 0x00218def416bdb1a] },	// 10^(-6) * 2^137
  Bid128{ w: [0xaf0f4ca41d811a47, 0x0035afe535795e90] },	// 10^(-7) * 2^141
  Bid128{ w: [0xbf3f70834acdaea0, 0x002af31dc4611873] },	// 10^(-8) * 2^144
  Bid128{ w: [0x65cc5a02a23e254d, 0x00225c17d04dad29] },	// 10^(-9) * 2^147
  Bid128{ w: [0x6fad5cd10396a214, 0x0036f9bfb3af7b75] },	// 10^(-10) * 2^151
  Bid128{ w: [0xbfbde3da69454e76, 0x002bfaffc2f2c92a] },	// 10^(-11) * 2^154
  Bid128{ w: [0x32fe4fe1edd10b92, 0x00232f33025bd422] },	// 10^(-12) * 2^157
  Bid128{ w: [0x84ca19697c81ac1c, 0x00384b84d092ed03] },	// 10^(-13) * 2^161
  Bid128{ w: [0x03d4e1213067bce4, 0x002d09370d425736] },	// 10^(-14) * 2^164
  Bid128{ w: [0x3643e74dc052fd83, 0x0024075f3dceac2b] },	// 10^(-15) * 2^167
  Bid128{ w: [0x56d30baf9a1e626b, 0x0039a5652fb11378] },	// 10^(-16) * 2^171
  Bid128{ w: [0x12426fbfae7eb522, 0x002e1dea8c8da92d] },	// 10^(-17) * 2^174
  Bid128{ w: [0x41cebfcc8b9890e8, 0x0024e4bba3a48757] },	// 10^(-18) * 2^177
  Bid128{ w: [0x694acc7a78f41b0d, 0x003b07929f6da558] },	// 10^(-19) * 2^181
  Bid128{ w: [0xbaa23d2ec729af3e, 0x002f394219248446] },	// 10^(-20) * 2^184
  Bid128{ w: [0xfbb4fdbf05baf298, 0x0025c768141d369e] },	// 10^(-21) * 2^187
  Bid128{ w: [0x2c54c931a2c4b759, 0x003c7240202ebdcb] },	// 10^(-22) * 2^191
  Bid128{ w: [0x89dd6dc14f03c5e1, 0x00305b66802564a2] },	// 10^(-23) * 2^194
  Bid128{ w: [0xd4b1249aa59c9e4e, 0x0026af8533511d4e] },	// 10^(-24) * 2^197
  Bid128{ w: [0x544ea0f76f60fd49, 0x003de5a1ebb4fbb1] },	// 10^(-25) * 2^201
  Bid128{ w: [0x76a54d92bf80caa1, 0x00318481895d9627] },	// 10^(-26) * 2^204
  Bid128{ w: [0x921dd7a89933d54e, 0x00279d346de4781f] },	// 10^(-27) * 2^207
  Bid128{ w: [0x8362f2a75b862215, 0x003f61ed7ca0c032] },	// 10^(-28) * 2^211
  Bid128{ w: [0xcf825bb91604e811, 0x0032b4bdfd4d668e] },	// 10^(-29) * 2^214
  Bid128{ w: [0x0c684960de6a5341, 0x00289097fdd7853f] },	// 10^(-30) * 2^217
  Bid128{ w: [0x3d203ab3e521dc34, 0x002073accb12d0ff] },	// 10^(-31) * 2^220
  Bid128{ w: [0x2e99f7863b696053, 0x0033ec47ab514e65] },	// 10^(-32) * 2^224
  Bid128{ w: [0x587b2c6b62bab376, 0x002989d2ef743eb7] },	// 10^(-33) * 2^227
  Bid128{ w: [0xad2f56bc4efbc2c5, 0x00213b0f25f69892] },	// 10^(-34) * 2^230
  Bid128{ w: [0x0f2abc9d8c9689d1, 0x01a95a5b7f87a0ef] },	// 35 extra digits
];

pub fn bid_power10_table_128(index: i32) -> Bid128 {
  BID_POWER10_TABLE_128[index as usize]
}

#[rustfmt::skip]
const BID_POWER10_TABLE_128: [Bid128; 39] = [
  Bid128{ w: [0x0000000000000001, 0x0000000000000000] },	// 10^0
  Bid128{ w: [0x000000000000000a, 0x0000000000000000] },	// 10^1
  Bid128{ w: [0x0000000000000064, 0x0000000000000000] },	// 10^2
  Bid128{ w: [0x00000000000003e8, 0x0000000000000000] },	// 10^3
  Bid128{ w: [0x0000000000002710, 0x0000000000000000] },	// 10^4
  Bid128{ w: [0x00000000000186a0, 0x0000000000000000] },	// 10^5
  Bid128{ w: [0x00000000000f4240, 0x0000000000000000] },	// 10^6
  Bid128{ w: [0x0000000000989680, 0x0000000000000000] },	// 10^7
  Bid128{ w: [0x0000000005f5e100, 0x0000000000000000] },	// 10^8
  Bid128{ w: [0x000000003b9aca00, 0x0000000000000000] },	// 10^9
  Bid128{ w: [0x00000002540be400, 0x0000000000000000] },	// 10^10
  Bid128{ w: [0x000000174876e800, 0x0000000000000000] },	// 10^11
  Bid128{ w: [0x000000e8d4a51000, 0x0000000000000000] },	// 10^12
  Bid128{ w: [0x000009184e72a000, 0x0000000000000000] },	// 10^13
  Bid128{ w: [0x00005af3107a4000, 0x0000000000000000] },	// 10^14
  Bid128{ w: [0x00038d7ea4c68000, 0x0000000000000000] },	// 10^15
  Bid128{ w: [0x002386f26fc10000, 0x0000000000000000] },	// 10^16
  Bid128{ w: [0x016345785d8a0000, 0x0000000000000000] },	// 10^17
  Bid128{ w: [0x0de0b6b3a7640000, 0x0000000000000000] },	// 10^18
  Bid128{ w: [0x8ac7230489e80000, 0x0000000000000000] },	// 10^19
  Bid128{ w: [0x6bc75e2d63100000, 0x0000000000000005] },	// 10^20
  Bid128{ w: [0x35c9adc5dea00000, 0x0000000000000036] },	// 10^21
  Bid128{ w: [0x19e0c9bab2400000, 0x000000000000021e] },	// 10^22
  Bid128{ w: [0x02c7e14af6800000, 0x000000000000152d] },	// 10^23
  Bid128{ w: [0x1bcecceda1000000, 0x000000000000d3c2] },	// 10^24
  Bid128{ w: [0x161401484a000000, 0x0000000000084595] },	// 10^25
  Bid128{ w: [0xdcc80cd2e4000000, 0x000000000052b7d2] },	// 10^26
  Bid128{ w: [0x9fd0803ce8000000, 0x00000000033b2e3c] },	// 10^27
  Bid128{ w: [0x3e25026110000000, 0x00000000204fce5e] },	// 10^28
  Bid128{ w: [0x6d7217caa0000000, 0x00000001431e0fae] },	// 10^29
  Bid128{ w: [0x4674edea40000000, 0x0000000c9f2c9cd0] },	// 10^30
  Bid128{ w: [0xc0914b2680000000, 0x0000007e37be2022] },	// 10^31
  Bid128{ w: [0x85acef8100000000, 0x000004ee2d6d415b] },	// 10^32
  Bid128{ w: [0x38c15b0a00000000, 0x0000314dc6448d93] },	// 10^33
  Bid128{ w: [0x378d8e6400000000, 0x0001ed09bead87c0] },	// 10^34
  Bid128{ w: [0x2b878fe800000000, 0x0013426172c74d82] },	// 10^35
  Bid128{ w: [0xb34b9f1000000000, 0x00c097ce7bc90715] },	// 10^36
  Bid128{ w: [0x00f436a000000000, 0x0785ee10d5da46d9] },	// 10^37
  Bid128{ w: [0x098a224000000000, 0x4b3b4ca85a86c47a] },	// 10^38
];

pub fn bid_recip_scale(index: i32) -> i32 {
  BID_RECIP_SCALE[index as usize]
}

#[rustfmt::skip]
const BID_RECIP_SCALE: [i32; 36] = [
  129 - 128, // 1
  129 - 128, // 1/10
  129 - 128, // 1/10^2
  129 - 128, // 1/10^3
  3,         // 131 - 128
  6,         // 134 - 128
  9,         // 137 - 128
  13,        // 141 - 128
  16,        // 144 - 128
  19,        // 147 - 128
  23,        // 151 - 128
  26,        // 154 - 128
  29,        // 157 - 128
  33,        // 161 - 128
  36,        // 164 - 128
  39,        // 167 - 128
  43,        // 171 - 128
  46,        // 174 - 128
  49,        // 177 - 128
  53,        // 181 - 128
  56,        // 184 - 128
  59,        // 187 - 128
  63,        // 191 - 128
  66,        // 194 - 128
  69,        // 197 - 128
  73,        // 201 - 128
  76,        // 204 - 128
  79,        // 207 - 128
  83,        // 211 - 128
  86,        // 214 - 128
  89,        // 217 - 128
  92,        // 220 - 128
  96,        // 224 - 128
  99,        // 227 - 128
  102,       // 230 - 128
  109,       // 237 - 128, 1/10^35
];
