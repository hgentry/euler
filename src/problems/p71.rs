use num::rational::*;

pub fn solve() -> i64 {
	let max = 1000000;
	let ts = 7.0 / 3.0;
	let mut best = Ratio::<i64>::new(0, 1);
	for i in 1..=max {
		let j_min = (ts * (i as f64)).floor() as i64;
		if j_min == 0 {
			continue;
		}
		for j in j_min + 1..=max {
			let t = Ratio::<i64>::new(i, j);
			if t > best {
				best = t;
			} else {
				break;
			}
		}
	}
	let result = best;
	return *result.numer();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 428570);
	}
}
