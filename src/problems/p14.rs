pub fn solve() -> i64 {
    let mut results: Vec<i64> = vec![];
    let mut longest = 0;
    let mut longest_index = 0;
    for i in 1..1000000 {
        let mut count = 1;
        let mut iter = i;
        while iter != 1 {
            if iter % 2 == 0 {
                iter = iter / 2;
                if iter < i {
                    count += 1 + results[iter - 1];
                    break;
                }
            } else {
                iter = 3 * iter + 1;
            }
            count += 1;
        }
        results.append(&mut vec![count as i64]);
        if count > longest {
            longest = count;
            longest_index = i;
        }
    }
    longest_index as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct() {
        assert_eq!(solve(), 837799);
    }
}
