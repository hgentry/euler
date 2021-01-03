use crate::utils::math;
use num::bigint::*;
use num::rational::*;
use num::traits::One;
use num::traits::Zero;

pub fn solve() -> i64 {
	//brute force

	let mut squares = vec![0, 1, 4, 9, 16];
	for i in 5..=5000000 {
		squares.push(i * i);
	}

	let mut best_v: BigInt = Zero::zero();
	let mut best_i = 0;

	for d in 2..=1000 {
		if squares.contains(&(d)) {
			continue;
		}
		let mut len = 1;
		loop {
			let seq = expand_root_len(d, len);
			let r = expanded_sum(seq);
			if r.numer() * r.numer() - r.denom() * r.denom() * d == One::one() {
				if r.numer().clone() > best_v {
					best_v = r.numer().clone();
					best_i = d;
				}
				break;
			}

			len += 1;
		}
	}

	return best_i;
}

pub fn expand_root_len(x: i64, len: i64) -> Vec<i64> {
	let mut a0;
	let mut numer;
	let mut denom = 1;
	let mut fact;
	let mut sequence = vec![];

	a0 = (x as f64).sqrt() as i64;
	sequence.push(a0);
	fact = a0;
	for _ in 1..len {
		numer = denom;
		denom = x - fact * fact;
		a0 = (numer as f64 * ((x as f64).sqrt() + (fact) as f64) / (denom as f64)) as i64;
		sequence.push(a0);
		fact = fact * numer;
		fact -= a0 * denom;

		fact /= numer;
		denom /= numer;
		fact = fact.abs();
	}

	return sequence;
}

pub fn expanded_sum(sequence: Vec<i64>) -> BigRational {
	let i = sequence.len() - 1;
	let mut temp: BigRational = Ratio::from_integer(sequence[i].to_bigint().unwrap());
	for j in 0..i {
		temp = Ratio::new_raw(temp.denom().clone(), temp.numer().clone());
		temp = math::add_ratios_carelessly(
			temp,
			Ratio::from_integer(sequence[i - j - 1].to_bigint().unwrap()),
		);
		//temp = temp + Ratio::from_integer(sequence[i-j-1].to_bigint().unwrap());
	}
	return temp;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 661);
	}
}
