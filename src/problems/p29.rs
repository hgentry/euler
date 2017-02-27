extern crate num_bigint;
use num_bigint::*;
use utils;

pub fn solve() -> i64 {
	let mut v: Vec<BigInt> = vec![];
	for a in 2..101 {
		for b in 2..101 {
			let n = utils::pow_big(a as i64,b as i64);
			let mut found = false;
			for num in &v {
				if n == *num {
					found = true;
					break;
				}
			}
			if !found {
				v.push(n);
			}
		}
	}
	v.len() as i64
}
