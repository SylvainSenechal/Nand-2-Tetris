use Signal::*;
use std::convert::TryInto;

// todo rename in1 en bit1 ou bits1

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Signal {
    High,
    Low,
}

pub type Signals<const BITS: usize> = [Signal; BITS];

pub fn nand(in1: Signal, in2: Signal) -> Signal {
  match (in1, in2) {
      (Low, Low) => High,
      (Low, High) => High,
      (High, Low) => High,
      (High, High) => Low,
  }
}


pub fn not(in1: Signal) -> Signal {
  nand(in1, in1)
}

pub fn and(in1: Signal, in2: Signal) -> Signal {
  nand(nand(in1, in2), nand(in1, in2))
}

pub fn or(in1: Signal, in2: Signal) -> Signal {
  nand(nand(in1, in1), nand(in2, in2))
}

pub fn xor(in1: Signal, in2: Signal) -> Signal {
  nand(nand(nand(in1, in2), in1), nand(nand(in1, in2), in2))
}

pub fn mux(in1: Signal, in2: Signal, sel: Signal) -> Signal {
  nand(
      nand(
          in1, 
          nand(sel, sel)
      ), 
      nand(in2, sel)
  )
}

pub fn dmux(in1: Signal, sel: Signal) -> (Signal, Signal) {
  (nand(
      nand(in1, nand(sel, sel)),
      nand(in1, nand(sel, sel))
  ),
  nand(
      nand(in1, sel),
      nand(in1, sel)
  ))
}

// pub fn not16(in1: Signals<16>) -> Signals<16> {
//     in1.iter().map(|&s| not(s)).collect::<Vec<Signal>>().try_into().unwrap()
// }

pub fn not_n<const BITS: usize> (in1: Signals<BITS>) -> Signals<BITS> {
  in1.iter().map(|&s| not(s)).collect::<Vec<Signal>>().try_into().unwrap()
}

pub fn and_n<const BITS: usize> (in1: Signals<BITS>, in2: Signals<BITS>) -> Signals<BITS> {
  in1.iter().zip(in2.iter()).map(|(&s1, &s2)| and(s1, s2)).collect::<Vec<Signal>>().try_into().unwrap()
}

pub fn or_n<const BITS: usize> (in1: Signals<BITS>, in2: Signals<BITS>) -> Signals<BITS> {
  in1.iter().zip(in2.iter()).map(|(&s1, &s2)| or(s1, s2)).collect::<Vec<Signal>>().try_into().unwrap()
}

pub fn mux_n<const BITS: usize> (in1: Signals<BITS>, in2: Signals<BITS>, sel: Signal) -> Signals<BITS> {
  in1.iter().zip(in2.iter()).map(|(&s1, &s2)| mux(s1, s2, sel)).collect::<Vec<Signal>>().try_into().unwrap()
}

pub fn or_m_way<const BITS: usize> (in1: Signals<BITS>) -> Signal {
  in1.iter().fold(Low, |acc, &s| or(acc, s))
}

pub fn and_m_way<const BITS: usize> (in1: Signals<BITS>) -> Signal {
  in1.iter().fold(High, |acc, &s| and(acc, s))
}

pub fn mux_4_way_n<const BITS: usize>(in1: Signals<BITS>, in2: Signals<BITS>, in3: Signals<BITS>, in4: Signals<BITS>, sel1: Signal, sel2: Signal) -> Signals<BITS> {
  mux_n(
      mux_n(in1, in2, sel1),
      mux_n(in3, in4, sel1), 
      sel2
  )
}
pub fn mux_8_way_n<const BITS: usize>(
  in1: Signals<BITS>,
  in2: Signals<BITS>,
  in3: Signals<BITS>,
  in4: Signals<BITS>, 
  in5: Signals<BITS>,
  in6: Signals<BITS>,
  in7: Signals<BITS>,
  in8: Signals<BITS>,
  sel1: Signal,
  sel2: Signal,
  sel3: Signal
) -> Signals<BITS> {
  mux_n(
      mux_4_way_n(in1, in2, in3, in4, sel1, sel2),
      mux_4_way_n(in5, in6, in7, in8, sel1, sel2),
      sel3
  )
}

