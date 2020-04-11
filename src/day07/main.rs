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

struct Program {
    initial_state: Vec<i32>,
    memory: Vec<i32>,
    i: usize,
    inputs: Vec<i32>,
    i_input: usize,
    is_done: bool,
}

impl Program {
    fn from(memory: &Vec<i32>) -> Program {
        Program {
            initial_state: memory.clone(),
            memory: memory.clone(),
            i: 0,
            inputs: Vec::new(),
            i_input: 0,
            is_done: false,
        }
    }
    fn set_input(&mut self, input: &Vec<i32>) {
        self.inputs = input.clone();
        self.i_input = 0;
    }
    fn reset(&mut self) {
        self.memory = self.initial_state.clone();
        self.i_input = 0;
        self.is_done = false;
        self.inputs.clear();
        self.i = 0;
    }
    fn step(&mut self) -> i32 {
        // references
        let memory = &mut self.memory;
        loop {
            // parse instruction
            let digits = get_digits(memory[self.i]);
            let opcode = 10 * digits[1] + digits[0];
            let mode_par1 = digits[2];
            let mode_par2 = digits[3];
            let mode_par3 = digits[4]; // will never be immediate
            if mode_par3 != 0 {
                panic!("What");
            }

            if opcode == 99 {
                self.is_done = true;
                return -1;
            }

            match opcode {
                1 | 2 | 7 | 8 => {
                    let input_1 = match mode_par1 {
                        0 => memory[memory[self.i + 1] as usize],
                        _ => memory[self.i + 1], // 1
                    };
                    let input_2 = match mode_par2 {
                        0 => memory[memory[self.i + 2] as usize],
                        _ => memory[self.i + 2], // 1
                    };

                    let to = memory[self.i + 3] as usize;

                    if opcode == 1 {
                        // add
                        memory[to] = input_1 + input_2;
                    } else if opcode == 2 {
                        // multiply
                        memory[to] = input_1 * input_2;
                    } else if opcode == 7 {
                        // less than
                        memory[to] = (input_1 < input_2) as i32;
                    } else if opcode == 8 {
                        // equals
                        memory[to] = (input_1 == input_2) as i32;
                    }

                    self.i += 4;
                }
                3 => {
                    //input
                    let to = memory[self.i + 1] as usize;
                    if self.i_input >= self.inputs.len() {
                        panic!("No more inputs available!");
                    }
                    memory[to] = self.inputs[self.i_input];
                    self.i_input += 1;
                    self.i += 2;
                }
                4 => {
                    // output
                    let input_1 = match mode_par1 {
                        0 => memory[memory[self.i + 1] as usize],
                        _ => memory[self.i + 1], // 1
                    };

                    self.i += 2;
                    return input_1;
                }
                5 | 6 => {
                    let input_1 = match mode_par1 {
                        0 => memory[memory[self.i + 1] as usize],
                        _ => memory[self.i + 1], // 1
                    };
                    let input_2 = match mode_par2 {
                        0 => memory[memory[self.i + 2] as usize],
                        _ => memory[self.i + 2], // 1
                    };

                    if opcode == 5 {
                        // jump if true
                        if input_1 != 0 {
                            self.i = input_2 as usize;
                        } else {
                            self.i += 3;
                        }
                    } else if opcode == 6 {
                        // jump if false
                        if input_1 == 0 {
                            self.i = input_2 as usize;
                        } else {
                            self.i += 3;
                        }
                    }
                }
                _ => panic!("Encountered invalid opcode '{}'!", opcode),
            }
        }
    }
}

/** Returns digits in reversed order in vector of size 5: 123 -> [0,0,3,2,1] */
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

struct Permutation {
    elements: Vec<i32>,
}

impl Permutation {
    fn from(elements: &Vec<i32>) -> Permutation {
        let mut elements = elements.clone();
        elements.sort();
        Permutation { elements: elements }
    }
}

