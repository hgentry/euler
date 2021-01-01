use utils::primes;

pub fn solve() -> i64 {
	let mut n: i64 = 1;
	let mut primes = 0;
	let mut nums = 1;
	let mut pos = 1;
	loop {
		n += 2;
		pos += 1;
		nums += 4;

		pos += n - 2;
		if primes::is_prime(pos) {
			primes += 1;
		}

		pos += n - 1;
		if primes::is_prime(pos) {
			primes += 1;
		}

		pos += n - 1;
		if primes::is_prime(pos) {
			primes += 1;
		}

		pos += n - 1;
		if primes::is_prime(pos) {
			primes += 1;
		}

		let ratio = (primes as f64) / (nums as f64);
		if ratio < 0.1 {
			return n;
		}
	}
}
