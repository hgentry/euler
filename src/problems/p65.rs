use utils::math;
extern crate num_bigint;

use num_bigint::*;
pub fn solve() -> BigInt {
    let add = e_seq(100);
    return math::sum_digits_big(add.0);
}

pub fn e_seq(n: i128) -> (BigInt, BigInt) {
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

pub fn expanded_sum(sequence: Vec<i128>) -> (BigInt, BigInt) {
    let i = sequence.len() - 1;
    let mut temp: (BigInt, BigInt) = (sequence[i].to_bigint().unwrap(),1.to_bigint().unwrap());
    for j in 0..i {
        temp = (temp.1, temp.0);
        temp = math::add_fractions_without_reduce_big(temp, (sequence[i-j-1].to_bigint().unwrap(),1.to_bigint().unwrap()));
    }
    return temp;
}


