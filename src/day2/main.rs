use std::error::Error;

fn get_input() -> Vec<usize> {
    vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 6, 1, 19, 1, 5, 19, 23, 1, 13, 23, 27,
        1, 6, 27, 31, 2, 31, 13, 35, 1, 9, 35, 39, 2, 39, 13, 43, 1, 43, 10, 47, 1, 47, 13, 51, 2,
        13, 51, 55, 1, 55, 9, 59, 1, 59, 5, 63, 1, 6, 63, 67, 1, 13, 67, 71, 2, 71, 10, 75, 1, 6,
        75, 79, 1, 79, 10, 83, 1, 5, 83, 87, 2, 10, 87, 91, 1, 6, 91, 95, 1, 9, 95, 99, 1, 99, 9,
        103, 2, 103, 10, 107, 1, 5, 107, 111, 1, 9, 111, 115, 2, 13, 115, 119, 1, 119, 10, 123, 1,
        123, 10, 127, 2, 127, 10, 131, 1, 5, 131, 135, 1, 10, 135, 139, 1, 139, 2, 143, 1, 6, 143,
        0, 99, 2, 14, 0, 0,
    ]
}

fn solve_program(program: &mut Vec<usize>) {
    let mut pos = 0;
    while program[pos] != 99 {
        let from_pos1 = program[pos + 1];
        let from_pos2 = program[pos + 2];
        let to_pos = program[pos + 3];
        match program[pos] {
            1 => program[to_pos] = program[from_pos1] + program[from_pos2],
            2 => program[to_pos] = program[from_pos1] * program[from_pos2],
            _ => panic!("Invalid opcode!"),
        }
        pos += 4;
    }
}

fn pt1() -> Result<(), Box<dyn Error>> {
    let mut program = get_input();

    program[1] = 12;
    program[2] = 2;

    solve_program(&mut program);

    println!("Solution Part 1: {}", program[0]);

    Ok(())
}

fn pt2() -> Result<(), Box<dyn Error>> {
    let output = 19690720;
    let mut solution: usize = 0;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = get_input();
            program[1] = noun;
            program[2] = verb;
            solve_program(&mut program);
            if program[0] == output {
                solution = 100 * noun + verb;
                break;
            }
        }
    }

    println!("Solution Part 2: {}", solution);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = pt1() {
        return Err(e);
    }
    if let Err(e) = pt2() {
        return Err(e);
    }

    Ok(())
}
