use crate::gates::*;
use Signal::*;

pub fn half_adder(in1: Signal, in2: Signal) -> (Signal, Signal) {
  (xor(in1, in2), and(in1, in2))
}


pub fn full_adder(in1: Signal, in2: Signal, carry: Signal) -> (Signal, Signal) {
  let (s1, c1) = half_adder(in1, in2);
  let (s2, c2) = half_adder(carry, s1);
  (s2, or(c1, c2))
}


pub fn n_adder<const BITS: usize>(in1: Signals<BITS>, in2: Signals<BITS>) -> Signals<BITS> {
  let mut carry = Low;
  let mut added: [Signal; BITS] = [Signal::Low; BITS];
  for (i, (bit1, bit2)) in in1.iter().zip(in2.iter()).enumerate().rev() {
      let (sum, carry_tmp) = full_adder(*bit1, *bit2, carry);
      carry = carry_tmp;
      added[i] = sum
  }
  added
}

pub fn n_incrementor<const BITS: usize>(in1: Signals<BITS>) -> Signals<BITS> {
  let mut carry = High;
  let mut added: [Signal; BITS] = [Signal::Low; BITS];
  for (i, bit1) in in1.iter().enumerate().rev() {
      let (sum, carry_tmp) = full_adder(*bit1, Low, carry);
      carry = carry_tmp;
      added[i] = sum
  }
  added
}

pub fn alu<const BITS: usize>(
  bits1: Signals<BITS>,
  bits2: Signals<BITS>,
  zx: Signal,
  nx: Signal,
  zy: Signal,
  ny: Signal,
  f: Signal,
  no: Signal
) -> (Signals<BITS>, Signal, Signal) {
  let bits1 = mux_n(bits1, [Low; BITS], zx);
  let bits1 = mux_n(bits1, not_n(bits1), nx);
  let bits2 = mux_n(bits2, [Low; BITS], zy);
  let bits2 = mux_n(bits2, not_n(bits2), ny);
  let out = mux_n(and_n(bits1, bits2), n_adder(bits1, bits2), f);
  let out = mux_n(out, not_n(out), no);
  let zr = not(or_m_way(out));
  let ng = out[0];
  (out, zr, ng)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::*;  
    #[test]
    fn thalf_adder() {
        assert_eq!(half_adder(Low, Low), (Low, Low));
        assert_eq!(half_adder(Low, High), (High, Low));
        assert_eq!(half_adder(High, Low), (High, Low));
        assert_eq!(half_adder(High, High), (Low, High));
    }
    #[test]
    fn tfull_adder() {
        assert_eq!(full_adder(Low, Low, Low), (Low, Low));
        assert_eq!(full_adder(Low, Low, High), (High, Low));
        assert_eq!(full_adder(Low, High, Low), (High, Low));
        assert_eq!(full_adder(Low, High, High), (Low, High));
        assert_eq!(full_adder(High, Low, Low), (High, Low));
        assert_eq!(full_adder(High, Low, High), (Low, High));
        assert_eq!(full_adder(High, High, Low), (Low, High));
        assert_eq!(full_adder(High, High, High), (High, High));
    }
    #[test]
    fn tn_adder() {
        assert_eq!(n_adder(
            [Low, Low, Low, Low],
            [Low, Low, Low, Low]
        ), ([Low, Low, Low, Low]));
        assert_eq!(n_adder(
            [Low, Low, Low, High],
            [Low, Low, Low, High]
        ), ([Low, Low, High, Low]));
        assert_eq!(n_adder(
            [High, High, High, High],
            [High, High, High, High]
        ), ([High, High, High, Low]));
        assert_eq!(n_adder(
            [Low, Low, Low, Low],
            [Low, Low, Low, Low]
        ), ([Low, Low, Low, Low]));
        assert_eq!(n_adder(
            [High, Low, High, Low],
            [Low, Low, High, Low]
        ), ([High, High, Low, Low]));
    }
    #[test]
    fn tn_incrementor() {
        assert_eq!(n_incrementor(
            [Low, Low, Low, Low],
        ), ([Low, Low, Low, High]));
        assert_eq!(n_incrementor(
            [High, High, High, High],
        ), ([Low, Low, Low, Low]));
        assert_eq!(n_incrementor(
            [High, High, Low, Low],
        ), ([High, High, Low, High]));
        assert_eq!(n_incrementor(
            [Low, Low, High, High],
        ), ([Low, High, Low, Low]));
    }

    fn check_alu(x: i16, y: i16, zx: Signal, nx: Signal, zy: Signal, ny: Signal, f: Signal, no: Signal, expected: i16) {
        let (out, zr, ng) = alu(int_to_binary16(x), int_to_binary16(y), zx, nx, zy, ny, f, no);
        assert_eq!(binary_to_int16(out), expected);
        if expected == 0 {
            assert_eq!(zr, High);
        } else {
            assert_eq!(zr, Low);
        }
        if expected < 0 {
            assert_eq!(ng, High);
        } else {
            assert_eq!(ng, Low);
        }
    }
    #[test]
    fn talu() {
        let val_a = [1,2,3,4];
        let val_b = [1,2,3,4];
        for a in val_a.iter() {
            for b in val_b.iter() {
                check_alu(*a, *b, High, Low, High, Low, High, Low, 0);
                check_alu(*a, *b, High, High, High, High, High, High, 1);
                check_alu(*a, *b, High, High, High, Low, High, Low, - 1);
                check_alu(*a, *b, Low, Low, High, High, Low, Low, *a);
                check_alu(*a, *b, High, High, Low, Low, Low, Low, *b);

                check_alu(*a, *b, Low, Low, High, High, Low, High, !*a);
                check_alu(*a, *b, High, High, Low, Low, Low, High, !*b);

                check_alu(*a, *b, Low, Low, High, High, High, High, -*a);
                check_alu(*a, *b, High, High, Low, Low, High, High, -*b);

                check_alu(*a, *b, Low, High, High, High, High, High, *a + 1);
                check_alu(*a, *b, High, High, Low, High, High, High, *b + 1);

                check_alu(*a, *b, Low, Low, High, High, High, Low, *a - 1);
                check_alu(*a, *b, High, High, Low, Low, High, Low, *b - 1);

                check_alu(*a, *b, Low, Low, Low, Low, High, Low, *a + *b);
                check_alu(*a, *b, Low, High, Low, Low, High, High, *a - *b);

                check_alu(*a, *b, Low, Low, Low, Low, Low, Low, *a & *b);
                check_alu(*a, *b, Low, High, Low, High, Low, High, *a | *b);
            }
        }
    }
}