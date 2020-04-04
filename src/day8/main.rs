use std::fs::File;
use std::io::{BufReader, Read};

fn get_input<R: Read>(io: R) -> Vec<u32> {
    let mut br = BufReader::new(io);
    let mut string: String = String::default();
    let _result = br.read_to_string(&mut string);
    string.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn pt1() {
    let image_data = get_input(File::open("src/day8/input.txt").unwrap());
    let n_digits_layer = 25 * 6;
    let mut zeros_per_layer = Vec::new();

    for layer in image_data.chunks(n_digits_layer) {
        zeros_per_layer.push(layer.iter().filter(|&d| *d == 0).count() as i32);
    }

    let mut argmin = 0;
    let mut min = n_digits_layer as i32;
    for (i, value) in zeros_per_layer.iter().enumerate() {
        if *value < min {
            min = *value;
            argmin = i;
        }
    }

    let layer_in_question = &image_data[argmin * n_digits_layer..(argmin + 1) * n_digits_layer];
    let solution = layer_in_question.iter().filter(|&d| *d == 1).count()
        * layer_in_question.iter().filter(|&d| *d == 2).count();

    println!("Solution Part 1: {}", solution);
}

fn pt2() {
    let image_data = get_input(File::open("src/day8/input.txt").unwrap());
    let n_digits_layer = 25 * 6;

    let mut final_image = image_data.chunks(n_digits_layer).last().unwrap().to_vec();

    for layer in image_data.chunks(n_digits_layer).rev().skip(1) {
        for (old, new) in final_image.iter_mut().zip(layer) {
            if *new == 2 {
                continue;
            } else {
                *old = *new;
            }
        }
    }

    println!("Solution Part 2:");
    for row in final_image.chunks(25) {
        println!("{:?}", row);
    }
}

fn main() {
    pt1();
    pt2();
}
