static CONTENTS: &str = include_str!("input.txt");


fn main() {
    let mut grid = CONTENTS.lines().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut possible_locations: Vec<(usize,usize)> = Vec::new();

    // S location
    for (y,row) in grid.clone().iter().enumerate() {
        for (x,col) in row.iter().enumerate() {
            if *col == 'S' {
                possible_locations.push((x,y));
                grid[y][x] = '.';
            }
        }
    }

    let round_count = 64;
    for _ in 0..round_count {
        let mut new_possible_locations: Vec<(usize,usize)> = Vec::new();
        for (x,y) in possible_locations {
            if x > 0 && grid[y][x-1] == '.' {
                new_possible_locations.push((x-1,y));
            }
            if x < grid[y].len()-1 && grid[y][x+1] == '.' {
                new_possible_locations.push((x+1,y));
            }
            if y > 0 && grid[y-1][x] == '.' {
                new_possible_locations.push((x,y-1));
            }
            if y < grid.len()-1 && grid[y+1][x] == '.' {
                new_possible_locations.push((x,y+1));
            }
        }

        new_possible_locations.sort();
        new_possible_locations.dedup();
    
        possible_locations = new_possible_locations;
    } 

    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            if grid[y][x] == '.' {
                print!(" ");
            } else {
                print!("{}",grid[y][x]);
            }
        }
        println!();
    }

    possible_locations.sort();
    possible_locations.dedup();

    println!("Part 1: {}",possible_locations.len());

}