impl Iterator for Permutation {
    type Item = Vec<i32>;
    //** Heap's Algorithm to find permutations */
    fn next(&mut self) -> Option<Self::Item> {
        // Find the largest index k such that a[k] < a[k + 1]. If no such index exists, the permutation is the last permutation.
        let mut k_greatest = self.elements.len();
        for k in 0..self.elements.len() - 1 {
            if self.elements[k] < self.elements[k + 1] {
                k_greatest = k;
            } else if k == self.elements.len() - 2 && k_greatest == self.elements.len() {
                // does not exist
                return None;
            }
        }

        // Find the largest index l greater than k such that a[k] < a[l].
        let mut l_greatest = self.elements.len();
        for l in 0..self.elements.len() {
            if self.elements[k_greatest] < self.elements[l] {
                l_greatest = l;
            }
        }

        // Swap the value of a[k] with that of a[l].
        self.elements.swap(k_greatest, l_greatest);

        // Reverse the sequence from a[k + 1] up to and including the final element a[n].
        self.elements[(k_greatest + 1)..].reverse();

        return Some(self.elements.clone());
    }
}

fn pt1() {
    let phase_settings = vec![0, 1, 2, 3, 4];
    let permutations = Permutation::from(&phase_settings);
    let mut current_best_power = 0;

    for phase_setting in permutations {
        // A
        let mut program_a = Program::from(&get_input());
        program_a.set_input(&vec![phase_setting[0], 0]);
        let output_a = program_a.step();
        // B
        let mut program_b = Program::from(&get_input());
        program_b.set_input(&vec![phase_setting[1], output_a]);
        let output_b = program_b.step();
        // C
        let mut program_c = Program::from(&get_input());
        program_c.set_input(&vec![phase_setting[2], output_b]);
        let output_c = program_c.step();
        // D
        let mut program_d = Program::from(&get_input());
        program_d.set_input(&vec![phase_setting[3], output_c]);
        let output_d = program_d.step();
        // E
        let mut program_e = Program::from(&get_input());
        program_e.set_input(&vec![phase_setting[4], output_d]);
        let output_e = program_e.step();

        if output_e > current_best_power {
            current_best_power = output_e;
        }
    }

    println!("Solution Part 1: {}", current_best_power);
}

fn pt2() {
    let phase_settings = vec![5, 6, 7, 8, 9];
    let permutations = Permutation::from(&phase_settings);
    let mut program_a = Program::from(&get_input());
    let mut program_b = Program::from(&get_input());
    let mut program_c = Program::from(&get_input());
    let mut program_d = Program::from(&get_input());
    let mut program_e = Program::from(&get_input());

    let mut max_power = 0;

    for phase_setting in permutations {
        program_a.reset();
        program_b.reset();
        program_c.reset();
        program_d.reset();
        program_e.reset();

        program_a.set_input(&vec![phase_setting[0], 0]);
        let mut output_a = program_a.step();
        program_b.set_input(&vec![phase_setting[1], output_a]);
        let mut output_b = program_b.step();
        program_c.set_input(&vec![phase_setting[2], output_b]);
        let mut output_c = program_c.step();
        program_d.set_input(&vec![phase_setting[3], output_c]);
        let mut output_d = program_d.step();
        program_e.set_input(&vec![phase_setting[4], output_d]);
        let mut output_e = program_e.step();

        let mut last_output_e = output_e;
        loop {
            program_a.set_input(&vec![output_e]);
            output_a = program_a.step();

            program_b.set_input(&vec![output_a]);
            output_b = program_b.step();

            program_c.set_input(&vec![output_b]);
            output_c = program_c.step();

            program_d.set_input(&vec![output_c]);
            output_d = program_d.step();

            program_e.set_input(&vec![output_d]);
            output_e = program_e.step();

            // break if halted
            if program_e.is_done {
                if last_output_e > max_power {
                    max_power = last_output_e;
                }
                break;
            } else {
                last_output_e = output_e.clone();
            }
        }
    }
    println!("Solution Part 2: {}", max_power);
}

fn main() {
    pt1();
    pt2();
}
