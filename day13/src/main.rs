use lazy_static::lazy_static;

lazy_static! {
    static ref CONTENTS: String = include_str!("input.txt").to_string();
}

fn check_similarty(first: &str, second: &str) -> usize {
    let mut difference_count = 0;

    for (i, character) in first.chars().enumerate() {
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
            let mut similarity = check_similarty(line, lines[i - 1]);
            if similarity < 2 {
                println!("ID: {}, {:?}", i + 1, line);

                let mut k = 1;

                while i + k < lines.len() && i - k > 0 && similarity < 2 {
                    similarity += check_similarty(lines[i + k], lines[i - k - 1]);
                    println!("Similarity for {} : {}", i - k - 1, similarity);
                    k += 1;
                }
                println!("{} {} {}", i, i + k, i - k - 1);

                if (i + k == lines.len() || i - k == 0) && similarity == 1 {
                    return i;
                }
            }
        }
    }
    return 0;
}

fn main() {
    let mut part1 = 0;

    let mut found_indexes = Vec::new();

    for (i, group) in CONTENTS.split("\n\n").enumerate() {
        if found_indexes.contains(&i) {
            continue;
        }

        println!("Trying: {}", i);
        let count = count(group);
        part1 += count * 100;
        if count != 0 {
            found_indexes.push(i);
        }
        println!("Horizontal Count: {}", count);
    }

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

    for (i, group) in vertical_groups.split("\n\n").enumerate() {
        let count = count(group.trim());
        part1 += count;
        println!("Vertical Count: {}", count);
        if count != 0 {
            found_indexes.push(i);
        }
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
