use crate::utils::math;
use num::bigint::*;

pub fn solve() -> BigInt {
	let mut max = 0.to_bigint().unwrap();
	for i in 1..100 {
		for j in 1..100 {
			let test = math::pow_big(i, j);
			let sum = math::sum_digits(&test);
			if sum > max {
				max = sum.clone();
			}
		}
	}
	return max;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 972.to_bigint().unwrap());
	}
}
