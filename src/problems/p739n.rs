extern crate num_bigint;
use num_bigint::*;
use utils::math;
extern crate colored;
use colored::*;

pub fn solve() -> i64 {
     return solve_v2();
}

pub fn solve_v1() -> i64 {
    return brute_force(8, true);
}

pub fn solve_v7() -> i64 {
    bigint_method_reverse(8);
    return 0;
}

pub fn brute_force(n: i64, generate_pyramid: bool) -> i64 {
    let mut a0 = vec!(1);
    let mut a1 = vec!(3, 3);
    let mut vecs = vec!(a0.to_vec(), a1.to_vec());
    

    for i in 2..n {
        let mut a2 = vec!();
        a2.push(a0[0] + a1[0]);
        for i in 1..a1.len(){
            a2.push((a1[i] + a2[i-1]) % 1000000007);
        }
        a2.push(a2[a2.len()-1]);
        if generate_pyramid {
            vecs.push(a2.to_vec());
        }
        a0 = a1;
        a1 = a2;
    }
    if generate_pyramid  {
        print_pyramid(vecs);
    }
    return a1[a1.len()-1];
}

pub fn print_pyramid(vecs: Vec<Vec<i64>>) {
    for i in 0..vecs.len() {
        for v in &vecs {
            if i < v.len() {
                print!("{:4.} ", v[i]); // should find the length dynamically instead of using 4 but who cares
            } else {
                print!("     ");
            }
        }
        println!("");
    }
}

/*
Problems with this solution

* It's wrong
* Takes too long with all the bigint multiplying

*/
pub fn solve_v2() -> i64 {
    let answers = [7, 21, 67, 223, 763, 2663];
    for i in 3..9 {
        let answer = bigint_method_reverse(i);
        let correct = answer == answers[(i-3) as usize].to_bigint().unwrap();
        let out: String;
        if correct {
            out = format!("{}: {} is correct!",i,answer);
            println!("{}", out.bright_green());
        } else {
            out = format!("{}: {} is inorrect! Expected {}",i,answer, answers[(i-3) as usize]);
            println!("{}", out.bright_red());
        }
    }

    return 0;
}

pub fn bigint_method(n: i64) -> BigInt {
    //Lucas sequence initialization
    let mut a1 = 1;
    let mut a2 = 3;

    let mut factor : BigInt = (1).to_bigint().unwrap();  //Initial factor = 1
    let mut running : BigInt = 0.to_bigint().unwrap();
    let mut factors: String;
    factors = "[".to_string();
    for i in 1..n {
        //Calculate the new factor from the old factor
        //if i <= n/2  {
            let k = i ;
            factor = factor.clone() * (n - k) / (k)-1;
            
       //} else {
           //let k = n - i;
           //factor = factor.clone() * (k) / (n-k);
       //}

       
       //Update the running total
       factors = format!("{}{}, ",factors,factor);
       running = running.clone() + factor.clone() * a2;
       

       //Lucas sequence
       let temp = a2;
       a2 = a1 + a2;
       a1 = temp;
    }
    factors = factors.trim_right().to_string();
    factors = factors.trim_right_matches(',').to_string();
    factors += "]";

    println!("{}: {}",n, factors);
    
    return running;
}

pub fn bigint_method_reverse(n: i64) -> BigInt {
    //Lucas sequence initialization
    let mut a1 = 1;
    let mut a2 = 3;

    let mut factor : BigInt = (1).to_bigint().unwrap();  //Initial factor = 1
    let mut running : BigInt = 0.to_bigint().unwrap();
    let mut factors: String;
    let mut factors = vec!();
    for i in 3..n+1 {
        let temp = a2;
        a2 = a1 + a2;
        a1 = temp;
    }
    let mut distance = 1;
    for i in 1..n {

        //Update the running total
        factors.insert(0,factor.clone());
       running = running.clone() + factor.clone() * a2;
        //Calculate the new factor from the old factor
        if i != n-1 {
            if i == 1 {
                
                distance = 2;
                println!("calculating factor for i={} with dist {} (S)", i+1, distance);

                factor = (n - 2).to_bigint().unwrap();
            }
            else if i < n/2 && n % 2 == 0 || i < n/2 + 1 && n % 2 == 1 {
                distance += 1;
                println!("calculating factor for i={} with dist {} (L)", i+1, distance);
                factor = factor.clone() *  (n - 2 + i - 1) / (i + 1) + 1;
            } else {
                let mut k = 0;
                if !(n % 2 == 0 && i == n/2 || n % 2 == 1 && i == n/2 + 1) {
                    distance -= 1;
                    k = distance - 1;
                    println!("calculating factor for i={} with dist {} (H)", i+1, distance);
                    //factor = factor.clone() *  (i+1) / (n - i + 2);
                    //factor = factor.clone() *  (n - i + 2) / (i+1);
                    factor = factor.clone() * (n-2) / (k );
                } else {
                    k = distance;
                    //factor = factor.clone() *  (i+1) / (n - i + 1);
                    println!("calculating factor for i={} with dist {} (HN)", i+1, distance);
                }
                    
                    
            
            }
        }

       
       
       

       //Lucas sequence
       let temp = a1;
       a1 = a2 - a1;
       a2 = temp;
    }
    
    let mut factor_str: String = "[".to_string();
    for f in factors {
        factor_str = format!("{}{}, ",factor_str, f);
    }
    factor_str = factor_str.trim_right().to_string();
    factor_str = factor_str.trim_right_matches(',').to_string();
    factor_str = format!("{}]",factor_str);
    println!("{}: {}",n, factor_str);
    
    return running;
}

