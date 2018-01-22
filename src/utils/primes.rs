pub fn is_prime(x: i64) -> bool {
	let mut i = 2;
	let upper_limit = (x as f64).sqrt() as i64 + 1;
	while i < upper_limit {
		if x % i == 0 {
			return false;
		}
		i += 1;
	}
	return true;
}
