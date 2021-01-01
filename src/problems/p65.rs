use utils::math;
extern crate num;

use num::rational::{Ratio, BigRational};
use num::bigint::*;

pub fn solve() -> BigInt {
    let add = e_seq(100);
    return math::sum_digits(add.numer());
}

pub fn e_seq(n: i64) -> BigRational {
    let  mut r = vec!(2,1,2);
    for i in 0..n-3 {
        if (i + 1) % 3 == 0 {
            r.push(2*((i + 1) / 3 + 1));
        }
        else {
            r.push(1);
        }
    }
    return expanded_sum(r);
}

pub fn expanded_sum(sequence: Vec<i64>) -> BigRational {
    let i = sequence.len() - 1;
    let mut temp: BigRational = Ratio::from_integer(sequence[i].to_bigint().unwrap());
    for j in 0..i {
        temp = Ratio::new_raw(temp.denom().clone(), temp.numer().clone());
        temp = math::add_ratios_carelessly(temp, Ratio::from_integer(sequence[i-j-1].to_bigint().unwrap()));
        //temp = temp + Ratio::from_integer(sequence[i-j-1].to_bigint().unwrap());
    }
    return temp;
}


