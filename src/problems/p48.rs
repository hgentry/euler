pub fn solve() -> i64 {
	let mut sum = 0;
	for i in 1..=1000 {
		let mut product = 1;
		for _ in 1..=i {
			product = (product * i) % 10000000000;
		}
		sum = (sum + product) % 10000000000;
	}
	return sum;
}
