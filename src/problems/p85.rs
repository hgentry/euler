pub fn solve() -> i64 {
	let mut closest_a = 0;
	let mut closest_v = 6000000;
	for i in 1..2000000 {
		let mut done = false;
		for j in 1..2000000 {
			let mut sum = 0;
			for k in -0..i {
				for l in -0..j {
					sum += (i - k) * (j - l);
				}
			}
			//println!("{},{} => {}", i, j, sum);
			if num::abs(2000000 - sum) < closest_v {
				closest_v = num::abs(2000000 - sum);
				closest_a = i * j;
			}
			if sum > 2000200 {
				if j == 1 {
					done = true;
				}
				break;
			}
		}
		if done {
			break;
		}
	}
	return closest_a;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 2772);
	}
}
