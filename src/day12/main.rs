fn gcd(mut a: i64, mut b: i64) -> i64 {
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

fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

fn main() {
    let positions0: [[i64; 3]; 4] = [[5, -1, 5], [0, -14, 2], [16, 4, 0], [18, 1, 16]];
    let velocities0: [[i64; 3]; 4] = [[0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0]];

    let mut positions = positions0;
    let mut velocities = velocities0;

    let mut positions_1000 = positions0;
    let mut velocities_1000 = velocities0;

    let mut periods: [i64; 3] = [0, 0, 0];

    let mut step = 0;
    while periods.iter().any(|p| *p == 0) {
        for moon1 in 0..4 {
            for moon2 in 0..4 {
                if moon1 == moon2 {
                    continue;
                }

                // gravity
                for axis in 0..3 {
                    let difference = positions[moon2][axis] - positions[moon1][axis];
                    if difference < 0 {
                        velocities[moon1][axis] -= 1;
                    } else if difference > 0 {
                        velocities[moon1][axis] += 1;
                    }
                }
            }
        }

        // position
        for moon in 0..4 {
            for axis in 0..3 {
                positions[moon][axis] += velocities[moon][axis];
            }
        }

        step += 1;

        for axis in 0..3 {
            if periods[axis] == 0 {
                let mut same_state = true;
                for moon in 0..4 {
                    same_state &= positions[moon][axis] == positions0[moon][axis];
                    same_state &= velocities[moon][axis] == velocities0[moon][axis];
                }
                if same_state {
                    periods[axis] = step;
                }
            }
        }

        if step == 1000 {
            positions_1000 = positions;
            velocities_1000 = velocities;
        }
    }

    let e_pot: Vec<i64> = positions_1000
        .iter()
        .map(|p| p[0].abs() + p[1].abs() + p[2].abs())
        .collect();
    let e_kin: Vec<i64> = velocities_1000
        .iter()
        .map(|v| v[0].abs() + v[1].abs() + v[2].abs())
        .collect();

    let e_tot = e_pot
        .iter()
        .zip(e_kin.iter())
        .fold(0, |prod, (p, k)| prod + p * k);

    println!("Solution Part 1: {}", e_tot);
    println!(
        "Solution Part 2: {}",
        periods.iter().fold(1, |l, p| lcm(l, *p))
    );
}
