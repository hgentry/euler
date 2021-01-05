use crate::utils::math;
use crate::utils::primes;

pub fn solve() -> i64 {
	let limit = 10000000;
	let totients = primes::list_totients(limit);

	let mut min_i: i64 = 0;
	let mut min_f: f64 = limit as f64;
	for i in 2..limit {
		let t = totients[i as usize];
		if (i as f64 / t as f64) < min_f && math::is_permutation_of(t, i as i64) {
			min_f = i as f64 / t as f64;
			min_i = i as i64;
		}
	}

	return min_i;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 8319823);
	}
}