pub fn dmux_4_way(in1: Signal, sel1: Signal, sel2: Signal) -> Signals<4> {
  [
      and_m_way([in1, not(sel1), not(sel2)]),
      and_m_way([in1, not(sel1), sel2]),
      and_m_way([in1, sel1, not(sel2)]),
      and_m_way([in1, sel1, sel2]),        
  ]
}
pub fn dmux_8_way(in1: Signal, sel1: Signal, sel2: Signal, sel3: Signal) -> Signals<8> {
  [
      and_m_way([in1, not(sel1), not(sel2), not(sel3)]),
      and_m_way([in1, not(sel1), not(sel2), sel3]),
      and_m_way([in1, not(sel1), sel2, not(sel3)]),
      and_m_way([in1, not(sel1), sel2, sel3]),
      and_m_way([in1, sel1, not(sel2), not(sel3)]),
      and_m_way([in1, sel1, not(sel2), sel3]),
      and_m_way([in1, sel1, sel2, not(sel3)]),
      and_m_way([in1, sel1, sel2, sel3]),      
  ]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tnand() {
        assert_eq!(nand(Low, Low), High);
        assert_eq!(nand(Low, High), High);
        assert_eq!(nand(High, Low), High);
        assert_eq!(nand(High, High), Low);
    }
    #[test]
    fn tnot() {
        assert_eq!(not(Low), High);
        assert_eq!(not(High), Low);
    }
    #[test]
    fn tand() {
        assert_eq!(and(Low, Low), Low);
        assert_eq!(and(Low, High), Low);
        assert_eq!(and(High, Low), Low);
        assert_eq!(and(High, High), High);
    }
    #[test]
    fn tor() {
        assert_eq!(or(Low, Low), Low);
        assert_eq!(or(Low, High), High);
        assert_eq!(or(High, Low), High);
        assert_eq!(or(High, High), High);
    }
    #[test]
    fn txor() {
        assert_eq!(xor(Low, Low), Low);
        assert_eq!(xor(Low, High), High);
        assert_eq!(xor(High, Low), High);
        assert_eq!(xor(High, High), Low);
    }
    #[test]
    fn tmux() {
        assert_eq!(mux(Low, Low, Low), Low);
        assert_eq!(mux(Low, High, Low), Low);
        assert_eq!(mux(High, Low, Low), High);
        assert_eq!(mux(High, High, Low), High);
        assert_eq!(mux(Low, Low, High), Low);
        assert_eq!(mux(Low, High, High), High);
        assert_eq!(mux(High, Low, High), Low);
        assert_eq!(mux(High, High, High), High);
    }
    #[test]
    fn tdmux() {
        assert_eq!(dmux(Low, Low), (Low, Low));
        assert_eq!(dmux(Low, High), (Low, Low));
        assert_eq!(dmux(High, Low), (High, Low));
        assert_eq!(dmux(High, High), (Low, High));
    }

    #[test]
    fn tnot_n() {
        assert_eq!(not_n([Low, High]), [High, Low]);
        assert_eq!(not_n([High, High]), [Low, Low]);
    }
    #[test]
    fn tand_n() {
        assert_eq!(and_n([Low, High], [Low, High]), [Low, High]);
        assert_eq!(and_n([High, High], [Low, Low]), [Low, Low]);
    }
    #[test]
    fn tor_n() {
        assert_eq!(or_n([Low, High], [Low, High]), [Low, High]);
        assert_eq!(or_n([High, High], [Low, Low]), [High, High]);
    }
    #[test]
    fn tmux_n() {
        assert_eq!(mux_n([Low, High], [High, High], Low), [Low, High]);
        assert_eq!(mux_n([Low, High], [High, High], High), [High, High]);
    }
    #[test]
    fn tor_m_way() {
        assert_eq!(or_m_way([Low, Low, Low]), Low);
        assert_eq!(or_m_way([Low, High, Low]), High);
    }
    #[test]
    fn tmux_4_way_n() {
        assert_eq!(mux_4_way_n([Low], [Low], [High], [Low], Low, Low), [Low]);
        assert_eq!(mux_4_way_n([Low], [Low], [High], [Low], High, Low), [Low]);
        assert_eq!(mux_4_way_n([Low], [Low], [High], [Low], Low, High), [High]);
        assert_eq!(mux_4_way_n([Low], [Low], [High], [Low], High, High), [Low]);
    }
    #[test]
    fn tmux_8_way_n() {
        assert_eq!(mux_8_way_n([Low], [Low], [Low], [Low], [Low], [Low], [High], [Low], Low, Low, Low), [Low]);
        assert_eq!(mux_8_way_n([Low], [Low], [Low], [Low], [Low], [Low], [High], [Low], Low, Low, Low), [Low]);
        assert_eq!(mux_8_way_n([Low], [Low], [Low], [Low], [Low], [Low], [High], [Low], Low, Low, Low), [Low]);
        assert_eq!(mux_8_way_n([Low], [Low], [Low], [Low], [Low], [Low], [High], [Low], Low, Low, Low), [Low]);
        assert_eq!(mux_8_way_n([Low], [Low], [Low], [Low], [Low], [Low], [High], [Low], Low, Low, Low), [Low]);
        assert_eq!(mux_8_way_n([Low], [Low], [Low], [Low], [Low], [Low], [High], [Low], Low, Low, Low), [Low]);
        assert_eq!(mux_8_way_n([Low], [Low], [Low], [Low], [Low], [Low], [High], [Low], Low, Low, Low), [Low]);
        assert_eq!(mux_8_way_n([Low], [Low], [Low], [Low], [Low], [Low], [High], [Low], Low, High, High), [High]);
    }
    #[test]
    fn tdmux_4_way() {
        assert_eq!(dmux_4_way(Low, Low, Low), [Low, Low, Low, Low]);
        assert_eq!(dmux_4_way(High, Low, Low), [High, Low, Low, Low]);
        assert_eq!(dmux_4_way(Low, Low, High), [Low, Low, Low, Low]);
        assert_eq!(dmux_4_way(High, Low, High), [Low, High, Low, Low]);
        assert_eq!(dmux_4_way(Low, High, Low), [Low, Low, Low, Low]);
        assert_eq!(dmux_4_way(High, High, Low), [Low, Low, High, Low]);
        assert_eq!(dmux_4_way(Low, High, High), [Low, Low, Low, Low]);
        assert_eq!(dmux_4_way(High, High, High), [Low, Low, Low, High]);
    }
    #[test]
    fn tdmux_8_way() {
        assert_eq!(dmux_8_way(Low, Low, Low, Low), [Low, Low, Low, Low, Low, Low, Low, Low]);
        assert_eq!(dmux_8_way(High, Low, Low, Low), [High, Low, Low, Low, Low, Low, Low, Low]);
        assert_eq!(dmux_8_way(High, Low, Low, High), [Low, High, Low, Low, Low, Low, Low, Low]);
        assert_eq!(dmux_8_way(Low, Low, Low, High), [Low, Low, Low, Low, Low, Low, Low, Low]);
        assert_eq!(dmux_8_way(High, High, High, High), [Low, Low, Low, Low, Low, Low, Low, High]);
        assert_eq!(dmux_8_way(High, High, High, Low), [Low, Low, Low, Low, Low, Low, High, Low]);
    }
}