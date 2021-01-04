use crate::utils::math;
use crate::utils::primes;

pub fn solve() -> i64 {
	let totients = primes::list_totients(10000000);

	let mut min_i: i64 = 0;
	let mut min_f: f64 = 10000000.0;
	for i in 2..10000000 {
		let t = totients[i - 1];
		if is_permutation_of(t, i as i64) {
			if (i as f64 / t as f64) < min_f {
				min_f = i as f64 / t as f64;
				min_i = i as i64;
			}
		}
	}

	return min_i;
}

pub fn is_permutation_of(a: i64, b: i64) -> bool {
	if math::num_digits(a) != math::num_digits(b) {
		return false;
	}
	let a_v = math::to_vec(&a);
	let mut b_v = math::to_vec(&b);
	for v in a_v {
		let mut found = false;
		for i in 0..b_v.len() {
			if b_v[i] == v {
				b_v.remove(i);
				found = true;
				break;
			}
		}
		if !found {
			return false;
		}
	}
	b_v.len() == 0
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 8319823);
	}
}
