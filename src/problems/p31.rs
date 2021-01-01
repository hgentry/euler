pub fn solve() -> i64 {
    let coin_values = vec![200, 100, 50, 20, 10, 5, 2, 1];
    let target = 200;
    coin_combos(coin_values, target)
}

pub fn coin_combos(coin_values: Vec<i64>, target: i64) -> i64 {
    if target == 0 || coin_values.len() == 1 {
        return 1;
    }
    let mut working_total = 0;
    let mut ways = 0;

    while working_total <= target {
        let mut sub_coin_values = coin_values.clone();
        sub_coin_values.remove(0);
        ways += coin_combos(sub_coin_values, target - working_total);
        working_total += coin_values[0];
    }
    return ways;
}
