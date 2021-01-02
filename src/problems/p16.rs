use crate::utils::math;

pub fn solve() -> i64 {
	let big = math::pow_big(2, 1000);
	let s = format!("{}", big);
	let mut sum = 0;
	for c in s.chars() {
		sum += c.to_digit(10).unwrap() as i64;
	}
	return sum;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 1366);
	}
}
