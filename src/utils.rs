extern crate rusqlite;
extern crate time;
extern crate num_bigint;

use num_bigint::*;
use self::rusqlite::Connection;

pub struct Prime {
		pub prime: i64,
		pub prime_index: i64
}

pub struct Triangle {
		pub triangle: i64,
		pub triangle_index: i64
}

pub fn is_prime(x: i64) -> bool {
	let mut i = 2;
	let upper_limit = (x as f64).sqrt() as i64 + 1;
	while i < upper_limit {
		if x % i == 0 {
			return false;
		}
		i += 1;
	}
	return true;
}

pub fn add_primes_small() {
    let conn = Connection::open("math.sqlite").unwrap();
	let mut stmt = conn.prepare(&format!("{}","SELECT prime, prime_index FROM primes WHERE prime = (SELECT MAX(prime) FROM primes)")).unwrap();
    let prime_iter = stmt.query_map(&[], |row| {
        Prime {
			prime: row.get(0),
			prime_index: row.get(1)
    }}).unwrap();

	let mut top_prime = Prime{prime : 0, prime_index : 0};
	for prime in prime_iter {
		let p = prime.unwrap();
		top_prime.prime = p.prime;
		top_prime.prime_index = p.prime_index;
	}
    
	let i = top_prime.prime+2;
	let mut count = top_prime.prime_index;

	let mut query = format!("INSERT INTO primes (prime, prime_index) VALUES");
	//for k in 1..11 {
		//println!("{}",i+1000*(k-1));
		//for j in i+1000*(k-1)..i+1000*k {
		for j in i..i+100000 {
			if is_prime(j) {
				count += 1;
				let tmpq = format!(" ({}, {}),",j,count);
			
				query = format!("{}{}",query, tmpq);
			}
		}
		//println!("{}",i+1000*k-1);
		conn.execute(query.trim_matches(|c| c == ','), &[]).unwrap();
	//}
}

pub fn add_primes() {
	println!("Adding primes to db...");
	for _ in 1..11 {
		add_primes_small();
	}
}

pub fn create_prime_db() {
	println!("Initializing primes db...");
	let conn = Connection::open("math.sqlite").unwrap();
	conn.execute("DROP TABLE primes",&[]).unwrap();
    conn.execute("CREATE TABLE primes (
                  prime_index              INTEGER PRIMARY KEY,
                  prime            INTEGER
                  )", &[]).unwrap();
}

/*pub fn fetch_primes_by_count(count: i64) -> Vec<i64> {
	let conn = Connection::open("math.sqlite").unwrap();
	let mut stmt = conn.prepare(&format!("{}{}","SELECT prime, prime_index FROM primes LIMIT ", count)).unwrap();
    let prime_iter = stmt.query_map(&[], |row| {
        Prime {
			prime: row.get(0),
			prime_index: row.get(1)
    }}).unwrap();

	let mut primes = vec![];
	for prime in prime_iter {
		primes.push(prime.unwrap().prime);
	}
	return primes;
}*/

pub fn fetch_primes_under_val(count: i64) -> Vec<i64> {
	let conn = Connection::open("math.sqlite").unwrap();

	//Find the current biggest prime
	let mut stmt = conn.prepare(&format!("{}","SELECT prime, prime_index FROM primes WHERE prime = (SELECT MAX(prime) FROM primes)")).unwrap();
    let prime_iter = stmt.query_map(&[], |row| {
        Prime {
			prime: row.get(0),
			prime_index: row.get(1),
    }}).unwrap();

	let mut current_max = 0;
	for prime in prime_iter {
		current_max = prime.unwrap().prime;
	}

	while current_max < count {
		add_primes();
		//Find the current biggest prime
		let mut stmt2 = conn.prepare(&format!("{}","SELECT prime, prime_index FROM primes WHERE prime = (SELECT MAX(prime) FROM primes)")).unwrap();
		let prime_iter2 = stmt2.query_map(&[], |row| {
		    Prime {
				prime: row.get(0),
				prime_index: 0
		}}).unwrap();
		for prime in prime_iter2 {
			current_max = prime.unwrap().prime;
		}
	}

	let mut stmt3 = conn.prepare(&format!("{}{}","SELECT prime, prime_index FROM primes WHERE prime <= ", count)).unwrap();
    let prime_iter3 = stmt3.query_map(&[], |row| {
        Prime {
			prime: row.get(0),
			prime_index: row.get(1)
    }}).unwrap();

	let mut primes = vec![];
	for prime in prime_iter3 {
		primes.push(prime.unwrap().prime);
	}
	return primes;
}

pub fn fetch_prime_by_index(count: i64) -> i64 {
	let conn = Connection::open("math.sqlite").unwrap();

	//Find the current biggest prime
	let mut stmt = conn.prepare(&format!("{}","SELECT prime, prime_index FROM primes WHERE prime_index = (SELECT MAX(prime_index) FROM primes)")).unwrap();
    let prime_iter = stmt.query_map(&[], |row| {
        Prime {
			prime_index: row.get(1),
			prime: 0
    }}).unwrap();

	let mut current_max = 0;
	for prime in prime_iter {
		current_max = prime.unwrap().prime_index;
	}

	while current_max < count {
		add_primes();
		//Find the current biggest prime
		let mut stmt2 = conn.prepare(&format!("{}","SELECT prime, prime_index FROM primes WHERE prime = (SELECT MAX(prime) FROM primes)")).unwrap();
		let prime_iter2 = stmt2.query_map(&[], |row| {
		    Prime {
				prime: row.get(0),
				prime_index: row.get(1)
		}}).unwrap();
		for prime in prime_iter2 {
			current_max = prime.unwrap().prime_index;
		}
	}

	let mut stmt3 = conn.prepare(&format!("{}{}","SELECT prime, prime_index FROM primes WHERE prime_index = ", count)).unwrap();
    let prime_iter3 = stmt3.query_map(&[], |row| {
        Prime {
			prime: row.get(0),
			prime_index: row.get(1)
    }}).unwrap();

	for prime in prime_iter3 {
		return prime.unwrap().prime;
	}
	return 0;
}

