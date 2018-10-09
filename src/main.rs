mod problems;
mod utils;
mod scheduler;
mod problem;


extern crate time;
extern crate num_bigint;
extern crate colored;
extern crate ord_subset;

static T_COUNT: i64 = 32;
static PROBLEMS: f64 = 618.0;
static GOAL: f64 = 60.0;

use scheduler::*;

fn main() {
	println!("---------------------------------------------");
    println!("| {0: <7} | {1: >20} | {2: >8} |",
        "Problem", "Solution","Time");
    println!("---------------------------------------------");
    let start = time::now();
    //let status = Status{to_solve: vec!(42)};
    let status = Status{to_solve: vec!(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,67)};
    let total_problems = status.to_solve.len();
    let solver = Solver::new(status, T_COUNT);
    solver.solve();
    let end = time::now();
	let duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;

    println!("---------------------------------------------");
    println!("Solved {} problems in {:.5} seconds.", total_problems, duration);

    let avg: f64 = duration / total_problems as f64;
    if avg * PROBLEMS > GOAL {
        println!("Average time per problem: {:.5}", avg);
        println!("This must be lowered to {:.5}", GOAL/PROBLEMS);
    } else {
		let score = GOAL/PROBLEMS/avg*100.0;
        println!("Average time per problem: {:.5}", avg);
        println!("This average meets expectations ({:.2}%).", score);
    }
    println!("---------------------------------------------");
}
