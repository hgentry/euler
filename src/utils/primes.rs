use num::rational::*;

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
		for j in 2..max / i + 1 {
			sieve[(i * j) as usize] = false;
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

pub fn list_totients(n: i64) -> Vec<i64> {
	let mut totients = vec![];
	let primes = list_primes(n);
	for i in 1..=n {
		totients.push(Ratio::<i64>::from_integer(i));
	}

	for p in primes {
		let mut i = p;
		loop {
			if i >= totients.len() as i64 + 1 {
				break;
			}
			totients[i as usize - 1] = totients[i as usize - 1] * Ratio::<i64>::new(p - 1, p);

			i += p;
		}
	}

	let mut res = vec![];
	for t in totients {
		res.push(*t.numer());
	}

	return res;
}
