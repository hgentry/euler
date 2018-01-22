use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;

use problem;

pub struct Status {
    pub to_solve: Vec<i64>
}

pub struct Solver {
    pub status: Arc<Mutex<Status>>,
    pub t_count: i64
}

impl Solver {
    pub fn new(s: Status, t: i64) -> Solver {
        Solver{status: Arc::new(Mutex::new(s)), t_count : t}
    }

    pub fn solve(&self) {
        let (sender, receiver) = channel();
        for _ in 0..self.t_count {
            let status = self.status.clone();
            let sender = sender.clone();
            thread::spawn(move || {
                loop {
                    let mut task = 0;
                    {
                        let mut data = status.lock().unwrap();
                        if (*data).to_solve.len() > 0 {
                            task = (*data).to_solve.remove(0);
                        }
                    }
                    if task != 0 {  
                        problem::problem(task);
                    } else {
                        break;
                    }
                }
                sender.send(1).unwrap();
            });
        }
        for _ in 0..self.t_count {
            if receiver.recv().unwrap() > 0 {continue;}
        }
    }
}
