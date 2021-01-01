//TODO: This is fast enough, but can be generalized for arbitrary a, b

pub fn solve() -> i64 {
    let mut distinct: i64 = 81 * 99 + 4 * 149;
    let vals_with_dups = vec![vec![2, 4, 8, 16, 32, 64], vec![3, 9, 27, 81]];
    for row in vals_with_dups.iter() {
        let mut distinct_pows: Vec<i64> = vec![];
        for i in 2..101 {
            distinct_pows.push(i as i64);
        }
        for i in 0..row.len() {
            let pow = i + 1;
            let mut exp = 2;
            while exp <= 100 {
                let new_pow: i64 = exp * pow as i64;
                if !distinct_pows.contains(&new_pow) {
                    distinct_pows.push(new_pow);
                }
                exp += 1;
            }
        }
        distinct += distinct_pows.len() as i64;
    }
    distinct
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct() {
        assert_eq!(solve(), 9183);
    }
}
