//TODO this can be done much more quickly by generating permutations more efficiently, counting as it goes, and stopping at #1000000

pub fn solve() -> String {
	let mut v : Vec<i64> = vec![];
	for i in 0..10 {
		v.push(i as i64);
	}
	let r = permute(v);
	let mut list : Vec<String> = vec![];
	for i in 0..r.len() {
		let mut s : String = "".to_string();
		let iter = r[i].iter();
		for j in iter {
			s = format!("{}{}",s,*j);
		}
		list.push(s);
	}
	list.sort();
	list[999999].to_string()
}

pub fn permute(v: Vec<i64>) -> Vec<Vec<i64>> {
	let mut res : Vec<Vec<i64>> = vec![];

	if v.len() == 1 {
		let mut tmp = vec![];
		tmp.push(v[0]);
		res.push(tmp);
		return res;
	}

	for i in v.iter() {
		let mut vi : Vec<i64> = vec![];
		for j in v.iter() {
			if *i != *j {
				vi.push(*j);
			}
		}
		
		let vs = permute(vi);
		for p in vs {
			let mut tmp = vec![];
			tmp.push(*i);
			let mut q = p.clone();
			tmp.append(&mut q);
			res.push(tmp);
		}
	}

	//println!("{}", res.len());
	return res;
}
