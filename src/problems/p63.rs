pub fn solve() -> i64 {
	let mut count = 1;
	let mut i = 2;

	loop {
		let mut j = 1;
		let l = (i as f64).log10();
		let mut first = true;
		loop {
			if i >= 10 {
				break;
			}
			let v = (j as f64 * l).floor() as i64;

			if v + 1 == j {
				count += 1;
			} else if v + 1 < j {
				break;
			}
			first = false;
			j += 1;
		}
		if first {
			break;
		}
		i += 1;
	}
	return count;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 49);
	}
}
