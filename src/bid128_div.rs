use crate::bid_convert_data::*;
use crate::bid_decimal_data::*;
use crate::bid_div_macros::*;
use crate::bid_functions::*;
use crate::bid_internal::*;
use crate::bid128_common::*;
use crate::{BidUint32, BidUint64, BidUint128, BidUint256, IdecFlags, IdecRound};

/// Divides two 128-bit decimal floating-point values.
pub fn bid128_div(x: BidUint128, y: BidUint128, rnd_mode: IdecRound, pfpsf: &mut IdecFlags) -> BidUint128 {
  let mut ca4: BidUint256 = Default::default();
  let mut ca4r: BidUint256 = Default::default();
  let mut p256: BidUint256 = Default::default();

  let mut cx: BidUint128 = Default::default();
  let mut cy: BidUint128 = Default::default();
  let mut t128: BidUint128 = Default::default();
  let mut cq: BidUint128 = Default::default();
  let mut cr: BidUint128 = Default::default();
  let mut ca: BidUint128 = Default::default();
  let mut tp128: BidUint128 = Default::default();
  let mut qh: BidUint128 = Default::default();
  let mut ql: BidUint128 = Default::default();
  let mut res: BidUint128 = Default::default();

  let mut sign_x: BidUint64 = Default::default();
  let mut sign_y: BidUint64 = Default::default();
  let t: BidUint64;
  let mut carry64: BidUint64;
  let d: BidUint64;
  let q_high: BidUint64;
  let q_low: BidUint64;
  let qx: BidUint64;
  let pd: BidUint64;

  let mut fx: IntFloat = Default::default();
  let mut fy: IntFloat = Default::default();
  let mut f64: IntFloat = Default::default();

  let mut qx32: BidUint32;
  let mut tdigit: [BidUint32; 3] = Default::default();
  let mut digit: BidUint32;
  let mut digit_h: BidUint32;
  let digit_low: BidUint32;

  let mut exponent_x: i32 = 0;
  let mut exponent_y: i32 = 0;
  let bin_index: i32;
  let bin_expon: i32;
  let mut diff_expon: i32;
  let mut ed2: i32;
  let mut digits_q: i32;
  let amount: i32;
  let mut nzeros: i32;
  let i: i32;
  let mut j: i32;
  let mut k: i32;
  let d5: i32;
  let mut rmode: u32;

  let valid_y = unpack_bid128_value(&mut sign_y, &mut exponent_y, &mut cy, y);

  // unpack arguments, check for NaN or Infinity
  if unpack_bid128_value(&mut sign_x, &mut exponent_x, &mut cx, x) == 0 {
    // test if x is NaN
    if (x.w[1] & MASK_NAN) == MASK_NAN {
      #[cfg(feature = "bid-set-status-flags")]
      {
        if (x.w[1] & MASK_SNAN) == MASK_SNAN || (y.w[1] & MASK_SNAN) == MASK_SNAN {
          set_status_flags!(pfpsf, BID_INVALID_EXCEPTION);
        }
      }
      res.w[1] = cx.w[1] & QUIET_MASK64;
      res.w[0] = cx.w[0];
      return res;
    }
    // x is Infinity?
    if (x.w[1] & 0x7800000000000000) == 0x7800000000000000 {
      // check if y is Inf.
      if (y.w[1] & 0x7c00000000000000) == 0x7800000000000000
      // return NaN
      {
        #[cfg(feature = "bid-set-status-flags")]
        set_status_flags!(pfpsf, BID_INVALID_EXCEPTION);
        res.w[1] = 0x7c00000000000000;
        res.w[0] = 0;
        return res;
      }
      // y is NaN?
      if (y.w[1] & 0x7c00000000000000) != 0x7c00000000000000
      // return NaN
      {
        // return +/-Inf
        res.w[1] = ((x.w[1] ^ y.w[1]) & 0x8000000000000000) | 0x7800000000000000;
        res.w[0] = 0;
        return res;
      }
    }
    // x is 0
    if (y.w[1] & 0x7800000000000000) < 0x7800000000000000 {
      if cy.w[0] == 0 && (cy.w[1] & 0x0001ffffffffffff) == 0 {
        #[cfg(feature = "bid-set-status-flags")]
        set_status_flags!(pfpsf, BID_INVALID_EXCEPTION);
        // x=y=0, return NaN
        res.w[1] = 0x7c00000000000000;
        res.w[0] = 0;
        return res;
      }
      // return 0
      res.w[1] = (x.w[1] ^ y.w[1]) & 0x8000000000000000;
      exponent_x = exponent_x.wrapping_sub(exponent_y).wrapping_add(DECIMAL_EXPONENT_BIAS_128);
      exponent_x = exponent_x.clamp(0, DECIMAL_MAX_EXPON_128);
      res.w[1] |= (exponent_x as u64) << 49;
      res.w[0] = 0;
      return res;
    }
  }

  if valid_y == 0 {
    // y is Inf. or NaN

    // test if y is NaN
    if (y.w[1] & 0x7c00000000000000) == 0x7c00000000000000 {
      #[cfg(feature = "bid-set-status-flags")]
      {
        if (y.w[1] & 0x7e00000000000000) == 0x7e00000000000000 {
          set_status_flags!(pfpsf, BID_INVALID_EXCEPTION);
        }
      }
      res.w[1] = cy.w[1] & QUIET_MASK64;
      res.w[0] = cy.w[0];
      return res;
    }
    // y is Infinity?
    if (y.w[1] & 0x7800000000000000) == 0x7800000000000000 {
      // return +/-0
      res.w[1] = sign_x ^ sign_y;
      res.w[0] = 0;
      return res;
    }
    // y is 0, return +/-Inf
    #[cfg(feature = "bid-set-status-flags")]
    set_status_flags!(pfpsf, BID_ZERO_DIVIDE_EXCEPTION);

    res.w[1] = ((x.w[1] ^ y.w[1]) & 0x8000000000000000) | 0x7800000000000000;
    res.w[0] = 0;
    return res;
  }

  // #ifdef UNCHANGED_BINARY_STATUS_FLAGS
  // (void) fegetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
  // #endif

  diff_expon = exponent_x.wrapping_sub(exponent_y).wrapping_add(DECIMAL_EXPONENT_BIAS_128);

  if unsigned_compare_gt_128!(cy, cx) {
    // cx < cy

    // 2^64
    f64.i = 0x5f800000;

    // fx ~ cx,   fy ~ cy
    fx.d = (cx.w[1] as f32) * unsafe { f64.d } + (cx.w[0] as f32);
    fy.d = (cy.w[1] as f32) * unsafe { f64.d } + (cy.w[0] as f32);
    // expon_cy - expon_cx
    bin_index = ((unsafe { fy.i } - unsafe { fx.i }) >> 23) as i32;

    if cx.w[1] > 0 {
      t = bid_power10_index_binexp_128![bin_index].w[0];
      mul_64x128_short!(ca, t, cx);
    } else {
      t128 = bid_power10_index_binexp_128![bin_index];
      mul_64x128_short!(ca, cx.w[0], t128);
    }

    ed2 = 33;
    if unsigned_compare_gt_128!(cy, ca) {
      inc!(ed2);
    }

    t128 = bid_power10_table_128![ed2];
    mul_128x128_to_256!(ca4, ca, t128);

    ed2 += bid_estimate_decimal_digits![bin_index];
    cq.w[0] = 0;
    cq.w[1] = 0;
    dec!(diff_expon, ed2);
  } else {
    // get cq = cx/cy
    bid_div_128_by_128(&mut cq, &mut cr, cx, cy);

    if cr.w[1] == 0 && cr.w[0] == 0 {
      bid_get_bid128(&mut res, sign_x ^ sign_y, diff_expon, cq, rnd_mode, pfpsf);
      // #ifdef UNCHANGED_BINARY_STATUS_FLAGS
      // (void) fesetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
      // #endif
      return res;
    }
    // get number of decimal digits in cq
    // 2^64
    f64.i = 0x5f800000;
    fx.d = (cq.w[1] as f32) * unsafe { f64.d } + (cq.w[0] as f32);
    // binary expon. of cq
    bin_expon = ((unsafe { fx.i } - 0x3f800000) >> 23) as i32;

    digits_q = bid_estimate_decimal_digits![bin_expon];
    tp128.w[0] = bid_power10_index_binexp_128![bin_expon].w[0];
    tp128.w[1] = bid_power10_index_binexp_128![bin_expon].w[1];
    if unsigned_compare_ge_128!(cq, tp128) {
      inc!(digits_q);
    }

    ed2 = 34 - digits_q;
    t128.w[0] = bid_power10_table_128![ed2].w[0];
    t128.w[1] = bid_power10_table_128![ed2].w[1];
    mul_128x128_to_256!(ca4, cr, t128);
    dec!(diff_expon, ed2);
    mul_128x128_low!(cq, cq, t128);
  }

  bid_div_256_by_128(&mut cq, &mut ca4, cy);

  let mut feature_gate = false;
  if cfg!(feature = "bid-set-status-flags") {
    if ca4.w[0] > 0 || ca4.w[1] > 0 {
      set_status_flags!(pfpsf, BID_INEXACT_EXCEPTION);
    } else if cfg!(not(feature = "leave-trailing-zeros")) {
      feature_gate = true;
    }
  } else if cfg!(not(feature = "leave-trailing-zeros")) {
    feature_gate = ca4.w[0] == 0 && ca4.w[1] == 0;
  }
  // check whether result is exact
  if feature_gate {
    // check whether cx, cy are short
    if cx.w[1] == 0 && cy.w[1] == 0 && (cx.w[0] <= 1024) && (cy.w[0] <= 1024) {
      i = (cy.w[0] as i32) - 1;
      j = (cx.w[0] as i32) - 1;
      // difference in powers of 2 bid_factors for Y and X
      nzeros = ed2 - bid_factors![i, 0] + bid_factors![j, 0];
      // difference in powers of 5 bid_factors
      d5 = ed2 - bid_factors![i, 1] + bid_factors![j, 1];
      if d5 < nzeros {
        nzeros = d5;
      }
      // get P*(2^M[extra_digits])/10^extra_digits
      mul_128x128_full!(qh, ql, cq, bid_reciprocals10_128![nzeros]);

      // now get P/10^extra_digits: shift q_high right by M[extra_digits]-128
      amount = bid_recip_scale![nzeros];
      shr_128_long!(cq, qh, amount);

      diff_expon += nzeros;
    } else {
      // decompose Q as qh*10^17 + ql
      //t128 = bid_reciprocals10_128[17];
      t128.w[0] = 0x44909befeb9fad49;
      t128.w[1] = 0x000b877aa3236a4b;
      mul_128x128_to_256!(p256, cq, t128);
      //amount = bid_recip_scale[17];
      q_high = (p256.w[2] >> 44) | (p256.w[3] << (64 - 44));
      q_low = cq.w[0].wrapping_sub(q_high.wrapping_mul(100000000000000000));

      if q_low == 0 {
        diff_expon += 17;

        tdigit[0] = (q_high & 0x3ffffff) as u32;
        tdigit[1] = 0;
        qx = q_high >> 26;
        qx32 = qx as u32;
        nzeros = 0;

        j = 0;
        while qx32 > 0 {
          k = (qx32 & 127) as i32;
          inc!(tdigit[0], bid_convert_table![j, k, 0]);
          inc!(tdigit[1], bid_convert_table![j, k, 1]);
          if tdigit[0] >= 100000000 {
            dec!(tdigit[0], 100000000);
            inc!(tdigit[1]);
          }
          j += 1;
          qx32 >>= 7;
        }

        if tdigit[1] >= 100000000 {
          dec!(tdigit[1], 100000000);
          if tdigit[1] >= 100000000 {
            dec!(tdigit[1], 100000000);
          }
        }

        digit = tdigit[0];
        if digit == 0 && tdigit[1] == 0 {
          nzeros += 16;
        } else {
          if digit == 0 {
            nzeros += 8;
            digit = tdigit[1];
          }
          // decompose digit
          pd = (digit as u64) * 0x068DB8BB;
          digit_h = (pd >> 40) as u32;
          digit_low = digit - digit_h * 10000;

          if digit_low == 0 {
            nzeros += 4;
          } else {
            digit_h = digit_low;
          }

          if (digit_h & 1) == 0 {
            inc!(nzeros, (3 & ((bid_packed_10000_zeros![digit_h >> 3] >> (digit_h & 7)) as u32)) as i32);
          }
        }

        if nzeros > 0 {
          mul_64x64_to_128!(cq, q_high, bid_reciprocals10_64![nzeros]);

          // now get P/10^extra_digits: shift C64 right by M[extra_digits]-64
          amount = bid_short_recip_scale![nzeros];
          cq.w[0] = cq.w[1] >> amount;
        } else {
          cq.w[0] = q_high;
        }
        cq.w[1] = 0;

        diff_expon += nzeros;
      } else {
        tdigit[0] = (q_low & 0x3ffffff) as u32;
        tdigit[1] = 0;
        qx = q_low >> 26;
        qx32 = qx as u32;
        nzeros = 0;

        j = 0;
        while qx32 > 0 {
          k = (qx32 & 127) as i32;
          tdigit[0] += bid_convert_table![j, k, 0];
          tdigit[1] += bid_convert_table![j, k, 1];
          if tdigit[0] >= 100000000 {
            dec!(tdigit[0], 100000000);
            inc!(tdigit[1]);
          }
          j += 1;
          qx32 >>= 7;
        }

        if tdigit[1] >= 100000000 {
          tdigit[1] -= 100000000;
          if tdigit[1] >= 100000000 {
            tdigit[1] -= 100000000;
          }
        }

        digit = tdigit[0];
        if digit == 0 && tdigit[1] == 0 {
          nzeros += 16;
        } else {
          if digit == 0 {
            nzeros += 8;
            digit = tdigit[1];
          }
          // decompose digit
          pd = (digit as u64) * 0x068DB8BB;
          digit_h = (pd >> 40) as u32;
          digit_low = digit - digit_h * 10000;

          if digit_low == 0 {
            nzeros += 4;
          } else {
            digit_h = digit_low;
          }

          if (digit_h & 1) == 0 {
            inc!(nzeros, (3 & ((bid_packed_10000_zeros![digit_h >> 3] >> (digit_h & 7)) as u32)) as i32);
          }
        }

        if nzeros > 0 {
          // get P*(2^M[extra_digits])/10^extra_digits
          mul_128x128_full!(qh, ql, cq, bid_reciprocals10_128![nzeros]);

          //now get P/10^extra_digits: shift q_high right by M[extra_digits]-128
          amount = bid_recip_scale![nzeros];
          shr_128!(cq, qh, amount);
        }
        diff_expon += nzeros;
      }
    }

    bid_get_bid128(&mut res, sign_x ^ sign_y, diff_expon, cq, rnd_mode, pfpsf);
    // #ifdef UNCHANGED_BINARY_STATUS_FLAGS
    // (void) fesetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
    // #endif
    return res;
  }

  if diff_expon >= 0 {
    if cfg!(feature = "ieee-round-nearest") {
      // rounding
      // 2*ca4 - cy
      ca4r.w[1] = (ca4.w[1].wrapping_add(ca4.w[1])) | (ca4.w[0] >> 63);
      ca4r.w[0] = ca4.w[0].wrapping_add(ca4.w[0]);
      sub_borrow_out!(ca4r.w[0], carry64, ca4r.w[0], cy.w[0]);
      ca4r.w[1] = ca4r.w[1].wrapping_sub(cy.w[1]).wrapping_sub(carry64);

      d = if ca4r.w[1] | ca4r.w[0] > 0 { 1 } else { 0 };
      carry64 = 1u64.wrapping_add(((ca4r.w[1] as i64) >> 63) as u64) & (cq.w[0] | d);

      cq.w[0] += carry64;
      if cq.w[0] < carry64 {
        inc!(cq.w[1]);
      }
    } else if cfg!(feature = "ieee-round-nearest-ties-away") {
      // rounding
      // 2*ca4 - cy
      ca4r.w[1] = (ca4.w[1].wrapping_add(ca4.w[1])) | (ca4.w[0] >> 63);
      ca4r.w[0] = ca4.w[0].wrapping_add(ca4.w[0]);
      sub_borrow_out!(ca4r.w[0], carry64, ca4r.w[0], cy.w[0]);
      ca4r.w[1] = ca4r.w[1].wrapping_sub(cy.w[1]).wrapping_sub(carry64);

      d = if ca4r.w[1] | ca4r.w[0] > 0 { 0 } else { 1 };
      carry64 = 1u64.wrapping_add(((ca4r.w[1] as i64) >> 63) as u64) | d;

      cq.w[0] += carry64;
      if cq.w[0] < carry64 {
        inc!(cq.w[1]);
      }
    } else {
      rmode = rnd_mode;
      if sign_x ^ sign_y > 0 && rmode.wrapping_sub(1) < 2 {
        rmode = 3 - rmode;
      }
      match rmode {
        BID_ROUNDING_TO_NEAREST => {
          // rounding
          // 2*ca4 - cy
          ca4r.w[1] = ca4.w[1].wrapping_add(ca4.w[1]) | (ca4.w[0] >> 63);
          ca4r.w[0] = ca4.w[0].wrapping_add(ca4.w[0]);
          sub_borrow_out!(ca4r.w[0], carry64, ca4r.w[0], cy.w[0]);
          ca4r.w[1] = ca4r.w[1].wrapping_sub(cy.w[1]).wrapping_sub(carry64);
          d = if ca4r.w[1] | ca4r.w[0] > 0 { 1 } else { 0 };
          carry64 = 1_i64.wrapping_add((ca4r.w[1] as i64) >> 63) as u64 & (cq.w[0] | d);
          inc!(cq.w[0], carry64);
          if cq.w[0] < carry64 {
            inc!(cq.w[1]);
          }
        }
        BID_ROUNDING_TIES_AWAY => {
          // rounding
          // 2*ca4 - cy
          ca4r.w[1] = ca4.w[1].wrapping_add(ca4.w[1]) | (ca4.w[0] >> 63);
          ca4r.w[0] = ca4.w[0].wrapping_add(ca4.w[0]);
          sub_borrow_out!(ca4r.w[0], carry64, ca4r.w[0], cy.w[0]);
          ca4r.w[1] = ca4r.w[1].wrapping_sub(cy.w[1]).wrapping_sub(carry64);
          d = if ca4r.w[1] | ca4r.w[0] > 0 { 0 } else { 1 };
          carry64 = 1_i64.wrapping_add((ca4r.w[1] as i64) >> 63) as u64 | d;
          inc!(cq.w[0], carry64);
          if cq.w[0] < carry64 {
            inc!(cq.w[1]);
          }
        }
        BID_ROUNDING_DOWN | BID_ROUNDING_TO_ZERO => {}
        _ => {
          inc!(cq.w[0]);
          if cq.w[0] == 0 {
            inc!(cq.w[1]);
          }
        }
      }
    }
  } else {
    if cfg!(feature = "bid-set-status-flags") && (ca4.w[0] > 0 || ca4.w[1] > 0) {
      set_status_flags!(pfpsf, BID_INEXACT_EXCEPTION);
    }

    bid_handle_uf_128_rem(&mut res, sign_x ^ sign_y, diff_expon, cq, ca4.w[1] | ca4.w[0], rnd_mode, pfpsf);

    // #ifdef UNCHANGED_BINARY_STATUS_FLAGS
    // (void) fesetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
    // #endif
    return res;
  }

  bid_get_bid128(&mut res, sign_x ^ sign_y, diff_expon, cq, rnd_mode, pfpsf);
  // #ifdef UNCHANGED_BINARY_STATUS_FLAGS
  // (void) fesetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
  // #endif

  res
}
