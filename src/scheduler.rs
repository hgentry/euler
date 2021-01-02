use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::problem;

pub struct Status {
	pub to_solve: Vec<i64>,
}

pub struct Result {
	pub n: i64,
	pub s: String,
	pub duration: f64,
}

pub struct Solver {
	pub status: Arc<Mutex<Status>>,
	pub t_count: i64,
	pub results: Vec<Result>,
	pub p_count: i64,
}

impl Solver {
	pub fn new(s: Status, t: i64) -> Solver {
		let p = s.to_solve.len() as i64;
		Solver {
			status: Arc::new(Mutex::new(s)),
			t_count: t,
			results: vec![],
			p_count: p,
		}
	}

	pub fn solve(&mut self) {
		let (sender, receiver) = channel();
		for _ in 0..self.t_count {
			let status = self.status.clone();
			let sender = sender.clone();
			thread::spawn(move || loop {
				let mut task = 0;
				{
					let mut data = status.lock().unwrap();
					if (*data).to_solve.len() > 0 {
						task = (*data).to_solve.remove(0);
					}
				}
				if task != 0 {
					let result = problem::problem(task);
					sender
						.send(Result {
							n: result.0,
							s: result.1,
							duration: result.2,
						})
						.unwrap();
				} else {
					break;
				}
			});
		}
		for _ in 0..self.p_count {
			let result = receiver.recv().unwrap();
			self.results.push(result);
			continue;
		}
	}
}
