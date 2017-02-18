mod problems;
mod utils;
extern crate time;
extern crate num_bigint;

fn main() {

	utils::create_prime_db();
	utils::create_triangle_db();

	let mut start : time::Tm;
	let mut end : time::Tm;
	let mut duration : f64 = 0.0;
	let mut total = duration;
	let mut probs = 0;

	start = time::now();
	let p21 = problems::p21::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 21 is {} ({}s)", p21, duration);

	start = time::now();
	let p1 = problems::p1::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 1 is {} ({}s)", p1, duration);

	start = time::now();
	let p2 = problems::p2::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 2 is {} ({}s)", p2, duration);

	start = time::now();
	let p3 = problems::p3::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 3 is {} ({}s)", p3, duration);

	start = time::now();
	let p4 = problems::p4::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 4 is {} ({}s)", p4, duration);

	start = time::now();
	let p5 = problems::p5::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 5 is {} ({}s)", p5, duration);

	start = time::now();
	let p6 = problems::p6::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 6 is {} ({}s)", p6, duration);

	start = time::now();
	let p7 = problems::p7::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 7 is {} ({}s)", p7, duration);

	start = time::now();
	let p8 = problems::p8::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 8 is {} ({}s)", p8, duration);

	start = time::now();
	let p9 = problems::p9::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 9 is {} ({}s)", p9, duration);

	start = time::now();
	let p10 = problems::p10::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 10 is {} ({}s)", p10, duration);

	start = time::now();
	let p11 = problems::p11::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 11 is {} ({}s)", p11, duration);

	start = time::now();
	let p12 = problems::p12::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 12 is {} ({}s)", p12, duration);

	start = time::now();
	let p13 = problems::p13::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 13 is {} ({}s)", p13, duration);

	start = time::now();
	let p14 = problems::p14::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 14 is {} ({}s)", p14, duration);


	start = time::now();
	let p15 = problems::p15::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 15 is {} ({}s)", p15, duration);

	start = time::now();
	let p16 = problems::p16::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 16 is {} ({}s)", p16, duration);

	start = time::now();
	let p17 = problems::p17::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 17 is {} ({}s)", p17, duration);

	start = time::now();
	let p18 = problems::p18::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 18 is {} ({}s)", p18, duration);

	start = time::now();
	let p19 = problems::p19::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 19 is {} ({}s)", p19, duration);

	start = time::now();
	let p20 = problems::p20::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 20 is {} ({}s)", p20, duration);

	start = time::now();
	let p67 = problems::p67::solve();
	end = time::now();
	duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
	total += duration;
	probs += 1;
	println!("The answer to problem 67 is {} ({}s)", p67, duration);

	println!("Total time: {}s", total);
	println!("Average solution time: {}s", total/(probs as f64));
}
