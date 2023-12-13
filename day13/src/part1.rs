use std::io::Write;

use lazy_static::lazy_static;

lazy_static! {
    static ref CONTENTS: String = include_str!("input.txt").to_string();
}

fn check_similarty(first: &str, second: &str) -> usize {
    let mut difference_count = 0;

    for (i,        character) in first.chars().enumerate() {
        if second.chars().nth(i).unwrap() != character {
            difference_count += 1;
        }
    }
    
    return difference_count;
}

fn count(group: &str) -> usize {
    let lines: Vec<&str> = group.lines().collect();
    for (i, line) in group.lines().enumerate() {
        if i != 0 {
            if line == lines[i - 1] {
                // possible mirror
                // line 4
                println!("ID: {}, {:?}", i + 1, line);

                // let mut j = group.lines().count() - 1;
                let mut k = 0;
                while i + k < lines.len() && i - k > 0 && lines[i + k] == lines[i - k - 1] {
                    k += 1;
                    println!("{} {} {}", i,  i + k, i - k - 1);
                }

                if i + k == lines.len() || i - k == 0 {
                    return i;
                }
                
                // println!("Len {}-{} is not a mirror {}", i - k + 1, i + k + 1, k );
            }
        }
    }
    return 0;
}

fn main() {
    let mut part1 = 0;

    let mut found_indexes = Vec::new();
    
    let mut vertical_groups = String::new();
    
    for group in CONTENTS.split("\n\n") {
        let mut new_group = String::new();
        for i in 0..group.lines().nth(0).unwrap().len() {
            for line in group.lines() {
                if line == "" {
                    // group.push_str("\n\n");
                    break;
                }
                new_group.push(line.chars().nth(i).unwrap())
            }
            new_group.push('\n');
        }
        // println!("{}", new_group);

        vertical_groups.push_str(&new_group);
        vertical_groups.push_str("\n\n"); 
        
        // write to file 
    }
    let mut file = std::fs::File::create("vertical.txt").unwrap();
    file.write_all(vertical_groups.as_bytes()).unwrap();
    
    for (i,group) in vertical_groups.split("\n\n").enumerate() {
        let count = count(group.trim());
        part1 += count;
        
        if count != 0 {
            found_indexes.push(i);
        }
        
        // println!("Count: {}", count);
    }
    
    for (i,group) in CONTENTS.split("\n\n").enumerate() {
        if found_indexes.contains(&i) {
            continue;
        } 

        println!("Trying: {}", i);
        let count = count(group);
        part1 += count * 100;
        if count != 0 {
            found_indexes.push(i);
        }
        println!("Count: {}", count);
    }
    
    
    // missing indexes
    let mut missing_indexes = Vec::new();
    for i in 0..vertical_groups.split("\n\n").count() {
        if !found_indexes.contains(&i) {
            missing_indexes.push(i);
        }
    }

    println!("Missing indexes: {:?}", missing_indexes);

    println!("Part 1: {}", part1);
}
