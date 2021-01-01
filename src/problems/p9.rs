pub fn solve() -> i64 {
	for a in 1..998 {
		for b in 1..998 {
			let c = ((a*a + b*b) as f64).sqrt();
			if c == c.round() {
				if a + b + (c as i64) == 1000 {
				 return a*b*(c as i64);
				}
			}
		}
	}
	return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
	
	#[test]
    fn correct() {
		assert_eq!(solve(), 31875000);
    }
}