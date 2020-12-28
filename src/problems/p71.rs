use utils::math;
use std::cmp::Ordering;

pub fn solve() -> i64 {
    let max = 1000000;
    let ts = 7.0/3.0;
    let mut best = (0,1);
    for i in 1..= max {
        let j_min = (ts*(i as f64)).floor() as i64;
        if j_min == 0 {
            continue;
        }
        for j in j_min+1..=max {
            //et reduced = math::reduce_fraction(i as i64,j as i64);
            //println!("{}/{} {}", reduced.0, reduced.1, j_min);
            if compare_fractions((i,j),best) == Ordering::Greater {
                best = (i,j);
            } else {
                break;
            }
        }
    }
    let mut result = best;
    result = math::reduce_fraction(result.0, result.1);
    return result.0;
}

pub fn compare_fractions(a: (i64, i64), b: (i64, i64)) -> Ordering {
    let n1 = a.0 * b.1;
    let n2 = a.1 * b.0;
    if n1 < n2 {
        return Ordering::Less;
    }
    if n1 == n2 {
        return Ordering::Equal;
    }
    if n1 > n2 {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}