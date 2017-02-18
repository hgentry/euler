pub fn solve() -> i64 {
	let mut sun = 0;
	let mut dow = 1;
	for y in 1901..2001 {
		for m in 1..13 {
			let mut d = 0;
			loop {
				d += 1;
				dow = (dow + 1) % 7;

				if dow == 6 && d == 1 {
					sun += 1;
				}
				if m == 2 && y % 4 == 0 && d == 29{
					break;
				}
				if m == 2 && d == 28 && y % 4 != 0 {
					break;
				}	
				if d == 31 && (m == 1 || m == 3 || m == 5 || m == 7 || m == 8 || m == 10 || m == 12) {
					break;
				}
				if d == 30 && (m == 4 || m == 6 || m == 9 || m == 11) {
					break;
				}
				
			}
		}
	}
	sun
}

