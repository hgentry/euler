use utils::primes;
use utils::math;
use utils::strings;
pub fn solve() -> i64 {
    let s : String = strings::read_file_to_string("input/p042_words.txt");
	let mut list : Vec<String> = strings::process_list(s);
	let mut count = 0;
	for l in list {
		let t = strings::wordscore(&l);
		if math::is_triangle(t) {
			count += 1;
		}
	}
	count
}


