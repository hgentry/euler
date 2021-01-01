use num::bigint::*;
use utils;

pub fn solve() -> BigInt {
    utils::math::combination_big(40, 20)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct() {
        assert_eq!(solve(), (137846528820 as i128).to_bigint().unwrap());
    }
}
