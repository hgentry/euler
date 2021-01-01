use num::bigint::*;
use utils::math;

pub fn solve() -> i64 {
	let mut count: i64 = 0;

	let mut a0 = 3.to_bigint().unwrap();
	let mut b0 = 2.to_bigint().unwrap();
	for _i in 1..999 {
		let b1 = b0.clone() + a0.clone();
		let a1 = b1.clone() + b0.clone();
		if math::num_digits_bigint(a1.clone()) > math::num_digits_bigint(b1.clone()) {
			count += 1;
		}
		a0 = a1.clone();
		b0 = b1.clone();
	}

	return count;
}
