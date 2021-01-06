pub fn solve_v1() -> i128 {
	let mut coins = vec![];
	let mut results: Vec<i128> = vec![0; 101];
	let val = 5;
	for i in 1..val {
		coins.push(i);
	}
	return changecounter(val, &coins, val as usize - 1, &mut results, 0);
}

pub fn changecounter(
	m: i64,
	coins: &Vec<i64>,
	c_e: usize,
	results: &mut Vec<i128>,
	depth: i64,
) -> i128 {
	if m == 0 {
		return 1;
	}
	if m == 1 {
		return 1;
	}
	let mut count = 0;
	for c1 in 0..c_e {
		let c = coins[c_e - c1 - 1];
		if m - c > 0 {
			let t = changecounter(m - c, coins, (c) as usize, results, depth + 1);
			count += t;
		}
		if m - c == 0 {
			count += 1;
		}
	}
	results[m as usize] = count;
	return count;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 190569291);
	}
}

pub fn solve() -> i64 {
	let mut p = vec![1];
	for n in 1..=100 {
		let mut n1 = -1;
		let mut sum = 0;
		for k in 1..=n {
			n1 *= -1;
			let a: i64 = (n - k * (3 * k - 1) / 2) as i64;
			let b: i64 = (n - k * (3 * k + 1) / 2) as i64;
			let p1;
			let p2;
			if a < 0 {
				p1 = 0;
			} else {
				p1 = p[a as usize];
			}
			if b < 0 {
				p2 = 0;
			} else {
				p2 = p[b as usize];
			}
			//println!("{} {}", a, b);
			sum += n1 * (p1 + p2);
		}
		//println!("{}: {}",n, sum);
		p.push(sum);
	}

	return p[100] - 1;
}
