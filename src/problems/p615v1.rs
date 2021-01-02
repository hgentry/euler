use crate::utils;

pub fn solve() -> i64 {
	let primes = utils::primes::list_primes();
	let mut printed = 1;
	let limit = 12;
	let printout = 100;
	let mut max_change_length = 0;
	for i in 2..10000000 {
		let factors = list_factors(i, &primes);
		let mut s: String =  format!("{:>7} (", printed);
		let len = factors.len();
		let mut modifier = 1.0;
		let mut change_length = 0;
		let mut pre_limit = false;
		let mut post_limit = false;
		if len >= limit {
			for f in 0..len {
				if f < limit {
					modifier *= (factors[f] as f64)/2.0;
					if factors[f] != 2 {
						pre_limit = true;
					}
				} else {
					modifier *= factors[f] as f64;
					post_limit = true;
				}


				if f < limit && factors[f] == 2 {
					s = format!("{}{}",s,"  ");
				} else {
					change_length += 1;
					s = format!("{}{:>2}",s,factors[f]);
				}
				if f != len - 1 {
					s = format!("{},",s);
				} else {
					//s = format!("{}) {}",s, i/utils::math::pow(2,(limit as f64).log(2.0).floor() as i64 + 2));
					//s = format!("{}) {}",s, i/utils::math::pow(2,limit as i64 - 2));
					//s = format!("{}) {}",s, i/utils::math::pow(2,limit as i64 - (printed as f64 + 1.0).log(2.0) as i64));
					s = format!("{})", s);
				}

				if change_length > max_change_length {
					max_change_length = change_length;
				}
			}
			if pre_limit && post_limit {
				//let output = format!("{} -- {} -- {} -- {}",printed,modifier, (i as f64).log(2.0).floor(), max_change_length);
				let output = format!("{} -- {}",printed, s);
				println!("{}",output);
				printed += 1;
				if printed > printout {
					break;
				}
			}
		}
	}
	0
}

pub fn list_factors(to_factor: i64, primes: &Vec<i64>) -> Vec<i64> {
	let mut primes_iter = primes.iter();
	let mut n = to_factor;
	let mut p = *primes_iter.next().unwrap();
	let mut list = vec!();
	while n != 1 {
	//	println!("{}",n);
		if n % p == 0 {
			list.push(p);
			n /= p;
		} else {
			match primes_iter.next() {
                Some(val) => p = *val,
                None => break,
            };
		}
	}
	list
}
