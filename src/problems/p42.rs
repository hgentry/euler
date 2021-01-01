use utils::math;
use utils::strings;
pub fn solve() -> i64 {
	let s: String = strings::read_file_to_string("input/p042_words.txt");
	let list: Vec<String> = strings::process_list(s);
	let mut count = 0;
	for l in list {
		let t = strings::wordscore(&l);
		if math::is_triangle(t) {
			count += 1;
		}
	}
	count
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 162);
	}
}
