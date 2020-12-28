use utils::primes;

pub fn solve() -> i64 {
    return solve_v2();
}

pub fn solve_v1() -> i64 {
    let primes = primes::list_primes(30000);
    for i in 26033..26034 {
        let primelist = prime_breakdown(i, 5, &primes,1);
        for j in primelist {
            if check_permutation(&j) {
                let mut out = "".to_string();
                let mut sum = 0;
                for k in &j {
                    out = format!("{} {}", out, k);
                    sum += k;
                }
                println!("{}", out);
                return sum;
            }
        }
    }
    return 0;
}

pub fn prime_breakdown(n: i64, l: i64, primes: &Vec<i64>, ind: i64) -> Vec<Vec<i64>> {
    let mut good_perms = vec!();
    if l == 1 {
        if primes::is_prime(n) && n > primes[ind as usize - 1] {
            good_perms.push(vec!(n));
            return good_perms;
        } else {
            return good_perms;
        }
    }
    for k in ind..primes.len() as i64 {
        let p = k as usize;
        if primes[p] == 5 {
            continue;
        }
        if primes[p] < n {
            let res = prime_breakdown(n-primes[p], l-1, primes, p as i64 + 1);
            for v in &res {
                if v.len() as i64 == l - 1 {
                    let mut add: Vec<i64> = vec!();
                    for x in v {
                        add.push(*x);
                    }
                    add.push(primes[p]);
                    if l >= 3 {
                        if check_permutation(&add) && !contains_vec(&good_perms, &add) {
                            good_perms.push(add);
                            
                        }
                    } else {
                        good_perms.push(add);
                    }
                    
                }
            }
        } else {
            break;
        }
    }
    return good_perms;
}

pub fn check_permutation(v: &Vec<i64>) -> bool {

    for i in 0..v.len() {
        for j in 0..v.len() {
            if i != j {
                let out = format!("{}{}", v[i], v[j]);
                let x: i64 = out.parse().unwrap();
                if !primes::is_prime(x) {
                    return false;
                }
            }
        }
    }
    if v.len() == 5 {
        out_vec(&v);
    }
    return true;
}

pub fn out_vec(v: &Vec<i64>) {
    let mut out = "".to_string();
                for k in v {
                    out = format!("{} {}", out, *k);
                }
                println!("{}", out);
}

pub fn contains_vec(v1: &Vec<Vec<i64>>, v2: &Vec<i64>) -> bool {
    for v in v1 {
        let mut matches = true;
        for v1b in 0..v.len() {
            if v[v1b] != v2[v1b] {
                matches = false;
                break;
            }
        }
        if matches {
            return true;
        }
    }
    return false;
}

pub fn solve_v2() -> i64 {
    let primes = primes::list_primes(10000);
    let max = 27000;
    let mut solutions_2 = vec!();
    for i in 792..max {
        let two = prime_breakdown(i, 2, &primes, 1);
        for t in two {
            if check_permutation(&t)  && !contains_vec(&solutions_2, &t) {
                solutions_2.push(t);
            }
        }
    }


    
    let mut solutions_3: Vec<Vec<i64>> = vec!();
    for i in 792..max {
        for t in &solutions_2 {
            let mut sum = 0;
            for t2 in t {
                sum += *t2;
            }
            if sum > i {
                continue;
            }
            
                if i-sum > t[t.len() - 1] && primes::is_prime(i - sum) && !t.contains(&(i-sum)) {
                    let mut to_test: Vec<i64> = vec!();
                    for t2 in t {
                        to_test.push(*t2);
                    }
                    to_test.push(i-sum);
                    if check_permutation(&to_test)  && !contains_vec(&solutions_3, &to_test) {
                        solutions_3.push(to_test);
                    }
                }
            
        }
    }
    

    let mut solutions_4: Vec<Vec<i64>> = vec!();
    for i in 792..max {
        for t in &solutions_3 {
            let mut sum = 0;
            for t2 in t {
                sum += *t2;
            }
            
                if i-sum > t[t.len() - 1] && primes::is_prime(i - sum) && !t.contains(&(i-sum)) {
                    let mut to_test: Vec<i64> = vec!();
                    for t2 in t {
                        to_test.push(*t2);
                    }
                    to_test.push(i-sum);
                    to_test.sort();
                    if !contains_vec(&solutions_4, &to_test) {
                        if check_permutation(&to_test)  {
                            solutions_4.push(to_test);
                        }
                        
                    }
                }
            
        }
    }
    
    let mut solutions_5: Vec<Vec<i64>> = vec!();
    for i in 1..max {
        for t in &solutions_4 {
            let mut sum = 0;
            for t2 in t {
                sum += *t2;
            }
                if i-sum > t[t.len() - 1] && primes::is_prime(i - sum) && !t.contains(&(i-sum)) {
                    let mut to_test: Vec<i64> = vec!();
                    for t2 in t {
                        to_test.push(*t2);
                    }
                    to_test.push(i-sum);
                    to_test.sort();
                    if !contains_vec(&solutions_5, &to_test) {
                        if check_permutation(&to_test)  {
                            solutions_5.push(to_test);
                        }
                        
                    }
            
            }
        }
    }

    return solutions_4.len() as i64;
}