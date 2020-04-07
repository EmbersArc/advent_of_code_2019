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

fn pt1() {
    // o-> x
    // |
    // v
    // y

    let mut map = get_input();
    let width = map[0].len() as i32;
    let height = map.len() as i32;

    let mut max_visible = 0;

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
            }
        }
    }

    println!("Solution Part 1: {}", max_visible);
}

// fn print_map(map: &Vec<Vec<char>>) {
//     for line in map.iter() {
//         let string: String = line.into_iter().collect();
//         println!("{}", string);
//     }
// }

fn main() {
    pt1();
}
