use utils;
use std::collections::HashMap;

pub fn solve() -> i64 {
	let mut sum  : i64 = 0;

	let mut abundant = HashMap::new();
    let mut index = 0;
	for i in 1..28124 {
		let f = utils::math::factors(i);
		let mut f_sum :i64 = 0;
		for j in 0..f.len()-1 {
			f_sum += f[j] as i64;
		}
		if f_sum > i {
			abundant.insert(i, index);
            index += 1;
		}

		let mut found = false;
		for (k,_) in abundant.iter() {
			match abundant.get(&(i - k)) {
                Some(_) => {
				    found = true;
				    break;
                },
                None => continue
			}
		}
		if !found {
			sum += i as i64;
		}
        //println!("{}", i);
	}
	 
	return sum;
}


