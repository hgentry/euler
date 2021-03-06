use crate::utils::math;

pub fn solve() -> i64 {
	let mut found = 0;
	for i in 1..10000 {
		let mut sum: i128 = i;
		for _ in 0..=50 {
			sum += math::reverse(&sum);
			if math::is_palindrome(&sum) {
				found += 1;
				break;
			}
		}
	}
	return 9999 - found;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 249);
	}
}