pub fn solve_v3() -> i64 {
    let n: u64 = 8;
    let r: u64 = 1000000007;

    //Lucas sequence initialization
    let mut a1 = 1;
    let mut a2 = 1;

    let mut running : u64 = 0;

    let mut factor_n: u64 = 1;
    let mut factor_d: u64 = 1;

    
    for i in 2..n-1 {
        let k = i - 1;
        factor_n *= n - k;
        factor_d *= k;
        factor_n /= factor_d;
        //factor_n %= r;
         factor_d = 1;
    }
    
    
    for i in 2..n+1 {
        //Update the running total
       running = (running + a2 * factor_n) % r;
        println!("{}: {}", i, factor_n);

        //Calculate the new factor from the old factor
        
        
            let k = n - i;
            factor_n *= k;
            factor_d *= n - k;
            factor_n /= factor_d;
            //factor_n %= r;
            factor_d = 1;
       
       

        //Lucas sequence
        //let temp = a2;
        //a2 = a1 + a2;
        //a1 = temp;
    }
    println!("end result: {}", running);
    
    return 0;
}

pub fn solve_v4() -> i64 {
    let n: u64 = 8;
    let r: u64 = 1000000007;

    //Lucas sequence initialization
    let mut a1 = 1;
    let mut a2 = 1;

    let mut running : u64 = 0;

    let mut factor_n: u64 = 1;
    let mut factor_d: u64 = 1;
    
    for i in 2..n+1 {
        

        //Calculate the new factor from the old factor
        let k = n - 1;
        let j = i - 2;
        factor_n = (n-2)*(n-2) - (i - 3) * (i - 3);


        //Update the running total
       running = (running + a2 * factor_n) % r;
       println!("{}: {} * {}", i, factor_n, a2);


        //Lucas sequence
        //let temp = a2;
        //a2 = a1 + a2;
        //a1 = temp;
    }
    println!("end result: {}", running);
    
    return 0;
}


pub fn solve_v5() -> i64 {
    let n = 8;
    let mut sum = 0;
    let mut a1 = 1;
    let mut a2 = 3;
    let mut factor = 1;

    for i in 3..n+1 {
        let temp = a2;
        a2 = a1 + a2;
        a1 = temp;
    }
    for i in 1..n {
        println!("{}: {}", a2, factor);
        sum += factor*a2;
        if i <= n/2 + 1 {
            factor *= n - i;
            factor /= i;
        } else {
            factor /= n - i;
            factor *= i;
            
        }
        let temp = a1;
        a1 = a2 - a1;
        a2 = temp;
    }

    return sum;
}

pub fn solve_v6() -> i64 {
    let v8 = vec!(3, 4, 7, 11, 18, 29, 47);


    return 0;
}

pub fn bigint_method_reverse_backup(n: i64) -> BigInt {
    //Lucas sequence initialization
    let mut a1 = 1;
    let mut a2 = 3;

    let mut factor : BigInt = (1).to_bigint().unwrap();  //Initial factor = 1
    let mut running : BigInt = 0.to_bigint().unwrap();
    let mut factors: String;
    let mut factors = vec!();
    for i in 3..n+1 {
        let temp = a2;
        a2 = a1 + a2;
        a1 = temp;
    }
    let mut distance = 1;
    for i in 1..n {

        //Update the running total
        factors.insert(0,factor.clone());
       running = running.clone() + factor.clone() * a2;
        //Calculate the new factor from the old factor
        if i != n {
            if i == 1 {
                
                distance = 2;
                println!("calculating factor for i={} with dist {} (S)", i+1, distance);

                factor = (n - 2).to_bigint().unwrap();
            }
            else if i < n/2 && n % 2 == 0 || i < n/2 + 1 && n % 2 == 1 {
                distance += 1;
                let k = distance;
                println!("calculating factor for i={} with dist {} (L)", i+1, distance);
                factor = factor.clone() *  (n - i + 2) / (i+1);
            } else {
                let mut k = 0;
                if !(n % 2 == 0 && i == n/2) {
                    distance -= 1;
                    k = distance + 1;
                    k = distance;
                    println!("calculating factor for i={} with dist {} (H)", i+1, distance);
                    factor = factor.clone() *  (n - i+1) / (i);
                } else {
                    k = distance;
                    println!("calculating factor for i={} with dist {} (HN)", i+1, distance);
                }
                    
                    
            
            }
        }

       
       
       

       //Lucas sequence
       let temp = a1;
       a1 = a2 - a1;
       a2 = temp;
    }
    
    let mut factor_str: String = "[".to_string();
    for f in factors {
        factor_str = format!("{}{}, ",factor_str, f);
    }
    factor_str = factor_str.trim_right().to_string();
    factor_str = factor_str.trim_right_matches(',').to_string();
    factor_str = format!("{}]",factor_str);
    println!("{}: {}",n, factor_str);
    
    return running;
}