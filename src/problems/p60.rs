use std::collections::HashMap;
use utils::math;
use utils::primes;

pub fn solve() -> i64 {
	let primes = primes::list_primes(10000);
	let mut map: HashMap<i64, Vec<i64>> = HashMap::new();
	for p in 0..primes.len() {
		map.insert(primes[p], vec![]);
	}
	for p in 0..primes.len() {
		for q in p + 1..primes.len() {
			if primes[q] == 2 || primes[q] == 5 || !check_concat(primes[p], primes[q]) {
				continue;
			} else {
				let mut fail = false;
				let m = map.get_mut(&primes[p]).unwrap();
				for v in 0..m.len() {
					if !check_concat(m[v], primes[q]) {
						fail = true;
						break;
					}
				}
				if !fail {
					m.push(primes[q]);
					if m.len() == 4 {
						return primes[p] + m[0] + m[1] + m[2] + m[3];
					}
				}
				fail = false;
				let m2 = map.get_mut(&primes[q]).unwrap();
				for v in 0..m2.len() {
					if !check_concat(m2[v], primes[p]) {
						fail = true;
						break;
					}
				}
				if !fail {
					m2.push(primes[p]);
				}
			}
		}
	}
	return 0;
}

pub fn check_concat(a: i64, b: i64) -> bool {
	let ai = (a as f64).log10().floor() as i64 + 1;
	let bi = (b as f64).log10().floor() as i64 + 1;
	let ab = b + a * math::pow(10, bi);
	let ba = a + b * math::pow(10, ai);
	if primes::is_prime(ab) && primes::is_prime(ba) {
		return true;
	}
	return false;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 26033);
	}
}
