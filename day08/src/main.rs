use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

pub fn gcd(mut n: i32, mut m: i32) -> i32 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn main() {
    let regex = Regex::new(r"(.{3}) = \((.{3}), (.{3})\)").unwrap();

    let contents = include_str!("input.txt");

    let input = contents.split_once("\n").unwrap();
    let instructions = input.0;

    let mut graph: HashMap<String, Node> = HashMap::new();
    for line in input.1.trim().lines() {
        let captures = regex.captures(line).unwrap();

        graph.insert(
            captures.get(1).unwrap().as_str().to_string(),
            Node {
                left: captures.get(2).unwrap().as_str().to_string(),
                right: captures.get(3).unwrap().as_str().to_string(),
            },
        );
    }

    let current_instructions: Vec<&String> = graph
        .keys()
        .into_iter()
        .filter(|k| k.chars().last().unwrap() == 'A')
        .collect();

    let mut instruction_loop_cycle: Vec<i32> = Vec::new();

    for current_instruction in current_instructions.clone().iter_mut().enumerate() {
        let mut cycle_count = 0;
        let index = current_instruction.0;
        let mut current_instruction = current_instruction.1.clone();

        while current_instruction.chars().last().unwrap() != 'Z' {
            instructions.chars().for_each(|instruction| {
                let node = graph.get(&current_instruction).unwrap();

                match instruction {
                    'L' => {
                        current_instruction = node.left.clone();
                    }
                    'R' => {
                        current_instruction = node.right.clone();
                    }
                    _ => {}
                }
                cycle_count += 1;
            });
        }

        instruction_loop_cycle.insert(index, cycle_count);
    }

    for loop_cycle in instruction_loop_cycle.chunks(2) {
        print!("{},", gcd(loop_cycle[0], loop_cycle[1]));
        // gcd of all numbers are 269
    }

    let mut part2: i64 = 1;
    for loop_cycle in instruction_loop_cycle {
        part2 *= loop_cycle as i64 / 269;
    }

    println!("{}", part2 * 269)
}
