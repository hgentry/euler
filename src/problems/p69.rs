use crate::utils::primes;

pub fn solve() -> i64 {
	let mut best_n: i64 = 0;
	let mut best_f = 0.0;

	let tot = primes::list_totients(1000000);
	for i in 2..=1000000 {
		let f = i as f64 / tot[i - 1] as f64;
		if f > best_f {
			best_f = f;
			best_n = i as i64;
		}
	}

	return best_n;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 510510);
	}
}
