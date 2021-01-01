//TODO Make this function robust

pub fn subvec<T: Copy>(v: &Vec<T>, a: usize, b: usize) -> Vec<T> {
    let mut results = vec![];
    for i in a..b {
        results.push(v[i]);
    }
    results
}
