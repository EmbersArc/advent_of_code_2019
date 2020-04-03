// returns digits in reversed order in vector of size 5: 123 -> [0,0,3,2,1]
fn get_digits(number: i32) -> Vec<i32> {
    let mut digits = vec![0; 5];
    let mut n = number;
    let mut i = 0;
    while n > 9 {
        digits[i] = n % 10;
        n = n / 10;
        i += 1;
    }
    digits[i] = n;

    digits
}

fn get_input() -> Vec<i32> {
    vec![
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 38, 63, 76, 93, 118, 199, 280, 361, 442, 99999, 3,
        9, 101, 3, 9, 9, 102, 3, 9, 9, 101, 4, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 101, 5, 9, 9,
        1002, 9, 5, 9, 101, 5, 9, 9, 1002, 9, 4, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 102, 3, 9, 9, 4,
        9, 99, 3, 9, 101, 2, 9, 9, 102, 5, 9, 9, 1001, 9, 5, 9, 4, 9, 99, 3, 9, 102, 4, 9, 9, 1001,
        9, 3, 9, 1002, 9, 5, 9, 101, 2, 9, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9,
        3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
        1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2,
        9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9,
        3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101,
        1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4,
        9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 99, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9,
        101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2,
        9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3,
        9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
        102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2,
        9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3,
        9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102,
        2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4,
        9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9,
        101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99,
    ]
}

fn solve_program(program: &mut Vec<i32>, inputs: &Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    let mut pos: usize = 0;
    let mut input_pos = 0;
    while program[pos] != 99 {
        let digits = get_digits(program[pos]);
        let opcode = 10 * digits[1] + digits[0];
        let mode_par1 = digits[2];
        let mode_par2 = digits[3];
        let mode_par3 = digits[4]; // will never be immediate
        if mode_par3 != 0 {
            panic!("What");
        }

        match opcode {
            1 | 2 | 7 | 8 => {
                let input_1 = match mode_par1 {
                    0 => program[program[pos + 1] as usize],
                    _ => program[pos + 1], // 1
                };
                let input_2 = match mode_par2 {
                    0 => program[program[pos + 2] as usize],
                    _ => program[pos + 2], // 1
                };

                let to = program[pos + 3] as usize;

                if opcode == 1 {
                    // add
                    program[to] = input_1 + input_2;
                } else if opcode == 2 {
                    // multiply
                    program[to] = input_1 * input_2;
                } else if opcode == 7 {
                    // less than
                    program[to] = (input_1 < input_2) as i32;
                } else if opcode == 8 {
                    // equals
                    program[to] = (input_1 == input_2) as i32;
                }

                pos += 4;
            }
            3 => {
                //input
                let to = program[pos + 1] as usize;
                program[to] = inputs[input_pos];
                input_pos += 1;
                pos += 2;
            }
            4 => {
                // output
                let input_1 = match mode_par1 {
                    0 => program[program[pos + 1] as usize],
                    _ => program[pos + 1], // 1
                };

                output.push(input_1);
                pos += 2;
            }
            5 | 6 => {
                let input_1 = match mode_par1 {
                    0 => program[program[pos + 1] as usize],
                    _ => program[pos + 1], // 1
                };
                let input_2 = match mode_par2 {
                    0 => program[program[pos + 2] as usize],
                    _ => program[pos + 2], // 1
                };

                if opcode == 5 {
                    // jump if true
                    if input_1 != 0 {
                        pos = input_2 as usize;
                    } else {
                        pos += 3;
                    }
                } else if opcode == 6 {
                    // jump if false
                    if input_1 == 0 {
                        pos = input_2 as usize;
                    } else {
                        pos += 3;
                    }
                }
            }
            _ => panic!("Invalid opcode '{}'!", opcode),
        }
    }

    output
}

//** Heap's Algorithm to find permutations */
fn get_permutations(elements: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut elements = elements.clone();
    let n_elements = elements.len();
    elements.sort();
    let mut permutations: Vec<Vec<i32>> = vec![elements.clone()];

    loop {
        // Find the largest index k such that a[k] < a[k + 1]. If no such index exists, the permutation is the last permutation.
        let mut k_greatest = None;
        for k in 0..n_elements - 1 {
            if elements[k] < elements[k + 1] {
                k_greatest = Some(k);
            } else if k == n_elements - 2 && k_greatest == None {
                // does not exist
                return permutations;
            }
        }

        let k_greatest = k_greatest.unwrap();

        // Find the largest index l greater than k such that a[k] < a[l].
        let mut l_greatest = 1000;
        for l in 0..n_elements {
            if elements[k_greatest] < elements[l] {
                l_greatest = l;
            }
        }

        // Swap the value of a[k] with that of a[l].
        elements.swap(k_greatest, l_greatest);

        // Reverse the sequence from a[k + 1] up to and including the final element a[n].
        elements[(k_greatest + 1)..].reverse();

        permutations.push(elements.clone());
    }
}

fn pt1() {
    let phase_settings = vec![0, 1, 2, 3, 4];
    let permutations = get_permutations(&phase_settings);
    println!("{:?}", permutations);
    let mut current_best_power = 0;
    let mut current_best_setting = None;

    for phase_setting in permutations.iter() {

        // A
        let mut program = get_input();
        let output_a = solve_program(&mut program, &vec![phase_setting[0], 0]);
        
        // B
        let mut program = get_input();
        let output_b = solve_program(&mut program, &vec![phase_setting[1], output_a[0]]);
        
        // C
        let mut program = get_input();
        let output_c = solve_program(&mut program, &vec![phase_setting[2], output_b[0]]);
        
        // D
        let mut program = get_input();
        let output_d = solve_program(&mut program, &vec![phase_setting[3], output_c[0]]);
        
        // E
        let mut program = get_input();
        let output_d = solve_program(&mut program, &vec![phase_setting[4], output_d[0]]);
        
        if output_d[0] > current_best_power {
            current_best_setting = Some(phase_setting);
            current_best_power = output_d[0];
        }
    }

    println!(
        "Solution Part 1: {}",
        current_best_power
    );
}

fn main() {
    pt1();
}
