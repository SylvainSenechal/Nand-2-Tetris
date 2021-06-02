mod gates;
mod alu;
mod utilities;
use gates::Signal::*;
use alu::*;
use utilities::*;

fn main() {
    let x = 5;
    let y = - 9;
    let a = int_to_binary16(x);
    let b = int_to_binary16(y);
    let result = alu(a, b, Low, Low, Low, Low, High, Low);
    println!("{:?} {:?}", x, y);
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", result);
    println!("{:?}", binary_to_int16(result.0));
}