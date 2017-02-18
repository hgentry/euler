pub fn solve() -> i64 {
	
	let mut longest = 0;
	let mut longest_index = 0;
	for i in 1..1000000 {
		let mut count = 1;
		let mut iter = i;
		while iter != 1 {
			if iter % 2 == 0 {
				iter = iter/2;
			} else {
				iter = 3*iter + 1;
			}
			count += 1;
		}
		if count > longest {
			longest = count;
			longest_index = i;
		}
	}	
	return longest_index;
}
