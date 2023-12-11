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
    let mut galaxy_index = 0;

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

    let mut j = 0;
    for index in 0..map[0].len() {
        if !map.iter().any(|x| x[index + j].node_type == NodeType::Galaxy) {
            empty_rows.push(index);
            j += 1;
            for row in map.iter_mut() {
                row.insert(
                    index + j,
                    Node {
                        node_type: NodeType::Space,
                        galaxy_id: 0,
                        x: 0,
                        y: 0,
                    },
                );
            }
        }
    }

    j = 0;
    for (index, line) in map.clone().iter().enumerate() {
        if !line.iter().any(|x| x.node_type == NodeType::Galaxy) {
            map.insert(index + j, (*line.clone()).to_vec());
            j += 1;
        }
    }

    // for line in map.iter() {
    //     for node in line.iter() {
    //         if node.node_type == NodeType::Space {
    //             print!(".");
    //         } else {
    //             print!("{}", node.galaxy_id);
    //         }
    //     }
    //     println!("");
    // }

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

    let mut part1: i32 = 0;
    for (galaxy_1, galaxy_2) in galaxies.iter().tuple_combinations() {
        pairs += 1;
        // println!(
        //     "Distance between {} and {} is {} + {} = {}",
        //     galaxy_1.galaxy_id,
        //     galaxy_2.galaxy_id,
        //     (galaxy_1.x - galaxy_2.x).abs(),
        //     (galaxy_1.y - galaxy_2.y).abs(),
        //     (galaxy_1.x - galaxy_2.x).abs() + ((galaxy_1.y - galaxy_2.y).abs()).max(0)
        // );

        // println!("{} {}", galaxy_2.x, galaxy_1.x);

        part1 += (galaxy_1.x - galaxy_2.x).abs() + (galaxy_1.y - galaxy_2.y).abs();
    }

    println!("Part 1: {}\nPairs {}", part1);
}
