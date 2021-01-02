extern crate num;

use num::bigint::*;
use num::rational::*;
use num::traits::*;
use std::str::FromStr;
use std::vec;
use utils::primes;

pub fn factorial_big<T: num::Integer + Clone + CheckedAdd + FromPrimitive + ToBigInt>(x: &T) -> BigInt {
	let mut f = 1.to_bigint().unwrap();
	for i in num::range_step::<T>(One::one(),x.clone() + One::one(),One::one()) {
		f = f * i.to_bigint().unwrap();
	}
	f
}

pub fn combination_big<T: num::Integer + Clone + CheckedAdd + FromPrimitive + ToBigInt>(n: &T, k: &T) -> BigInt {
	let mut f = permutation_big(n, k);

	for i in num::range_step::<T>(One::one(),n.clone() - k.clone() + One::one(),One::one()) {
		f = f / i.to_bigint().unwrap();
	}
	return f;
}

pub fn permutation_big<T: num::Integer + Clone + CheckedAdd + FromPrimitive + ToBigInt>(n: &T, k: &T) -> BigInt {
	let mut f = 1.to_bigint().unwrap();
	for i in num::range_step::<T>(n.clone() - k.clone() + One::one(), n.clone() + One::one(), One::one()) {
		f = f * i.to_bigint().unwrap();
	}
	f
}

pub fn permutation(n: i64, k: i64) -> i64 {
	let mut f = 1;
	for i in n - k + 1..n + 1 {
		f = f * i;
	}
	f
}

pub fn factors(x: i64) -> Vec<i64> {
	let mut v: Vec<i64> = vec![];
	let upper_limit = (x as f64).sqrt() as i64 + 1;
	v.push(1);
	for i in 2..upper_limit {
		if x % i == 0 && x != i {
			v.push(i);
			if x / i != i {
				v.push(x / i);
			}
		}
	}

	if x != 1 {
		v.push(x);
	}
	return v;
}

