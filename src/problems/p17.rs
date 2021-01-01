use std::str::FromStr;
	
pub fn solve() -> i64 {
	let mut sum = 0;
	for i in 1..1001 {
		let s = to_words(i);
		for c in s.chars() {
			if c != ' ' && c != '-' {
				sum += 1;
			}
		}
	}
	return sum;
}


pub fn to_words(x: i64) -> String {
	if x == 1 {
		return String::from_str("one").unwrap();
	}
	if x == 2 {
		return String::from_str("two").unwrap();
	}
	if x == 3 {
		return String::from_str("three").unwrap();
	}
	if x == 4 {
		return String::from_str("four").unwrap();
	}
	if x == 5 {
		return String::from_str("five").unwrap();
	}
	if x == 6 {
		return String::from_str("six").unwrap();
	}
	if x == 7 {
		return String::from_str("seven").unwrap();
	}
	if x == 8 {
		return String::from_str("eight").unwrap();
	}
	if x == 9 {
		return String::from_str("nine").unwrap();
	}
	if x == 10 {
		return String::from_str("ten").unwrap();
	}
	if x == 11 {
		return String::from_str("eleven").unwrap();
	}
	if x == 12 {
		return String::from_str("twelve").unwrap();
	}
	if x == 13 {
		return String::from_str("thirteen").unwrap();
	}
	if x == 14 {
		return String::from_str("fourteen").unwrap();
	}
	if x == 15 {
		return String::from_str("fifteen").unwrap();
	}
	if x == 16 {
		return String::from_str("sixteen").unwrap();
	}
	if x == 17 {
		return String::from_str("seventeen").unwrap();
	}
	if x == 18 {
		return String::from_str("eighteen").unwrap();
	}
	if x == 19 {
		return String::from_str("nineteen").unwrap();
	}
	if x == 20 {
		return String::from_str("twenty").unwrap();
	}
	if x == 30 {
		return String::from_str("thirty").unwrap();
	}
	if x == 40 {
		return String::from_str("forty").unwrap();
	}
	if x == 50 {
		return String::from_str("fifty").unwrap();
	}
	if x == 60 {
		return String::from_str("sixty").unwrap();
	}
	if x == 70 {
		return String::from_str("seventy").unwrap();
	}
	if x == 80 {
		return String::from_str("eighty").unwrap();
	}
	if x == 90 {
		return String::from_str("ninety").unwrap();
	}
	if x == 1000 {
		return String::from_str("one thousand").unwrap();
	}

	if x >= 100 {
		if x % 100 != 0 {
			return format!("{} hundred and {}", to_words(x/100), to_words(x%100));
		} else {
			return format!("{} hundred", to_words(x/100));
		}
	}
	if x < 100 {
		 return format!("{}-{}", to_words(x/10*10), to_words(x%10));
	}

	return "".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
	
	#[test]
    fn correct() {
		assert_eq!(solve(), 21124);
    }
}
