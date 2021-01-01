pub fn solve() -> i64 {
    let sum_of_squares = sum_squares(100);
    let square_of_sum = square_sum(100);
    square_of_sum - sum_of_squares
}

fn sum_squares(x: i64) -> i64 {
    let mut sum = 0;
    for i in 1..x + 1 {
        sum += i * i;
    }
    sum
}

fn square_sum(x: i64) -> i64 {
    let mut sum = 0;
    for i in 1..x + 1 {
        sum += i;
    }
    sum * sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct() {
        assert_eq!(solve(), 25164150);
    }
}
