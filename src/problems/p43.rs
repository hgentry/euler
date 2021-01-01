use utils::math;

pub fn solve() -> i64 {
	let mut sum = 0;
	let primes = [2, 3, 5, 7, 11, 13, 17];
	let mut seventeen = 17;
	while seventeen < 1000 {
		let mut digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
		let sv = to_vec(seventeen);
		for i in 0..sv.len() {
			let index = (digits).iter().position(|&x| x == sv[i]);
			match index {
				Some(_i) => {}
				None => {
					continue;
				}
			}
			digits.remove(index.unwrap());
		}
		if sv.len() == 2 {
			digits.remove(0);
		}
		if digits.len() > 7 {
			seventeen += 17;
			continue;
		}

		while digits != vec![] {
			digits = math::next_permutation_in_place(digits);
			let test = math::to_num(&digits) * 1000 + seventeen;
			let mut n = test;
			let mut valid = true;
			for i in 0..7 {
				if (n % 1000) % primes[6 - i] != 0 {
					valid = false;
					break;
				}
				n /= 10;
			}
			if valid {
				sum += test;
			}
		}

		seventeen += 17;
	}
	sum
}

pub fn to_vec(x: i64) -> Vec<i64> {
	let mut result = vec![];
	let mut n = x;
	while n > 0 {
		result.push(n % 10);
		n /= 10;
	}
	result.reverse();
	result
}
