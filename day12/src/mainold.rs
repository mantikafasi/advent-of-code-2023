// This is my old terrible solution dont use

use std::iter;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CONTENTS: String = include_str!("input.txt").to_string();
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    springs: Vec<char>,
    groups: Vec<u32>,
}

fn is_valid(node: &Node) -> bool {
    let mut valid = true;
    let mut spring: String = node.springs.clone().into_iter().collect();
    spring = spring.replace("?", ".");
    spring = ".".to_owned() + &spring + ".";
    while spring.contains("..") {
        spring = spring.replace("..", ".");
    }

    for x in node.groups.clone() {
        let pattern = ".".to_owned() + &"#".repeat(x as usize) + ".";
        let matc = spring.starts_with(pattern.as_str());

        //println!("{:?}", pattern);
        if !matc {
            valid = false;
            break;
        }

        // println!("{:?}", spring);
        // println!("{:?}", matc.unwrap());
        spring = spring[pattern.len() - 1 as usize..].to_string().replace("..", ".");
    }
    if spring.contains("#") {
        valid = false;
    }
    valid
}

fn main() {
    let map: Vec<Node> = CONTENTS
        .lines()
        .map(|line| {
            let contents = line.split_once(" ").unwrap();
            let mut springs: Vec<char> = contents.0.chars().collect();
            let mut groups: Vec<u32> = contents
                .1
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            // groups = groups.repeat(5);
            // springs.insert(0, '?');
            // springs = springs.repeat(5);
            Node { springs, groups }
        })
        .collect();

    let mut part1 = 0;

    for (i,line) in map.iter().enumerate() {
        let route = 0;

        let indexes = line.springs.iter().enumerate().filter(|(_, x)| **x == '?').map(|(i, _)| i).collect_vec();
        for combination in indexes.iter().powerset() {
            let mut new_node = line.clone();
            for i in combination {
                new_node.springs[*i] = '#';
            }
            if is_valid(&new_node) {
                part1 += 1;
                route += 1;
            }
        }
        println!("On line {}: {}", i, route);

    }


    // println!("{}", is_valid(&Node { springs: "..#..#....###.".chars().collect(), groups: vec!(1,1,3) }));
    println!("Part 1: {}", part1);
}