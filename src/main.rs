#![allow(dead_code)]

mod problems;
mod utils;
mod scheduler;
mod problem;
mod initializer;


extern crate time;
extern crate num_bigint;
extern crate colored;
extern crate ord_subset;
extern crate substring;

use colored::*;
use std::env;

static T_COUNT: i64 = 16;
static PROBLEMS: f64 = 739.0;
static GOAL: f64 =  60.0;

use scheduler::*;

fn main() {
    let start = time::now();
    //let status = Status{to_solve: vec!(59)};
    let mut status = Status{to_solve: vec!(
        /* Level 1 */ 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,
        /* Level 2 */ 26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,
        /* Level 3 */ 51,52,53,54,55,56,57,58,59,60,61,62,63,67,71,
        /* Other */ 79,92,97,206,700
    )};
    

    let mut doing_init = false;
    let mut to_init = "".to_string();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut args2 = vec!();
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
            status = Status{to_solve: args2};
        }
    }

    if doing_init {
        let initialized = initializer::initialize(to_init.clone());
        if initialized {
            println!("Initialized {}", to_init.clone());
        } else {
            println!("Could not initialize {}",to_init.clone());
        }
        return;
    }
    
    let total_problems = status.to_solve.len();
    let mut solver = Solver::new(status, T_COUNT);
    solver.solve();
    solver.results.sort_by_key(|x| x.n);
    println!("---------------------------------------------");
    println!("| {0: <7} | {1: >20} | {2: >8} |",
        "Problem", "Solution","Time");
    println!("---------------------------------------------");
    let mut solved = 0;
    for result in solver.results {
        let n = result.n;
        let s = result.s;
        let duration = result.duration;
        let output1 = format!("| {0: >7} | {1: >20} | {2: >8.5} |",
        n, s, duration);
            let output = format!("{0: <44}", output1);
            if duration > 60.0 / 618.0 && duration < 1.0 {
		        println!("{}", output.white());
            } else if duration > 1.0 {
                println!("{}", output.bright_red());
            } else {
                println!("{}", output.bright_green());
            }
        solved += 1;
        if solved % 25 == 0 {
            println!( "---------------------------------------------");
            let level = format!("Level {}", solved/25);
            let out = format!("| You leveled up!                {:>10} |", level);
            println!("{}", out.bright_blue());
            println!( "---------------------------------------------");
        }
    }
    let end = time::now();
	let duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;

    println!("---------------------------------------------");
    println!("Solved {} problems in {:.5} seconds.", total_problems, duration);

    let avg: f64 = duration / total_problems as f64;
    if avg * PROBLEMS > GOAL {
        println!("Average time per problem: {:.5}", avg);
        println!("This must be lowered to {:.5}", total_problems as f64*GOAL/(PROBLEMS));
    } else {
		let score = total_problems as f64*GOAL/PROBLEMS/avg*100.0;
        println!("Average time per problem: {:.5}", avg);
        println!("This average meets expectations ({:.2}%).", score);
    }
    println!("---------------------------------------------");
}
