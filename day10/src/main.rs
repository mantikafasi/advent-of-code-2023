
use std::ops::IndexMut;

use lazy_static::lazy_static;

#[derive(Debug,PartialEq, Eq, Clone)]
enum PipeType {
    Empty,
    Vertical,
    Horizontal,
    Start,
    NE,
    NW,
    SW,
    SE,
}

lazy_static! {
    static ref CONTENTS: String = include_str!("input.txt").to_string();
}

#[derive(Clone, Debug)]
struct Node {
    distance: u32,
    pipe_type: PipeType,
    valid_directions: (bool, bool, bool, bool), // NSEW
}

fn run(map:&mut Vec<Vec<Node>>, location: (i32,i32), previous_direction: i32) {

    let pipe = map[location.0 as usize][location.1 as usize].clone();
    let mut directions = pipe.valid_directions;
    
    if pipe.pipe_type == PipeType::Start {
        println!(" Ended {} {} {:?}", location.0, location.1,pipe);
        if pipe.distance > 0 {
            return;
        }
    }

    match previous_direction {
        0 => directions.0 = false,
        1 => directions.1 = false,
        2 => directions.2 = false,
        3 => directions.3 = false,
        _ => (),   
    }
        
    if directions.0 && (location.0 - 1 >= 0) { // N
        let mut up = map[(location.0 - 1) as usize][location.1 as usize].clone();

        if up.valid_directions.1 && (up.distance >= pipe.distance + 1 || up.distance == 0) {
            up.distance = pipe.distance + 1;
            map.index_mut((location.0 - 1) as usize).index_mut(location.1 as usize).distance = up.distance;

            run(map, (location.0 - 1, location.1), 1);
        }
    }

    if directions.1 && (location.0 + 1 <= (map.len() - 1) as i32) { // S
        let mut down = map[(location.0 + 1) as usize][location.1 as usize].clone();
        if down.valid_directions.0 && (down.distance >= pipe.distance + 1 || down.distance == 0) {
            down.distance = pipe.distance + 1;
            map.index_mut((location.0 + 1) as usize).index_mut(location.1 as usize).distance = down.distance;

            run(map, (location.0 + 1, location.1), 0);
        }
    }

    if directions.2 && (location.1 - 1 >= 0) { // E
        let mut right = map[location.0 as usize][(location.1 - 1) as usize].clone();
        if right.valid_directions.3 && (right.distance >= pipe.distance + 1 || right.distance == 0) {
            right.distance = pipe.distance + 1;
            map.index_mut(location.0 as usize).index_mut((location.1 - 1) as usize).distance = right.distance;

            run(map, (location.0, location.1 - 1), 3);
        }
    }

    if directions.3 && (location.1 + 1 <= (map[0].len() - 1) as i32) { // W
        let mut left = map[location.0 as usize][(location.1 + 1) as usize].clone();
        if left.valid_directions.2 && (left.distance >= pipe.distance + 1 || left.distance == 0) {
            left.distance = pipe.distance + 1;
            map.index_mut(location.0 as usize).index_mut((location.1 + 1) as usize).distance = left.distance;

            run(map, (location.0, location.1 + 1), 2);
        }
    }
    
}

fn main() {
    let mut map: Vec<Vec<Node>> = {
        let mut map_1 = Vec::new();
        CONTENTS.lines().for_each(|line| {
            let mut row: Vec<Node> = Vec::new();
            line.chars().for_each(|c| {
                let node = match c {
                    '.' => Node { distance: 0, pipe_type: PipeType::Empty, valid_directions: (false, false, false, false) },
                    '|' => Node { distance: 0, pipe_type: PipeType::Vertical, valid_directions: (true, true, false, false) },
                    '-' => Node { distance: 0, pipe_type: PipeType::Horizontal, valid_directions: (false, false, true, true) },
                    'S' => Node { distance: 0, pipe_type: PipeType::Start, valid_directions: (true, true, true, false) },
                    'L' => Node { distance: 0, pipe_type: PipeType::NE, valid_directions: (true, false, false, true) },
                    'J' => Node { distance: 0, pipe_type: PipeType::NW, valid_directions: (true, false, true, false) },
                    '7' => Node { distance: 0, pipe_type: PipeType::SW, valid_directions: (false, true, true, false) },
                    'F' => Node { distance: 0, pipe_type: PipeType::SE, valid_directions: (false, true, false, true) },
                    _ => {Node { distance: 0, pipe_type: PipeType::Empty, valid_directions: (false, false, false, false) }},
                };
                row.push(node); 
            });
            map_1.push(row);
        });
        map_1
    };

    let start: (i32, i32) = {
        let mut start: (i32, i32) = (0, 0);
        map.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, node)| {
                if node.pipe_type == PipeType::Start {
                    start = (i as i32, j as i32);
                }
            });
        });
        start
    };
    
    run(&mut map, start,4);
    
    let part1 = map.iter().flat_map(|row| row.iter()).filter(|node| node.distance != 0).count();
    let mut part2 = 0;

    for (i,line) in map.clone().iter().enumerate().skip(1) {
        for (j,node) in line.iter().enumerate().skip(1) {
            let mut left = 0;
            let mut top = 0;

            if node.distance > 0 {
                continue;
            }
            
            for x in 0..j {
                let target = &map[i][x];
                if target.distance > 0 && (target.pipe_type == PipeType::NE || target.pipe_type == PipeType::NW || target.pipe_type == PipeType::Vertical) {
                    left += 1;
                }
            }

            for h in 0..i {
                let target = &map[h][j];
                if target.distance > 0 && (target.pipe_type == PipeType::Horizontal || target.pipe_type == PipeType::NE || target.pipe_type == PipeType::SE) {
                    top += 1;
                }
            }

            if left > 0 && top > 0 && left % 2 == 1 && top % 2 == 1 {
                part2 += 1;
            }
        }
    }

    println!("Part 1: {}\nPart 2: {}", part1 / 2, part2);
}
