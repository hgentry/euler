pub fn solve() -> i64 {
	let mut sum = 5;
	let mut primes = vec![2, 3];
	let mut checking = 5;
	while checking < 2000000 {
		let upper_limit = (checking as f64).sqrt() as i64 + 1;
		let mut found = false;
		let len = primes.len();
		for i in 0..len {
			let p = primes[i];
			if p > upper_limit {
				found = true;
				break;
			} else {
				if checking % p == 0 {
					break;
				}
			}
			if i == len - 1 {
				found = true;
				break;
			}
		}
		if found {
			sum += checking;
			primes.push(checking);
		}
		checking += 2;
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 142913828922);
	}
}
