/*
The decimal number, 585 = 1001001001 (binary), is palindromic in both bases.

Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.

(Please note that the palindromic number, in either base, may not include leading zeros.)

*/

pub fn solve() -> i64 {
	let mut sum: i64 = 0;
	for i in 1..1000000 {
		if is_palindrome(i.to_string()) && is_palindrome(to_binary_str(i as i64)) {
			sum += i;
		}
	}
	sum
}

pub fn to_binary_str(x: i64) -> String {
	format!("{:b}", x)
}

pub fn is_palindrome(x_str: String) -> bool {
	x_str.bytes().eq(x_str.bytes().rev())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct() {
		assert_eq!(solve(), 872187);
	}
}
