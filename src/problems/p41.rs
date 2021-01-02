use crate::utils::math;
use crate::utils::primes;
pub fn solve() -> i64 {
	let mut perm = vec![7, 6, 5, 4, 3, 2, 1];
	while !primes::is_prime(to_num(&perm)) {
		perm = math::prev_permutation(&perm);
	}
	to_num(&perm)
}

pub fn to_num(v: &Vec<i64>) -> i64 {
	let mut sum = 0;
	for i in 0..v.len() {
		sum += math::pow(10, i as i64) * v[(v.len() - i - 1) as usize];
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 7652413);
	}
}
