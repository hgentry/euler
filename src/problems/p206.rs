pub fn solve() -> i64 {
	let mut num = 99999999;
	loop {
		let val = build_num(num);
		if (val as f64).sqrt() == (val as f64).sqrt().floor() {
			if val % ((val as f64).sqrt() as i64) == 0 {
				return (val as f64).sqrt() as i64 * 10;
			}
		}
		num -= 1;
	}
}

pub fn build_num(x: i64) -> i64 {
	let mut sum = 0;
	sum += 10203040506070809;
	sum += x / 1 % 10 * 10;
	sum += x / 10 % 10 * 1000;
	sum += x / 100 % 10 * 100000;
	sum += x / 1000 % 10 * 10000000;
	sum += x / 10000 % 10 * 1000000000;
	sum += x / 100000 % 10 * 100000000000;
	sum += x / 1000000 % 10 * 10000000000000;
	sum += x / 10000000 % 10 * 1000000000000000;
	return sum;
}
