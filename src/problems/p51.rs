use crate::utils::math;
use crate::utils::primes;

pub fn solve() -> i64 {
	let primes = primes::list_primes(1000000);
	for p in primes {
		for j in 1..math::pow(2, (p as f64).log10().floor() as i64 + 1) {
			let alterations = num_alter(p, j);
			let mut found_primes = 0;
			for k in 0..alterations.len() {
				if primes::is_prime(alterations[k]) {
					found_primes += 1;
				}
			}
			if found_primes == 8 && alterations.contains(&p) {
				return p;
			}
		}
	}

	return 0;
}

pub fn num_alter(x: i64, b: i64) -> Vec<i64> {
	let start_num: Vec<i64> = math::to_vec(&x);
	let mut results: Vec<i64> = vec![];

	for j in 0..10 {
		let mut r = b;
		let mut mutilate = math::to_vec(&x);
		for i in 0..start_num.len() {
			if r % 2 == 1 {
				mutilate[i] = j as i64;
			}
			r = ((r as f64) / 2.0).floor() as i64;
		}
		let addition = math::from_vec(&mutilate);
		if addition >= x {
			results.push(addition);
		}
	}

	return results;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 121313);
	}
}
