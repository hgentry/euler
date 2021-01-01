pub fn solve() -> i64 {
    let mut a0 = 1;
    let mut next_square = 4;
    let mut count = 0;
    for i in 2..=10000 {
        if i == next_square {
            a0 += 1;
            next_square = (a0 + 1) * (a0 + 1);
            continue;
        }

        let ex = expand_root_len(i);
        if ex % 2 == 1 {
            count += 1;
        }
    }
    return count;
}

pub fn expand_root_len(x: i64) -> i64 {
    let mut a0;
    let mut numer;
    let mut denom = 1;
    let mut fact;
    let mut init: (i64, i64, i64) = (0, 0, 0);

    a0 = (x as f64).sqrt() as i64;
    fact = a0;
    for i in 1..=2 * x {
        numer = denom;
        denom = x - fact * fact;
        if i == 1 {
            init = (numer, fact, denom);
        } else {
            if init.0 == numer && init.1 == fact && init.2 == denom {
                return i - 1;
            }
        }
        a0 = (numer as f64 * ((x as f64).sqrt() + (fact) as f64) / (denom as f64)) as i64;
        fact = fact * numer;
        fact -= a0 * denom;

        fact /= numer;
        denom /= numer;
        fact = fact.abs();
    }

    return 0;
}
