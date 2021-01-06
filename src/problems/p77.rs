use crate::utils::{math, primes};

pub fn solve() -> i64 {
	let coins = primes::list_primes(75);
	for i in 1.. {
		let c = math::changecounter(i, &coins, coins.len(), 5000000, 0);
		//println!("{}: {}", i , c);
		if c > 5000 {
			return i;
		}
	}
	return 0;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 71);
	}
}
