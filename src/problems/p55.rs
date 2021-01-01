use utils::math;

pub fn solve() -> i64 {
    let mut found = 0;
    for i in 1..10000 {
        let mut sum: i128 = i;
        for _ in 0..=50 {
            sum += math::reverse_i128(sum);
            if math::is_palindrome(&sum) {
                found += 1;
                break;
            }
        }
    }
    return 9999-found;

}