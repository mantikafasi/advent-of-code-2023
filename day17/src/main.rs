// this is not complete
use std::{collections::{BinaryHeap, BTreeMap}, thread::sleep, time::Duration, cmp::Ordering};

use lazy_static::lazy_static;

lazy_static! {
    static ref CONTENTS: String = include_str!("input.txt").to_string();
    static ref LINE_LEN: i32 = CONTENTS.lines().nth(0).unwrap().len() as i32;    
}

#[derive(Debug, Clone,PartialEq)]
struct Node {
    x: i32,
    y: i32,
    heat: i32,
}

#[derive(Debug, Clone,Eq,Hash)]
struct State {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    cost: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.dx == other.dx && self.dy == other.dy
    }
}


fn run(nodes:&mut Vec<Vec<Node>>) {
    // println!("{:?}", current_node.distance);
    let mut visited_nodes: BTreeMap<State,i32> = BTreeMap::new();

    let mut max_distance = 0;

    for line in nodes.iter() {
        for node in line.iter() {
            max_distance += node.heat;
        }
    }

    let mut states_to_search: BinaryHeap<State> = BinaryHeap::new();
    
    states_to_search.push(State {
        x: 0,
        y: 0,
        dx: 0,
        dy: 0,
        cost: 0,
    });

    while states_to_search.len() > 0 {
        let current_state = states_to_search.pop().unwrap();

        println!("{} ",current_state.cost);
        sleep(Duration::from_millis(10));

        let neighbors = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        for &(dx, dy) in &neighbors {
            let new_x = current_state.x + dx;
            let new_y = current_state.y + dy;

            if new_x < 0 || new_y < 0 || new_x >= *LINE_LEN || new_y >= nodes.len() as i32 {
                continue;
            }

            if (current_state.dx > 0 && dx < 0) || (current_state.dx < 0 && dx > 0) || (current_state.dy > 0 && dy < 0) || (current_state.dy < 0 && dy > 0) {
                continue;
            }
            
            let mut state = State {
                x: new_x,
                y: new_y,
                dx: current_state.dx + dx,
                dy: current_state.dy + dy,
                cost: current_state.cost + nodes[new_y as usize][new_x as usize].heat,
            };
            

            if state.dx != 0 && state.dy != 0 {
                state.dx = dx;
                state.dy = dy;
            }
            // println!("{:?}",state);
            if state.cost > max_distance {
                continue;
            }

            if !visited_nodes.contains_key(&state) || *visited_nodes.get(&state).unwrap() > state.cost { // !visited_nodes.contains(&state)

                states_to_search.push(state.clone());

                visited_nodes.insert(state.clone(),state.cost);

                if state.x == *LINE_LEN - 1 && state.y == nodes.len() as i32 - 1 {
                    println!("{}",state.cost);
                }
            }
        }
    }

    
    for (state,cost) in visited_nodes.iter() {
        if state.x == *LINE_LEN - 1 && state.y == nodes.len() as i32 - 1 {
            println!("{} ",cost);
        }
    }
}

fn main() {
    let mut nodes: Vec<Vec<Node>> = Vec::new();

    for (i,line) in CONTENTS.lines().enumerate() {
        let mut line_nodes: Vec<Node> = Vec::new();
        for (k,c) in line.chars().enumerate() {
            line_nodes.push(Node {
                x: k as i32,
                y: i as i32,
                heat: c.to_digit(10).unwrap() as i32,
            });
        }
        nodes.push(line_nodes);
    }
    

    run(&mut nodes);

    println!("");
    // println!("Part 1: {}", res);
}
