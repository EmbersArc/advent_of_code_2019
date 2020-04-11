use std::fs;

fn read(filename: &str) -> Vec<i64> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents.lines().map(|l| l.parse().unwrap()).collect()
}

fn pt1() {
    let vec = read("src/day1/input.txt");

    let mut total_fuel = 0;

    for module_mass in vec.iter() {
        total_fuel += module_mass / 3 - 2;
    }
    println!("Part 1: {}", total_fuel);
}

fn pt2() {
    let vec = read("src/day1/input.txt");

    let mut total_fuel = 0;

    for module_mass in vec.iter() {
        let mut fuel_for_module: i64 = *module_mass;

        loop {
            fuel_for_module = fuel_for_module / 3 - 2;
            if fuel_for_module <= 0 {
                break;
            }
            total_fuel += fuel_for_module;
        }
    }
    println!("Part 2: {}", total_fuel);
}

fn main() {
    pt1();
    pt2();
}
