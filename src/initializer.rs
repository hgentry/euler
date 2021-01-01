use std::io::Write;
use utils::strings;

pub fn initialize(s: String) -> bool {
    let mut problem = strings::read_file_to_string("src/problem.rs");
    let mut pmod = strings::read_file_to_string("src/problems/mod.rs");
    let p = strings::read_file_to_string("src/problems/p.rs");

    let p_filename = format!("src/problems/p{}.rs", s);
    let mut p_file = std::fs::File::create(p_filename.as_str()).unwrap();
    p_file.write_all(p.as_str().as_bytes()).unwrap();

    pmod = format!("{}\npub mod p{};", pmod, s);
    let mut pmod_file = std::fs::File::create("src/problems/mod.rs").unwrap();
    pmod_file.write_all(pmod.as_str().as_bytes()).unwrap();

    let index = problem.find("match n {").unwrap();
    let problem_start: String = problem.chars().skip(0).take(index + 10).collect();
    let problem_end: String = problem.chars().skip(index + 10).collect();
    problem = format!(
        "{}{} => s = {{found = true; problems::p{}::solve().to_string()}},\n{}",
        problem_start, s, s, problem_end
    );
    let mut problem_file = std::fs::File::create("src/problem.rs").unwrap();
    problem_file.write_all(problem.as_str().as_bytes()).unwrap();

    return true;
}
