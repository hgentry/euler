use utils;

pub fn solve() -> i64 {
	let mut sum = 0;
	for i in 1..10000 {
		let i_friend = amicability(i);
		if amicability(i_friend) == i && i_friend != i {
			sum += i;
		}
	}
	sum
}

pub fn amicability(x : i64) -> i64 {
	let factors = utils::math::factors(x);
	let mut f_sum = 0;
	for factor in factors {
		f_sum += factor;
	}
	if x > 1 {
		f_sum -= x;
	}
	f_sum
}
