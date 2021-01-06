use crate::utils::math;
use num::{bigint::*, ToPrimitive};

pub fn solve() -> i64 {
	let mut squares = vec![0];
	let mut sum = 0;
	for i in 1..100 {
		squares.push(i * i);
	}
	for i in 2..=99 {
		if squares.contains(&i) {
			continue;
		}
		let f = sqrt(i);
		let s: i64 = math::sum_digits(&f).to_i64().unwrap() - 5;
		sum += s;
	}

	return sum;
}

pub fn sqrt(x: i64) -> BigInt {
	let mut a: BigInt = 5 * x.to_bigint().unwrap();
	let mut b: BigInt = 5.to_bigint().unwrap();
	while math::num_digits_bigint(b.clone()) < 102 {
		if a >= b.clone() {
			a = a - b.clone();
			b = b.clone() + 10;
		} else {
			a = a.clone() * 100;
			b = b.clone() / 10 * 100 + 5;
		}
	}

	return b;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 40886);
	}
}
