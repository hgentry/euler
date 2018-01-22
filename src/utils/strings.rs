use std::fs::File;
use std::io::prelude::*;

pub fn read_file_to_string(filename : &str) -> String {
	let f = File::open(filename);
	let mut buffer = String::new();
	f.unwrap().read_to_string(&mut buffer).unwrap();
	buffer
}
