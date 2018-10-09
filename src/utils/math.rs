extern crate num_bigint;

use num_bigint::*;
use std::vec;

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

pub fn factors_count(x: i64) -> i64 {
	let mut count: i64 = 1;
	let upper_limit = (x as f64).sqrt() as i64;
	for i in 2..upper_limit {
		if x % i == 0 {
			count += 2;
		}
	}
	if x/upper_limit == upper_limit {
		count -= 1;
	}
	if x != 1 {
		count += 1;
	}
	count
}

pub fn pow(x: i64, e: i64) -> i64 {
	let mut pow = 1;
	if e == 0 {
		return 1;
	}
	for _ in 1..e+1 {
		pow = pow * x;
	}
	pow
}

pub fn pow_big(x: i64, e: i64) -> BigInt {
	let mut pow = 1.to_bigint().unwrap();
	for _ in 1..e+1 {
		pow = pow * x.to_bigint().unwrap();
	}
	pow
}

pub fn triangle(n: i64) -> i64 {
	n*(n-1)/2
}

pub fn next_permutation(mut v: Vec<i64>) -> Vec<i64> {
	let mut k0:i64 = -1;

	for i in 0..v.len()-1 {
		if v[i] < v[i+1] {
			k0 = i as i64;
		}
	}
	if k0 == -1 {
		return vec![];
	}


	let mut l0 = k0 + 1;
	for i in k0 + 1..v.len() as i64 {
		if v[k0 as usize] < v[i as usize] {
			l0 = i;
		}
	}
	//swap k0, l0
	v.swap(k0 as usize, l0 as usize);
	let mut i = k0 + 1;
	let len = v.len();
	while i < v.len() as i64 - (i - k0) {
		//swap i, v.len() - (i-k0)
		v.swap(i as usize, (len - (i-k0) as usize) as usize);
		i+=1;
	}
	v
}

pub fn prev_permutation(mut v: Vec<i64>) -> Vec<i64> {
	let mut k0:i64 = -1;

	for i in 0..v.len()-1 {
		if v[i] > v[i+1] {
			k0 = i as i64;
		}
	}
	if k0 == -1 {
		return vec![];
	}


	let mut l0 = k0 + 1;
	for i in k0 + 1..v.len() as i64 {
		if v[k0 as usize] > v[i as usize] {
			l0 = i;
		}
	}
	//swap k0, l0
	v.swap(k0 as usize, l0 as usize);
	let mut i = k0 + 1;
	let len = v.len();
	while i < v.len() as i64 - (i - k0) {
		//swap i, v.len() - (i-k0)
		v.swap(i as usize, (len - (i-k0) as usize) as usize);
		i+=1;
	}
	v
}

pub fn is_pandigital(mut x: i64, digits: &Vec<i64>) -> bool {
	let mut arr: Vec<i64> = vec![];
	while x >= 1 {
		let m = x % 10;
		if arr.contains(&m) || !digits.contains(&m) {
			return false;
		}
		arr.push(m);
		x /= 10;
	}
	if arr.len() == digits.len() {
		true
	} else {
		false
	}
}
