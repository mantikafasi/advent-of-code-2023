use itertools::Itertools;
use lazy_static::lazy_static;

lazy_static! {
    static ref CONTENTS: String = include_str!("input.txt").to_string();
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum NodeType {
    Space,
    Galaxy,
}

#[derive(Debug, Clone, Copy)]
struct Node {
    node_type: NodeType,
    galaxy_id: u32,
    x: i32,
    y: i32,
}

fn main() {
    let mut map: Vec<Vec<Node>> = Vec::new();
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_columns: Vec<usize> = Vec::new();

    for line in CONTENTS.lines() {
        let mut row: Vec<Node> = Vec::new();

        for c in line.chars() {
            row.push(Node {
                node_type: if c == '.' {
                    NodeType::Space
                } else {
                    NodeType::Galaxy
                },
                galaxy_id: 0,
                x: 0,
                y: 0,
            });
        }
        map.push(row);
    }

    for index in 0..map[0].len() {
        if !map.iter().any(|x| x[index].node_type == NodeType::Galaxy) {
            empty_rows.push(index);
        }
    }

    for (index, line) in map.clone().iter().enumerate() {
        if !line.iter().any(|x| x.node_type == NodeType::Galaxy) {
            empty_columns.push(index);
        }
    }

    let mut galaxies: Vec<Node> = Vec::new();

    for (i, line) in map.iter().enumerate() {
        for (j, node) in line.iter().enumerate() {
            if node.node_type == NodeType::Galaxy {
                galaxies.push(Node {
                    node_type: NodeType::Galaxy,
                    galaxy_id: node.galaxy_id,
                    x: j as i32,
                    y: i as i32,
                });
            }
        }
    }

    let mut part2: i64 = 0;
    for (galaxy_1, galaxy_2) in galaxies.iter().tuple_combinations() {
        for i in empty_rows.clone() {
            if (galaxy_1.x < i as i32 && galaxy_2.x > i as i32)
                || (galaxy_1.x > i as i32 && galaxy_2.x < i as i32)
            {
                part2 += 999999;
            }
        }

        for i in empty_columns.clone() {
            if (galaxy_1.y < i as i32 && galaxy_2.y > i as i32)
                || (galaxy_1.y > i as i32 && galaxy_2.y < i as i32)
            {
                part2 += 999999;
            }
        }

        part2 += ((galaxy_1.x - galaxy_2.x).abs() + (galaxy_1.y - galaxy_2.y).abs()) as i64;
    }

    println!("Part 2: {}", part2);
}
