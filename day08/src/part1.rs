use std::collections::{BTreeMap, HashMap};

use regex::Regex;

#[derive(Debug)]
struct Node {
    left: String,
    right: String
}

fn main() {
    let regex = Regex::new(r"([A-Z]{1,3}) = \(([A-Z]{1,3}), ([A-Z]{1,3})\)").unwrap();

    let contents = include_str!("input.txt");
    
    let input = contents.split_once("\n").unwrap();
    let instructions = input.0;

    let mut nodes: HashMap<String, Node> = HashMap::new();
    for line in input.1.trim().lines() {
        let captures = regex.captures(line).unwrap();
        
        nodes.insert(captures.get(1).unwrap().as_str().to_string(), Node {
            left: captures.get(2).unwrap().as_str().to_string(),
            right: captures.get(3).unwrap().as_str().to_string()
        });
    }

    let mut part1 = 0;

    let mut current_instruction = &String::from("AAA");
    let last_instruction = &String::from("ZZZ");

    while *current_instruction != *last_instruction {
        instructions.chars().for_each( |instruction|{
            let i = nodes.get(current_instruction).unwrap();
    
            match instruction {
                'L' => {
                    current_instruction = &(*i).left;     
                },
                'R' => {
                    current_instruction = &(*i).right;     
                },
                _ => {}
            }
            part1 += 1;
        });
    }


    println!("{}, {:?}", part1, current_instruction);
}
