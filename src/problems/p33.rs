/*
The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to simplify it may incorrectly believe that 49/98 = 4/8, which is correct, is obtained by cancelling the 9s.

We shall consider fractions like, 30/50 = 3/5, to be trivial examples.

There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two digits in the numerator and denominator.

If the product of these four fractions is given in its lowest common terms, find the value of the denominator.
*/

pub fn solve() -> i64 {
	let mut fraction = (1, 1);
	for bottom in 10..100 {
		for top in 10..bottom {
			if reduce((top, bottom)) == wrong_reduce((top, bottom), 0)
				|| reduce((top, bottom)) == wrong_reduce((top, bottom), 1)
			{
				fraction = multiply(fraction, (top, bottom));
			}
		}
	}
	fraction.1
}

pub fn multiply((top1, bottom1): (i64, i64), (top2, bottom2): (i64, i64)) -> (i64, i64) {
	reduce((top1 * top2, bottom1 * bottom2))
}

pub fn reduce((top, bottom): (i64, i64)) -> (i64, i64) {
	let mut reduced_top = top;
	let mut reduced_bottom = bottom;
	for i in 2..bottom - 1 {
		while reduced_top % i == 0 && reduced_bottom % i == 0 {
			reduced_top /= i;
			reduced_bottom /= i;
		}
	}
	(reduced_top, reduced_bottom)
}

pub fn wrong_reduce((top, bottom): (i64, i64), pos: i64) -> (i64, i64) {
	let mut top_digits = vec![];
	let mut bottom_digits = vec![];
	let n_pos;
	let mut top_parse = top;
	let mut bottom_parse = bottom;

	if pos == 0 {
		n_pos = 1;
	} else {
		n_pos = 0;
	}
	while top_parse != 0 {
		let digit = top_parse % 10;
		top_parse /= 10;
		top_digits.push(digit);
	}
	while bottom_parse != 0 {
		let digit = bottom_parse % 10;
		bottom_parse /= 10;
		bottom_digits.push(digit);
	}

	let removed = top_digits.remove(n_pos as usize);
	let mut bottom_pos = 2;
	for i in 0..2 {
		if bottom_digits[i] == removed {
			bottom_digits.remove(i);
			bottom_pos = i;
			break;
		}
	}
	if removed != 0 && bottom_pos != 2 {
		reduce((top_digits[0], bottom_digits[0]))
	} else {
		(-1, -1)
	}
}
