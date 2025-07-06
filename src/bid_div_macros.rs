use crate::bid_internal::*;
use crate::{BidUint64, BidUint128, BidUint256};

#[inline(always)]
pub fn bid_div_128_by_128(p_cq: &mut BidUint128, p_cr: &mut BidUint128, cx0: BidUint128, cy: BidUint128) {
  let mut cy36: BidUint128 = Default::default();
  let mut cy51: BidUint128 = Default::default();
  let mut cq: BidUint128 = Default::default();
  let mut a2: BidUint128 = Default::default();
  let mut cx: BidUint128 = Default::default();
  let mut cqt: BidUint128 = Default::default();
  let mut q: BidUint64;
  let mut t64: IntDouble = Default::default();
  let mut d49: IntDouble = Default::default();
  let mut d60: IntDouble = Default::default();

  if cx0.w[1] == 0 && cy.w[1] == 0 {
    p_cq.w[0] = cx0.w[0] / cy.w[0];
    p_cq.w[1] = 0;
    p_cr.w[1] = 0;
    p_cr.w[0] = 0;
    p_cr.w[0] = cx0.w[0].wrapping_sub(p_cq.w[0].wrapping_mul(cy.w[0]));
    return;
  }

  cx.w[1] = cx0.w[1];
  cx.w[0] = cx0.w[0];

  // 2^64
  t64.i = 0x43f0000000000000;
  let mut lx = (cx.w[1] as f64) * unsafe { t64.d } + (cx.w[0] as f64);
  let ly = (cy.w[1] as f64) * unsafe { t64.d } + (cy.w[0] as f64);
  let mut lq = lx / ly;

  cy36.w[1] = cy.w[0] >> (64 - 36);
  cy36.w[0] = cy.w[0] << 36;

  cq.w[1] = 0;
  cq.w[0] = 0;

  // q >= 2^100 ?
  if cy.w[1] == 0 && cy36.w[1] == 0 && (cx.w[1] >= cy36.w[0]) {
    // then q >= 2^100

    // 2^(-60)*cx/cy
    d60.i = 0x3c30000000000000;
    lq *= unsafe { d60.d };
    q = (lq as u64) - 4;

    // q*cy
    mul_64x64_to_128!(a2, q, cy.w[0]);

    // a2 <<= 60
    a2.w[1] = (a2.w[1] << 60) | (a2.w[0] >> (64 - 60));
    a2.w[0] <<= 60;

    sub_128_128!(cx, cx, a2);

    lx = (cx.w[1] as f64) * unsafe { t64.d } + (cx.w[0] as f64);
    lq = lx / ly;

    cq.w[1] = q >> (64 - 60);
    cq.w[0] = q << 60;
  }

  cy51.w[1] = (cy.w[1] << 51) | (cy.w[0] >> (64 - 51));
  cy51.w[0] = cy.w[0] << 51;

  if cy.w[1] < (1 << (64 - 51)) && (unsigned_compare_gt_128!(cx, cy51)) {
    // q > 2^51

    // 2^(-49)*cx/cy
    d49.i = 0x3ce0000000000000;
    lq *= unsafe { d49.d };

    q = (lq as u64) - 1;

    // q*cy
    mul_64x64_to_128!(a2, q, cy.w[0]);
    a2.w[1] += q * cy.w[1];

    // a2 <<= 49
    a2.w[1] = (a2.w[1] << 49) | (a2.w[0] >> (64 - 49));
    a2.w[0] <<= 49;

    sub_128_128!(cx, cx, a2);

    cqt.w[1] = q >> (64 - 49);
    cqt.w[0] = q << 49;
    add_128_128!(cq, cq, cqt);

    lx = (cx.w[1] as f64) * unsafe { t64.d } + (cx.w[0] as f64);
    lq = lx / ly;
  }

  q = lq as u64;

  mul_64x64_to_128!(a2, q, cy.w[0]);
  inc!(a2.w[1], q.wrapping_mul(cy.w[1]));

  sub_128_128!(cx, cx, a2);
  if (cx.w[1] as i64) < 0 {
    dec!(q);
    inc!(cx.w[0], cy.w[0]);
    if cx.w[0] < cy.w[0] {
      inc!(cx.w[1]);
    }
    inc!(cx.w[1], cy.w[1]);
    if (cx.w[1] as i64) < 0 {
      dec!(q);
      cx.w[0] += cy.w[0];
      if cx.w[0] < cy.w[0] {
        inc!(cx.w[1]);
      }
      cx.w[1] += cy.w[1];
    }
  } else if unsigned_compare_ge_128!(cx, cy) {
    inc!(q);
    sub_128_128!(cx, cx, cy);
  }

  add_128_64!(cq, cq, q);

  p_cq.w[1] = cq.w[1];
  p_cq.w[0] = cq.w[0];
  p_cr.w[1] = cx.w[1];
  p_cr.w[0] = cx.w[0];
}

