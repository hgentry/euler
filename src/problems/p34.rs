pub fn solve() -> i64 {
	let factorials: Vec<i64> = vec![1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
	let mut sum: i64 = 0;
	for i in 3..3628800 {
		let mut n: i64 = i;
		let mut i_sum: i64 = 0;
		while n >= 1 {
			let digit = n % 10;
			n = n / 10;
			i_sum += factorials[digit as usize];
		}
		if i_sum == i {
			sum += i_sum;
		}
	}
	sum
}