pub fn factors_i128(x: i128) -> Vec<i128> {
	let mut v: Vec<i128> = vec![];
	let upper_limit = (x as f64).sqrt() as i128 + 1;
	v.push(1);
	for i in 2..upper_limit {
		if x % i == 0 && x != i {
			v.push(i);
			if x / i != i {
				v.push(x / i);
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
	if x / upper_limit == upper_limit {
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
			if x / i != i && primes::is_prime(x / i) {
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
	for _ in 1..e + 1 {
		pow = pow * x;
	}
	pow
}

pub fn pow_big(x: i64, e: i64) -> BigInt {
	let mut pow = 1.to_bigint().unwrap();
	for _ in 1..e + 1 {
		pow = pow * x.to_bigint().unwrap();
	}
	pow
}

pub fn triangle(n: i64) -> i64 {
	n * (n - 1) / 2
}

pub fn next_permutation(input: &Vec<i64>) -> Vec<i64> {
	let mut v = input.clone();
	let mut k0: i64 = -1;

	for i in 0..v.len() - 1 {
		if v[i] < v[i + 1] {
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
		v.swap(i as usize, (len - (i - k0) as usize) as usize);
		i += 1;
	}
	v
}

pub fn next_permutation_in_place(mut v: Vec<i64>) -> Vec<i64> {
	let mut k0: i64 = -1;

	for i in 0..v.len() - 1 {
		if v[i] < v[i + 1] {
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
		v.swap(i as usize, (len - (i - k0) as usize) as usize);
		i += 1;
	}
	v
}

pub fn next_permutation_in_place_2(v: &mut Vec<i64>) {
	let mut k0: i64 = -1;

	for i in 0..v.len() - 1 {
		if v[i] < v[i + 1] {
			k0 = i as i64;
		}
	}
	if k0 == -1 {
		v.drain(0..v.len());
		return;
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
		v.swap(i as usize, (len - (i - k0) as usize) as usize);
		i += 1;
	}
}

pub fn prev_permutation(input: &Vec<i64>) -> Vec<i64> {
	let mut v = input.clone();
	let mut k0: i64 = -1;

	for i in 0..v.len() - 1 {
		if v[i] > v[i + 1] {
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
		v.swap(i as usize, (len - (i - k0) as usize) as usize);
		i += 1;
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

pub fn quadratic(a: f64, b: f64, c: f64) -> Vec<f64> {
	let x1 = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
	let x2 = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
	return vec![x1, x2];
}

pub fn is_triangle(x: i64) -> bool {
	let roots = quadratic(0.5, 0.5, (0 - x) as f64);
	if roots[0] == roots[0].floor() {
		return true;
	}
	return false;
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

pub fn extended_euclidean(a: u64, b: u64) -> (i64, i64) {
	let ee = extended_euclidean_recurse(a as i64, b as i64, 0, 0);
	return (ee.2, ee.3);
}

fn extended_euclidean_recurse(a: i64, b: i64, mut x: i64, mut y: i64) -> (i64, i64, i64, i64) {
	if a == 0 {
		x = 0;
		y = 1;
		return (a, b, x, y);
	}

	let gcd = extended_euclidean_recurse(b % a, a, x, y);
	let x1 = gcd.2;
	let y1 = gcd.3;

	x = y1 - (b / a) * x1;
	y = x1;

	return (a, b, x, y);
}

pub fn from_vec(v: &Vec<i64>) -> i64 {
	let mut sum = 0;
	let mut mult = 1;
	for i in 0..v.len() {
		sum += v[v.len() - i - 1] * mult;
		mult *= 10;
	}
	return sum;
}

pub fn from_sorted_vec_with_zeroes(v: &Vec<i64>) -> i64 {
	let mut sum = 0;
	let mut mult = 1;
	for i in 0..v.len() {
		if v[v.len() - i - 1] == 0 {
			sum *= 10;
		} else {
			sum += v[v.len() - i - 1] * mult;
		}
		mult *= 10;
	}
	return sum;
}

pub fn to_vec<T: num::Integer + Clone + FromPrimitive>(x1: &T) -> Vec<T> {
	let mut x = x1.clone();
	let ten = T::from_i64(10).unwrap();
	let mut ret = vec![];
	while x > Zero::zero() {
		ret.insert(0, x.clone() % ten.clone());
		x = x.clone() / ten.clone();
	}
	return ret;
}

pub fn is_palindrome<T: num::Integer + Clone + std::fmt::Display>(x: &T) -> bool {
	let x_str: String = format!("{}", x);
	let x_reversed: String = x_str.chars().rev().collect();
	return x_str == x_reversed;
}

pub fn reverse<T: num::Integer + Clone + std::fmt::Display + FromStr + std::fmt::Debug>(x: &T) -> T
where
	<T as FromStr>::Err: std::fmt::Debug,
{
	let x_str: String = format!("{}", x);
	let x_reversed: String = x_str.chars().rev().collect();
	return x_reversed.parse().unwrap();
}

pub fn next_permutation_i64(x: i64, pows: &Vec<i64>) -> i64 {
	let mut k0: i64 = -1;

	for i in 0..pows.len() - 1 {
		if x / pows[pows.len() - i - 1] % 10 < x / pows[pows.len() - (i + 1) - 1] % 10 {
			k0 = i as i64;
		}
	}
	if k0 == -1 {
		return 0;
	}

	let mut l0 = k0 + 1;
	for i in k0 + 1..pows.len() as i64 {
		if x / pows[pows.len() - 1 - k0 as usize] % 10 < x / pows[pows.len() - 1 - i as usize] % 10
		{
			l0 = i;
		}
	}
	//swap k0, l0
	let mut x1 = x;
	x1 = swap_i64(x1, k0, l0, &pows);

	let mut i = k0 + 1;
	let len = pows.len() as i64;
	while i < pows.len() as i64 - (i - k0) + 1 {
		//swap i, v.len() - (i-k0)
		x1 = swap_i64(x1, i, len - (i as i64 - k0), &pows);
		i += 1;
	}
	return x1;
}

pub fn swap_i64(x: i64, k0: i64, l0: i64, pows: &Vec<i64>) -> i64 {
	let mut x1 = x;
	let k = x / pows[pows.len() - 1 - k0 as usize] % 10;
	let l = x / pows[pows.len() - 1 - l0 as usize] % 10;
	x1 -= k * pows[pows.len() - 1 - k0 as usize];
	x1 -= l * pows[pows.len() - 1 - l0 as usize];
	x1 += k * pows[pows.len() - 1 - l0 as usize];
	x1 += l * pows[pows.len() - 1 - k0 as usize];
	return x1;
}

pub fn add_ratios_carelessly<T: num::Integer + Clone>(a: Ratio<T>, b: Ratio<T>) -> Ratio<T> {
	return Ratio::new_raw(
		a.numer().clone() * b.denom().clone() + b.numer().clone() * a.denom().clone(),
		a.denom().clone() * b.denom().clone(),
	);
}

pub fn sum_digits<T: num::Integer + Clone + FromPrimitive>(x1: &T) -> T {
	let mut x = x1.clone();
	let mut sum = Zero::zero();
	let ten = T::from_i64(10).unwrap();
	while x != Zero::zero() {
		let mut next = x.clone() % ten.clone();
		if next < Zero::zero() {
			next = next.clone() - next.clone() - next.clone();
		}
		sum = sum + next;
		x = x / ten.clone();
	}
	return sum;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_add_ratios_carelessly_1() {
		let add = add_ratios_carelessly(Ratio::new(7, 5), Ratio::new(3, 20));
		assert_eq!(*add.numer(), 155);
		assert_eq!(*add.denom(), 100);
	}

	#[test]
	fn test_add_ratios_carelessly_2() {
		let add = add_ratios_carelessly(Ratio::new_raw(0, 5), Ratio::new(3, 20));
		assert_eq!(*add.numer(), 15);
		assert_eq!(*add.denom(), 100);
	}

	#[test]
	fn test_add_ratios_carelessly_3() {
		let add = add_ratios_carelessly(
			Ratio::new(7.to_bigint().unwrap(), 5.to_bigint().unwrap()),
			Ratio::new(3.to_bigint().unwrap(), 20.to_bigint().unwrap()),
		);
		assert_eq!(*add.numer(), 155.to_bigint().unwrap());
		assert_eq!(*add.denom(), 100.to_bigint().unwrap());
	}

	#[test]
	fn test_add_ratios_carelessly_4() {
		let add = add_ratios_carelessly(
			Ratio::new_raw(0.to_bigint().unwrap(), 5.to_bigint().unwrap()),
			Ratio::new(3.to_bigint().unwrap(), 20.to_bigint().unwrap()),
		);
		assert_eq!(*add.numer(), 15.to_bigint().unwrap());
		assert_eq!(*add.denom(), 100.to_bigint().unwrap());
	}

	#[test]
	fn test_sum_digits_1() {
		assert_eq!(sum_digits(&-3), 3);
	}

	#[test]
	fn test_sum_digits_2() {
		assert_eq!(sum_digits(&40056), 15);
	}

	#[test]
	fn test_sum_digits_3() {
		assert_eq!(sum_digits(&-3112), 7);
	}

	#[test]
	fn test_sum_digits_4() {
		assert_eq!(sum_digits(&15.to_bigint().unwrap()), 6.to_bigint().unwrap());
	}

	#[test]
	fn test_sum_digits_5() {
		assert_eq!(
			sum_digits(&(-15).to_bigint().unwrap()),
			6.to_bigint().unwrap()
		);
	}

	#[test]
	fn test_sum_digits_6() {
		assert_eq!(
			sum_digits(&(0).to_bigint().unwrap()),
			0.to_bigint().unwrap()
		);
	}

	#[test]
	fn test_sum_digits_7() {
		assert_eq!(sum_digits(&0), 0);
	}

	#[test]
	fn test_is_palindrome_1() {
		assert_eq!(is_palindrome(&0), true);
	}

	#[test]
	fn test_is_palindrome_2() {
		assert_eq!(is_palindrome(&31), false);
	}

	#[test]
	fn test_is_palindrome_3() {
		assert_eq!(is_palindrome(&34343), true);
	}

	#[test]
	fn test_is_palindrome_4() {
		assert_eq!(is_palindrome(&(0.to_bigint().unwrap())), true);
	}

	#[test]
	fn test_is_palindrome_5() {
		assert_eq!(is_palindrome(&(31.to_bigint().unwrap())), false);
	}

	#[test]
	fn test_is_palindrome_6() {
		assert_eq!(is_palindrome(&(34343.to_bigint().unwrap())), true);
	}

	#[test]
	fn test_reverse_1() {
		assert_eq!(reverse(&1), 1);
	}

	#[test]
	fn test_reverse_2() {
		assert_eq!(reverse(&21), 12);
	}
}
