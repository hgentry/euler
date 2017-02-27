mod problems;
mod utils;
extern crate time;
extern crate num_bigint;

pub struct Problem {
		pub answer: String,
		pub duration: f64
}

#[derive(Copy)]
#[derive(Clone)]
pub struct ResultSet {
	problems: i32,
	duration: f64
}

impl ResultSet {
	fn problem(&mut self, n: i32) {
		let s : String;
		let start = time::now();
		match n {
			1 => s = problems::p1::solve().to_string(),
			2 => s = problems::p2::solve().to_string(),
			3 => s = problems::p3::solve().to_string(),
			4 => s = problems::p4::solve().to_string(),
			5 => s = problems::p5::solve().to_string(),
			6 => s = problems::p6::solve().to_string(),
			7 => s = problems::p7::solve().to_string(),
			8 => s = problems::p8::solve().to_string(),
			9 => s = problems::p9::solve().to_string(),
			10 => s = problems::p10::solve().to_string(),
			11 => s = problems::p11::solve().to_string(),
			12 => s = problems::p12::solve().to_string(),
			13 => s = problems::p13::solve().to_string(),
			14 => s = problems::p14::solve().to_string(),
			15 => s = problems::p15::solve().to_string(),
			16 => s = problems::p16::solve().to_string(),
			17 => s = problems::p17::solve().to_string(),
			18 => s = problems::p18::solve().to_string(),
			19 => s = problems::p19::solve().to_string(),
			20 => s = problems::p20::solve().to_string(),
			21 => s = problems::p21::solve().to_string(),
			22 => s = problems::p22::solve().to_string(),
			23 => s = problems::p23::solve().to_string(),
			24 => s = problems::p24::solve().to_string(),
			25 => s = problems::p25::solve().to_string(),
			26 => s = problems::p26::solve().to_string(),
			27 => s = problems::p27::solve().to_string(),
			28 => s = problems::p28::solve().to_string(),
			29 => s = problems::p29::solve().to_string(),
			45 => s = problems::p45::solve().to_string(),
			67 => s = problems::p67::solve().to_string(),
			_ => s = "That is not solved yet".to_string()
		}
		let end = time::now();
		let duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
		self.duration += duration;
		self.problems += 1;

		println!("The answer to problem {} is {} in {}s", n, s, duration);
	}
}

fn main() {
	println!("----------");
	utils::create_prime_db();
	utils::create_triangle_db();

	let mut r: ResultSet = ResultSet{duration: 0.0, problems: 0};
	r.problem(1);
	r.problem(2);
	r.problem(3);
	r.problem(4);
	r.problem(5);
	r.problem(6);
	r.problem(7);
	r.problem(8);
	r.problem(9);
	r.problem(10);
	r.problem(11);
	r.problem(12);
	r.problem(13);
	r.problem(14);
	r.problem(15);
	r.problem(16);
	r.problem(17);
	r.problem(18);
	r.problem(19);
	r.problem(20);
	r.problem(21);
	r.problem(22);
	r.problem(23);
	r.problem(24);
	r.problem(25);
	r.problem(26);
	r.problem(27);
	r.problem(28);
	r.problem(29);
	r.problem(45);
	r.problem(67);
	
	println!("----------");
	println!("{} Problems solved. User is level {}", r.problems, r.problems/25);
	println!("Total time: {}s", r.duration);
	println!("Average solution time: {}s", r.duration/(r.problems as f64));
}
