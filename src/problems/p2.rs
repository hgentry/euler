pub fn solve() -> i64 {
	let mut a1 = 1;
	let mut a2 = 1;
	let mut sum = 0;

	while a2 <= 4000000 {
		if a2 % 2 == 0 {
			sum += a2;
		}
		let tmp = a2;
		a2 = a1 + a2;
		a1 = tmp;
	}

	return sum;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 4613732);
	}
}
