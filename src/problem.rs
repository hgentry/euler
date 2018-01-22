
extern crate time;
extern crate colored;
use problems;
use colored::*;

pub fn problem(n: i64) {
		let s : String;
		let start = time::now();
        let mut found = false;
		match n {
			1 => s = {found = true; problems::p1::solve().to_string()},
			2 => s = {found = true; problems::p2::solve().to_string()},
			3 => s = {found = true; problems::p3::solve().to_string()},
			4 => s = {found = true; problems::p4::solve().to_string()},
			5 => s = {found = true; problems::p5::solve().to_string()},
			6 => s = {found = true; problems::p6::solve().to_string()},
			7 => s = {found = true; problems::p7::solve().to_string()},
			8 => s = {found = true; problems::p8::solve().to_string()},
			9 => s = {found = true; problems::p9::solve().to_string()},
			10 => s = {found = true; problems::p10::solve().to_string()},
			11 => s = {found = true; problems::p11::solve().to_string()},
			12 => s = {found = true; problems::p12::solve().to_string()},
			13 => s = {found = true; problems::p13::solve().to_string()},
			14 => s = {found = true; problems::p14::solve().to_string()},
			15 => s = {found = true; problems::p15::solve().to_string()},
			16 => s = {found = true; problems::p16::solve().to_string()},
			17 => s = {found = true; problems::p17::solve().to_string()},
			18 => s = {found = true; problems::p18::solve().to_string()},
			19 => s = {found = true; problems::p19::solve().to_string()},
			20 => s = {found = true; problems::p20::solve().to_string()},
			21 => s = {found = true; problems::p21::solve().to_string()},
			22 => s = {found = true; problems::p22::solve().to_string()},
			23 => s = {found = true; problems::p23::solve().to_string()},
			24 => s = {found = true; problems::p24::solve().to_string()},
			25 => s = {found = true; problems::p25::solve().to_string()},
			26 => s = {found = true; problems::p26::solve().to_string()},
			27 => s = {found = true; problems::p27::solve().to_string()},
			28 => s = {found = true; problems::p28::solve().to_string()},
			29 => s = {found = true; problems::p29::solve().to_string()},
            30 => s = {found = true; problems::p30::solve().to_string()},
			45 => s = {found = true; problems::p45::solve().to_string()},
			67 => s = {found = true; problems::p67::solve().to_string()},
			_ => s = format!("Problem {} has not been solved.", n)
		}
		let end = time::now();
		let duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
        if found {
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
        } else {
            println!("{}",s);
        }
	}
