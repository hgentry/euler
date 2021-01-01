pub fn solve() -> i64 {
	let mut n: f64 = 286.0;
	let val: f64;
	loop {
		let tn: f64 = n * (n + 1.0) / 2.0;
		let pn: f64 = (0.5 + (0.25 + 6.0 * tn).sqrt()) / 3.0;
		let hn: f64 = (1.0 + (1.0 + 8.0 * tn).sqrt()) / 4.0;

		if (pn - pn.floor()).abs() < 0.00001 && (hn - hn.floor()).abs() < 0.00001 {
			val = tn;
			break;
		}
		n += 1.0;
	}
	val as i64
}
