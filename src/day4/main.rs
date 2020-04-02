fn get_digits(number: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    let mut n = number;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();

    digits
}

fn is_ascending_order(digits: &Vec<u32>) -> bool {
    digits[0..digits.len() - 1]
        .iter()
        .zip(&digits[1..])
        .all(|(a, b)| a <= b)
}

fn has_double_sequence(digits: &Vec<u32>) -> bool {
    // any sequence of 2
    digits[0..digits.len() - 1]
        .iter()
        .zip(&digits[1..])
        .any(|(a, b)| a == b)
}

fn has_double_sequence_strict(digits: &Vec<u32>) -> bool {
    let mut has_double_sequence = false;
    for i in 1..digits.len() {
        if digits[i - 1] == digits[i] {
            let mut not_triple_sequence = true;
            if i != 1 {
                if digits[i - 2] == digits[i - 1] {
                    not_triple_sequence = false;
                }
            }
            if i != 5 {
                if digits[i] == digits[i + 1] {
                    not_triple_sequence = false;
                }
            }
            if not_triple_sequence {
                has_double_sequence = true;
            }
        }
    }
    has_double_sequence
}

fn check_criteria_pt1(digits: &Vec<u32>) -> bool {
    let mut ok = true;
    ok &= digits.len() == 6;
    ok &= has_double_sequence(&digits);
    ok &= is_ascending_order(&digits);

    ok
}

fn check_criteria_pt2(digits: &Vec<u32>) -> bool {
    let mut ok = true;
    ok &= digits.len() == 6;
    ok &= has_double_sequence_strict(&digits);
    ok &= is_ascending_order(&digits);

    ok
}

fn main() {
    let range_start = 125730;
    let range_end = 579381;

    let mut num_viable_pt1 = 0;
    let mut num_viable_pt2 = 0;

    for candidate in range_start..=range_end {
        let digits = get_digits(candidate);

        if check_criteria_pt1(&digits) {
            num_viable_pt1 += 1;
        }
        if check_criteria_pt2(&digits) {
            num_viable_pt2 += 1;
        }
    }

    println!("Solution Part 1: {}", num_viable_pt1);
    println!("Solution Part 2: {}", num_viable_pt2);
}
