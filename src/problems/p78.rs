pub fn solve() -> i64 {
	let mut p = vec![1];
	for n in 1.. {
		let mut n1 = -1;
		let mut sum: i64 = 0;
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
		sum %= 1000000;
		p.push(sum);
		if sum == 0 {
			return n;
		}
	}

	return 0;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[ignore]
	fn correct() {
		assert_eq!(solve(), 0);
	}
}
