use num::bigint::*;
use utils::math;

pub fn solve() -> BigInt {
    let mut max = 0.to_bigint().unwrap();
    for i in 1..100 {
        for j in 1..100 {
            let test = math::pow_big(i, j);
            let sum = digit_sum(test);
            if sum > max {
                max = sum.clone();
            }
        }
    }
    return max;
}

pub fn digit_sum(x: BigInt) -> BigInt {
    let s = x.to_string();
    let v: Vec<char> = s.chars().collect();
    let mut sum = 0;
    for i in 0..v.len() {
        sum += v[i] as i64 - '0' as i64;
    }
    return sum.to_bigint().unwrap();
}