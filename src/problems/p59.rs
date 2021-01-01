use utils::strings;

pub fn solve() -> i64 {
	let input = strings::read_file_to_string("input/p059_cipher.txt");
	let letters: Vec<&str> = input.split(',').collect();
	for i in 97..97 + 26 {
		for j in 97..97 + 26 {
			for k in 97..97 + 26 {
				let mut deciphered = "".to_string();
				let mut count = 0;
				for c in &letters {
					//let bits = binary_bit_vec_from_string(c.to_string());
					let n: u8 = (*c).to_string().parse().unwrap();
					let nc: u8;
					if count == 0 {
						nc = n ^ (i as u8);
					} else if count == 1 {
						nc = n ^ (j as u8);
					} else {
						nc = n ^ (k as u8);
					}
					count = (count + 1) % 3;
					deciphered = format!("{}{}", deciphered, nc as char);
				}
				if deciphered.contains(" the ") {
					let mut sum: i64 = 0;
					for c in deciphered.chars() {
						sum += (c as u8) as i64;
					}
					return sum;
				}
			}
		}
	}
	return 0;
}
