pub fn solve() -> i64 {
    let mut triangles = vec![];
    let mut squares = vec![];
    let mut pentagons = vec![];
    let mut hexagons = vec![];
    let mut heptagons = vec![];
    let mut octagons = vec![];

    let mut latest;
    for i in 1..10000 {
        let n = i as i64;
        latest = n * (n + 1) / 2;
        if latest >= 1000 && latest < 10000 {
            triangles.push(latest);
        }
        if latest >= 10000 {
            break;
        }
    }
    for i in 1..10000 {
        let n = i as i64;
        latest = n * n;
        if latest >= 1000 && latest < 10000 {
            squares.push(latest);
        }
        if latest >= 10000 {
            break;
        }
    }
    for i in 1..10000 {
        let n = i as i64;
        latest = n * (3 * n - 1) / 2;
        if latest >= 1000 && latest < 10000 {
            pentagons.push(latest);
        }
        if latest >= 10000 {
            break;
        }
    }
    for i in 1..10000 {
        let n = i as i64;
        latest = n * (2 * n - 1);
        if latest >= 1000 && latest < 10000 {
            hexagons.push(latest);
        }
        if latest >= 10000 {
            break;
        }
    }
    for i in 1..10000 {
        let n = i as i64;
        latest = n * (5 * n - 3) / 2;
        if latest >= 1000 && latest < 10000 {
            heptagons.push(latest);
        }
        if latest >= 10000 {
            break;
        }
    }
    for i in 1..10000 {
        let n = i as i64;
        latest = n * (3 * n - 2);
        if latest >= 1000 && latest < 10000 {
            octagons.push(latest);
        }
        if latest >= 10000 {
            break;
        }
    }

    for i in 1000..10000 {
        let x = i as i64;
        let mut sum;
        if triangles.contains(&x) {
            sum = x;
            let mut found = vec![(0, false), (0, false), (0, false), (0, false), (0, false)];
            let mut found_order = vec![];
            let mut set = vec![x];
            let mut j = x % 100 * 100;
            let mut tries = 0;
            loop {
                if !found[0].1 {
                    //still looking for square
                    if squares.contains(&j) {
                        sum += j;
                        set.push(j);
                        found[0] = (j, true);
                        j = j % 100 * 100 + 1;
                        tries = 0;

                        found_order.push(0);
                        continue;
                    }
                }

                if !found[1].1 {
                    //still looking for pentagon
                    if pentagons.contains(&j) {
                        sum += j;
                        set.push(j);
                        found[1] = (j, true);
                        j = j % 100 * 100 + 1;
                        tries = 0;
                        found_order.push(1);
                        continue;
                    }
                }

                if !found[2].1 {
                    //still looking for hexagon
                    if hexagons.contains(&j) {
                        sum += j;
                        set.push(j);
                        found[2] = (j, true);
                        j = j % 100 * 100 + 1;
                        tries = 0;
                        found_order.push(2);
                        continue;
                    }
                }

                if !found[3].1 {
                    //still looking for heptagon
                    if heptagons.contains(&j) {
                        sum += j;
                        set.push(j);
                        found[3] = (j, true);
                        j = j % 100 * 100 + 1;
                        tries = 0;
                        found_order.push(3);
                        continue;
                    }
                }

                if !found[4].1 {
                    //still looking for octagon
                    if octagons.contains(&j) {
                        sum += j;
                        set.push(j);
                        found[4] = (j, true);
                        j = j % 100 * 100 + 1;
                        tries = 0;
                        found_order.push(4);
                        continue;
                    }
                }

                let mut all_found = true;
                for v in &found {
                    all_found = all_found && v.1;
                }

                if all_found && set[set.len() - 1] % 100 == x / 100 {
                    return sum;
                }

                j += 1;
                tries += 1;
                if tries == 99 {
                    if found_order.len() == 0 {
                        break;
                    } else {
                        j = found[found_order[found_order.len() - 1]].0 + 1;
                        sum -= found[found_order[found_order.len() - 1]].0;
                        found[found_order[found_order.len() - 1]].1 = false;
                        tries = j % 100 - 1;
                        found_order.remove(found_order.len() - 1);
                    }
                }
            }
        }
    }
    return 0;
}
