use lazy_static::lazy_static;
use memoize::memoize;

lazy_static! {
    static ref CONTENTS: String = include_str!("input.txt").to_string();
}

fn rotate(lines: &mut Vec<String>) {
    let len = lines[0].len();
    let mut new_group = String::new();
    for i in 0..len {

        for line in lines.iter().rev() {
            if line == "" {
                break;
            }
            new_group.push(line.chars().nth(i).unwrap())
        }
        new_group.push('\n');
    }

    *lines = new_group.trim().split('\n').map(|line| line.to_string()).collect();
}

fn run(lines: Vec<String>) -> Vec<String>  {
    let mut lines = lines.clone();

    for _ in 0..4 {
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

        rotate(&mut lines);
    }

    return lines;
}

fn main() {
    let mut lines: Vec<String> = CONTENTS.lines().map(|line| line.to_string()).collect();

    let mut part1 = 0;
    lines = run(lines);    
    let mut cache: Vec<Vec<String>> = Vec::new();
    
    for i in 0..1000000000 {
        lines = run(lines);

        if cache.contains(&lines) {
            let j = cache.iter().position(|x| *x == lines).unwrap();

            println!("Found loop at {}", i);
            for _ in 0..1000000000 % (100 - j) {
                lines = run(lines);
            }
            break;
        }

        if cache.len() > 100 {
            cache.remove(0);
        }

        cache.push(lines.clone());
    }

    for (y,line) in lines.iter().enumerate() {
        for character in line.chars() {
            if character == 'O' {
                part1 += lines.len() - y;
            }
        }
    }
    
    println!("Part 2: {}", part1)
}
