use utils::math;

pub fn solve() -> i64 {
	for i in 1..150000 {
		let mut multiples = vec![];
		for j in 2..7 {
			multiples.push(j * i as i64);
		}
		let mut valid = true;
		for j in 0..multiples.len() {
			valid = valid && same_digits(i as i64, multiples[j]);
		}
		if valid {
			return i;
		}
	}
	return 0;
}

pub fn same_digits(x: i64, y: i64) -> bool {
	let x_v = math::to_vec(&x);
	let mut y_v = math::to_vec(&y);
	if x_v.len() != y_v.len() {
		return false;
	}

	let mut digits: Vec<i64> = vec![];
	for i in 0..x_v.len() {
		digits.push(x_v[i]);
	}
	for i in 0..y_v.len() {
		for j in 0..digits.len() {
			if digits[j] == y_v[i] {
				y_v[i] = -1;
				digits[j] = -1;
				break;
			}
		}
	}

	let mut content = false;
	for i in 0..digits.len() {
		if digits[i] != -1 {
			content = true;
		}
	}
	for i in 0..y_v.len() {
		if y_v[i] != -1 {
			content = true;
		}
	}
	return !content;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 142857);
	}
}