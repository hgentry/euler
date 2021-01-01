use utils::math;

pub fn solve() -> i64 {
	let big = math::factorial_big(100);
	let s = format!("{}",big);
	let mut sum = 0;
	for c in s.chars() {
			sum += c.to_digit(10).unwrap() as i64;
	}
	return sum;
}
