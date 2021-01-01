use utils;

pub fn solve() -> i64 {
    //I did math by hand to find a better starting point
    let mut count = 725761;
    let mut permutation = vec![2, 0, 1, 3, 4, 5, 6, 7, 8, 9];
    let mut alter_pos = 1;

    while count != 1000000 {
        //println!("{}", print_perm(&permutation));
        //Increase by a lot by knowing basic statistics, back up if it goes too far
        if count < 1000000 {
            let old_val = permutation[alter_pos];
            let mut tmp_perm = permutation.split_off(alter_pos);
            tmp_perm[0] += 1;
            while permutation.contains(&tmp_perm[0]) {
                tmp_perm[0] += 1;
                tmp_perm[0] = tmp_perm[0] % 10;
            }
            permutation.append(&mut tmp_perm);
            let len: i64 = 10 - alter_pos as i64 - 1;
            count += utils::math::permutation(len, len);
            if alter_pos <= 9 {
                for i in alter_pos + 1..10 {
                    if permutation[i] == permutation[alter_pos] {
                        permutation[i] = old_val;
                    }
                }
            }
            let mut perm2 = permutation.split_off(alter_pos + 1);
            perm2.sort();
            permutation.append(&mut perm2);
        } else if count > 1000000 {
            let old_val = permutation[alter_pos];
            let mut tmp_perm = permutation.split_off(alter_pos);
            tmp_perm[0] -= 1;
            while permutation.contains(&tmp_perm[0]) {
                tmp_perm[0] -= 1;
                tmp_perm[0] = tmp_perm[0] % 10;
            }
            permutation.append(&mut tmp_perm);
            let len: i64 = 10 - alter_pos as i64 - 1;
            count -= utils::math::permutation(len, len);
            if alter_pos <= 9 {
                for i in alter_pos + 1..10 {
                    if permutation[i] == permutation[alter_pos] {
                        permutation[i] = old_val;
                    }
                }
            }
            alter_pos += 1;
            let mut perm2 = permutation.split_off(alter_pos + 1);
            perm2.sort();
            permutation.append(&mut perm2);
        }
    }

    print_perm(&permutation)
}

fn print_perm(permutation: &Vec<i64>) -> i64 {
    let mut perm = 0;
    for i in 0..10 {
        let mut digit = permutation[9 - i];
        let mut j = 0;
        while j < i {
            digit *= 10;
            j += 1;
        }
        perm += digit;
    }
    perm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct() {
        assert_eq!(solve(), 2783915460);
    }
}
