use crate::utils::math;

//Good solution
pub fn solve() -> i64 {
	let mut count = 0;
	let limit: i64 = 1500000;

	let mut salud = vec![0; limit as usize + 1];

	for n in 1..613 {
		let mut m = n + 1;
		loop {
			if 2 * m * (m + n) > limit {
				break;
			}
			if math::gcd(n, m) == 1 {
				let mut k = 1;
				while 2 * k * m * (m + n) <= limit {
					salud[(2 * k * m * (m + n)) as usize] += 1;
					k += 1;
				}
			}
			m += 2;
		}
	}

	for i in 0..salud.len() {
		if salud[i] == 1 {
			count += 1;
		}
	}

	return count;
}

//My attempt! Didn't know about the generating primitive triples thing.
/*
pub fn solve1() -> i64 {
  let mut count = 0;
  let mut squares: Vec<i64> = vec!();
  let  limit: i64 = 15000;

  let mut salud = vec!(0; limit as usize +1);

  for i in 0..=limit/2 {
	squares.push(i as i64*i as i64);
  }

  let mut f = math::Factorizer::new();
  for r_i in 1..squares.len() {
	if r_i % 10000 == 0 {
	  println!("{}", r_i);
	}
	if squares[r_i] % 2 != 0 {
	  continue;
	}
	let f_v = f.get_factors(squares[r_i] / 2);
	for f_i in 0..f_v.len()/2 {
	  let s: i64 = f_v[f_i];
	  let t = squares[r_i]/2/s;
	  let r = r_i;
	  let index = 3*r + 2*s as usize + 2*t as usize;
	  if index as i64 <= limit {
		salud[index] += 1;
	  }
	}
	//for v in f_v {
	  //print!("{}, ", v);
   // }
	//println!("");
  }

  for i in 0..salud.len() {
	if salud[i] == 1 {
	  count += 1;
	  //println!("{}",i);
	}
  }

  return count;

}
*/

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 161667);
	}
}
