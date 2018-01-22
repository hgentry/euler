use utils;

pub fn solve() -> i64 {
	let mut sum = 10;
    let mut checking = 5;
    loop {
        checking += 2;
        if checking > 2000000 {
            return sum;
        }
        if utils::primes::is_prime(checking) {
            sum += checking;
        }
    }
}
