fn get_input() -> Vec<Vec<bool>> {
    // let map = ".#..##.###...#######
    // ##.############..##.
    // .#.######.########.#
    // .###.#######.####.#.
    // #####.##.#.##.###.##
    // ..#####..#.#########
    // ####################
    // #.####....###.#.#.##
    // ##.#################
    // #####.##.###..####..
    // ..######..##.#######
    // ####.##.####...##..#
    // .#####..#.######.###
    // ##...#.##########...
    // #.##########.#######
    // .####.#.###.###.#.##
    // ....##.##.###..#####
    // .#.#.###########.###
    // #.#.#.#####.####.###
    // ###.##.####.##.#..##";

    //     let map = "..........
    // .#..#.....
    // ....#.....
    // ...#......
    // ....#.#...
    // ......#...
    // ........#.
    // ....#.....
    // ....#.....
    // ..........";

    let map = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";

    let mut bool_map: Vec<Vec<bool>> = Vec::default();

    for line in map.lines() {
        let bool_line: Vec<bool> = line
            .chars()
            .map(|c| match c {
                '#' => true,
                '.' => false,
                _ => panic!("outch"),
            })
            .collect();
        bool_map.push(bool_line);
    }

    bool_map
}

struct Combination {
    choices: Vec<i32>,
    selected: usize,
    vector: Vec<usize>,
}

impl Combination {
    fn from(choices: &Vec<i32>, size: usize) -> Combination {
        Combination {
            choices: choices.clone(),
            selected: size,
            vector: vec![0; size + 1],
        }
    }
}

impl Iterator for Combination {
    type Item = Vec<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            for i in 0..self.selected {
                if self.vector[i] > self.choices.len() - 1 {
                    self.vector[i + 1] += 1;
                    for k in i..=0 {
                        self.vector[k] = self.vector[i + 1];
                    }
                }
            }

            if self.vector[self.selected] > 0 {
                return None;
            }

            let mut new_vec = Vec::default();
            for i in 0..self.selected {
                new_vec.push(self.choices[self.vector[i]]);
            }

            self.vector[0] += 1;
            return Some(new_vec);
        }
    }
}

fn erase_invisible(map: &mut Vec<Vec<bool>>, pos: (usize, usize), step: (i32, i32)) {
    if step == (0, 0) {
        return;
    }

    let width = map[0].len() as i32;
    let height = map.len() as i32;

    for (dx, dy) in &[
        (step.0, step.1),
        (-step.0, -step.1),
        (-step.0, step.1),
        (step.0, -step.1),
        (step.1, step.0),
        (-step.1, -step.0),
        (-step.1, step.0),
        (step.1, -step.0),
    ] {
        println!("{}, {}", dx, dy);
        let mut x = pos.0 as i32;
        let mut y = pos.1 as i32;
        let mut obstructed = false;
        loop {
            x += dx;
            y += dy;
            println!("x: {}, y: {}", x, y);

            if y < 0 || y >= height || x < 0 || x >= width {
                break;
            }

            let is_asteroid = map[y as usize][x as usize];
            if is_asteroid {
                if obstructed {
                    map[y as usize][x as usize] = false;
                } else {
                    obstructed = true;
                }
            }
        }
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn pt1() {
    let mut map = get_input();
    print_map(&map);

    // o-> x
    // |
    // v
    // y
    let pos = (5, 8);

    for combination in Combination::from(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 2) {
        let gcd = gcd(combination[0], combination[1]);
        if gcd != 0 && gcd != 1 {
            continue;
        }
        let combination = (combination[0], combination[1]);

        println!("{:?}", combination);

        erase_invisible(&mut map, pos, combination);
        print_map(&map);
    }

    let mut visible_asteroids = 0;

    for line in map.iter() {
        visible_asteroids += line.iter().filter(|&b| *b == true).count();
    }

    visible_asteroids -= 1;

    println!("Solution Part 1: {}", visible_asteroids);
}

fn print_map(map: &Vec<Vec<bool>>) {
    for line in map.iter() {
        let line: String = line
            .iter()
            .map(|c| match c {
                true => '#',
                false => '.',
            })
            .collect();
        println!("{}", line);
    }
}

fn main() {
    pt1();
}
