use std::collections::HashMap;

use regex::Regex;

static mut CONTENTS: String = String::new();

fn main() {
    let mut gear_map: HashMap<usize, Vec<usize>> = HashMap::new();

    // WHY UNSAFE
    // I HATE RUST GLOBAL VARIABLES
    unsafe { CONTENTS = include_str!("input.txt").to_string() };

    let groups = Regex::new(r"[0-9]*\d").unwrap();
    let mut part1 = 0;

    for i in unsafe { 0..CONTENTS.lines().count() } {
        let line = unsafe { CONTENTS.lines().nth(i).unwrap() };
        groups.captures_iter(line).for_each(|f| {
            let capture = f.get(0).unwrap();
            let number = capture.as_str().parse::<usize>().unwrap();
            println!("group {:?}", f.get(0));
            if is_adjacent(number, i, capture.start(), capture.end(), &mut gear_map) {
                part1 += capture.as_str().parse::<usize>().unwrap();
            }
        })
    }

    let mut part2 = 0;
    for (_, v) in gear_map.iter() {
        if v.len() > 1 {
            let mut ratio = 1;
            for g in v.iter() {
                ratio *= g;
            }
            part2 += ratio;
        }
    }

    println!("part1 {}", part1);
    println!("part2 {}", part2);
}

fn is_adjacent(
    number: usize,
    line: usize,
    start: usize,
    end: usize,
    gear_map: &mut HashMap<usize, Vec<usize>>,
) -> bool {
    // apparently rust doesnt allow dynamic alloced stuff on global level, sorry for putting map as parameter

    let line_end = if line + 1 < unsafe { CONTENTS.lines().count() } {
        line + 2
    } else {
        line + 1
    };
    // (line - 1).max(0) I hate usize
    let line_start = if line > 0 { line - 1 } else { line };

    for i in line_start..line_end {
        let chars: Vec<char> = unsafe { CONTENTS.lines().nth(i).unwrap().chars().collect() };

        let start_i = if start > 0 { start - 1 } else { start };

        for j in start_i..(end + 1).min(chars.len()) {
            let char = chars[j];

            if char == '*' {
                if gear_map.get(&((j + 1) * chars.len() + i)).is_none() {
                    gear_map.insert((j + 1) * chars.len() + i, vec![number]);
                } else {
                    gear_map
                        .get_mut(&((j + 1) * chars.len() + i))
                        .unwrap()
                        .push(number);
                }
            }
            if !char.is_numeric() && char != '.' {
                return true;
            }
        }
    }
    return false;
}
