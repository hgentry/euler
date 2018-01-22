extern crate num_bigint;

use num_bigint::*;

pub fn factorial_big(x: i64) -> BigInt {
	let mut f = 1.to_bigint().unwrap();
	for i in 1.. x+1 {
		f = f*i.to_bigint().unwrap();
	}
	f
}

pub fn combination_big(n: i64, k: i64) -> BigInt {
	let mut f = permutation_big(n, k);

	for i in 1..n-k+1 {
		f = f/i.to_bigint().unwrap();
	}
	return f;
}

pub fn permutation_big(n: i64, k: i64) -> BigInt  {
	let mut f = 1.to_bigint().unwrap();
	for i in n-k+1..n+1 {
		f = f*i.to_bigint().unwrap();
	}
	f
}

pub fn permutation(n: i64, k: i64) -> i64  {
	let mut f = 1;
	for i in n-k+1..n+1 {
		f = f*i;
	}
	f
}

pub fn factors(x : i64) -> Vec<i64> {
	let mut v : Vec<i64> = vec![];
	let upper_limit = (x as f64).sqrt() as i64 + 1;
	v.push(1);
	for i in 2..upper_limit {
		//println!("this {}", i);
		if x % i == 0 && x != i {
			v.push(i);
			if x/i != i {
				v.push(x/i);
			}
		}
	}
	if x != 1 {
		v.push(x);
	}
	return v;
}

/*pub fn pow(x: i64, e: i64) -> i64 {
	let mut pow = 1;	
	for _ in 1..e+1 {
		pow = pow * x;
	}
	pow
}*/

pub fn pow_big(x: i64, e: i64) -> BigInt {
	let mut pow = 1.to_bigint().unwrap();	
	for _ in 1..e+1 {
		pow = pow * x.to_bigint().unwrap();
	}
	pow
}
