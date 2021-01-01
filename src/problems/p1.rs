pub fn solve() -> i64 {
	let mut i = 1;
	let mut sum = 0;

	while i < 1000 {
		if i % 3 == 0 || i % 5 == 0 {
			sum += i;
		}
		i += 1;
	}

	return sum;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 233168);
	}
}
