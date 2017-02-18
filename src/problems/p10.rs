use utils;

pub fn solve() -> i64 {
	let mut sum = 0;
	for p in utils::fetch_primes_under_val(2000000) {
		sum += p;
	}
	sum
}
