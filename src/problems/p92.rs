pub fn solve() -> i64 {
	let mut count = 0;
	for i in 1..10000000 {
		let mut entries = vec![];
		entries.push(i);
		let mut val = i;
		loop {
			val = digit_squares_sum(val);
			if val == 89 {
				count += 1;
				break;
			}
			if entries.contains(&val) {
				break;
			}
			entries.push(val);
		}
	}
	return count;
}

fn digit_squares_sum(x: i64) -> i64 {
	let mut c = x;
	let mut sum = 0;
	while c > 0 {
		sum += (c % 10) * (c % 10);
		c /= 10;
	}
	return sum;
}
