use utils::math;

pub fn solve() -> u64 {
	let m: u64 = 4503599627370517;
	let c: u64 = 1504170715041707;
	let mut sum: u64 = c;
	let mut prev = c;
	let mut f = prev;
	let mut i = 0;


	// Brute force the first 15 values
	loop {

		// This was significantly faster than doing mod
		f += c;
		if f > m {
			f -= m;
		}
		
		if f < prev {
			sum += f;
			prev = f;
			
			i += 1;
		}
		
		if i == 15 {
			break;
		}
	}

	// The goal here was, for i in 1 through Eulercoin 15, find the n of each i.
	// Then go though them, sorted by n value, grabbing each new lowest as a coin.
	// It didn't quite work right, I'm not sure what I did wrong.
	// But instead, I wind up with a list of the differences between successive coins.
	i = f - 1;
	let mut list = vec!();
	let ec = math::extended_euclidean(c, m); // Get the modular inverse of 1504170715041707
	loop {
		// cn = i mod m
		// Get the next value by multiplying i by the inverse of c
		// as in, n = i*c^(-1) mod m 
		let diff: u64 = i * ec.0.abs() as u64;
		list.push((i, diff % m));
		
		i -= 1;
		if i == 0 {
			break;
		}
	}
	list.sort_by_key(|x| x.1); // Make sure they're in order by n value, so that we can keep track of lowest

	// Go through and grab each new lowest
	let mut subs = vec!();
	for i in 0..list.len() {
		if list[i].0 < prev {
			prev = list[i].0;
			subs.push(prev);
		}
	}

	// Having noticed that 'list' is (erroneously) the sequence of differences between coins, I just
	// do some arithmetic to generate each new coin.
	let mut additive = f;
	for i in 0..subs.len() {
		while additive > subs[i] {
			additive -= subs[i];
			sum += additive;
		}
	}

	return sum;
}