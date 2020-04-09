use std::f64::consts::PI;

fn get_input() -> Vec<Vec<char>> {
    //     let map = ".###..#......###..#...#
    // #.#..#.##..###..#...#.#
    // #.#.#.##.#..##.#.###.##
    // .#..#...####.#.##..##..
    // #.###.#.####.##.#######
    // ..#######..##..##.#.###
    // .##.#...##.##.####..###
    // ....####.####.#########
    // #.########.#...##.####.
    // .#.#..#.#.#.#.##.###.##
    // #..#.#..##...#..#.####.
    // .###.#.#...###....###..
    // ###..#.###..###.#.###.#
    // ...###.##.#.##.#...#..#
    // #......#.#.##..#...#.#.
    // ###.##.#..##...#..#.#.#
    // ###..###..##.##..##.###
    // ###.###.####....######.
    // .###.#####.#.#.#.#####.
    // ##.#.###.###.##.##..##.
    // ##.#..#..#..#.####.#.#.
    // .#.#.#.##.##########..#
    // #####.##......#.#.####.";
    let map = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

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

fn pt1() {
    // o-> x
    // |
    // v
    // y

    let mut map = get_input();
    let width = map[0].len() as i32;
    let height = map.len() as i32;

    let mut max_visible = 0;
    let mut best_location = (0, 0);

    for y in 0..height {
        for x in 0..width {
            map = get_input();

            erase_invisible(&mut map, (x, y));

            let mut visible_asteroids = 0;
            for line in map.iter() {
                visible_asteroids += line.iter().filter(|&b| *b == '#').count();
            }
            visible_asteroids -= 1;

            if visible_asteroids > max_visible {
                max_visible = visible_asteroids;
                best_location = (x, y);
            }
        }
    }

    println!("Solution Part 1: {:?}: {}", best_location, max_visible);
}

fn pt2() {
    // o-> x
    // |
    // v
    // y

    let map = get_input();
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    let mut asteroid_info = Vec::new(); // coordinates, distance and angle
    let tracker_pos = (11, 13);

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

    // use std::cmp::Ordering;
    // fn compare_info(info1: &((i32, i32), f64, f64), info2: &((i32, i32), f64, f64)) -> Ordering {
    //     if info1.2 == info2.2 {
    //         // same angle
    //         if info1.1 < info2.1 {
    //             // smaller distance goes first
    //             return Ordering::Less;
    //         } else {
    //             return Ordering::Greater;
    //         }
    //     } else if info1.2 < info2.2 {
    //         // smaller angle goes first
    //         return Ordering::Less;
    //     } else {
    //         return Ordering::Greater;
    //     }
    // }
    asteroid_info.sort_by(&compare_info);

    // asteroid_info.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // by distance ascending
    // asteroid_info.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap()); // by angle ascending

    println!("{:?}", asteroid_info);
    println!("Solution Part 2: {:?}", asteroid_info[199]);
    println!("{:?}", asteroid_info.iter().position(|i| i.0 == (8, 2)));
}

// fn print_map(map: &Vec<Vec<char>>) {
//     for line in map.iter() {
//         let string: String = line.into_iter().collect();
//         println!("{}", string);
//     }
// }

fn catan2(x: f64, y: f64) -> f64 {
    if x < 0. {
        2. * PI + x.atan2(y)
    } else {
        x.atan2(y)
    }
}

fn main() {
    pt1();
    pt2();
}
