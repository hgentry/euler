pub fn solve() -> i64 {
    let mut count = 0;
    for n in 1..=100 {
        for r in 1..n {
            if combination_exceeds(n,r,1000000) {
                count += 1;
            }
        }
    }
    return count;
}

pub fn combination_exceeds(n: i64, r: i64, limit: i64) -> bool {
    let mut start = r + 1;
    let mut denom = 1;
    let mut result = 1;

    while denom <= n - r && start <= n {
        result *= start;
        result /= denom;
        start += 1;
        denom += 1;
        if result > limit {
            return true;
        }
    }

    while denom <= n - r {
        result /= denom;
        denom += 1;
    }
    while start <= n {
        result *= start;
        start += 1;
        if result > limit {
            return true;
        }
    }
    return false;
}