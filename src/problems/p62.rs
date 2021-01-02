use utils::math;

pub fn solve() -> i64 {
	let mut cubes: Vec<(i64, i64, i64)> = vec![];
	let desired_perms = 5;

	let mut pows = vec![1];
	for _ in 1..15 {
		let next = 10 * pows[pows.len() - 1];
		pows.push(next);
	}

	let mut i = 1;
	loop {
		let mut cube = i * i * i;
		let cube_init = cube;
		let mut cube_v = math::to_vec(&cube);
		cube_v.sort_by_key(|x| -x);
		cube = math::from_vec(&cube_v);

		let mut found = false;

		for j in 0..cubes.len() {
			if cubes[j].0 == cube {
				found = true;
				cubes[j].1 += 1;
				break;
			}
		}
		if !found {
			cubes.push((cube, 1, cube_init));
			while cubes[0].0 < cube_init {
				if cubes[0].1 == desired_perms {
					return cubes[0].2;
				}
				cubes.remove(0);
			}
		}
		i += 1;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 127035954683);
	}
}
