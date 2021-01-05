use crate::utils::math;

pub fn solve() -> i64 {
	let mut count = 0;
	let limit: i64 = 1500000;
	let mut salud = vec![0; limit as usize + 1];

	for n in 1..1000 {
		let mut m = n + 1;
		loop {
			if 2 * m * (m + n) > limit {
				break;
			}
			if math::gcd(n, m) == 1 {
				let mut i = 1;
				while 2 * i * m * (m + n) <= limit {
					salud[(2 * i * m * (m + n)) as usize] += 1;
					i += 1;
				}
			}
			m += 2;
		}
	}

	for i in 0..salud.len() {
		if salud[i] == 1 {
			count += 1;
		}
	}

	return count;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 161667);
	}
}
