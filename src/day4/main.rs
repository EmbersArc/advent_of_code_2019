fn check_criteria(candidate: u32) -> bool {
    let mut digits = Vec::new();
    let mut n = candidate;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();

    let mut ok = true;

    ok &= digits.len() == 6;

    // any sequence of 2
    ok &= digits[0..digits.len() - 1]
        .iter()
        .zip(&digits[1..])
        .any(|(a, b)| a == b);

    // ascending order
    ok &= digits[0..digits.len() - 1]
        .iter()
        .zip(&digits[1..digits.len()])
        .all(|(a, b)| a <= b);

    ok
}

fn main() {
    let range_start = 125730;
    let range_end = 579381;

    let mut viable = Vec::new();

    for candidate in range_start..=range_end {
        if check_criteria(candidate) {
            viable.push(candidate)
        }
    }

    println!("Solution Part 1: {}", viable.len());
}
