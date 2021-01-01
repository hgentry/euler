use utils::math;

pub fn solve() -> i64 {
    let mut most_factors = 0;
    let mut n = 6;
    while most_factors < 500 {
        let factors = math::factors_count(math::triangle(n));

        if factors > most_factors {
            most_factors = factors
        }
        n += 1;
    }
    math::triangle(n - 1) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct() {
        assert_eq!(solve(), 76576500);
    }
}
