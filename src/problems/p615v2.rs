use std::collections::BTreeMap;
use ord_subset::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ChangeSet {
	pre: Vec<i64>,
	post: Vec<i64>
}

pub fn solve() -> i64 {
	let primes = RefCell::new(primes_as_changesets(100000));
	let posts = RefCell::new(generate_post_changesets(20000, &primes.borrow()));
	{
		posts.borrow_mut().append(&mut primes.borrow());
	}
	expand_changesets(&posts);
	let len = posts.borrow().len();
	len as i64
}

pub fn expand_changesets(sets: &RefCell<BTreeMap<OrdVar<f64>, ChangeSet>>) {
	for (modifier, set) in sets.borrow().iter() {
		for (earlier_modifier, earlier_set) in sets.borrow().iter() {
			if earlier_modifier > modifier || earlier_set.post.len() > 0 {
				break;
			}
			else {;
				let mut new_pre_set: Vec<i64> = vec!();
				let mut new_post_set: Vec<i64> = vec!();
				for p in &set.pre {
					new_pre_set.push(*p);
				}
				for p in &earlier_set.pre {
					new_pre_set.push(*p);
				}
				for p in &set.post {
					new_post_set.push(*p);
				}
				for p in &earlier_set.post {
					new_post_set.push(*p);
				}
				let mut new_changeset = ChangeSet{pre: new_pre_set, post: new_post_set};
				let new_modifier = calculate_modifier(&new_changeset.pre, &new_changeset.post);
				sets.borrow_mut().insert(OrdVar::new(new_modifier),new_changeset);
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

pub fn primes_as_changesets(max: i64) -> BTreeMap<OrdVar<f64>, ChangeSet> {
	let mut primes = BTreeMap::new();
	//let mut primes = vec!(2,3);
	primes.insert(OrdVar::new(1.0), ChangeSet{pre: vec!(2), post: vec!()});
	primes.insert(OrdVar::new(1.5), ChangeSet{pre: vec!(3), post: vec!()});
	let mut checking: i64 = 5;
	let mut failed = false;
	while checking <= max {
		let sq = (checking as f64).sqrt();
		for (_, changeset) in primes.iter() {
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
			primes.insert(OrdVar::new(checking as f64 / 2.0), ChangeSet{pre: vec!(checking), post: vec!()});
		}
		failed = false;
		checking += 2;
	}
	primes
}

pub fn generate_post_changesets(max: i64, primes: &BTreeMap<OrdVar<f64>, ChangeSet>) -> BTreeMap<OrdVar<f64>, ChangeSet> {
	let mut posts = BTreeMap::new();
	for i in 2..max+1 {
		let post_list = list_factors(i, primes);
		let mut modifier = 1.0;
		for p in &post_list {
			modifier *= *p as f64;
		}
		posts.insert(OrdVar::new(modifier), ChangeSet{pre: vec!(), post: post_list});
	}
	posts
}

pub fn list_factors(to_factor: i64, primes:  &BTreeMap<OrdVar<f64>, ChangeSet>) -> Vec<i64> {
	let mut primes_iter = primes.iter();
	let mut n = to_factor;
	let mut p = primes_iter.next().unwrap().1.pre[0];
	let mut list = vec!();
	while n != 1 {
		if n % p == 0 {
			list.push(p);
			n /= p;
		} else {
			match primes_iter.next() {
                Some(val) => p = val.1.pre[0],
                None => break,
            };
		}
	}
	list
}
