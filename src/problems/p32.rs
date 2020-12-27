/*
We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.

The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand, multiplier, and product is 1 through 9 pandigital.

Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.
HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.
*/

pub fn solve() -> i64 {
    let mut sum = 0;
    let mut list = vec!();
    let perms = generate_permutations();
    for i in perms {
        for j in 0..i {
            let n = i*j;
            let mut digits = vec!();
            let parse = vec!(i, j, n);
            let mut failure = false;
            let mut len = 0;
            for k in parse {
                let mut k_p = k;
                while k_p > 0 {
                    let digit = k_p % 10;
                    k_p = k_p / 10;
                    if digit != 0 && !digits.contains(&digit) {
                        digits.push(digit);
                        len += 1;
                    } else {
                        failure = true; break;
                    }
                }
                if failure {
                    break;
                }
            }
            if len != 9 {
                failure = true;
            }
            if failure {
                continue;
            } else {
                if !list.contains(&n) {
                    sum += n;
                    list.push(n);
                }
            }
        }
    }
    sum
}

pub fn generate_permutations() -> Vec<i64> {
    let mut perms = vec!();
    for i in 0..1970 {
        let mut i_p = i;
        let mut digits = vec!();
        let mut failure = false;
        while i_p != 0 {
            let digit = i_p % 10;
            i_p = i_p / 10;
            if !digits.contains(&digit) && digit != 0 {
                digits.push(digit);
            } else {
                failure = true;
                break;
            }
        }
        if failure {
            continue;
        } else {
            perms.push(i);
        }
    }
    perms
}











