use utils::math;
pub fn solve() -> i64 {
    let mut index = 1;
    let mut length = 0;
    let mut product = 1;
    let mut count = 1;
    while length < 1000000 {
        let mut increase = (count as f64).log10().ceil() as i64;
        if count == math::pow(10, increase) {
            increase += 1;
        }
        if length + increase >= index {
            let str = to_vec(count);
            let digit = str.len() - (length + increase - index + 1) as usize;
            product *= str[digit];
            index *= 10;
        }
        length += increase;
        count += 1;
    }
    product
}

pub fn to_vec(x: i64) -> Vec<i64> {
    let mut result = vec![];
    let mut n = x;
    while n > 0 {
        result.push(n % 10);
        n /= 10;
    }
    result.reverse();
    result
}
