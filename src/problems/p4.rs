use utils::math;

pub fn solve() -> i64 {
    let mut best = 1;
    let mut i = 100;
    let mut j = 100;
    while i <= 999 {
        while j <= i {
            let current = i * j;
            if current > best {
                if math::is_palindrome(&current) {
                    best = current;
                }
            }
            j += 1;
        }
        i += 1;
        j = 100;
    }
    return best;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct() {
        assert_eq!(solve(), 906609);
    }
}
