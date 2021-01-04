use crate::utils::math;

pub fn solve() -> i64 {
	let max = 12000;
	let lowest: f64 = 1.0 / 3.0;
	let greatest: f64 = 0.5;
	let mut count = 0;

	for i in 1..=max {
		for j in i + 1..=max {
			if math::gcd(i, j) == 1 {
				let r = i as f64 / j as f64;
				if r > lowest && r < greatest {
					count += 1;
				}
			}
		}
	}

	return count;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 7295372);
	}
}
