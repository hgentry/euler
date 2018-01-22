use utils;

pub fn solve() -> i64 {
	let s : String = utils::strings::read_file_to_string("input/p022_names.txt");
	let mut list : Vec<String> = process_list(s);
	list.sort();

	let mut sum = 0;
	for i in 0..list.len() {
		let mut namesum = 0;
		for c in list[i].chars() {
			namesum += c as i64 - 'A' as i64 + 1;
		}
		sum += namesum * (i as i64 + 1);
	}
	return sum;
}

//Read the list into a vec
pub fn process_list(s : String) -> Vec<String> {
	let mut list : Vec<&str> = s.split(',').collect();

	for i in 0..list.len() {
		list[i] = list[i].trim();
		list[i] = list[i].trim_matches(|c| c == '"');
	}

	let res: Vec<String> = list.iter().map(|s| s.to_string()).collect();
	return res;
}

