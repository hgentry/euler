use std::collections::HashMap;

pub fn solve() -> i64 {
	let mut count = 0;
	let mut solved = HashMap::new();
	for i in 1..1000000 {
		let mut chain = vec![i];
		let mut c = i as i64;
		loop {
			c = digitorial(c);
			if solved.contains_key(&c) {
				let clen = chain.len() + solved.get(&c).unwrap();
				for k in 0..chain.len() {
					if !solved.contains_key(&chain[k]) {
						solved.insert(chain[k] as i64, clen - k);
					}
				}
				if clen == 60 {
					count += 1;
				}
				break;
			}
			if chain.contains(&c) {
				if chain.len() == 60 {
					count += 1;
				}
				for k in 0..chain.len() {
					if !solved.contains_key(&chain[k]) {
						solved.insert(chain[k] as i64, chain.len() - k);
					}
				}
				break;
			}
			chain.push(c);
			if chain.len() > 10000 {
				println!("{}", i);
			}
		}
	}
	return count;
}

pub fn digitorial(x: i64) -> i64 {
	let df = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
	let mut manip = x;
	let mut sum = 0;
	while manip != 0 {
		sum += df[(manip % 10) as usize];
		manip /= 10;
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 402);
	}
}
