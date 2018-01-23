/*
The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.

There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.

How many circular primes are there below one million?
*/

use utils;

pub fn solve() -> i64 {
    let mut primes = vec!(2,3);
    let mut checking = 5;
    let mut count = 0;
    while checking < 1000000 {
        let upper_limit = (checking as f64).sqrt() as i64 + 1;
        let mut found = false;
        let len = primes.len();
	    for i in 0..len {
            let p = primes[i];
            if p > upper_limit {
                found = true;
                break;
            } else {
                if checking % p == 0 {
                    break;
                }
            }
            if i == len-1 {
                found = true;
                break;
            }
        }
        if found {
            primes.push(checking);
        }
        checking += 2;
    }
    

    for p in 0..primes.len() {
        let prime = primes[p];
        let mut parse = prime;
        let mut digits = vec!();
        let mut failure = false;
        while parse != 0 {
            let digit = parse % 10;
            if digit % 2 == 0 && prime != 2 ||  digit % 5 == 0 && prime != 5{
                failure = true;
                break;
            }
            parse /= 10;
            digits.insert(0,digit);
        }
        if failure {
            continue;
        }
        let len = digits.len();
        for i in 0..len {
            let mut val = 0;
            for j in 0..len {
                val += digits[(i+j)%len]*utils::math::pow(10,(len-j-1) as i64);
            }
            if !primes.contains(&val) {
                failure = true;
                break;
            }
        }
        if failure {
            continue;
        } else {
            count += 1;
        }
    }
    count as i64
}
