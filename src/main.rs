mod problems;
mod utils;
mod scheduler;
mod problem;


extern crate time;
extern crate num_bigint;
extern crate colored;

static T_COUNT: i64 = 1;

use scheduler::*;

fn main() {
	println!("---------------------------------------------");
    println!("| {0: <7} | {1: >20} | {2: >8} |", 
        "Problem", "Solution","Time");
    println!("---------------------------------------------");
    let start = time::now();
    //let status = Status{to_solve: vec!(30)};
    let status = Status{to_solve: vec!(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,45,67)};
    let total_problems = status.to_solve.len();
    let solver = Solver::new(status, T_COUNT);
    solver.solve();
    let end = time::now();
	let duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;

    println!("---------------------------------------------");
    println!("Solved {} problems in {:.5} seconds.", total_problems, duration);
    
    let avg: f64 = duration / total_problems as f64;
    if avg * 618.0 > 60.0 {
        println!("Average time per problem: {:.5}", avg);
        println!("This must be lowered to {:.5}", 60.0/618.0);
    } else {
        println!("Average time per problem: {:.5}", avg);
        println!("This average meets expectations.");
    }
    println!("---------------------------------------------");
}
