use utils;

pub fn solve() -> i64 {
	let mut sum  : i64 = 0;

	let mut abundant : Vec<i64> = vec![];

	for i in 1..28124 {
		let f = utils::factors(i);
		let mut f_sum :i64 = 0;
		for j in 0..f.len()-1 {
			f_sum += f[j] as i64;
		}
		if f_sum > i {
			abundant.push(i);
		}

		let mut found = false;
		for j in abundant.iter() {
			let a = *j;
			if abundant.contains(&(i - a)) {
				found = true;
				break;
			}
		}
		if !found {
			sum += i as i64;
		}
	}
	 
	return sum;
}


