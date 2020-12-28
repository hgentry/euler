pub fn is_prime(x: i64) -> bool {
	if x < 2 {
		return false;
	}
	let mut i = 2;
	let upper_limit = (x as f64).sqrt() as i64;
	while i <= upper_limit {
		if x % i == 0 {
			return false;
		}
		i += 1;
	}
	return true;
}

pub fn list_primes(max: i64) -> Vec<i64> {
	let mut sieve = vec![true; max as usize + 1];
	sieve[0] = false;
	sieve[1] = false;
	for i in 2..(max as f64).sqrt().floor() as i64 + 1 {
		for j in 2..max/i+1 {
			sieve[(i*j) as usize] = false;
		}
	}
	let mut primes = vec![];
	for i in 2..max as usize + 1 {
		if sieve[i] {
			primes.push(i as i64);
		}
	}
	primes
}
