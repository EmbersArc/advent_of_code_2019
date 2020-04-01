use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn pt1() -> Result<(), Error> {
    let vec = read(File::open("src/day1/input.txt")?)?;

    let mut total_fuel = 0;

    for module_mass in vec.iter() {
        total_fuel += module_mass / 3 - 2;
    }
    println!("Part 1: {}", total_fuel);
    Ok(())
}

fn pt2() -> Result<(), Error> {
    let vec = read(File::open("src/day1/input.txt")?)?;

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
    Ok(())
}

fn main() -> Result<(), Error> {
    if let Err(e) = pt1() {
        return Err(e);
    }
    if let Err(e) = pt2() {
        return Err(e);
    }

    Ok(())
}
