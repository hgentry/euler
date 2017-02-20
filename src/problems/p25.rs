extern crate num_bigint;
use num_bigint::*;

pub fn solve() -> num_bigint::BigInt {
	let mut index = 2.to_bigint().unwrap();
	let mut f1 = 1.to_bigint().unwrap();
	let mut f2 = 1.to_bigint().unwrap();
	loop {
		let tmp = f2.clone() + f1.clone();
		f1 = f2;
		f2 = tmp;
		index = index + 1.to_bigint().unwrap();

		let s = f2.to_string();
		if s.len() == 1000 {
			break;
		}
	}

	return index;
}


