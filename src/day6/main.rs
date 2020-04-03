use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn get_input() -> Vec<Vec<String>> {
    let file = File::open("src/day6/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut all_relations: Vec<Vec<String>> = Vec::new();

    for line in reader.lines() {
        let relation = line.unwrap();
        let split = relation
            .split(")")
            .map(String::from)
            .collect::<Vec<String>>();
        all_relations.push(split);
    }

    all_relations
}

fn main() -> io::Result<()> {
    let inputs = get_input();
    let mut relations = HashMap::new();

    for input in inputs.iter() {
        relations.insert(input[1].clone(), input[0].clone());
    }

    let mut total = 0;
    for (_, inner) in relations.iter() {
        // pick leaf and backtrack
        total += 1;
        let mut current_inner = inner;
        while *current_inner != "COM" {
            current_inner = relations.get(current_inner).unwrap();
            total += 1;
        }
    }

    println!("Solution Part 1: {}", total);

    let mut path_you: Vec<String> = Vec::new();
    let mut path_san = Vec::new();

    let start_you = "YOU";
    let mut current_position = relations.get(start_you).unwrap();
    while current_position != "COM" {
        path_you.push(current_position.clone());
        current_position = relations.get(current_position).unwrap();
    }
    let start_san = "SAN";
    let mut current_position = relations.get(start_san).unwrap();
    while current_position != "COM" {
        path_san.push(current_position.clone());
        current_position = relations.get(current_position).unwrap();
    }

    let mut i = 0;
    while path_you[path_you.len() - i - 1] == path_san[path_san.len() - i - 1] {
        i += 1;
    }

    println!(
        "Solution Part 2: {}",
        (path_you.len() - i) + (path_san.len() - i)
    );

    Ok(())
}
