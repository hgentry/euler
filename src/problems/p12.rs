use utils;
extern crate rusqlite;
extern crate time;

use self::rusqlite::Connection;

//I might be doing this the wrong way. Maybe duplicates count.

pub fn solve() -> i64 {
	let conn = Connection::open("math.sqlite").unwrap();

	let mut maxval = 0;
	while maxval == 0 {
		let mut stmt = conn.prepare(&format!("{}","SELECT triangle FROM triangles WHERE triangle_factor_count > 500 LIMIT 1")).unwrap();

		let triangle_iter = stmt.query_map(&[], |row| {
		    utils::Triangle {
			triangle: row.get(0),
			triangle_index: 0
		}}).unwrap();
		for triangle in triangle_iter {
			maxval = triangle.unwrap().triangle;
		}
		if maxval == 0 {
			utils::add_triangles();
		}
	}
	return maxval;
}
