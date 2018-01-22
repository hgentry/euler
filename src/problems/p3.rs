use utils;

pub fn solve() -> i64 {
	let big = 600851475143;
	let upper_limit = (big as f64).sqrt() as i64 + 1;
	let mut i = 2;
	let mut best = 1;
	while i < upper_limit {
		if big % i == 0 {
			if utils::primes::is_prime(i) {
				best = i;
			}
		}
		i += 1;
	}

	return best;
}
