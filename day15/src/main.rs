use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref CONTENTS: String = include_str!("input.txt").to_string();
}

fn hash(algorithm: &str) -> u32 {
    let mut result = 0;
    for c in algorithm.chars() {
        result += c as u32;
        result *= 17;
        result = result % 256;
    }
    return result;
}

#[derive(Debug, Clone)]
struct Operation {
    op: char,
    focal_length: u32,
    label: String
}

fn main() {
    let map: Vec<Operation> = CONTENTS.split(",").map(|instruction| {
        let split;
        let op;
        let label: String;
        let focal_length ;

        if instruction.contains("=") {
            op = '=';
            split = instruction.split_once("=").unwrap();
            label = split.0.to_string();
            focal_length = split.1.parse::<u32>().unwrap();
        } else {
            op = '-';
            label = instruction[0..instruction.len() - 1].to_string();
            focal_length = 0;
        }
        Operation { op, focal_length, label }
    }).collect();

    let mut boxes:HashMap<u32,Vec<Operation>> = HashMap::new();

    for operation in map {
        let box_id = hash(operation.label.as_str());
        if operation.op == '=' {
            // if same label exists relpace
            let entry = boxes.entry(box_id).or_insert(vec![operation.clone()]);
            let position = entry.iter().position(|x| *x.label == operation.label);
            if position.is_some() {
                entry[position.unwrap()] = operation.clone();
            } else {
                entry.push(operation.clone());
            }
        } else {
            boxes.entry(box_id).and_modify(|e| e.retain(|x| x.label != operation.label));
        } 
    }

    let mut part2 = 0;
    for (i,bx) in boxes.iter() {        
        for (j,x) in bx.iter().enumerate() {
            part2 += (i + 1) * (j + 1) as u32 * x.focal_length;
        }
    }

    let mut part1 = 0;
    CONTENTS.split(",").for_each(|instruction| {
        part1 += hash(instruction);
    });

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
