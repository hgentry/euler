use rand::Rng;

pub fn solve() -> String {
	let mut hits = vec![0; 40];
	let mut pos = 0;
	let mut past_double_count = 0;
	let mut draw_cc = 0;
	let mut draw_ch = 0;
	let mut rng = rand::thread_rng();
	for _ in 0..500000 {
		let r1: i64 = num::abs(rng.gen::<i64>()) % 4 + 1;
		let r2: i64 = num::abs(rng.gen::<i64>()) % 4 + 1;

		if r1 == r2 {
			past_double_count += 1;
			if past_double_count > 2 {
				pos = 10;
				past_double_count = 0;
				hits[pos as usize] += 1;
				continue;
			}
		} else {
			past_double_count = 0;
		}

		let roll = r1 + r2;
		pos = (pos + roll) % 40;

		if pos == 7 || pos == 22 || pos == 36 {
			draw_ch = (draw_ch + 1) % 16;
			match draw_ch {
				1 => pos = 0,  //Advance to GO
				2 => pos = 10, //Go to JAIL
				3 => pos = 11, //Go to C1
				4 => pos = 24, //Go to E3
				5 => pos = 39, //Go to H2
				6 => pos = 5,  //Go to R1
				7 => {
					//Go to next R
					if pos == 7 {
						pos = 15;
					}
					if pos == 22 {
						pos = 25;
					}
					if pos == 36 {
						pos = 5;
					}
				}
				8 => {
					// Go to next R
					if pos == 7 {
						pos = 15;
					}
					if pos == 22 {
						pos = 25;
					}
					if pos == 36 {
						pos = 5;
					}
				}
				9 => {
					//Go to next U
					if pos == 22 {
						pos = 28;
					} else {
						pos = 12;
					}
				} //Go back 3
				10 => {
					pos = pos - 3;
				}
				_ => {}
			};
		}
		if pos == 2 || pos == 17 || pos == 33 {
			draw_cc = (draw_cc + 1) % 16;
			match draw_cc {
				1 => pos = 0,  //Advance to GO
				2 => pos = 10, //Go to JAIL
				_ => {}
			};
		}
		if pos == 30 {
			pos = 10; //Go to JAIL
		}

		hits[pos as usize] += 1;
	}
	let mut top = 0;
	let mut top_h = 0;
	let mut second = 0;
	let mut second_h = 0;
	let mut third = 0;
	let mut third_h = 0;
	for i in 0..hits.len() {
		if hits[i] > top_h {
			third = second;
			third_h = second_h;
			second = top;
			second_h = top_h;
			top = i;
			top_h = hits[i];
		} else if hits[i] > second_h {
			third = second;
			third_h = second_h;
			second = i;
			second_h = hits[i];
		} else if hits[i] > third_h {
			third = i;
			third_h = hits[i];
		}
	}

	let out = format!("{:2.}{:2.}{:2.}", top, second, third);
	return out;
}

pub struct MonopolySquare {
	name: String,
	squaretype: String,
	pos: usize,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[ignore]
	fn correct() {
		assert_eq!(solve(), "101524");
	}
}
