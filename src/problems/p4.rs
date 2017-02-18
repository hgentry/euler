pub fn solve() -> i64 {
	let mut best = 1;
	let mut i = 100;
	let mut j = 100;
	while i <= 999 {
		while j <= i {
			let current = i*j;
			if current > best {
				if is_palindrome(current) {
					best = current;
				}
			}
			j+=1;
		}
		i+=1;
		j = 100;
	}
	return best;
}

fn is_palindrome(x: i64) -> bool {
	let x_str = x.to_string();
	x_str.bytes().eq(x_str.bytes().rev())
}
