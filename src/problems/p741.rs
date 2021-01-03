pub fn solve() -> i64 {
	let n = 4;
	let mut boxy: BigBox = BigBox{row_limits: vec!(2; n),column_limits: vec!(2; n)};


	return box_analysis(&mut boxy, 0, n-1);
}

pub fn box_analysis(boxy: &mut BigBox, start: usize, end: usize) -> i64 {
	//println!("Analyzing permutation for {} {}", start, end);
	let mut sum = 0;

	let mut perm = PerimeterPermuter::new(&boxy.row_limits, &boxy.column_limits, end-start + 1);
	let iter_option = perm.first_iteration();
	let mut iter;
	match iter_option {
		Some(x) => iter = x,
		None => return 0
	}
	loop {
		boxy.forward_applique(iter);
		let for_sum = box_analysis(boxy, start + 1, end - 1);
		sum += for_sum;
		boxy.reverse_applique(iter);
		let next = perm.next_iteration(&boxy.row_limits, &boxy.column_limits);
		match next {
			Some(x) => iter = x,
			None => break
		}
	}
	return sum;
}
 
pub struct BigBox {
	pub row_limits: Vec<u8>,
	pub column_limits: Vec<u8>,
}

pub struct Applique {
	pub coords: Vec<Coords>,
}

pub struct Coords {
	pub row: usize,
	pub column: usize,
}

pub struct PerimeterPermuter {
	current_perm: Applique,
	width: usize
}

impl<'a> PerimeterPermuter {
	pub fn new(row_limits: &Vec<u8>, column_limits: &Vec<u8>, n: usize) -> PerimeterPermuter {
		let first_perm = Applique{coords:vec!()};
		return PerimeterPermuter{current_perm: first_perm, width: n};
	}

	pub fn first_iteration(&mut self) -> Option<&Applique> {
		let mut a = Applique{coords:vec!()};
		a.coords.push(Coords{row:0,column: 0});
		a.coords.push(Coords{row:0,column: self.width - 1});
		a.coords.push(Coords{row:self.width - 1, column: self.width - 1});
		a.coords.push(Coords{row:self.width - 1, column: 0});
		self.current_perm = a;
		return Some(&self.current_perm);
	}

	pub fn next_iteration(&mut self, row_limits: &Vec<u8>, column_limits: &Vec<u8>) -> Option<&Applique> {
		/*
			      3
			  0 0 1 1 0
			  1       0
			2 1       1 4
			  0       1 <- end
			  0 1 1 0 0 <- start
				  1
				  
			Permuter initialization:
				side_1_init = 0 ... 1 1 0
				side_2 etc...

			Generate next permutation:
				Increment side 1 to next val that preserves side 2
				if looped around to side_1_init 
					increment side 2 to next val that preserves side 3
					if looped around to side_2_init
						increment side 3 to next val that preserves side 4
						if looped around to side_3_init
						increment side 4
						if looped around to side_4_init 
							return None
						set side_3_init and side_3 to lowest val that preserves side 4
					set side_2_init and side_2 to lowest val that preserves side 3
				set side_1_init and side_1 to lowest val that preserves side 4
				return this configuration
		
		*/

		// Figure out what side 1 is
		// Figure out what side 2 is
		// Figure out what side 3 is
		// Figure out what side 4 is

		return Some(&self.current_perm);
	}

	fn sub_iter(&mut self, side: u8) {

	}
}

impl BigBox {
	pub fn reverse_applique(&mut self, a: &Applique) {
		for c in 0..a.coords.len() {
			self.row_limits[a.coords[c].row] += 1;
			self.column_limits[a.coords[c].column] += 1;
		}
	}

	pub fn forward_applique(&mut self, a: &Applique) {
		for c in 0..a.coords.len() {
			self.row_limits[a.coords[c].row] -= 1;
			self.column_limits[a.coords[c].column] -= 1;
		}
	}

	pub fn can_forward_applique(&mut self, a: &Applique) -> bool {
		for c in 0..a.coords.len() {
			if self.row_limits[a.coords[c].row] == 0 || self.column_limits[a.coords[c].column] == 0 {
				return false;
			}
		}
		return true;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[ignore]
	fn correct() {
		assert_eq!(solve(), 0);
	}
}
