use crate::utils::math;

pub fn solve() -> i64 {
	let mut perm = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
	let mut bes: i64 = 0;
	loop {
		if perm.len() == 0 {
			break;
		}
		let tips = vec![10, perm[0], perm[1], perm[2], perm[3]];
		let mut min_tip = 11;
		let mut min_tip_index = 0;
		for t in 0..tips.len() {
			if tips[t] < min_tip {
				min_tip = tips[t];
				min_tip_index = t;
			}
		}
		let count = tips[(0 + min_tip_index) % 5] + perm[4] + perm[5];

		if tips[(1 + min_tip_index) % 5] + perm[5] + perm[6] != count {
			perm = math::next_permutation(&perm);
			continue;
		}

		if tips[(2 + min_tip_index) % 5] + perm[6] + perm[7] != count {
			perm = math::next_permutation(&perm);
			continue;
		}

		if tips[(3 + min_tip_index) % 5] + perm[8] + perm[7] != count {
			perm = math::next_permutation(&perm);
			continue;
		}

		if tips[(4 + min_tip_index) % 5] + perm[8] + perm[4] != count {
			perm = math::next_permutation(&perm);
			continue;
		}

		let s = format!(
			"{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
			tips[(0 + min_tip_index) % 5],
			perm[4],
			perm[5],
			tips[(1 + min_tip_index) % 5],
			perm[5],
			perm[6],
			tips[(2 + min_tip_index) % 5],
			perm[6],
			perm[7],
			tips[(3 + min_tip_index) % 5],
			perm[7],
			perm[8],
			tips[(4 + min_tip_index) % 5],
			perm[8],
			perm[4]
		);

		let n: i64 = s.parse().unwrap();
		if n > bes {
			bes = n;
		}

		perm = math::next_permutation(&perm);
		if perm.len() == 0 {
			break;
		}
	}

	return bes;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 6531031914842725);
	}
}
