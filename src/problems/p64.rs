use utils::math;

pub fn solve() -> i64 {
    return expand_root_len(23);
    let mut a0 = 1;
    let mut next_square = 4;
    for i in 2..=10000 {
        if i == 8 || i == 14 {
            continue;
        }
        if i == next_square {
            a0 += 1;
            next_square = (a0+1)*(a0+1);
            continue;
        }
        println!("{} {}",i, a0);
        expand_root_len(i);
    }
    return 0;
    //return expand_root_len(10,3);
}

/*

pub fn expand_root_len_1(n:i64, mut a0: i64) -> i64 {

    //Part 1: Generate sequence
    let mut sequence = vec!();
    let mut len = 0;
    let init = a0;
    let mut prev_numer = 1;
    let mut prev_denom = 1;
    let mut prev_fact = 4;
    let mut double_prev_numer = 1;

    //0: 4 + (1*r - 4)/1
    //1: 1 + (1*r - 3)/7
    //2: 3 + (7*r - 21)/14
    //3: 1 + (2*r - 8)/14
    //4: 8 + (1*rr - 4)/1

    //0: a0 + n*(r - f)/d
    //1: a1 + (d/n)*(r - )/(x-f*f)

    for i in 1..=2*n {
        let numer = prev_denom/prev_numer/double_prev_numer; //GOOD
        let denom = (n - prev_fact*prev_fact); //GOOD
        let mut fact =  numer*prev_fact;
        let mut a1 = 0;
        let reduced = math::reduce_fraction(fact, denom);
        fact= reduced.0;
        println!("{} {} {} {}", a1, numer, fact.abs(), denom);
        a0 = a1;
        sequence.push(a1);
        //println!("{}",a1);
        //println!("{}/{}", reduced.0,reduced.1);
        double_prev_numer = prev_numer;
        prev_numer = numer;
        prev_denom = denom;
        prev_fact = fact;
    }

    //Part 2: Detect repeating pattern

    return 0;
}
*/
pub fn expand_root_len(x: i64) -> i64 {
    let mut a0;
    let mut numer = 1;
    let mut denom = 1;
    let mut fact;
    
    a0 = (x as f64).sqrt() as i64;
    fact = a0;
    for i in 1..=2*x {
        numer = denom;
        denom = x - fact*fact;
        fact = fact * numer;
        a0 = (numer as f64*((x as f64).sqrt() +fact as f64)/(denom as f64)).sqrt() as i64;
        fact -= a0*denom;
        let reduced = math::reduce_fraction(fact.abs(),denom);
        fact = reduced.0;
        denom = reduced.1;
        println!("{} + {}(r + {})/{}", a0, numer, fact, denom);
    }

    return 0;
}
