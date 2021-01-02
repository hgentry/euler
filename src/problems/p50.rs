use crate::utils::primes;

pub fn solve() -> i64 {
	let primes = primes::list_primes(1000000);
	let mut longest_chain = 0;
	let mut winning_prime = 0;
	for i in 0..primes.len() {
		let mut running_sum = 0;
		let mut chain_length = 0;
		for q in i..primes.len() {
			running_sum += primes[q];
			chain_length += 1;
			if chain_length > longest_chain && primes::is_prime(running_sum) {
				winning_prime = running_sum;
				longest_chain = chain_length;
			}
			if running_sum > 1000000 {
				break;
			}
		}
	}
	return winning_prime;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 997651);
	}
}
