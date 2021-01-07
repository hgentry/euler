use crate::utils::strings;
use std::cmp::min;

pub fn solve() -> i64 {
	let input_str = strings::read_file_to_string("input/matrix.txt");
	let input_vec_1 = input_str.split("\n");
	let mut input_vec_2 = vec![];
	for s in input_vec_1 {
		input_vec_2.push(s.split(","));
	}
	let mut input: Vec<Vec<i64>> = vec![];
	for s in input_vec_2 {
		input.push(vec![]);
		let l = input.len();
		for t in s {
			input[l - 1].push(t.parse().unwrap());
		}
	}
	let mut result = i64::MAX;
	let len = input.len();

	let mut g = build_graph(&input);

	// Apply the algorithm!
	loop {
		let mut done = true;
		let count = g.node_count();
		for n_i in 0..count {
			let n = g.get_node(n_i as i64).clone();
			if n.outgoing.len() != 0 {
				let t;
				if n_i % len == 0 {
					if n_i == 0 {
						t = n.initial_content;
					} else {
						t = 31000000; //max is too high so random constant lol
					}
				} else {
					t = n.content;
				}
				for n_j in 0..n.outgoing.len() {
					let n2 = g.get_node(n.outgoing[n_j]).clone();
					let t2 = n2.content;
					let t2i = n2.initial_content;
					if t2 == t2i {
						g.update_node_content(&n2, t + t2i);
						done = false;
					} else {
						g.update_node_content(&n2, min(t2, t + t2i));
						if t2 > t + t2i {
							done = false;
						}
					}
				}
			}
		}
		if done {
			break;
		}

		result = g.get_node((g.node_count() - 1) as i64).content;
	}
	return result;
}

pub fn build_graph(input: &Vec<Vec<i64>>) -> Graph<i64> {
	let mut g = Graph::<i64>::new();
	let len = input.len() as i64;
	for y in 0..len {
		for x in 0..len {
			g.add_node(input[y as usize][x as usize]);
		}
	}

	for y in 0..len {
		for x in 0..len {
			let node1 = g.get_node(y * len + x).clone();
			if y < len - 1 && x != len - 1 {
				let node2 = g.get_node((y + 1) * len + x).clone();
				g.add_directional_link(&node1, &node2);
			}
			if x < len - 1 {
				let node2 = g.get_node(y * len + x + 1).clone();
				g.add_directional_link(&node1, &node2);
			}
		}
	}

	return g;
}

pub struct Graph<T: Clone> {
	nodes: Vec<Node<T>>,
}

#[derive(Clone)]
pub struct Node<T: Clone> {
	removed: bool,
	incoming: Vec<i64>,
	outgoing: Vec<i64>,
	pos: i64,
	initial_content: T,
	content: T,
}

impl<T: Clone> Graph<T> {
	pub fn new() -> Graph<T> {
		return Graph { nodes: vec![] };
	}

	pub fn node_count(&self) -> usize {
		return self.nodes.len();
	}

	pub fn get_node(&self, x: i64) -> &Node<T> {
		return &self.nodes[x as usize];
	}

	pub fn add_node(&mut self, content: T) -> &Node<T> {
		let n = Node {
			incoming: vec![],
			outgoing: vec![],
			content: content.clone(),
			pos: 0,
			removed: false,
			initial_content: content.clone(),
		};
		self.nodes.push(n);
		let l = self.nodes.len();
		self.nodes[l - 1].pos = self.nodes.len() as i64 - 1;
		return &self.nodes[self.nodes.len() - 1];
	}

	pub fn remove_node(&mut self, a: &Node<T>) {
		self.nodes[a.pos as usize].removed = true;
		while self.nodes[a.pos as usize].incoming.len() != 0 {
			let na = self.nodes[a.pos as usize].clone();
			let nb = self.nodes[self.nodes[a.pos as usize].incoming[0] as usize].clone();
			self.remove_link(&na, &nb);
		}
		while self.nodes[a.pos as usize].outgoing.len() != 0 {
			let na = self.nodes[a.pos as usize].clone();
			let nb = self.nodes[self.nodes[a.pos as usize].outgoing[0] as usize].clone();
			self.remove_link(&na, &nb);
		}
	}

	pub fn update_node_content(&mut self, n: &Node<T>, content: T) {
		let pos = n.pos as usize;
		self.nodes[pos].content = content;
	}

	pub fn add_directional_link(&mut self, a: &Node<T>, b: &Node<T>) {
		self.nodes[a.pos as usize].outgoing.push(b.pos);
		self.nodes[b.pos as usize].incoming.push(a.pos);
	}

	pub fn add_link(&mut self, a: &Node<T>, b: &Node<T>) {
		self.add_directional_link(a, b);
		self.add_directional_link(b, a);
	}

	pub fn remove_directional_link(&mut self, a: &Node<T>, b: &Node<T>) {
		let p_a = a.pos;
		let p_b = b.pos;
		for i_i in 0..self.nodes[p_a as usize].outgoing.len() {
			if i_i >= self.nodes[p_a as usize].outgoing.len() {
				break;
			}
			while self.nodes[p_a as usize].outgoing[i_i] == p_b {
				self.nodes[p_a as usize].outgoing.remove(i_i);
				if i_i >= self.nodes[p_a as usize].outgoing.len() {
					break;
				}
			}
		}
		for i_i in 0..self.nodes[p_b as usize].incoming.len() {
			if i_i >= self.nodes[p_b as usize].incoming.len() {
				break;
			}
			while self.nodes[p_b as usize].incoming[i_i] == p_a {
				self.nodes[p_b as usize].incoming.remove(i_i);
				if i_i >= self.nodes[p_b as usize].incoming.len() {
					break;
				}
			}
		}
	}

	pub fn remove_link(&mut self, a: &Node<T>, b: &Node<T>) {
		self.remove_directional_link(a, b);
		self.remove_directional_link(b, a);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 427337);
	}
}
