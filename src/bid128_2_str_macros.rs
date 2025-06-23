macro_rules! __l0_normalize_10to18 {
  ($x_hi:expr, $x_lo:expr) => {
    let l0_tmp: BidUint64 = $x_lo + BID_TWOTO60_M_10TO18;
    if (l0_tmp & BID_TWOTO60) > 0 {
      $x_hi += 1;
      $x_lo = (l0_tmp << 4) >> 4;
    }
  };
}

pub(crate) use __l0_normalize_10to18;

macro_rules! __l0_split_midi_2 {
  ($x:expr, $midi: expr, $ptr:expr) => {
    let mut l0_head: BidUint32 = $x >> 10;
    let mut l0_tail: BidUint32 = ($x & 0x03FF) + (l0_head << 5) - (l0_head << 3);
    let l0_tmp: BidUint32 = (l0_tail) >> 10;
    l0_head += l0_tmp;
    l0_tail = (l0_tail & (0x03FF)) + (l0_tmp << 5) - (l0_tmp << 3);
    if (l0_tail > 999) {
      l0_tail -= 1000;
      l0_head += 1;
    }
    set!(l0_head, $midi, $ptr);
    set!(l0_tail, $midi, $ptr);
  };
}

pub(crate) use __l0_split_midi_2;

macro_rules! __l0_split_midi_3 {
  ($x:expr, $midi: expr, $ptr:expr) => {
    let mut l0_x: BidUint32 = $x as u32;
    let mut l0_head: BidUint32 = ((l0_x >> 17) * 34359) >> 18;
    l0_x -= l0_head * 1000000;
    if (l0_x >= 1000000) {
      l0_x -= 1000000;
      l0_head += 1;
    }
    let mut l0_mid: BidUint32 = l0_x >> 10;
    let mut l0_tail: BidUint32 = (l0_x & (0x03FF)) + (l0_mid << 5) - (l0_mid << 3);
    let l0_tmp: BidUint32 = (l0_tail) >> 10;
    l0_mid += l0_tmp;
    l0_tail = (l0_tail & (0x3FF)) + (l0_tmp << 5) - (l0_tmp << 3);
    if (l0_tail > 999) {
      l0_tail -= 1000;
      l0_mid += 1;
    }
    set!(l0_head, $midi, $ptr);
    set!(l0_mid, $midi, $ptr);
    set!(l0_tail, $midi, $ptr);
  };
}

pub(crate) use __l0_split_midi_3;

macro_rules! __l0_split_midi_6 {
  ($x:expr, $midi: expr, $ptr:expr) => {
    let mut l1_xhi_64: BidUint64 = (($x >> 28) * (BID_INV_TENTO9 as u64)) >> 33;
    let mut l1_xlo_64: BidUint64 = $x - l1_xhi_64 * (BID_TENTO9 as u64);
    if (l1_xlo_64 >= (BID_TENTO9 as u64)) {
      l1_xlo_64 -= (BID_TENTO9 as u64);
      l1_xhi_64 += 1;
    }
    let l1_x_hi: BidUint32 = l1_xhi_64 as u32;
    let l1_x_lo: BidUint32 = l1_xlo_64 as u32;
    __l0_split_midi_3!(l1_x_hi, $midi, $ptr);
    __l0_split_midi_3!(l1_x_lo, $midi, $ptr);
  };
}

pub(crate) use __l0_split_midi_6;

macro_rules! __l0_split_midi_6_lead {
  ($x:expr, $midi: expr, $ptr:expr) => {
    let l1_x_lo: BidUint32;
    if $x >= BID_TENTO9 as u64 {
      let mut l1_xhi_64: BidUint64 = (($x >> 28) * BID_INV_TENTO9) >> 33;
      let mut l1_xlo_64: BidUint64 = $x - l1_xhi_64 * (BID_TENTO9 as u64);
      if l1_xlo_64 >= BID_TENTO9 as u64 {
        l1_xlo_64 -= (BID_TENTO9 as u64);
        l1_xhi_64 += 1;
      }
      let l1_x_hi: BidUint32 = l1_xhi_64 as u32;
      l1_x_lo = l1_xlo_64 as u32;
      if (l1_x_hi >= BID_TENTO6) {
        __l0_split_midi_3!(l1_x_hi, $midi, $ptr);
        __l0_split_midi_3!(l1_x_lo, $midi, $ptr);
      } else if (l1_x_hi >= BID_TENTO3) {
        __l0_split_midi_2!(l1_x_hi, $midi, $ptr);
        __l0_split_midi_3!(l1_x_lo, $midi, $ptr);
      } else {
        set!(l1_x_hi, $midi, $ptr);
        __l0_split_midi_3!(l1_x_lo, $midi, $ptr);
      }
    } else {
      l1_x_lo = $x as u32;
      if (l1_x_lo >= BID_TENTO6) {
        __l0_split_midi_3!(l1_x_lo, $midi, $ptr);
      } else if (l1_x_lo >= BID_TENTO3) {
        __l0_split_midi_2!(l1_x_lo, $midi, $ptr);
      } else {
        set!(l1_x_lo, $midi, $ptr);
      }
    }
  };
}

pub(crate) use __l0_split_midi_6_lead;

macro_rules! __l0_midi2str {
  ($x:expr, $str:expr, $ptr:expr) => {
    let digits = BID_MIDI_TBL[$x as usize];
    $str[$ptr] = digits[0];
    $ptr += 1;
    $str[$ptr] = digits[1];
    $ptr += 1;
    $str[$ptr] = digits[2];
    $ptr += 1;
  };
}

pub(crate) use __l0_midi2str;

macro_rules! __l0_midi2str_lead {
  ($x:expr, $str:expr, $ptr:expr) => {
    let digits = BID_MIDI_TBL[$x as usize];
    if $x >= 100 {
      $str[$ptr] = digits[0];
      $ptr += 1;
      $str[$ptr] = digits[1];
      $ptr += 1;
      $str[$ptr] = digits[2];
      $ptr += 1;
    } else if $x >= 10 {
      $str[$ptr] = digits[1];
      $ptr += 1;
      $str[$ptr] = digits[2];
      $ptr += 1;
    } else {
      $str[$ptr] = digits[2];
      $ptr += 1;
    }
  };
}

pub(crate) use __l0_midi2str_lead;