pub fn create_triangle_db() {
	println!("Initializing triangular number db...");
	let conn = Connection::open("math.sqlite").unwrap();
	conn.execute("DROP TABLE triangles",&[]).unwrap();
    conn.execute("CREATE TABLE triangles (
                  triangle              INTEGER PRIMARY KEY,
                  triangle_index            INTEGER,
                  triangle_factors    TEXT,
                  triangle_factor_count            INTEGER
                  )", &[]).unwrap();
}

pub fn add_triangles() {
	
	println!("Adding more triangles to db...");
    let conn = Connection::open("math.sqlite").unwrap();

	let mut stmt = conn.prepare(&format!("{}","SELECT triangle, triangle_index FROM triangles WHERE triangle = (SELECT MAX(triangle) FROM triangles)")).unwrap();
    let triangle_iter = stmt.query_map(&[], |row| {
        Triangle {
			triangle: row.get(0),
			triangle_index: row.get(1)
    }}).unwrap();

	let mut top_triangle = Triangle{triangle : 0, triangle_index : 0};
	for tri in triangle_iter {
		let p = tri.unwrap();
		top_triangle.triangle = p.triangle;
		top_triangle.triangle_index = p.triangle_index;
	}
    
	let mut count = top_triangle.triangle_index;
	let i = count + 1;

	let mut query = format!("INSERT INTO triangles (triangle, triangle_index, triangle_factors, triangle_factor_count) VALUES");
	for j in i..i+5000 {
		count += 1;
		let triangle = j * (j + 1) / 2;
		let v = factors(triangle);
		let fcount = v.len();
		let mut s = format!("");
		for n in v {
			s = format!("{}{},", s, n);
		}
		s = String::from(s.trim_matches(|c| c == ','));
		let tmpq = format!(" ({}, {}, \"{}\", {}),",triangle,count, s, fcount);	
		query = format!("{}{}",query, tmpq);
	}
	conn.execute(query.trim_matches(|c| c == ','), &[]).unwrap();
}

/*pub fn get_more_triangles() -> Vec<Triangle> {
	//Build connection
	let conn = Connection::open("math.sqlite").unwrap();

	//Find the highest triangle number in the db
	let mut stmt = conn.prepare(&format!("{}","SELECT triangle, triangle_index FROM triangles WHERE triangle = (SELECT MAX(triangle) FROM triangles)")).unwrap();
    let triangle_iter = stmt.query_map(&[], |row| {
        Triangle {
			triangle: row.get(0),
			triangle_index: row.get(1)
    }}).unwrap();

	//Turn the iterator into an actual Triangle value
	let mut top_triangle = Triangle{triangle : 0, triangle_index : 0};
	for tri in triangle_iter {
		let p = tri.unwrap();
		top_triangle.triangle = p.triangle;
		top_triangle.triangle_index = p.triangle_index;
	}

	//Make the db have more triangles in it
	add_triangles();

	//Get those triangles
	let mut stmt = conn.prepare(&format!("{}","SELECT triangle, triangle_index FROM triangles WHERE triangle_index > top_triangle.triangle_index")).unwrap();
    let triangle_iter = stmt.query_map(&[], |row| {
        Triangle {
			triangle: row.get(0),
			triangle_index: row.get(1)
    }}).unwrap();
	
	let mut triangle_vec : Vec<Triangle> = vec![];

	for t in triangle_iter {
		triangle_vec.push(t.unwrap());
	}

	return triangle_vec;
}*/


pub fn factorial_big(x: i64) -> BigInt {
	let mut f = 1.to_bigint().unwrap();
	for i in 1.. x+1 {
		f = f*i.to_bigint().unwrap();
	}
	f
}

pub fn combination_big(n: i64, k: i64) -> BigInt {
	let mut f = permutation_big(n, k);

	for i in 1..n-k+1 {
		f = f/i.to_bigint().unwrap();
	}
	return f;
}

pub fn permutation_big(n: i64, k: i64) -> BigInt  {
	let mut f = 1.to_bigint().unwrap();
	for i in n-k+1..n+1 {
		f = f*i.to_bigint().unwrap();
	}
	f
}

pub fn factors(x : i64) -> Vec<i64> {
	let mut v : Vec<i64> = vec![];
	let upper_limit = (x as f64).sqrt() as i64 + 1;
	v.push(1);
	for i in 2..upper_limit {
		//println!("this {}", i);
		if x % i == 0 && x != i {
			v.push(i);
			if x/i != i {
				v.push(x/i);
			}
		}
	}
	if x != 1 {
		v.push(x);
	}
	return v;
}

pub fn pow_big(x: i64, e: i64) -> BigInt {
	let mut pow = 1.to_bigint().unwrap();	
	for _ in 1..e+1 {
		pow = pow * x.to_bigint().unwrap();
	}
	pow
}
