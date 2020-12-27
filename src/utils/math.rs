extern crate num_bigint;

use num_bigint::*;
use std::vec;
use utils::primes;

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

pub fn factors_count_prime(x: i64) -> i64 {
	let mut count: i64 = 0;
	let upper_limit = (x as f64).sqrt() as i64;
	for i in 2..upper_limit {
		if x % i == 0 {
			if primes::is_prime(i) {
				//println!("{} {}", x, i);
				count += 1;
			}
			if x/i != i && primes::is_prime(x/i) {
				count += 1;
			}
		}
	}
	if x != 1 && primes::is_prime(x) {
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

pub fn next_permutation(input: &Vec<i64>) -> Vec<i64> {
	let mut v = input.clone();
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

pub fn next_permutation_in_place(mut v: Vec<i64>) -> Vec<i64> {
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


pub fn prev_permutation(input: &Vec<i64>) -> Vec<i64> {
	let mut v = input.clone();
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

pub fn quadratic(a: f64,b: f64,c: f64) -> Vec<f64> {
		let x1 = (-b + (b*b - 4.0*a*c).sqrt())/(2.0*a);
		let x2 = (-b + (b*b - 4.0*a*c).sqrt())/(2.0*a);
		return vec![x1,x2];
}

pub fn is_triangle(x: i64) -> bool{
	let roots = quadratic(0.5, 0.5, (0-x) as f64);
		if roots[0] == roots[0].floor() {
				return true;
		}
		return false;
}


pub fn to_num(v : &Vec<i64>) -> i64 {
    let mut sum = 0;
    for i in 0..v.len() {
        sum += pow(10, i as i64)*v[(v.len() - i - 1) as usize];
    }
    sum
}

pub fn num_digits(mut x: i64) -> i64 {
    let mut len = 0;
    loop {
        if x / 10 > 0 {
            len += 1;
            x /= 10;
        } else {
            len += 1;
            break;
        }
    }
    return len;
}

pub fn num_digits_bigint(x: BigInt) -> i64 {
    let len: i64;
	let arr = x.to_string();
	len = arr.len() as i64;
    return len + 0;
}

pub fn reduce_fraction(mut a : i64, mut b : i64) -> (i64, i64) {
    let va = factors(a);
    let vb = factors(b);
    let mut done = true;
    loop {
        for f in &va {
            if *f > 1 && a % *f == 0 && b % *f == 0 {
                a /= *f;
                b /= *f;
                done = false;
            }
        }
        for f in &vb {
            if *f > 1 && a % *f == 0 && b % *f == 0 {
                a /= *f;
                b /= *f;
                done = false;
            }
        }
        if done {
            break;
        }
        done = true;
    }
    return (a,b);
}

pub fn max(a: i64, b: i64) -> i64{
	if a > b {
		return a;
	} else {
		return b;
	}
}

pub fn extended_euclidean(a: u64, b: u64) -> (i64, i64) {
	let ee = extended_euclidean_recurse(a as i64, b as i64, 0, 0);
	return (ee.2, ee.3);
}

fn extended_euclidean_recurse(a: i64, b: i64, mut x: i64, mut y: i64) -> (i64, i64, i64, i64){
	if a == 0 {
		x = 0;
		y = 1;
		return (a, b, x, y);
	}

	let gcd = extended_euclidean_recurse(b%a, a, x, y);
	let x1 = gcd.2;
	let y1 = gcd.3;

	x = y1 - (b/a) * x1;
	y = x1;

	return (a, b, x, y);
}

pub fn from_vec(v: Vec<i64>) -> i64 {
	let mut sum = 0;
	let mut mult = 1;
	for i in 0..v.len() {
		sum += v[v.len()-i-1] * mult;
		mult *= 10;
	}
	return sum;
}