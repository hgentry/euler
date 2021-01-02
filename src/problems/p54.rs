//376

use substring::Substring;
use crate::utils::strings;

pub fn solve() -> i64 {
	let mut count = 0;
	let rounds = strings::read_file_to_string("input/poker.txt");
	let rounds_split = rounds.split('\n');
	for round in rounds_split {
		let round_s = round.to_string();
		let p1_hand = round_s.substring(0, 14);
		let p2_hand = round_s.substring(15, 29);
		let p1 = PokerHand::from_str(p1_hand.to_string());
		let p2 = PokerHand::from_str(p2_hand.to_string());
		let cmp = p1.cmp(&p2);
		if cmp == 1 {
			count += 1;
		}
	}

	return count;
}

#[derive(Clone)]
struct PokerHand {
	cards: Vec<PokerCard>,
	pub tier: i64,
	pub rank: i64,
}

impl PokerHand {
	pub fn from_cards(mut cards: Vec<PokerCard>) -> PokerHand {
		cards.sort_by_key(|x| x.rank);
		let mut hand = PokerHand {
			cards: cards,
			tier: 0,
			rank: 0,
		};
		hand.determine_tier();
		return hand;
	}

	pub fn to_string(&self) -> String {
		let mut out = "".to_string();
		for v in 0..self.cards.len() {
			out = format!("{} {}", out, self.cards[v].to_string());
		}
		return out;
	}

	pub fn from_str(s: String) -> PokerHand {
		let s_v = s.split(' ');
		let mut cards: Vec<PokerCard> = vec![];
		for v in s_v {
			cards.push(PokerCard::new(v));
		}
		//println!("{}",cards.len());
		return PokerHand::from_cards(cards);
	}

	pub fn pair_count(&self) -> PairCountResult {
		let mut best_found_rank = 0;
		let mut best_found_count = 0;

		for i in 0..self.cards.len() {
			let mut found = 1;
			for j in i + 1..self.cards.len() {
				if self.cards[j].rank == self.cards[i].rank {
					found += 1;
				}
			}
			if found >= best_found_count {
				best_found_rank = self.cards[i].rank;
				best_found_count = found;
			}
		}
		if best_found_count == 4 {
			return PairCountResult {
				tier: "Four Of A Kind".to_string(),
				rank: best_found_rank,
			};
		}
		if best_found_count == 3 {
			let mut pair_check = PairCountResult {
				tier: "".to_string(),
				rank: 0,
			};
			if self.cards.len() == 5 {
				let mut less_cards: Vec<PokerCard> = vec![];
				for i in 0..self.cards.len() {
					if self.cards[i].rank != best_found_rank {
						less_cards.push(self.cards[i]);
					}
				}
				let smaller_hand = PokerHand::from_cards(less_cards);
				pair_check = smaller_hand.pair_count();
			}
			if self.cards.len() == 5 && pair_check.tier == "One Pair" {
				return PairCountResult {
					tier: "Full House".to_string(),
					rank: best_found_rank,
				};
			} else {
				return PairCountResult {
					tier: "Three Of A Kind".to_string(),
					rank: best_found_rank,
				};
			}
		}
		if best_found_count == 2 {
			let mut pair_check = PairCountResult {
				tier: "".to_string(),
				rank: 0,
			};
			if self.cards.len() == 5 {
				let mut less_cards: Vec<PokerCard> = vec![];
				for i in 0..self.cards.len() {
					if self.cards[i].rank != best_found_rank {
						less_cards.push(self.cards[i]);
					}
				}
				let smaller_hand = PokerHand::from_cards(less_cards);
				pair_check = smaller_hand.pair_count();
			}
			if self.cards.len() == 5 && pair_check.tier == "One Pair" {
				return PairCountResult {
					tier: "Two Pair".to_string(),
					rank: best_found_rank,
				};
			} else {
				return PairCountResult {
					tier: "One Pair".to_string(),
					rank: best_found_rank,
				};
			}
		}

		return PairCountResult {
			tier: "High Card".to_string(),
			rank: best_found_rank,
		};
	}

