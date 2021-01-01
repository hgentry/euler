use utils::math;

pub fn solve() -> i64 {
	let mut consecutive = 0;
	for i in 5..150000 {
		let len = math::factors_count_prime(i as i64);
		if len >= 4 {
			consecutive += 1;
		} else {
			consecutive = 0;
		}
		if consecutive == 4 {
			return i - 3;
		}
	}

	return 0;
}
