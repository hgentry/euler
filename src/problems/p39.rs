pub fn solve() -> i64 {
	let mut triples = vec![0; 1001];
	for a in 1..500 {
		for b in a..500 {
			let c = ((a * a + b * b) as f64).sqrt();
			if c == c.floor() && a + b + (c.floor() as usize) < 1000 {
				triples[(a + b + (c.floor() as usize))] += 1;
			}
		}
	}
	let mut max = 0;
	let mut index = 0;
	for i in 0..1001 {
		if triples[i] > max {
			max = triples[i];
			index = i;
		}
	}

	index as i64
}
