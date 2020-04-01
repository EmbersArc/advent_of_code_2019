



use std::error::Error;
use std::mem::swap;

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn intersect_lines(
    mut start1: Point,
    mut end1: Point,
    mut start2: Point,
    mut end2: Point,
) -> Option<Point> {
    if (start1.y == end1.y && start2.y == end2.y) || (start1.x == end1.x && start2.x == end2.x) {
        return None;
    }

    // swap horizonally
    if start1.x > end1.x {
        swap(&mut start1, &mut end1);
    }
    if start2.x > end2.x {
        swap(&mut start2, &mut end2);
    }
    // swap vertically
    if start1.y > end1.y {
        swap(&mut start1, &mut end1);
    }
    if start2.y > end2.y {
        swap(&mut start2, &mut end2);
    }

    if (start1.x <= start2.x && start2.x <= end1.x) && (start2.y <= start1.y && end2.y >= start1.y)
    {
        return Some(Point {
            x: start2.x,
            y: start1.y,
        });
    }
    swap(&mut start1, &mut start2);
    swap(&mut end1, &mut end2);
    if (start1.x <= start2.x && start2.x <= end1.x) && (start2.y <= start1.y && end2.y >= start1.y)
    {
        return Some(Point {
            x: start2.x,
            y: start1.y,
        });
    }

    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("src/day3/input.csv")?;

    let mut first_path: Vec<Point> = vec![Point { x: 0, y: 0 }];
    let mut second_path: Vec<Point> = vec![Point { x: 0, y: 0 }];

    for (i, result) in rdr.records().enumerate() {
        let record = result.expect("a CSV record");
        for action in record.iter() {
            let length_str: String = action.chars().skip(1).take(action.len() - 1).collect();
            let length: i32 = length_str.parse()?;

            let mut position;
            match i {
                0 => position = first_path.last().cloned().unwrap(),
                1 => position = second_path.last().cloned().unwrap(),
                _ => panic!("More than two wires!"),
            }

            let direction = action.chars().next().unwrap();
            match direction {
                'L' => position.x -= length,
                'R' => position.x += length,
                'D' => position.y -= length,
                'U' => position.y += length,
                _ => panic!("Found unexpected direction!"),
            }

            match i {
                0 => first_path.push(position),
                1 => second_path.push(position),
                _ => panic!("More than two wires!"),
            }
        }
    }

    let mut intersections = Vec::new();

    for i in 0..first_path.len() - 1 {
        for j in 0..second_path.len() - 1 {
            let intersection = intersect_lines(
                first_path[i],
                first_path[i + 1],
                second_path[j],
                second_path[j + 1],
            );
            match intersection {
                Some(p) => intersections.push(p),
                None => (),
            }
        }
    }

    let min_dist = intersections
        .iter()
        .map(|p| (p.x.abs() + p.y.abs()))
        .min()
        .unwrap();

    println!("Solution Part 1: {}", min_dist);

    Ok(())
}

