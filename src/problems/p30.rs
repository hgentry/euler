pub fn solve() -> i64 {
    let fifths: Vec<i64> = vec![0, 1, 32, 243, 1024, 3125, 7776, 16807, 32768, 59049];
    let mut sum: i64 = 0;
    for i in 2..250000 {
        let mut n: i64 = i;
        let mut i_sum: i64 = 0;
        while n >= 1 {
            let digit = n % 10;
            n = n / 10;
            i_sum += fifths[digit as usize];
        }
        if i_sum == i {
            sum += i_sum;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct() {
        assert_eq!(solve(), 443839);
    }
}
