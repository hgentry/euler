extern crate rusqlite;
extern crate time;
extern crate num_bigint;

use self::rusqlite::Connection;

use utils;

pub struct Triangle {
		pub triangle: i64,
		pub triangle_index: i64
}

pub fn add_triangles() {
	
	//println!("Adding more triangles to db...");
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
		let v = utils::math::factors(triangle);
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
