use num::bigint::*;

extern crate colored;
use colored::*;

pub fn solve() -> i64 {
	return solve_v9();
}

pub fn solve_v1() -> i64 {
	brute_force(20, true);
	return 0;
}

pub fn solve_v7() -> i64 {
	bigint_method_reverse(8);
	return 0;
}

pub fn solve_v8() -> i64 {
	brute_force_coefficients_table(8, true);
	return 0;
}

pub fn solve_v9() -> i64 {
	return brute_force_faster(10000);
}

pub fn brute_force_faster(n: usize) -> i64 {
	let mut a0 = 1;
	let mut a1: Vec<i64> = vec![0; n];
	a1[0] = 3;
	a1[1] = 3;
	let mut temp;
	if n == 1 {
		return 1;
	}
	for i in 2..n {
		temp = a1[0];
		a1[0] = a0 + a1[0];
		if a1[0] > 5 * 1000000007 {
			a1[0] -= 5 * 1000000007;
		}
		a0 = temp;
		for j in 1..i {
			a1[j] = a1[j - 1] + a1[j];
			if a1[j] > 5 * 1000000007 {
				a1[j] -= 5 * 1000000007;
			}
		}
		a1[i] = a1[i - 1];
	}
	return a1[n - 1] % 1000000007;
}

pub fn brute_force(n: i64, generate_pyramid: bool) -> Vec<i64> {
	let mut a0 = vec![1];
	let mut a1 = vec![3, 3];
	let mut coefficients = vec![0];

	if n == 1 {
		return vec![1];
	}

	let mut vecs = vec![a0.to_vec(), a1.to_vec()];
	for _i in 1..n - 1 {
		let mut a2 = vec![];

		a2.push(a0[0] + a1[0]);

		for j in 1..a1.len() {
			a2.push(a1[j] + a2[j - 1]);
		}
		a2.push(a2[a2.len() - 1]);
		if generate_pyramid {
			vecs.push(a2.to_vec());
		}
		coefficients.push(a2[a2.len() - 1]);
		a0 = a1;
		a1 = a2;
	}
	if generate_pyramid {
		print_pyramid(vecs);
	}
	return coefficients;
}

pub fn brute_force_coefficients(n: i64, generate_pyramid: bool, a: i64) -> Vec<i64> {
	let mut a1 = vec![0];
	let _a0 = vec![0];
	let mut coefficients = vec![0];

	if a == 1 {
		return vec![1];
	}
	if a == 2 {
		//coefficients.push(1);
		a1[0] = 1;
	}
	let mut vecs = vec![a1.to_vec()];
	if generate_pyramid {
		println!("Generating coefficients for term {} up to n={}", a, n);
	}
	for i in 2..n + 1 {
		let mut a2 = vec![];

		if i == a {
			a2.push(1);
		} else {
			a2.push(0);
		}
		for j in 1..a1.len() {
			a2.push(a1[j] + a2[j - 1]);
		}
		a2.push(a2[a2.len() - 1]);
		if generate_pyramid {
			vecs.push(a2.to_vec());
		}
		coefficients.push(a2[a2.len() - 1]);
		a1 = a2;
	}
	if generate_pyramid {
		print_pyramid(vecs);
	}
	return coefficients;
}

pub fn longest_in_vec(v: &Vec<BigInt>) -> i64 {
	let mut best = 0;
	for f in v {
		if f.to_string().len() > best {
			best = f.to_string().len();
		}
	}
	return best as i64;
}

pub fn longest_in_vec_i64(v: &Vec<i64>) -> i64 {
	let mut best = 0;
	for f in v {
		if f.to_string().len() > best {
			best = f.to_string().len();
		}
	}
	return best as i64;
}

pub fn brute_force_coefficients_table(n: i64, generate_table: bool) -> Vec<Vec<i64>> {
	let mut table = vec![];
	for i in 1..n + 1 {
		table.push(brute_force_coefficients(n, generate_table, i));
	}

	if generate_table {
		let mut header = "__________|".to_string();
		for _i in 0..table.len() {
			header += "___________";
		}
		println!("{}", header);
		for a in 0..n {
			let mut line = format!("{:10.}|", a + 1);
			for i in 0..table.len() {
				if a < table[i as usize].len() as i64 && table[i as usize][a as usize] != 0 {
					line = format!("{}{:10.} ", line, table[i as usize][a as usize]);
				} else {
					line = format!("{}           ", line);
				}
			}
			println!("{}", line);
		}
	}

	return table;
}

pub fn print_pyramid(vecs: Vec<Vec<i64>>) {
	let answer_width = longest_in_vec_i64(&vecs[vecs.len() - 1]);
	for i in 0..vecs.len() {
		for v in &vecs {
			if i < v.len() - 1 && v[i] != 0 {
				print!("{:width$.} ", v[i], width = answer_width as usize); // should find the length dynamically instead of using 4 but who cares
			} else if i == v.len() - 1 {
				let mut out = format!("{:width$.} ", v[i], width = answer_width as usize);
				out = out.bright_green().to_string();
				print!("{}", out.to_string()); // should find the length dynamically instead of using 4 but who cares
			} else {
				print!("{:<width$}", "", width = (answer_width + 1) as usize);
			}
		}
		println!("");
	}
}

