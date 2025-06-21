//! # String format conversion

use crate::Bid128;
use crate::bid_conf::{IdecFlags, IdecRound};
use crate::bid_functions::*;
use crate::bid_internal::*;
use crate::bid64::Bid64;
use alloc::vec;

/// Maximum number of digits in string format for 128-bit value.
const MAX_FORMAT_DIGITS_128: i32 = 34;

/// Maximum number of characters in the input string.
const MAX_STRING_DIGITS_128: i32 = 100;

/// Maximum size of the input buffer for characters.
const MAX_BUFFER_SIZE: usize = MAX_STRING_DIGITS_128 as usize;

/// Utility macro for calulating the carry.
macro_rules! carry {
  ($a:expr) => {
    ((b'4' as i8).wrapping_sub($a as i8) as u32 >> 31) as u64
  };
}

/// Convert a decimal floating-point value represented in string format (decimal character sequence)
/// to 128-bit decimal floating-point format (binary encoding).
pub fn bid128_from_string(input: &str, _rnd_mode: IdecRound, _pfpsf: &mut IdecFlags) -> Bid128 {
  let mut res: Bid128 = Default::default();
  let mut cx: Bid128 = Default::default();
  let mut coeff_high: Bid64;
  let mut coeff_low: Bid64;
  let mut coeff2: Bid64;
  let mut coeff_l2: Bid64;
  let mut carry: Bid64 = Default::default();
  let mut scale_high: Bid64;
  let mut right_radix_leading_zeros: Bid64 = Default::default();
  let mut rdx_pt_enc: i32 = 0;
  let mut ndigits_before: i32;
  let mut ndigits_after: i32;
  let mut ndigits_total: i32;
  let mut dec_expon: i32;
  let mut sgn_exp: i32;
  let mut i: i32;
  let mut set_inexact = false;
  let mut buffer: [u8; MAX_BUFFER_SIZE] = [0_u8; MAX_BUFFER_SIZE];

  // If null string, then return NaN.
  if input.is_empty() {
    res.w[1] = 0x7c00000000000000;
    res.w[0] = 0;
    return res;
  }

  // Prepare a vector of input bytes, and index simulating C pointer on char.
  let mut p: usize = 0;
  let mut ps = vec![0u8; input.len() + 1];
  for (index, ch) in input.chars().enumerate() {
    ps[index] = ch as u8;
  }

  // Eliminate leading white spaces.
  while ps[p] == b' ' || ps[p] == b'\t' {
    p += 1;
  }

  // 'c' gets the first character.
  let mut c = ps[p];

  // If 'c' is null or not equal to a radix point, negative sign,
  // positive sign, or digit, then it might be SNaN, Infinity.
  if c == 0 || (c != b'.' && c != b'-' && c != b'+' && !c.is_ascii_digit()) {
    res.w[0] = 0;
    // Is it Infinity?
    if (tolower_macro!(ps[p]) == b'i' && tolower_macro!(ps[p + 1]) == b'n' && tolower_macro!(ps[p + 2]) == b'f')
      && (ps[p + 3] == 0 || (tolower_macro!(ps[p + 3]) == b'i' && tolower_macro!(ps[p + 4]) == b'n' && tolower_macro!(ps[p + 5]) == b'i' && tolower_macro!(ps[p + 6]) == b't' && tolower_macro!(ps[p + 7]) == b'y' && ps[p + 8] == 0))
    {
      res.w[1] = 0x7800000000000000;
      return res;
    }
    // Return sNaN.
    return if tolower_macro!(ps[0]) == b's' && tolower_macro!(ps[1]) == b'n' && tolower_macro!(ps[2]) == b'a' && tolower_macro!(ps[3]) == b'n' {
      // Return SNaN.
      res.w[1] = 0x7e00000000000000;
      res
    } else {
      // Return qNaN.
      res.w[1] = 0x7c00000000000000;
      res
    };
  }

  // If +Inf, -Inf, +Infinity, or -Infinity. Case-insensitive check for 'inf'.
  if (tolower_macro!(ps[p + 1]) == b'i' && tolower_macro!(ps[p + 2]) == b'n' && tolower_macro!(ps[p + 3]) == b'f')
    && (ps[p + 4] == 0 || (tolower_macro!(ps[p + 4]) == b'i' && tolower_macro!(ps[p + 5]) == b'n' && tolower_macro!(ps[p + 6]) == b'i' && tolower_macro!(ps[p + 7]) == b't' && tolower_macro!(ps[p + 8]) == b'y' && ps[p + 9] == 0))
  {
    res.w[0] = 0;
    if c == b'+' {
      res.w[1] = 0x7800000000000000;
    } else if c == b'-' {
      res.w[1] = 0xf800000000000000;
    } else {
      res.w[1] = 0x7c00000000000000;
    }
    return res;
  }

  // Check for case-insensitive +snan or -snan.
  if tolower_macro!(ps[p + 1]) == b's' && tolower_macro!(ps[p + 2]) == b'n' && tolower_macro!(ps[p + 3]) == b'a' && tolower_macro!(ps[p + 4]) == b'n' {
    res.w[0] = 0;
    if c == b'-' {
      res.w[1] = 0xfe00000000000000;
    } else {
      res.w[1] = 0x7e00000000000000;
    }
    return res;
  }

  // Check for case-insensitive +nan or -nan.
  if tolower_macro!(ps[p + 1]) == b'n' && tolower_macro!(ps[p + 2]) == b'a' && tolower_macro!(ps[p + 3]) == b'n' {
    res.w[0] = 0;
    if c == b'-' {
      res.w[1] = 0xfc00000000000000;
    } else {
      res.w[1] = 0x7c00000000000000;
    }
    return res;
  }

  // Set up sign_x to be OR'ed with the upper word later.
  let sign_x: Bid64 = if c == b'-' { 0x8000000000000000 } else { 0 };

  // Go to the next character if leading sign.
  if c == b'-' || c == b'+' {
    p += 1;
  }
  c = ps[p];

  // If c isn't a decimal point or a decimal digit the return NaN.
  if c != b'.' && !c.is_ascii_digit() {
    res.w[1] = 0x7c00000000000000 | sign_x;
    res.w[0] = 0;
    return res;
  }

  if c == b'.' {
    rdx_pt_enc = 1;
    p += 1;
  }

  // Detect zero and eliminate/ignore leading zeros.
  if ps[p] == b'0' {
    // If all numbers are zeros (with possibly 1 radix point, the number is zero.
    // Should catch cases such as: 000.0
    while ps[p] == b'0' {
      p += 1;

      // For numbers such as 0.0000000000000000000000000000000000001001,
      // we want to count the leading zeros.
      if rdx_pt_enc > 0 {
        right_radix_leading_zeros += 1;
      }
      // If this character is a radix point, make sure we haven't already encountered one.
      if ps[p] == b'.' {
        if rdx_pt_enc == 0 {
          rdx_pt_enc = 1;
          // if this is the first radix point, and the next character is NULL, we have a zero.
          if ps[p + 1] == 0 {
            res.w[1] = (0x3040000000000000 - (right_radix_leading_zeros << 49)) | sign_x;
            res.w[0] = 0;
            return res;
          }
          p += 1;
        } else {
          // If 2 radix points, return NaN.
          res.w[1] = 0x7c00000000000000 | sign_x;
          res.w[0] = 0;
          return res;
        }
      } else if ps[p] == 0 {
        if right_radix_leading_zeros > 6176 {
          right_radix_leading_zeros = 6176
        };
        res.w[1] = (0x3040000000000000 - (right_radix_leading_zeros << 49)) | sign_x;
        res.w[0] = 0;
        return res;
      }
    }
  }

  c = ps[p];

  // Initialize local variables.
  ndigits_before = 0;
  ndigits_after = 0;
  sgn_exp = 0;

  if rdx_pt_enc == 0 {
    // Investigate string (before radix point).
    while c.is_ascii_digit() {
      if ndigits_before < MAX_FORMAT_DIGITS_128 {
        buffer[ndigits_before as usize] = c;
      } else if ndigits_before < MAX_STRING_DIGITS_128 {
        buffer[ndigits_before as usize] = c;
        if c > b'0' {
          set_inexact = true;
        }
      } else if c > b'0' {
        set_inexact = true
      };
      p += 1;
      c = ps[p];
      ndigits_before += 1;
    }

    ndigits_total = ndigits_before;
    if c == b'.' {
      p += 1;
      c = ps[p];
      if c > 0 {
        // Investigate string (after radix point).
        while c.is_ascii_digit() {
          if ndigits_total < MAX_FORMAT_DIGITS_128 {
            buffer[ndigits_total as usize] = c;
          } else if ndigits_total < MAX_STRING_DIGITS_128 {
            buffer[ndigits_total as usize] = c;
            if c > b'0' {
              set_inexact = true;
            }
          } else if c > b'0' {
            set_inexact = true;
          }
          p += 1;
          c = ps[p];
          ndigits_total += 1;
        }
        ndigits_after = ndigits_total - ndigits_before;
      }
    }
  } else {
    // We encountered a radix point while detecting zeros.
    c = ps[p];
    ndigits_total = 0;
    // Investigate string (after radix point).
    while c.is_ascii_digit() {
      if ndigits_total < MAX_FORMAT_DIGITS_128 {
        buffer[ndigits_total as usize] = c;
      } else if ndigits_total < MAX_STRING_DIGITS_128 {
        buffer[ndigits_total as usize] = c;
        if c > b'0' {
          set_inexact = true;
        }
      } else if c > b'0' {
        set_inexact = true;
      }
      p += 1;
      c = ps[p];
      ndigits_total += 1;
    }
    ndigits_after = ndigits_total - ndigits_before;
  }

  // Get the exponent.
  dec_expon = 0;
  if c > 0 {
    if c != b'e' && c != b'E' {
      // Return NaN.
      res.w[1] = 0x7c00000000000000;
      res.w[0] = 0;
      return res;
    }
    p += 1;
    c = ps[p];

    if (c > 9 + b'0') && ((c != b'+' && c != b'-') || !ps[p + 1].is_ascii_digit()) {
      // Return NaN.
      res.w[1] = 0x7c00000000000000;
      res.w[0] = 0;
      return res;
    }

    if c == b'-' {
      sgn_exp = -1;
      p += 1;
      c = ps[p];
    } else if c == b'+' {
      p += 1;
      c = ps[p];
    }

    dec_expon = (c - b'0') as i32;
    i = 1;
    p += 1;

    if dec_expon == 0 {
      while (ps[p]) == b'0' {
        p += 1;
      }
    }
    c = ps[p].wrapping_sub(b'0');

    let mut d2: i32;
    while c <= 9 && i < 7 {
      d2 = dec_expon + dec_expon;
      dec_expon = (d2 << 2) + d2 + (c as i32);
      p += 1;
      c = ps[p].wrapping_sub(b'0');
      i += 1;
    }

    dec_expon = (dec_expon + sgn_exp) ^ sgn_exp;
  }

  if ndigits_total <= MAX_FORMAT_DIGITS_128 {
    dec_expon += DECIMAL_EXPONENT_BIAS_128 - ndigits_after - right_radix_leading_zeros as i32;
    if dec_expon < 0 {
      res.w[1] = sign_x;
      res.w[0] = 0;
    }
    if ndigits_total == 0 {
      cx.w[0] = 0;
      cx.w[1] = 0;
    } else if ndigits_total <= 19 {
      coeff_high = (buffer[0] - b'0') as u64;
      i = 1;
      while i < ndigits_total {
        coeff2 = coeff_high.wrapping_add(coeff_high);
        coeff_high = (coeff2 << 2).wrapping_add(coeff2).wrapping_add((buffer[i as usize] - b'0') as u64);
        i += 1;
      }
      cx.w[0] = coeff_high;
      cx.w[1] = 0;
    } else {
      coeff_high = (buffer[0] - b'0') as u64;
      i = 1;
      while i < ndigits_total - 17 {
        coeff2 = coeff_high.wrapping_add(coeff_high);
        coeff_high = (coeff2 << 2).wrapping_add(coeff2).wrapping_add((buffer[i as usize] - b'0') as u64);
        i += 1;
      }
      coeff_low = (buffer[i as usize] - b'0') as u64;
      i += 1;
      while i < ndigits_total {
        coeff_l2 = coeff_low.wrapping_add(coeff_low);
        coeff_low = (coeff_l2 << 2).wrapping_add(coeff_l2).wrapping_add((buffer[i as usize] - b'0') as u64);
        i += 1;
      }
      // Now form the coefficient as coeff_high*10^19+coeff_low+carry
      scale_high = 100000000000000000;
      mul_64x64_to_128_fast!(cx, coeff_high, scale_high);

      cx.w[0] = cx.w[0].wrapping_add(coeff_low);
      if cx.w[0] < coeff_low {
        cx.w[1] = cx.w[1].wrapping_add(1);
      }
    }
    bid_get_bid128(&mut res, sign_x, dec_expon, cx, _rnd_mode, _pfpsf);
    res
  } else {
    // Simply round using the digits that were read.

    dec_expon += ndigits_before + DECIMAL_EXPONENT_BIAS_128 - MAX_FORMAT_DIGITS_128 - right_radix_leading_zeros as i32;

    if dec_expon < 0 {
      res.w[1] = sign_x;
      res.w[0] = 0;
    }

    coeff_high = (buffer[0] - b'0') as u64;
    i = 1;
    while i < MAX_FORMAT_DIGITS_128 - 17 {
      coeff2 = coeff_high.wrapping_add(coeff_high);
      coeff_high = (coeff2 << 2).wrapping_add(coeff2).wrapping_add((buffer[i as usize] - b'0') as u64);
      i += 1;
    }
    coeff_low = (buffer[i as usize] - b'0') as u64;
    i += 1;
    while i < MAX_FORMAT_DIGITS_128 {
      coeff_l2 = coeff_low.wrapping_add(coeff_low);
      coeff_low = (coeff_l2 << 2).wrapping_add(coeff_l2).wrapping_add((buffer[i as usize] - b'0') as u64);
      i += 1;
    }
    match _rnd_mode {
      BID_ROUNDING_TO_NEAREST => {
        carry = carry!(buffer[i as usize]);
        if (buffer[i as usize] == b'5' && (coeff_low & 1) == 0) || dec_expon < 0 {
          if dec_expon >= 0 {
            carry = 0;
            i += 1;
          }
          while i < ndigits_total {
            if buffer[i as usize] > b'0' {
              carry = 1;
              break;
            }
            i += 1;
          }
        }
      }
      BID_ROUNDING_DOWN => {
        if sign_x > 0 {
          while i < ndigits_total {
            if buffer[i as usize] > b'0' {
              carry = 1;
              break;
            }
            i += 1;
          }
        }
      }
      BID_ROUNDING_UP => {
        if sign_x == 0 {
          while i < ndigits_total {
            if buffer[i as usize] > b'0' {
              carry = 1;
              break;
            }
            i += 1;
          }
        }
      }
      BID_ROUNDING_TO_ZERO => {
        carry = 0;
      }
      BID_ROUNDING_TIES_AWAY => {
        carry = carry!(buffer[i as usize]);
        if dec_expon < 0 {
          while i < ndigits_total {
            if buffer[i as usize] > b'0' {
              carry = 1;
              break;
            }
            i += 1;
          }
        }
      }

      _ => {}
    }

    // Now form the coefficient as coeff_high * 10^17 + coeff_low + carry
    scale_high = 100000000000000000;
    if dec_expon < 0 {
      if dec_expon > -MAX_FORMAT_DIGITS_128 {
        scale_high = 1000000000000000000;
        coeff_low = (coeff_low << 3).wrapping_add(coeff_low << 1);
        dec_expon -= 1;
      }
      if dec_expon == -MAX_FORMAT_DIGITS_128 && coeff_high > 50000000000000000 {
        carry = 0;
      }
    }

    mul_64x64_to_128_fast!(cx, coeff_high, scale_high);

    coeff_low = coeff_low.wrapping_add(carry);
    cx.w[0] = cx.w[0].wrapping_add(coeff_low);
    if cx.w[0] < coeff_low {
      cx.w[1] = cx.w[1].wrapping_add(1);
    }

    #[cfg(feature = "bid-set-status-flags")]
    if set_inexact {
      set_status_flags!(_pfpsf, BID_INEXACT_EXCEPTION);
    }
    #[cfg(not(feature = "bid-set-status-flags"))]
    {
      // Just for silencing warnings when feature 'bid-set-status-flags' is not set.
      let _ = set_inexact;
    }

    bid_get_bid128(&mut res, sign_x, dec_expon, cx, _rnd_mode, _pfpsf);
    res
  }
}
