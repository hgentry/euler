#![allow(dead_code)]

mod initializer;
mod problem;
mod problems;
mod scheduler;
mod utils;

extern crate colored;
extern crate num;
extern crate ord_subset;
extern crate substring;
extern crate time;

use colored::*;
use std::env;
use time::*;

static T_COUNT: i64 = 16;
static PROBLEMS: f64 = 739.0;
static GOAL: f64 = 60.0;

use scheduler::*;

fn main() {
	let spoilers_hidden = false;
	let start = Instant::now();
	//let status = Status{to_solve: vec!(59)};
	let mut status = Status {
		to_solve: vec![
			1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
			25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
			47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68,
			69, 71, 79, 92, 97, 206, 700,
		],
	};

	let mut doing_init = false;
	let mut to_init = "".to_string();

	let args: Vec<String> = env::args().collect();
	if args.len() > 1 {
		let mut args2 = vec![];
		let mut first = true;
		let mut second = true;
		for a in args {
			if first {
				first = false;
				continue;
			}
			if !first && second {
				if a == "init".to_string() {
					doing_init = true;
					continue;
				}
				second = false;
			}
			if !doing_init {
				args2.push(a.parse().unwrap());
			} else {
				to_init = a;
				break;
			}
		}

		if !doing_init {
			status = Status { to_solve: args2 };
		}
	}

	if doing_init {
		let initialized = initializer::initialize(to_init.clone());
		if initialized {
			println!("Initialized {}", to_init.clone());
		} else {
			println!("Could not initialize {}", to_init.clone());
		}
		return;
	}

	let total_problems = status.to_solve.len();
	let mut solver = Solver::new(status, T_COUNT);
	solver.solve();
	solver.results.sort_by_key(|x| x.n);
	println!("-------------------------------------------------------------------------------------------");
	let mut solved = 0;
	let mut skip = 0;
	for result in 0..solver.results.len() {
		if skip > 0 {
			skip -= 1;
			continue;
		}
		if result % 25 == 0 {
			println!(
				"| {0: <7} | {1: >20} | {2: >8} | | {0: <7} | {1: >20} | {2: >8} |",
				"Problem", "Solution", "Time"
			);
			println!("-------------------------------------------------------------------------------------------");
		}
		let n = solver.results[result].n;
		let mut s = solver.results[result].s.clone();
		let duration = solver.results[result].duration;
		let mut n2 = 0;
		let mut s2 = "".to_string();
		let mut duration2 = 0.0;
		if spoilers_hidden {
			s = "*spoilers hidden*".to_string();
		}
		if result + 25 < solver.results.len() {
			n2 = solver.results[result + 25].n;
			s2 = solver.results[result + 25].s.clone();
			duration2 = solver.results[result + 25].duration;
			if spoilers_hidden {
				s2 = "*spoilers hidden*".to_string();
			}
		}
		let output1 = format!("| {0: >7} | {1: >20} | {2: >8.5} |", n, s, duration);
		let output = format!("{0: <44}", output1);
		if duration > 60.0 / 618.0 && duration < 1.0 {
			print!("{} ", output.white());
		} else if duration > 1.0 {
			print!("{} ", output.bright_red());
		} else {
			print!("{} ", output.bright_green());
		}
		if n2 != 0 {
			let output2 = format!("| {0: >7} | {1: >20} | {2: >8.5} |", n2, s2, duration2);
			let output = format!("{0: <44}", output2);
			if duration2 > 60.0 / 618.0 && duration2 < 1.0 {
				print!("{}", output.white());
			} else if duration2 > 1.0 {
				print!("{}", output.bright_red());
			} else {
				print!("{}", output.bright_green());
			}
			solved += 1;
		} else {
			//-------------------------------------------
			print!("                                            |");
		}
		println!();
		solved += 1;
		if (result + 1) % 25 == 0 && total_problems > 1 {
			skip = 25;
			println!( "-------------------------------------------------------------------------------------------");
			let level = format!("Level {}", solved / 25);
			let out = format!("| Good Job!                                                                    {:>10} |", level);
			println!("{}", out.bright_blue());
			println!( "-------------------------------------------------------------------------------------------");
		}
	}

	if solved % 50 != 0 && solved % 50 != 49 && total_problems > 1 {
		println!( "-------------------------------------------------------------------------------------------");
		let level = format!("Level {}", solved / 25);
		let out = format!("| Final Results                                                                {:>10} |", level);
		println!("{}", out.bright_blue());
		println!( "-------------------------------------------------------------------------------------------");
	}
	if total_problems == 1 {
		println!( "-------------------------------------------------------------------------------------------");
	}
	let end = Instant::now();
	let duration = (end - start).whole_nanoseconds() as f64 / 1000000000.0;

	println!(
		"Solved {} problems in {:.5} seconds.",
		total_problems, duration
	);

	let avg: f64 = duration / total_problems as f64;
	if avg * PROBLEMS > GOAL {
		println!("Average time per problem: {:.5}", avg);
		println!(
			"This must be lowered to {:.5}",
			total_problems as f64 * GOAL / (PROBLEMS)
		);
	} else {
		let score = total_problems as f64 * GOAL / PROBLEMS / avg * 100.0;
		println!("Average time per problem: {:.5}", avg);
		println!("This average meets expectations ({:.2}%).", score);
	}
	println!("-------------------------------------------------------------------------------------------");
}