/*
Problems with this solution

* It's wrong
* Takes too long with all the bigint multiplying

*/
pub fn solve_v2(n: i64, printout: bool) -> i64 {
	let answers = brute_force(n, printout);
	let table = brute_force_coefficients_table(n, printout);
	let answer_width = longest_in_vec_i64(&answers);

	for i in 3..n + 1 {
		let answer = bigint_method_reverse(i);
		let correct = answer.0 == answers[(i - 2) as usize].to_bigint().unwrap();
		print!("{:width$.}: ", i, width = n.to_string().len());
		let _l = "{";
		let _r = "}";
		if correct {
			let out = format!("{:width$.}", answer.0, width = answer_width as usize).to_string();
			print!("{} ", out.bright_green());
		} else {
			let out = format!("{:width$.}", answer.0, width = answer_width as usize).to_string();
			print!("{} ", out.bright_red());
		}

		print!("[");
		for j in 0..answer.1.len() {
			if answer.1[j]
				== table[(j + 1) as usize][(i - 1) as usize]
					.to_bigint()
					.unwrap()
			{
				if j != answer.1.len() - 1 {
					print!("{}, ", (answer.1[j].to_string()).bright_green());
				} else {
					print!("{}", (answer.1[j].to_string()).bright_green());
				}
			} else {
				if j != answer.1.len() - 1 {
					print!("{}, ", (answer.1[j].to_string()).bright_red());
				} else {
					print!("{}", (answer.1[j].to_string()).bright_red());
				}
			}
		}
		print!("]");

		println!();
	}

	return 0;
}

pub fn bigint_method(n: i64) -> (BigInt, Vec<BigInt>) {
	//Lucas sequence initialization
	let mut a1 = 1;
	let mut a2 = 3;

	let mut factor: BigInt = (1).to_bigint().unwrap(); //Initial factor = 1
	let mut running: BigInt = 0.to_bigint().unwrap();
	let mut factors = vec![];
	for i in 1..n {
		//Calculate the new factor from the old factor
		//if i <= n/2  {
		let k = i;
		factor = factor.clone() * (n - k) / (k) - 1;

		//} else {
		//let k = n - i;
		//factor = factor.clone() * (k) / (n-k);
		//}

		//Update the running total
		factors.push(factor.clone());
		running = running.clone() + factor.clone() * a2;

		//Lucas sequence
		let temp = a2;
		a2 = a1 + a2;
		a1 = temp;
	}

	//println!("{}: {}",n, factors);

	return (running, factors);
}

pub fn bigint_method_reverse(n: i64) -> (BigInt, Vec<BigInt>) {
	//Lucas sequence initialization
	let mut a1 = 1;
	let mut a2 = 3;

	let mut factor: BigInt = (1).to_bigint().unwrap(); //Initial factor = 1
	let mut running: BigInt = 0.to_bigint().unwrap();
	let mut factors = vec![];
	for _i in 3..n + 1 {
		let temp = a2;
		a2 = a1 + a2;
		a1 = temp;
	}
	let mut distance = 1;
	for i in 1..n {
		//Update the running total
		factors.insert(0, factor.clone());
		running = running.clone() + factor.clone() * a2;
		//Calculate the new factor from the old factor
		if i != n - 1 {
			if i == 1 {
				distance = 2;
				//println!("calculating factor for i={} with dist {} (S)", i+1, distance);

				factor = (n - 2).to_bigint().unwrap();
			} else {
				distance += 1;
				let _k = distance - 1;
				//println!("calculating factor for i={} with dist {}, n {}, i {} (L)", i+1, distance, n, i);
				//factor = factor.clone() *  (n - 2 + i - 1) / (k);
				//n = 5, i = 2 should give 5/3 = 10/6
				//n = 6, i = 2 should give 7/4 = 14/8
				//n = 7, i = 2 should give 15/5 = 30/10
				//factor = factor.clone() * (n - i + 1) / (n + i + 1);
				factor = (((n - 2) * (n - 1) - (n - i - 1) * (n - i - 2)) / 2 + 1)
					.to_bigint()
					.unwrap();
			}
		}

		//Lucas sequence
		let temp = a1;
		a1 = a2 - a1;
		a2 = temp;
	}

	return (running, factors);
}

pub fn solve_v3() -> i64 {
	let n: u64 = 8;
	let r: u64 = 1000000007;

	//Lucas sequence initialization
	let _a1 = 1;
	let a2 = 1;

	let mut running: u64 = 0;

	let mut factor_n: u64 = 1;
	let mut factor_d: u64 = 1;

	for i in 2..n - 1 {
		let k = i - 1;
		factor_n *= n - k;
		factor_d *= k;
		factor_n /= factor_d;
		//factor_n %= r;
		factor_d = 1;
	}

	for i in 2..n + 1 {
		//Update the running total
		running = (running + a2 * factor_n) % r;
		println!("{}: {}", i, factor_n);

		//Calculate the new factor from the old factor

		let k = n - i;
		factor_n *= k;
		factor_d *= n - k;
		factor_n /= factor_d;
		//factor_n %= r;
		factor_d = 1;

		//Lucas sequence
		//let temp = a2;
		//a2 = a1 + a2;
		//a1 = temp;
	}
	println!("end result: {}", running);

	return 0;
}
