//! # String format conversion

use crate::bid_conf::*;
use crate::bid_functions::*;
use crate::bid_internal::*;
use crate::bid128::*;
use crate::bid128_2_str_macros::*;
use crate::bid128_2_str_tables::*;
use crate::bid128_common::*;
use crate::{BidUint32, BidUint64, BidUint128};
use alloc::string::String;
use alloc::vec;

/// Maximum number of digits in string format for 128-bit value.
const MAX_FORMAT_DIGITS_128: i32 = 34;

/// Maximum number of characters in the input string.
const MAX_STRING_DIGITS_128: i32 = 100;

/// Maximum size of the input buffer for characters.
const MAX_BUFFER_SIZE: usize = MAX_STRING_DIGITS_128 as usize;

/// Utility macro that assigns a value to table element.
macro_rules! set {
  ($value:expr, $table:expr, $index:expr) => {
    $table[$index] = $value; // Note: value is assigned at $index position.
    $index += 1;
  };
  ($value:expr, $table:expr, $index:expr, $pos:expr) => {
    $table[$pos] = $value; // Note: value is assigned at $pos position.
    $index += 1;
  };
}

/// Converts a 128-bit decimal floating-point value (binary encoding)
/// to string format (decimal character sequence).
pub fn bid128_to_string(x: BidUint128) -> String {
  let x_sign: BidUint64;
  let mut x_exp: BidUint64;
  let mut exp: i32; // Unbiased exponent.
  let mut str: [u8; MAX_BUFFER_SIZE + 1] = [0; MAX_BUFFER_SIZE + 1]; // Output characters.
  let mut k: usize = 0; // Number of characters in the string (index of the next free position in string).
  let mut c1: BidUint128 = BidUint128::default(); // Note: c1.w[1], c1.w[0] represent x_signif_hi, x_signif_lo (all are BID_UINT64)
  let ind: i32;
  let mut midi: [BidUint32; 12] = [0; 12];
  let mut ptr: usize;

  // Check for NaN or Infinity.
  if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
    // 'x' is special
    if (x.w[1] & MASK_NAN) == MASK_NAN {
      // 'x' is NaN
      if (x.w[1] & MASK_SNAN) == MASK_SNAN {
        // 'x' is SNaN
        // Set invalid flag.
        set!(if (x.w[1] as i64) < 0 { b'-' } else { b'+' }, str, k);
        set!(b'S', str, k);
        set!(b'N', str, k);
        set!(b'a', str, k);
        set!(b'N', str, k);
      } else {
        // 'x' is QNaN
        set!(if (x.w[1] as i64) < 0 { b'-' } else { b'+' }, str, k);
        set!(b'N', str, k);
        set!(b'a', str, k);
        set!(b'N', str, k);
      }
    } else {
      // 'x' is not a NaN, so it must be infinity
      if (x.w[1] & MASK_SIGN) == 0x0 {
        // 'x' is +inf
        set!(b'+', str, k);
        set!(b'I', str, k);
        set!(b'n', str, k);
        set!(b'f', str, k);
      } else {
        // 'x' is -inf
        set!(b'-', str, k);
        set!(b'I', str, k);
        set!(b'n', str, k);
        set!(b'f', str, k);
      }
    }
    str[..k].iter().map(|c| *c as char).collect()
  } else if (x.w[1] & MASK_COEFF) == 0 && x.w[0] == 0 {
    // 'x' is 0
    if x.w[1] & MASK_SIGN > 0 {
      set!(b'-', str, k);
    } else {
      set!(b'+', str, k);
    }
    set!(b'0', str, k);
    set!(b'E', str, k);

    // extract the exponent and print
    exp = ((x.w[1] & MASK_EXP) >> 49).wrapping_sub(6176) as i32;
    if exp > ((0x5ffe) >> 1) - 6176 {
      exp = (((x.w[1] << 2) & MASK_EXP) >> 49).wrapping_sub(6176) as i32;
    }
    if exp < 0 {
      set!(b'-', str, k);
      exp = -exp;
    } else {
      set!(b'+', str, k);
    }
    let pos = k;
    if exp > 999 {
      set!((exp % 10) as u8 + b'0', str, k, pos + 3);
      exp /= 10;
    }
    if exp > 99 {
      set!((exp % 10) as u8 + b'0', str, k, pos + 2);
      exp /= 10;
    }
    if exp > 9 {
      set!((exp % 10) as u8 + b'0', str, k, pos + 1);
      exp /= 10;
    }
    set!((exp % 10) as u8 + b'0', str, k, pos);
    str[..k].iter().map(|c| *c as char).collect()
  } else {
    // 'x' is not special and is not zero.
    // Unpack 'x'.
    x_sign = x.w[1] & MASK_SIGN; // 0 for positive, MASK_SIGN for negative
    x_exp = x.w[1] & MASK_EXP; // biased and shifted left 49 bit positions
    if (x.w[1] & 0x6000000000000000) == 0x6000000000000000 {
      x_exp = (x.w[1] << 2) & MASK_EXP; // biased and shifted left 49 bit positions
    }
    c1.w[1] = x.w[1] & MASK_COEFF;
    c1.w[0] = x.w[0];
    exp = (x_exp >> 49).wrapping_sub(6176) as i32;

    // Determine sign's representation as a character.
    if x_sign > 0 {
      set!(b'-', str, k); // Negative number.
    } else {
      set!(b'+', str, k); // // Positive number.
    }

    // Determine coefficient's representation as a decimal string.

    // If zero or non-canonical, set coefficient to '0'.
    if c1.w[1] > 0x0001ed09bead87c0 || (c1.w[1] == 0x0001ed09bead87c0 && c1.w[0] > 0x378d8e63ffffffff) || (x.w[1] & 0x6000000000000000) == 0x6000000000000000 || (c1.w[1] == 0 && c1.w[0] == 0) {
      set!(b'0', str, k);
    } else {
      /* ****************************************************
      This takes a bid coefficient in c1.w[1],c1.w[0]
      and put the converted character sequence at location
      starting at &(str[k]). The function returns the number
      of MiDi returned. Note that the character sequence
      does not have leading zeros EXCEPT when the input is of
      zero value. It will then output 1 character '0'
      The algorithm essentailly tries first to get a sequence of
      Millenial Digits "MiDi" and then uses table lookup to get the
      character strings of these MiDis.
      **************************************************** */
      /* Algorithm first decompose possibly 34 digits in hi and lo
      18 digits. (The high can have at most 16 digits). It then
      uses macro that handle 18 digit portions.
      The first step is to get hi and lo such that
      2^(64) c1.w[1] + c1.w[0] = hi * 10^18  + lo,   0 <= lo < 10^18.
      We use a table lookup method to obtain the hi and lo 18 digits.
      [c1.w[1],c1.w[0]] = c_8 2^(107) + c_7 2^(101) + ... + c_0 2^(59) + d
      where 0 <= d < 2^59 and each c_j has 6 bits. Because d fits in
      18 digits,  we set hi = 0, and lo = d to begin with.
      We then retrieve from a table, for j = 0, 1, ..., 8
      that gives us A and B where c_j 2^(59+6j) = A * 10^18 + B.
      hi += A ; lo += B; After each accumulation into lo, we normalize
      immediately. So at the end, we have the decomposition as we need. */

      let mut lo_18dig: BidUint64 = (c1.w[0] << 5) >> 5;
      let mut hi_18dig: BidUint64 = 0;
      let mut tmp = (c1.w[0] >> 59) + (c1.w[1] << 5);
      let mut i = 0;
      let mut a;
      while tmp > 0 {
        a = ((tmp & 0x000000000000003F) as i32) << 1;
        tmp >>= 6;
        hi_18dig += MOD10_18_TBL[i][a as usize];
        a += 1;
        lo_18dig += MOD10_18_TBL[i][a as usize];
        i += 1;
        __l0_normalize_10to18!(hi_18dig, lo_18dig);
      }
      ptr = 0;
      if hi_18dig == 0 {
        __l0_split_midi_6_lead!(lo_18dig, midi, ptr);
      } else {
        __l0_split_midi_6_lead!(hi_18dig, midi, ptr);
        __l0_split_midi_6!(lo_18dig, midi, ptr);
      }

      __l0_midi2str_lead!(midi[0], str, k);
      for i in 1..ptr {
        __l0_midi2str!(midi[i], str, k);
      }
    }

    // Print E and sign of the exponent.
    set!(b'E', str, k);
    if exp < 0 {
      set!(b'-', str, k);
      exp = -exp;
    } else {
      set!(b'+', str, k);
    }

    // Determine the exponent's representation as a decimal string.
    // d0 = exp / 1000;
    // Use Property 1
    let d0 = (exp * 0x418a) >> 24; // 0x418a * 2^-24 = (10^(-3))RP,15
    let d123 = exp - 1000 * d0;

    if d0 > 0 {
      // 1000 <= exp <= 6144 => 4 digits to return
      set!((d0 as u8) + b'0', str, k); // ASCII for decimal digit d0
      ind = 3 * d123;
      set!(BID_CHAR_TABLE3[ind as usize], str, k);
      set!(BID_CHAR_TABLE3[(ind + 1) as usize], str, k);
      set!(BID_CHAR_TABLE3[(ind + 2) as usize], str, k);
    } else {
      // 0 <= exp <= 999 => d0 = 0
      if d123 < 10 {
        // 0 <= exp <= 9 => 1 digit to return
        set!((d123 as u8) + b'0', str, k); // ASCII for decimal digit d123
      } else if d123 < 100 {
        // 10 <= exp <= 99 => 2 digits to return
        ind = 2 * (d123 - 10);
        set!(BID_CHAR_TABLE2[ind as usize], str, k);
        set!(BID_CHAR_TABLE2[(ind + 1) as usize], str, k);
      } else {
        // 100 <= exp <= 999 => 3 digits to return
        ind = 3 * d123;
        set!(BID_CHAR_TABLE3[ind as usize], str, k);
        set!(BID_CHAR_TABLE3[(ind + 1) as usize], str, k);
        set!(BID_CHAR_TABLE3[(ind + 2) as usize], str, k);
      }
    }
    str[..k].iter().map(|c| *c as char).collect()
  }
}

