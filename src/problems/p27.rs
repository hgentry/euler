use utils;

pub fn solve() -> i64 {
	let mut best = 0;
	let mut longest = 0;
	for a in -999..1000 {
		for b in -1000..1001 {
			let mut n = 0;
			loop {
				let p = n * n + a * n + b;
				if p >= 2 && (n >= 2 && b % n != 0 || n < 2) && utils::primes::is_prime(p) {
					if n > longest {
						longest = n;
						best = a * b;
					}
				} else {
					break;
				}
				n += 1;
			}
		}
	}
	return best;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), -59231);
	}
}
