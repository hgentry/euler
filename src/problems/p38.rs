use utils::math;

pub fn solve() -> i64 {
    let mut max = 0;
    let digits = vec![1,2,3,4,5,6,7,8,9];
    for i in 0.. 10000 {
        for j in 1 .. 11 {
            let mut sum = "".to_string();
            for k in 1..j+1 {
                sum = format!("{}{}",&sum,(i*k));
            }
            if sum.len() > 9 {
                break;
            }
            let sum_int = sum.parse::<i64>().unwrap();
            if math::is_pandigital(sum_int, &digits) && sum_int > max {
                max = sum_int;
            }
        }
    }
    return max;
}
