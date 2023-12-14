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

#[memoize(Capacity: 500)]
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

    let mut r = 0;

    lines = run(lines);

    let first = lines.clone();
    
    println!("{:?}", lines);
    
    for i in 0..1 {
        lines = run(lines);

        // if lines == first {
        //     println!("Found loop at {}", i);
        //     for _ in 0..999999987 % i {
        //         lines = run(lines);
        //     }
        //     break;
        // }

        r += 1;

        if r > 100000 {
            println!("{:.2}% Complete", (i as f64 / 10000000.0));
            r = 0;
        }
    }

    // lines = rotate(lines);

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
