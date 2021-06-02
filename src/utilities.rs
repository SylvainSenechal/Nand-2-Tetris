use crate::gates::*;
use crate::alu::n_incrementor;
use Signal::*;
use std::convert::TryInto;


pub fn int_to_binary16(int: i16) -> Signals<16> {
  let binary_string = format!("{:016b}", int);
  let mut binary: Vec<Signal> = vec![];
  for bit in binary_string.chars() {
      match bit {
          '0' => binary.push(Low),
          '1' => binary.push(High),
          _ => ()
      }
  }
  binary.try_into().unwrap()
}

pub fn binary_to_int16(mut binary: Signals<16>) -> i16 {
  let mut sign = 1;
  if binary[0] == High {
      sign = - 1;
      binary = n_incrementor(not_n(binary)); 
  }
  let binary_string = binary.iter().fold(String::from(""), |mut acc, &bit| {
      match bit {
          Low => acc.push('0'),
          High => acc.push('1')
      }
      acc
  });
  sign * i16::from_str_radix(&binary_string, 2).unwrap()
}