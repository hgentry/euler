use crate::utils::primes;

pub fn solve() -> i128 {
	let totients = primes::list_totients(1000000);
	let mut sum: i128 = 0;
	for i in totients {
		sum += i as i128;
	}
	return sum - 1;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[ignore]
	fn correct() {
		assert_eq!(solve(), 0);
	}
}
