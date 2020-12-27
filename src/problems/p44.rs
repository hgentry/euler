pub fn solve() -> f64 {
	let mut min: f64 = 999999999.0;
	for i in 1..10000 {
		for j in i+1..10000 {
			if i != j && is_pentagonal(pentagon(i) + pentagon(j)) && is_pentagonal((pentagon(i)- pentagon(j)).abs()) {
				let diff = (pentagon(i) - pentagon(j)).abs();
				if diff < min {
					min = diff;
				}
			}
		}
	}
	min
}


pub fn is_pentagonal(p: f64) -> bool {
	let roots = quadratic(1.5, -0.5, -p);
	if roots.0 == roots.0.round() {
		return true;
	}
	return false;
}

pub fn pentagon(a: i64) -> f64 {
	let n = a as f64;
	return n * (3.0*n - 1.0) * 0.5;
}

pub fn quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
	let x1: f64 = (-b as f64 + ((b*b - 4.0*a*c) as f64).sqrt())/((2.0*a) as f64);
	let x2: f64 = (-b as f64 - ((b*b - 4.0*a*c) as f64).sqrt())/((2.0*a) as f64);
	return (x1, x2);
}







