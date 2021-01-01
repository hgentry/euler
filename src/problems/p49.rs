use utils::math;
use utils::primes;
//296962999629
//37937097039

pub fn solve() -> String {
	let primes = primes::list_primes(10000);
	for p in primes {
		let p_vec = math::to_vec(&p);
		let mut factory = PandigitalFactory { current: p_vec };
		let mut j = factory.get();
		let init_j = j;
		let mut primes_found = vec![];
		let mut prime_str = "".to_string();
		while j != 0 {
			if primes::is_prime(j)
				&& (j - init_j == 0 || j - init_j == 3330 || j - init_j == 6660)
				&& j != 1487 && j != 4817
				&& j != 8147
			{
				primes_found.push(j);
				prime_str = format!("{}{}", prime_str, j);
			}

			j = factory.inc().get();
		}
		if primes_found.len() == 3 {
			return prime_str;
		}
	}

	return "".to_string();
}

struct PandigitalFactory {
	current: Vec<i64>,
}

impl PandigitalFactory {
	pub fn dec(&mut self) -> &PandigitalFactory {
		let mut k0: i64 = -1;
		for i in 0..self.current.len() - 1 {
			if self.current[i] < self.current[i + 1] {
				k0 = i as i64;
			}
		}
		if k0 == -1 {
			self.current = vec![];
			return self;
		}
		let mut l0: i64 = k0 + 1;
		for i in 0..self.current.len() {
			if self.current[k0 as usize] > self.current[i] {
				l0 = i as i64;
			}
		}
		self.swap(k0, l0);

		let mut i = k0 + 1;
		loop {
			if i as i64 >= self.current.len() as i64 - (i - k0) {
				break;
			}

			self.swap(i as i64, self.current.len() as i64 - (i - k0));

			i += 1;
		}

		return self;
	}

	pub fn inc(&mut self) -> &PandigitalFactory {
		let mut k0: i64 = -1;
		for i in 0..self.current.len() - 1 {
			if self.current[i] < self.current[i + 1] {
				k0 = i as i64;
			}
		}
		if k0 == -1 {
			self.current = vec![];
			return self;
		}
		let mut l0: i64 = k0 + 1;
		for i in k0 as usize + 1..self.current.len() {
			if self.current[k0 as usize] < self.current[i] {
				l0 = i as i64;
			}
		}
		self.swap(k0, l0);

		let mut i = k0 + 1;
		loop {
			if i as i64 >= self.current.len() as i64 - (i - k0) {
				break;
			}

			self.swap(i as i64, self.current.len() as i64 - (i - k0));

			i += 1;
		}

		return self;
	}

	pub fn get(&self) -> i64 {
		let mut out = "".to_string();
		for i in 0..self.current.len() {
			out = format!("{}{}", out, self.current[i]);
		}
		if self.current.len() == 0 {
			return 0;
		}
		return out.parse().unwrap();
	}

	pub fn swap(&mut self, a: i64, b: i64) {
		let x = self.current[a as usize];
		self.current[a as usize] = self.current[b as usize];
		self.current[b as usize] = x;
	}
}
