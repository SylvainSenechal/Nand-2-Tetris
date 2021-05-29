use Signal::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Signal {
    High,
    Low,
}

// enum Gate {
//     solo,
//     multi
// }

// enum Input <const N: usize> {
//     inside(N)
// }

// impl ee for Input {
//     match self {
//         input::solo, 
//         input::multi
//     }
// }


struct Input <const N: usize, const BITS: usize> {
    signals: [[Signal; N]; BITS]
}

trait Gate <const NB_IN: usize, const NB_OUT: usize> {
    fn compute(&self, input: [Signal; NB_IN]) -> [Signal; NB_OUT];
}

trait GGate <const SIZE: usize, const NB_IN: usize, const NB_OUT: usize> {
    fn compute(&self, input: [[Signal; NB_IN]; SIZE]) -> [[Signal; NB_OUT]; SIZE];

}

struct Nand;
struct Not;
struct And;
struct Or;
struct Xor;
struct Mux;
struct Dmux;
// struct Not16 <T: Gate, const N: usize> {
//     gates: [T; N]
// }
// let u = Not16{gates: [Xor, Xor]};

struct AndX(i32);



impl Gate<2, 1> for Nand {
    fn compute(&self, input: [Signal; 2]) -> [Signal; 1] {
        let a = AndX(5);
        let b = [[Low, Low], [High, High]];

        match (&input[0], &input[1]) {
            (Low, Low) => [High; 1],
            (Low, High) => [High; 1],
            (High, Low) => [High; 1],
            (High, High) => [Low; 1],
        }
    }
}

impl Gate<1, 1> for Not {
    fn compute(&self, input: [Signal; 1]) -> [Signal; 1] {
        Nand.compute([input[0], input[0]])
    }
}

impl Gate<2, 1> for And {
    fn compute(&self, input: [Signal; 2]) -> [Signal; 1] {
        let nand_result = Nand.compute([input[0], input[1]]);
        Nand.compute([nand_result[0], nand_result[0]])
    }
}
impl Gate<2, 1> for Or {
    fn compute(&self, input: [Signal; 2]) -> [Signal; 1] {
        let nand_result1 = Nand.compute([input[0], input[0]]);
        let nand_result2 = Nand.compute([input[1], input[1]]);
        Nand.compute([nand_result1[0], nand_result2[0]])
    }
}
impl Gate<2, 1> for Xor {
    fn compute(&self, input: [Signal; 2]) -> [Signal; 1] {
        let nand_result1 = Nand.compute([input[0], input[1]]);
        let nand_result21 = Nand.compute([nand_result1[0], input[0]]);
        let nand_result22 = Nand.compute([nand_result1[0], input[1]]);
        Nand.compute([nand_result21[0], nand_result22[0]])
    }
}
impl Gate<3, 1> for Mux {
    fn compute(&self, input: [Signal; 3]) -> [Signal; 1] {
        let nand_result1 = Nand.compute([input[2], input[2]]);
        let nand_result2 = Nand.compute([input[1], input[2]]);
        let nand_result3 = Nand.compute([input[0], nand_result1[0]]);
        let nand_result4 = Nand.compute([nand_result3[0], nand_result2[0]]);
        nand_result4
    }
}
impl Gate<2, 2> for Dmux {
    fn compute(&self, input: [Signal; 2]) -> [Signal; 2] {
        let nand_result1 = Nand.compute([input[1], input[1]]);
        let nand_result2 = Nand.compute([input[0], nand_result1[0]]);
        let nand_result3 = Nand.compute([input[0], input[1]]);
        let nand_result4 = Nand.compute([nand_result2[0], nand_result2[0]]);
        let nand_result5 = Nand.compute([nand_result3[0], nand_result3[0]]);
        [nand_result4[0], nand_result5[0]]
    }
}
#[derive(Debug)]
enum G {
    Nand,
    Not,
    And,
    Or,
}

// enum GX <const BITS: usize> {
//     Mux(BITS)
// }
enum GX {
    Mux(usize)
}
impl G {
    fn compute(&self) -> i32 {
        let a = GX::Mux(1);
        match self {
            G::Nand => self.oui(),
            G::Not => 1,
            G::And => 2,
            G::Or => 3,
        }
    }
    fn oui(&self) -> i32 {
        5656
    }
}

fn main() {
    let u = G::Nand;
    print!("{:?}", u);
    print!("{:?}", u.compute());
    print!("{:?}", u.compute());
    // let g: Nand = Nand;
    // println!("{:?}", g.compute([Low, Low]));
    // println!("{:?}", g.compute([Low, High]));
    // println!("{:?}", g.compute([High, Low]));
    // println!("{:?}", g.compute([High, High]));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nand() {
        assert_eq!(Nand.compute([Low, Low]), [High; 1]);
        assert_eq!(Nand.compute([Low, High]), [High; 1]);
        assert_eq!(Nand.compute([High, Low]), [High; 1]);
        assert_eq!(Nand.compute([High, High]), [Low; 1]);
    }
    #[test]
    fn not() {
        assert_eq!(Not.compute([Low]), [High; 1]);
        assert_eq!(Not.compute([High]), [Low; 1]);
    }
    #[test]
    fn and() {
        assert_eq!(And.compute([Low, Low]), [Low; 1]);
        assert_eq!(And.compute([Low, High]), [Low; 1]);
        assert_eq!(And.compute([High, Low]), [Low; 1]);
        assert_eq!(And.compute([High, High]), [High; 1]);
    }
    #[test]
    fn or() {
        assert_eq!(Or.compute([Low, Low]), [Low; 1]);
        assert_eq!(Or.compute([Low, High]), [High; 1]);
        assert_eq!(Or.compute([High, Low]), [High; 1]);
        assert_eq!(Or.compute([High, High]), [High; 1]);
    }
    #[test]
    fn xor() {
        assert_eq!(Xor.compute([Low, Low]), [Low; 1]);
        assert_eq!(Xor.compute([Low, High]), [High; 1]);
        assert_eq!(Xor.compute([High, Low]), [High; 1]);
        assert_eq!(Xor.compute([High, High]), [Low; 1]);
    }
    #[test]
    fn mux() {
        assert_eq!(Mux.compute([Low, Low, Low]), [Low; 1]);
        assert_eq!(Mux.compute([Low, High, Low]), [Low; 1]);
        assert_eq!(Mux.compute([High, Low, Low]), [High; 1]);
        assert_eq!(Mux.compute([High, High, Low]), [High; 1]);
        assert_eq!(Mux.compute([Low, Low, High]), [Low; 1]);
        assert_eq!(Mux.compute([Low, High, High]), [High; 1]);
        assert_eq!(Mux.compute([High, Low, High]), [Low; 1]);
        assert_eq!(Mux.compute([High, High, High]), [High; 1]);
    }
    #[test]
    fn dmux() {
        assert_eq!(Dmux.compute([Low, Low]), [Low, Low]);
        assert_eq!(Dmux.compute([Low, High]), [Low, Low]);
        assert_eq!(Dmux.compute([High, Low]), [High, Low]);
        assert_eq!(Dmux.compute([High, High]), [Low, High]);
    }
}