	pub fn is_straight(&self) -> bool {
		for i in 0..self.cards.len() {
			if self.cards[i].rank != self.cards[0].rank + i as i64 {
				return false;
			}
		}
		return true;
	}

	pub fn is_flush(&self) -> bool {
		for i in 0..self.cards.len() {
			if self.cards[i].suit != self.cards[0].suit {
				return false;
			}
		}
		return true;
	}

	pub fn is_straight_flush(&self) -> bool {
		return self.is_straight() && self.is_flush();
	}

	pub fn is_royal_flush(&self) -> bool {
		return self.is_straight_flush() && self.cards[0].rank == 10;
	}

	pub fn determine_tier(&mut self) {
		if self.is_royal_flush() {
			self.tier = 10;
			self.rank = 14;
			return;
		}
		if self.is_straight_flush() {
			self.tier = 9;
			self.rank = self.cards[self.cards.len() - 1].rank;
			return;
		}
		let pairs = self.pair_count();
		if pairs.tier == "Four Of A Kind" {
			self.tier = 8;
			self.rank = pairs.rank;
			return;
		}
		if pairs.tier == "Full House" {
			self.tier = 7;
			self.rank = pairs.rank;
			return;
		}
		if self.is_flush() {
			self.tier = 6;
			self.rank = self.cards[self.cards.len() - 1].rank;
			return;
		}
		if self.is_straight() {
			self.tier = 5;
			self.rank = self.cards[self.cards.len() - 1].rank;
			return;
		}
		if pairs.tier == "Three Of A Kind" {
			self.tier = 4;
			self.rank = pairs.rank;
			return;
		}
		if pairs.tier == "Two Pair" {
			self.tier = 3;
			self.rank = pairs.rank;
			return;
		}
		if pairs.tier == "One Pair" {
			self.tier = 2;
			self.rank = pairs.rank;
			return;
		}
		if pairs.tier == "High Card" {
			self.tier = 1;
			self.rank = pairs.rank;
			return;
		}
	}

	pub fn cmp(&self, p2: &PokerHand) -> i64 {
		if self.tier > p2.tier {
			return 1;
		}
		if p2.tier > self.tier {
			return -1;
		}
		if self.rank > p2.rank {
			return 1;
		}
		if p2.rank > self.rank {
			return -1;
		}
		for i in 0..self.cards.len() {
			if self.cards[self.cards.len() - i - 1].rank > p2.cards[self.cards.len() - i - 1].rank {
				return 1;
			}
			if self.cards[self.cards.len() - i - 1].rank < p2.cards[self.cards.len() - i - 1].rank {
				return -1;
			}
		}
		return 0;
	}
}

#[derive(Copy, Clone)]
struct PokerCard {
	rank: i64,
	suit: i64,
}

impl PokerCard {
	pub fn new(s: &str) -> PokerCard {
		let rank;
		let s_v: Vec<char> = s.chars().collect();
		match s_v[0] {
			'T' => rank = 10,
			'J' => rank = 11,
			'Q' => rank = 12,
			'K' => rank = 13,
			'A' => rank = 14,
			_ => rank = s_v[0] as i64 - '0' as i64,
		};
		//println!("{} {}", s_v[0], rank);
		let suit;
		match s_v[1] {
			'H' => suit = 0,
			'D' => suit = 1,
			'C' => suit = 2,
			'S' => suit = 3,
			_ => suit = 5,
		};
		//println!("{} {}", s_v[1], suit);
		return PokerCard {
			rank: rank,
			suit: suit,
		};
	}

	pub fn to_string(&self) -> String {
		let rank;
		let suit;
		match self.rank {
			14 => rank = "A".to_string(),
			13 => rank = "K".to_string(),
			12 => rank = "Q".to_string(),
			11 => rank = "J".to_string(),
			10 => rank = "T".to_string(),
			_ => rank = self.rank.to_string(),
		};
		match self.suit {
			0 => suit = "H".to_string(),
			1 => suit = "D".to_string(),
			2 => suit = "C".to_string(),
			3 => suit = "S".to_string(),
			_ => suit = self.suit.to_string(),
		};
		return format!("{}{}", rank, suit);
	}
}

struct PairCountResult {
	tier: String,
	rank: i64,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 376);
	}
}
