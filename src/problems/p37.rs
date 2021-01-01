use utils::math;
use utils::primes;
pub fn solve() -> i64 {
	let mut rt_primes = vec![2, 3, 5, 7];
	let mut successes = vec![];
	let digits = vec![1, 3, 7, 9];
	while successes.len() < 15 {
		let p = rt_primes.pop().unwrap();
		for d in &digits {
			let candidate = format!("{}{}", p, d).parse::<i64>().unwrap();
			if primes::is_prime(candidate) {
				rt_primes.push(candidate);
			}
		}
		if left_truncatable(p) {
			successes.push(p);
		}
	}

	let mut sum = 0;
	for p in successes {
		sum += p;
	}

	sum - 17
}

pub fn right_truncatable(p: i64) -> bool {
	let mut failed = false;
	let mut x = p;
	x = x / 10;

	while x > 0 {
		if !primes::is_prime(x) {
			failed = true;
			break;
		}
		x = x / 10;
	}
	!failed
}

pub fn left_truncatable(p: i64) -> bool {
	let mut k = 1;
	let mut failed = false;
	while p != p % math::pow(10, k) {
		if !primes::is_prime(p % math::pow(10, k)) {
			failed = true;
			break;
		}
		k += 1;
	}
	return !failed;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 748317);
	}
}
