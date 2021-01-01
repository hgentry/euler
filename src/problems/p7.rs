use utils;

pub fn solve() -> i64 {
    let mut primes = 3;
    let mut checking = 5;
    loop {
        checking += 2;
        if utils::primes::is_prime(checking) {
            primes += 1;
            if primes == 10001 {
                return checking;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct() {
        assert_eq!(solve(), 104743);
    }
}
