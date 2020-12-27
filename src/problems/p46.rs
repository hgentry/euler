use utils::primes;

pub fn solve() -> i64 {
	let primes = primes::list_primes(10000);

	for i in 3..10000 {
		if i%2 == 0 {
			continue;
		}

		if primes::is_prime(i) {
			continue;
		}

		let mut found = false;
		for j in 0..primes.len() {
			if primes[j] >= i {
				break;
			}
			let mut k = 0;
			loop {
				if 2*k*k >= i {
					break;
				}
				if primes[j] + 2*k*k == i {
					found = true;
					break;
				}

				k += 1;
			}
		}

		if !found {
			return i;
		}
	}
	return 0;
}








