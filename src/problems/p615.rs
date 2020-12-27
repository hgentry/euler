
use ord_subset::*;



#[derive(Clone)]
pub struct ChangeSet {
	pre: Vec<i64>,
	post: Vec<i64>,
	modifier: OrdVar<f64>
}

pub fn solve() -> i64 {
	let primes = primes_as_changesets(100000);
	let mut posts = generate_post_changesets(20000, &primes);
	posts.insert(0, primes[0].clone());
	posts.insert(1, primes[1].clone());
	let mut i = 2;
	while i < primes.len() {
		let mut j = 0;
		let mut found = false;
		while j < posts.len() {
			if primes[i].modifier < posts[j].modifier {
				posts.insert(j, primes[i].clone());
				found = true;
				break;
			}
			j += 1;
		}
		if !found {
			posts.push(primes[i].clone());
		}
		i += 1;
	}
	let mut merged = posts;
	expand_changesets(&mut merged);
	let len = merged.len();
	len as i64
}

pub fn expand_changesets(sets: &mut Vec<ChangeSet>) {
	for i in 0..sets.len() {
		println!("i {}", sets[i].modifier.into_inner());
		for j in 0..i+1 {
			if sets.len() % 10000 == 0 {
			}
			if sets[j].post.len() > 0 {
				continue;
			}
			else {
				let mut new_pre_set: Vec<i64> = vec!();
				let mut new_post_set: Vec<i64> = vec!();
				for p in &sets[i].pre {
					new_pre_set.push(*p);
				}
				for p in &sets[j].pre {
					new_pre_set.push(*p);
				}
				for p in &sets[i].post {
					new_post_set.push(*p);
				}
				for p in &sets[j].post {
					new_post_set.push(*p);
				}
				let mut new_changeset = ChangeSet{pre: new_pre_set, post: new_post_set, modifier: OrdVar::new(0.0)};
				new_changeset.modifier = OrdVar::new(calculate_modifier(&new_changeset.pre, &new_changeset.post));
				for k in i..sets.len() {
					if sets[k].modifier < new_changeset.modifier {
						let l = new_changeset.modifier.into_inner();
						sets.insert(k,new_changeset);
						println!("j {}", l);
						break;
					}
				}
			}
		}
	}
}

pub fn calculate_modifier(pre: &Vec<i64>, post: &Vec<i64>) -> f64 {
	let mut modifier = 1.0;
	for p in pre {
		modifier *= *p as f64 /2.0;
	}
	for p in post {
		modifier *= *p as f64;
	}
	modifier
}

pub fn primes_as_changesets(max: i64) -> Vec<ChangeSet> {
	let mut primes = vec!();
	primes.push(ChangeSet{pre: vec!(2), post: vec!(), modifier: OrdVar::new(1.0f64)});
	primes.push( ChangeSet{pre: vec!(3), post: vec!(), modifier: OrdVar::new(1.5f64)});
	let mut checking: i64 = 5;
	let mut failed = false;
	while checking <= max {
		let sq = (checking as f64).sqrt();
		for changeset in primes.iter() {
			let p = changeset.pre[0];
			if checking % p == 0  {
				failed = true;
				break;
			}
			if p as f64 > sq {
				break;
			}
		}
		if !failed {
			primes.push(ChangeSet{pre: vec!(checking), post: vec!(), modifier: OrdVar::new(checking as f64 / 2.0)});
		}
		failed = false;
		checking += 2;
	}
	primes
}

pub fn generate_post_changesets(max: i64, primes: &Vec<ChangeSet>) -> Vec<ChangeSet> {
	let mut posts = vec!();
	for i in 2..max+1 {
		let post_list = list_factors(i, primes);
		let mut modifier = 1.0;
		for p in &post_list {
			modifier *= *p as f64;
		}
		posts.push(ChangeSet{pre: vec!(), post: post_list, modifier: OrdVar::new(modifier)});
	}
	posts
}

pub fn list_factors(to_factor: i64, primes:  &Vec<ChangeSet>) -> Vec<i64> {
	let mut primes_iter = primes.iter();
	let mut n = to_factor;
	let mut p = primes_iter.next().unwrap().pre[0];
	let mut list = vec!();
	while n != 1 {
		if n % p == 0 {
			list.push(p);
			n /= p;
		} else {
			match primes_iter.next() {
                Some(val) => p = val.pre[0],
                None => break,
            };
		}
	}
	list
}
