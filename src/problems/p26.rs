pub fn solve() -> i64 {
    let mut max = 0;
    let mut maxindex = 0;
    for i in 3..1000 {
        let mut j = i;
        while j % 5 == 0 {
            j /= 5;
        }
        while j % 2 == 0 {
            j /= 2;
        }
        let mut k = 0;
        let mut pos = 1;
        loop {
            pos = pos * 10 % j;
            k += 1;
            if pos == 1 {
                if k >= max {
                    max = k;
                    maxindex = i;
                }
                break;
            }
            if pos == 0 {
                break;
            }
        }
    }

    return maxindex;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct() {
        assert_eq!(solve(), 983);
    }
}
