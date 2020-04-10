use std::f64::consts::PI;

fn get_input() -> Vec<Vec<char>> {
    let map = ".###..#......###..#...#
#.#..#.##..###..#...#.#
#.#.#.##.#..##.#.###.##
.#..#...####.#.##..##..
#.###.#.####.##.#######
..#######..##..##.#.###
.##.#...##.##.####..###
....####.####.#########
#.########.#...##.####.
.#.#..#.#.#.#.##.###.##
#..#.#..##...#..#.####.
.###.#.#...###....###..
###..#.###..###.#.###.#
...###.##.#.##.#...#..#
#......#.#.##..#...#.#.
###.##.#..##...#..#.#.#
###..###..##.##..##.###
###.###.####....######.
.###.#####.#.#.#.#####.
##.#.###.###.##.##..##.
##.#..#..#..#.####.#.#.
.#.#.#.##.##########..#
#####.##......#.#.####.";

    let char_map = map.lines().map(|l| l.chars().collect()).collect();

    char_map
}

fn erase_invisible(map: &mut Vec<Vec<char>>, tracker_pos: (i32, i32)) {
    let width = map[0].len() as i32;
    let height = map.len() as i32;

    for y in 0..height {
        for x in 0..width {
            if map[y as usize][x as usize] != '#' {
                continue;
            }
            if (x, y) == tracker_pos {
                continue;
            }
            let mut search_direction = (x - tracker_pos.0, y - tracker_pos.1);
            let gcd = gcd(search_direction.0, search_direction.1);

            match gcd {
                1 => continue, // no obstruction possible
                _ => {
                    search_direction.0 /= gcd;
                    search_direction.1 /= gcd;
                }
            }

            let mut pos_current = tracker_pos;
            let mut obstructed = false;

            loop {
                pos_current.0 += search_direction.0;
                pos_current.1 += search_direction.1;

                let current_symbol = &mut map[pos_current.1 as usize][pos_current.0 as usize];
                if *current_symbol == '#' {
                    if obstructed {
                        *current_symbol = '.';
                    } else {
                        obstructed = true;
                    }
                }

                if pos_current == (x, y) {
                    // at candidate
                    break;
                }
            }
        }
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    if a == 0 {
        return b.abs();
    }
    if b == 0 {
        return a.abs();
    }

    loop {
        let h = a % b;
        a = b;
        b = h;

        if b == 0 {
            break;
        }
    }
    a.abs()
}

fn pt1() -> (i32, i32) {
    // o-> x
    // |
    // v
    // y

    let mut map = get_input();
    let width = map[0].len() as i32;
    let height = map.len() as i32;

    let mut max_visible = 0;
    let mut best_location = (-1, -1);

    for y in 0..height {
        for x in 0..width {
            map = get_input();

            erase_invisible(&mut map, (x, y));

            let mut visible_asteroids = 0;
            for line in map.iter() {
                visible_asteroids += line.iter().filter(|&b| *b == '#').count();
            }
            visible_asteroids -= 1;

            if visible_asteroids >= max_visible {
                max_visible = visible_asteroids;
                best_location = (x, y);
            }
        }
    }

    println!("Solution Part 1: {:?}: {}", best_location, max_visible);

    best_location
}

fn pt2(best_location: (i32, i32)) {
    // o-> x
    // |
    // v
    // y

    let map = get_input();
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    let mut asteroid_info = Vec::new(); // coordinates, distance and angle
    let tracker_pos = best_location;

    for y in 0..height {
        for x in 0..width {
            if map[y as usize][x as usize] != '#' {
                continue;
            }
            if (x, y) == tracker_pos {
                continue;
            }

            let direction = ((x - tracker_pos.0) as f64, (y - tracker_pos.1) as f64);
            let distance = (direction.0.powi(2) + direction.1.powi(2)).sqrt();
            let angle = catan2(direction.0, -direction.1);
            asteroid_info.push(((x, y), distance, angle));
        }
    }

    asteroid_info.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap()); // distance ascending
    asteroid_info.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap()); // angle ascending

    let mut destroyed = vec![false; asteroid_info.len()];
    let mut destroyed_indices = Vec::new();

    let mut angle;
    let mut i = 0;
    // do rounds
    loop {
        // destroy one
        destroyed[i] = true;
        destroyed_indices.push(i);
        angle = asteroid_info[i].2;

        // skip while angle is the same or already destroyed
        while i < asteroid_info.len() && (angle == asteroid_info[i].2 || destroyed[i]) {
            i += 1;
        }

        // find the one with the lowest angle
        if i == asteroid_info.len() {
            match destroyed.iter().position(|&b| b == false) {
                Some(p) => i = p,
                None => break,
            }
        }
    }

    let coordinates = asteroid_info[destroyed_indices[199]].0;
    println!("Solution Part 2: {}", coordinates.0 * 100 + coordinates.1,);
}

// atan2 function in [0; 2PI]
fn catan2(x: f64, y: f64) -> f64 {
    if x < 0. {
        2. * PI + x.atan2(y)
    } else {
        x.atan2(y)
    }
}

fn main() {
    let best_location = pt1();
    pt2(best_location);
}
