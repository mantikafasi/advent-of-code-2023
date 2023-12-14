use lazy_static::lazy_static;

lazy_static! {
    static ref CONTENTS: String = include_str!("input.txt").to_string();
}

enum NodeType {
    RoundedRock,
    CubeRock,
    Empty,
}

struct Node {
    x: i32,
    y: i32,
    node_type: NodeType,   
}

fn rotate(lines: Vec<String>) -> Vec<String>  {
    let mut vertical_groups = String::new();

    for group in CONTENTS.split("\n\n") {
        let mut new_group = String::new();
        for i in 0..group.lines().nth(0).unwrap().len() {
            for line in group.lines() {
                if line == "" {
                    break;
                }
                new_group.push(line.chars().nth(i).unwrap())
            }
            new_group.push('\n');
        }

        vertical_groups.push_str(&new_group);
        vertical_groups.push_str("\n\n");

    }

    return vertical_groups.lines().map(|line| line.to_string()).collect(); 
}

fn run(lines: Vec<String>) {
     
}

fn main() {

    let mut nodes = Vec::new();
    let mut lines: Vec<String> = CONTENTS.lines().map(|line| line.to_string()).collect();
    let mut continue_loop = true;
    while continue_loop {
        continue_loop = false;
        for (y,line) in lines.clone().iter().enumerate() {
            for (x,character) in line.chars().enumerate() {
                if character == 'O' && y > 0 && lines[y-1].chars().nth(x).unwrap() == '.' {
                    lines[y - 1].replace_range(x..x+1, "O");
                    lines[y].replace_range(x..x+1, ".");
                    continue_loop = true;
                }
            }
        }
    }

    let mut part1 = 0;
    for (y,line) in lines.iter().enumerate() {
        for (x,character) in line.chars().enumerate() {
            if character == 'O' {
                part1 += lines.len() - y;
            }
        }
    }

    for line in lines {
        println!("{}", line);
    }
    println!("Part 1: {}", part1)
 
}
