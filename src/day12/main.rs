fn pt1() {
    let mut positions: [[i64; 3]; 4] = [[5, -1, 5], [0, -14, 2], [16, 4, 0], [18, 1, 16]];
    let mut velocities: [[i64; 3]; 4] = [[0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0]];

    let mut states = vec![(positions, velocities)];

    for _ in 0..1000000 {
        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    continue;
                }

                // gravity
                for axis in 0..3 {
                    let difference = positions[j][axis] - positions[i][axis];
                    if difference < 0 {
                        velocities[i][axis] -= 1;
                    } else if difference > 0 {
                        velocities[i][axis] += 1;
                    }
                }
            }
        }

        // position
        for i in 0..4 {
            for axis in 0..3 {
                positions[i][axis] += velocities[i][axis];
            }
        }

        states.push((positions, velocities));
    }

    let e_pot: Vec<i64> = states[1000]
        .0
        .iter()
        .map(|p| p[0].abs() + p[1].abs() + p[2].abs())
        .collect();
    let e_kin: Vec<i64> = states[1000]
        .1
        .iter()
        .map(|v| v[0].abs() + v[1].abs() + v[2].abs())
        .collect();

    let e_tot = e_pot
        .iter()
        .zip(e_kin.iter())
        .fold(0, |prod, (p, k)| prod + p * k);

    println!("Solution Part 1: {}", e_tot);

    let mut periods: [[usize; 3]; 4] = [[0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0]];

    for axis in 0..3 {
        for planet in 0..4 {
            let period_p = states[1..]
                .iter()
                .position(|&x| x.0[planet][axis] == states[0].0[planet][axis])
                .unwrap();

            let period_v = states[1..]
                .iter()
                .position(|&x| x.1[planet][axis] == states[0].1[planet][axis])
                .unwrap();

            periods[planet][axis] = lcm(period_p as i64, period_v as i64) as usize;
        }
    }

    println!("{:?}", periods);

    let lcm1: i64 = periods
        .iter()
        .map(|[x, y, z]| lcm(*x as i64, lcm(*y as i64, *z as i64)))
        .fold(1, |a, b| lcm(a, b));

    println!("{:?}", lcm1);
}

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
    pt1();
}
