use lazy_static::lazy_static;

lazy_static! {
    static ref CONTENTS: String = include_str!("input.txt").to_string();
}

fn main() {
    
    
    let values = CONTENTS.lines().map(|line| line.split_once(" (").unwrap().0.split_once(" ").unwrap());
    
    let grid_length: i32 = values.clone().filter(|(a,b)| *a == "R").map(|(a,b)| b.parse::<i32>().unwrap()).sum::<i32>() + 1;
    let grid_height: i32 = values.clone().filter(|(a,b)| *a == "D").map(|(a,b)| b.parse::<i32>().unwrap()).sum::<i32>() + 1;
    
    let mut grid: Vec<Vec<char>> = Vec::new();
    
    println!("Grid length: {}\nGrid Height: {}", grid_length, grid_height);

    for i in 0..(grid_height * 2) {
        let mut row: Vec<char> = Vec::new();
        for j in 0..(grid_length * 2) {
            row.push('.');
        }
        grid.push(row);
    }

    let mut position = ((grid_height - 1) as usize, (grid_length - 1) as usize);
    

    for line in CONTENTS.lines() {
        let mut split = line.split(" ");

        let direction = split.nth(0).unwrap();
        let count = split.nth(0).unwrap().parse::<i32>().unwrap();
    
        println!("At line {}", line);

        match direction {
            "R" => {
                for i in 0..count {
                    position = (position.0 + 1, position.1);
                    grid[position.1][position.0] = '#';
                }
            },
            "L" => {
                for i in 0..count {
                    position = (position.0 - 1, position.1);
                    grid[position.1][position.0 as usize] = '#';
                }
            },
            "U" => {
                for i in 0..count {
                    position = (position.0, position.1 - 1);
                    grid[position.1 as usize][position.0] = '#';
                }
            },
            "D" => {
                for i in 0..count {
                    position = (position.0, position.1 + 1);
                    grid[position.1 as usize][position.0] = '#';
                }
            },
            _ => {
                println!("Error");
            }

        }

    }

    let mut part1 = 0;

    let mut positions_to_go: Vec<(i32,i32)> = Vec::new();
    positions_to_go.push((grid_length + 1,grid_height - 1));

    while positions_to_go.clone().len() != 0 {
        for position in positions_to_go.clone() {
            if position.1 < grid_height * 2 - 1 && grid[(position.1 + 1) as usize][position.0 as usize] == '.' {
                grid[(position.1 + 1) as usize][position.0 as usize] = '#';
                positions_to_go.push((position.0, position.1 + 1));
            }
            if position.1 > 0 && grid[(position.1 - 1) as usize][position.0 as usize] == '.' {
                grid[(position.1 - 1) as usize][position.0 as usize] = '#';
                positions_to_go.push((position.0, position.1 - 1));
            }

            if position.0 < grid_length * 2 -1  && grid[position.1 as usize][(position.0 + 1) as usize] == '.' {
                grid[position.1 as usize][(position.0 + 1) as usize] = '#';
                positions_to_go.push((position.0 + 1, position.1));
            }

            if position.0 > 0 && grid[position.1 as usize][(position.0 - 1) as usize] == '.' {
                grid[position.1 as usize][(position.0 - 1) as usize] = '#';
                positions_to_go.push((position.0 - 1, position.1));
            }

            positions_to_go.iter().position(|&x| x == position).map(|e| positions_to_go.remove(e));
        }
    }

    for line in grid.clone() {
        part1 += line.iter().filter(|f| **f == '#').count();
    }

    for line in grid {
        println!("{}", line.iter().collect::<String>());
    }

    println!("Part 1: {}", part1);
}
