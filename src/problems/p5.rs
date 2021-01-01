pub fn solve() -> i64 {
	let mut i = 19 * 17 * 13 * 11 * 7 * 5 * 3 * 2;
	loop {
		let mut j = 1;
		let mut winner = true;
		while j <= 20 {
			if i % j != 0 {
				winner = false;
			}
			j += 1;
		}
		if winner {
			return i;
		}
		i += 19 * 17 * 13 * 11 * 7 * 5 * 3 * 2;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 232792560);
	}
}
