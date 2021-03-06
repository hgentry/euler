pub fn solve() -> i64 {
	let mut p = 1;
	for _ in 0..7830457 {
		p = p * 2 % 100000000000;
	}
	p = 28433 * p + 1;
	return p % 10000000000;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 8739992577);
	}
}