#[inline(always)]
pub fn bid_div_256_by_128(p_cq: &mut BidUint128, p_ca4: &mut BidUint256, cy: BidUint128) {
  let mut ca4: BidUint256 = Default::default();
  let mut ca2: BidUint256 = Default::default();
  let mut cy51: BidUint256 = Default::default();
  let mut cy36: BidUint256 = Default::default();
  let mut cq: BidUint128 = Default::default();
  let mut a2: BidUint128 = Default::default();
  let mut a2h: BidUint128 = Default::default();
  let mut cqt: BidUint128 = Default::default();
  let mut q: BidUint64;
  let mut carry64: BidUint64;
  let mut t64: IntDouble = Default::default();
  let mut d49: IntDouble = Default::default();
  let mut d60: IntDouble = Default::default();
  let mut lx: f64;
  let mut lq: f64;

  // The quotient is assumed to be at most 113 bits, as needed by BID128 divide routines.

  // Initial dividend
  ca4.w[3] = p_ca4.w[3];
  ca4.w[2] = p_ca4.w[2];
  ca4.w[1] = p_ca4.w[1];
  ca4.w[0] = p_ca4.w[0];
  cq.w[1] = p_cq.w[1];
  cq.w[0] = p_cq.w[0];

  // 2^64
  t64.i = 0x43f0000000000000;
  let d128: f64 = unsafe { t64.d } * unsafe { t64.d };
  let d192: f64 = d128 * unsafe { t64.d };
  lx = (ca4.w[3] as f64) * d192 + (ca4.w[2] as f64) * d128 + (ca4.w[1] as f64) * unsafe { t64.d } + (ca4.w[0] as f64);
  let ly: f64 = (cy.w[1] as f64) * unsafe { t64.d } + (cy.w[0] as f64);
  lq = lx / ly;

  cy36.w[2] = cy.w[1] >> (64 - 36);
  cy36.w[1] = (cy.w[1] << 36) | (cy.w[0] >> (64 - 36));
  cy36.w[0] = cy.w[0] << 36;

  //cq.w[1] = (*p_cq).w[1];
  //cq.w[0] = (*p_cq).w[0];

  // q >= 2^100 ?
  if ca4.w[3] > cy36.w[2] || (ca4.w[3] == cy36.w[2] && (ca4.w[2] > cy36.w[1] || (ca4.w[2] == cy36.w[1] && ca4.w[1] >= cy36.w[0]))) {
    // 2^(-60)*ca4/cy
    d60.i = 0x3c30000000000000;
    lq *= unsafe { d60.d };
    q = (lq as u64) - 4;

    // q*cy
    mul_64x128_to_192!(ca2, q, cy);

    // ca2 <<= 60
    // ca2.w[3] = ca2.w[2] >> (64-60);
    ca2.w[2] = (ca2.w[2] << 60) | (ca2.w[1] >> (64 - 60));
    ca2.w[1] = (ca2.w[1] << 60) | (ca2.w[0] >> (64 - 60));
    ca2.w[0] <<= 60;

    // ca4 -= ca2
    sub_borrow_out!(ca4.w[0], carry64, ca4.w[0], ca2.w[0]);
    sub_borrow_in_out!(ca4.w[1], carry64, ca4.w[1], ca2.w[1], carry64);
    ca4.w[2] = ca4.w[2].wrapping_sub(ca2.w[2]).wrapping_sub(carry64);

    lx = (ca4.w[2] as f64) * d128 + (ca4.w[1] as f64) * unsafe { t64.d } + (ca4.w[0] as f64);
    lq = lx / ly;

    cqt.w[1] = q >> (64 - 60);
    cqt.w[0] = q << 60;
    add_128_128!(cq, cq, cqt);
  }

  cy51.w[2] = cy.w[1] >> (64 - 51);
  cy51.w[1] = (cy.w[1] << 51) | (cy.w[0] >> (64 - 51));
  cy51.w[0] = cy.w[0] << 51;

  if ca4.w[2] > cy51.w[2] || ((ca4.w[2] == cy51.w[2]) && (unsigned_compare_gt_128!(ca4, cy51))) {
    // q > 2^51

    // 2^(-49)*ca4/cy
    d49.i = 0x3ce0000000000000;
    lq *= unsafe { d49.d };

    q = (lq as u64) - 1;

    // q*cy
    mul_64x64_to_128!(a2, q, cy.w[0]);
    mul_64x64_to_128!(a2h, q, cy.w[1]);
    inc!(a2.w[1], a2h.w[0]);
    if a2.w[1] < a2h.w[0] {
      inc!(a2h.w[1]);
    }

    // a2 <<= 49
    ca2.w[2] = (a2h.w[1] << 49) | (a2.w[1] >> (64 - 49));
    ca2.w[1] = (a2.w[1] << 49) | (a2.w[0] >> (64 - 49));
    ca2.w[0] = a2.w[0] << 49;

    sub_borrow_out!(ca4.w[0], carry64, ca4.w[0], ca2.w[0]);
    sub_borrow_in_out!(ca4.w[1], carry64, ca4.w[1], ca2.w[1], carry64);
    ca4.w[2] = ca4.w[2].wrapping_sub(ca2.w[2]).wrapping_sub(carry64);

    cqt.w[1] = q >> (64 - 49);
    cqt.w[0] = q << 49;
    add_128_128!(cq, cq, cqt);

    lx = (ca4.w[2] as f64) * d128 + (ca4.w[1] as f64) * unsafe { t64.d } + (ca4.w[0] as f64);
    lq = lx / ly;
  }

  q = lq as u64;
  mul_64x64_to_128!(a2, q, cy.w[0]);
  inc!(a2.w[1], q.wrapping_mul(cy.w[1]));

  sub_128_128!(ca4, ca4, a2);
  if (ca4.w[1] as i64) < 0 {
    dec!(q);
    inc!(ca4.w[0], cy.w[0]);
    if ca4.w[0] < cy.w[0] {
      inc!(ca4.w[1]);
    }
    inc!(ca4.w[1], cy.w[1]);
    if (ca4.w[1] as i64) < 0 {
      dec!(q);
      inc!(ca4.w[0], cy.w[0]);
      if ca4.w[0] < cy.w[0] {
        inc!(ca4.w[1]);
      }
      inc!(ca4.w[1], cy.w[1]);
    }
  } else if unsigned_compare_ge_128!(ca4, cy) {
    inc!(q);
    sub_128_128!(ca4, ca4, cy);
  }

  add_128_64!(cq, cq, q);

  p_cq.w[1] = cq.w[1];
  p_cq.w[0] = cq.w[0];
  p_ca4.w[1] = ca4.w[1];
  p_ca4.w[0] = ca4.w[0];
}
