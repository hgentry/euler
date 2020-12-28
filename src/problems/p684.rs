//922058210

pub fn solve() -> i64 {
	//return ssol(20);
	let mut f0 = 0;
	let mut f1 = 1;
	let mut f2;
	let mut sum = 0;
	for _ in 2..91 {
		f2 = f1 + f0;
		f0 = f1;
		f1 = f2;
		sum += ssol(f2);
	}
	return sum % 1000000007;
}

pub fn ssol(mut n: i64) -> i64 {
	let mut sum = 0;
	let mut pow = 1;
	let mut ten = 1;
	let m = 1000000007;
	let c = 9000000000;
	let div = c % m;
	
	while n > 9 && pow <= 10 {
		let mult = n - 9;
		if pow > 10 {
			sum += (mult * 9 + 45) * div % m;
		} else {
			sum += (mult * 9 + 45) * ten % m;
			ten *= 10;
		}
		n -= 9;
		pow += 1;
	}
	if n > 9 {
		// Find remaining number of nines
		// 9* sum(n-n%9) + (9-n%9) * (n/9 + 1)
		let mult =  9* summ(n-n%9) + (9-n%9) * (n/9 + 1);
		sum += (mult + 45*(n-9)) * div % m;
	}
	
	 
		sum += (n*(n+1)/2) % m;
	
	
	return sum % m;
}


pub fn summ(x: i64) -> i64 {
	return x * (x+1) / 2;
}