/// Converts a value represented in string format (decimal character sequence)
/// to 128-bit decimal floating-point format (binary encoding).
pub fn bid128_from_string(input: &str, rounding: IdecRound, flags: &mut IdecFlags) -> BidUint128 {
  let mut res: BidUint128 = Default::default();
  let mut cx: BidUint128 = Default::default();
  let mut coeff_high: BidUint64;
  let mut coeff_low: BidUint64;
  let mut coeff2: BidUint64;
  let mut coeff_l2: BidUint64;
  let mut carry: BidUint64 = Default::default();
  let mut scale_high: BidUint64;
  let mut right_radix_leading_zeros: BidUint64 = Default::default();
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

  // Prepare a vector of input bytes, and prepare an index that simulates C pointer on char.
  let mut p: usize = 0;
  let mut ps = vec![0u8; input.len() + 1];
  for (index, ch) in input.chars().enumerate() {
    ps[index] = ch as u8;
  }

  // Eliminate the leading white spaces.
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
  let sign_x: BidUint64 = if c == b'-' { 0x8000000000000000 } else { 0 };

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
    bid_get_bid128(&mut res, sign_x, dec_expon, cx, rounding, flags);
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
    match rounding {
      BID_ROUNDING_TO_NEAREST => {
        carry = ((b'4' as i8).wrapping_sub(buffer[i as usize] as i8) as u32 >> 31) as u64;
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
        carry = ((b'4' as i8).wrapping_sub(buffer[i as usize] as i8) as u32 >> 31) as u64;
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

    if cfg!(feature = "bid-set-status-flags") && set_inexact {
      set_status_flags!(flags, BID_INEXACT_EXCEPTION);
    }

    bid_get_bid128(&mut res, sign_x, dec_expon, cx, rounding, flags);
    res
  }
}
