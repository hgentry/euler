pub fn solve() -> i64 {
	let mut pos: i32 = 2;
	let mut sum: i32 = 1;
	let mut layer: i32 = 1;

	while pos < 1001 * 1001 {
		//move down
		pos += 2 * layer - 1;
		sum += pos;

		//move left
		pos += 2 * layer;
		sum += pos;

		//move up
		pos += 2 * layer;
		sum += pos;

		//move right
		pos += 2 * layer + 1;
		sum += pos - 1;
		layer += 1;
	}
	sum as i64
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 669171001);
	}
}
