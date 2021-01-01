use std::fs::File;
use std::io::prelude::*;

pub fn read_file_to_string(filename: &str) -> String {
	let f = File::open(filename);
	let mut buffer = String::new();
	f.unwrap().read_to_string(&mut buffer).unwrap();
	buffer
}

//Read the list into a vec
pub fn process_list(s: String) -> Vec<String> {
	let mut list: Vec<&str> = s.split(',').collect();

	for i in 0..list.len() {
		list[i] = list[i].trim();
		list[i] = list[i].trim_matches(|c| c == '"');
	}

	let res: Vec<String> = list.iter().map(|s| s.to_string()).collect();
	return res;
}

pub fn wordscore(str: &String) -> i64 {
	let mut namesum = 0;
	for c in (*str).chars() {
		namesum += c as i64 - 'A' as i64 + 1;
	}
	namesum
}

pub fn char_as_val(c: char) -> i64 {
	c as i64 - 'A' as i64 + 1
}
