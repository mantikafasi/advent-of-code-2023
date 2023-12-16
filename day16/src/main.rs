use crossterm::{cursor, style, ExecutableCommand};
use std::{io::{stdout, Write}, time::Duration, thread};
use lazy_static::lazy_static;

lazy_static! {
    static ref CONTENTS: String = include_str!("input.txt").to_string();
    static ref LINE_LEN: i32 = CONTENTS.lines().nth(0).unwrap().len() as i32;    
}

#[derive(Debug, Clone,PartialEq)]
struct Position {
    x: i32,
    y: i32,
    direction: char
}

fn print_lasers(lasers: &Vec<Position>) {
    
    let mut stdout = stdout();

    for place in lasers {
        if place.x < 0 || place.y < 0 {
            continue;
        }
        
        let symbol = match place.direction {
            'r' => ">",
            'l' => "<",
            'u' => "^",
            'd' => "v",
            _ => ""
        };
        stdout.execute(style::SetForegroundColor(style::Color::Green));
        stdout.execute(cursor::MoveTo(place.x as u16, place.y as u16));
        write!(stdout, "{}", style::Print(symbol));
    }

    // Flush stdout to ensure the screen is updated
    stdout.flush();
    thread::sleep(Duration::from_millis(50));
}


fn run(pos: Position) -> i32 {
    let mut lasers: Vec<Position> = Vec::new();
    let mut visited_places: Vec<Position> = Vec::new();
    let mut visited_indexes: Vec<i32> = Vec::new();
    let mut lasers_to_remove: Vec<Position> = Vec::new();

    lasers.push(pos);

    while lasers.len() != 0 {

        // let _ = lasers_to_remove.iter().map(|laser|lasers.iter().position(|x| *x == *laser).map(|x| lasers.remove(x)));
        // print_lasers(&lasers);

        for laser in lasers_to_remove.clone() {
            lasers.iter().position(|x| *x == laser).map(|x| lasers.remove(x));
        }
        
        lasers_to_remove.clear();

        for (i,laser) in lasers.clone().iter().enumerate() {
            if visited_places.contains(laser) {
                lasers_to_remove.push(laser.clone());
                continue;
            }

            let mut laser: Position = laser.clone();
            
            if laser.y > (CONTENTS.lines().count() - 1) as i32 || laser.y < 0 || laser.x > *LINE_LEN - 1 || laser.x < 0{
                lasers_to_remove.push(laser.clone());
                continue;
            }
            
            visited_places.push(laser.clone());
            visited_indexes.push(laser.x + laser.y * *LINE_LEN);
            let char = CONTENTS.lines().nth(laser.y as usize).unwrap().chars().nth(laser.x as usize).unwrap();

            match char {
                '/' => {
                    match laser.direction {
                        'r' => {
                            laser.direction = 'u';
                            laser.y -= 1;
                        },
                        'l' => {
                            laser.direction = 'd';
                            laser.y += 1;
                        },
                        'u' => {
                            laser.direction = 'r';
                            laser.x += 1;
                        },
                        'd' => {
                            laser.direction = 'l';
                            laser.x -= 1;
                        },
                        _ => {}
                    }
                },
                '\\' => {
                    match laser.direction {
                        'r' => {
                            laser.direction = 'd';
                            laser.y += 1;
                        },
                        'l' => {
                            laser.direction = 'u';
                            laser.y -= 1;
                        },
                        'u' => {
                            laser.direction = 'l';
                            laser.x -= 1;
                        },
                        'd' => {
                            laser.direction = 'r';
                            laser.x += 1;
                        },
                        _ => {}
                    }
                },
                '|' => {
                    match laser.direction {
                        'r' | 'l' => {
                            laser.direction = 'd';
                            lasers.push(Position { x: laser.x, y: laser.y - 1, direction: 'u' });
                            laser.y += 1;
                        },
                        'u' => {
                            laser.y -= 1;
                        },
                        'd' => {
                            laser.y += 1;
                        },
                        _ => {}
                    }

                },
                '-' => {
                    match laser.direction {
                        'r' => {
                            laser.x += 1;
                        },
                        'l' => {
                            laser.x -= 1;
                        },
                        'u' | 'd' => {
                            laser.direction = 'r';
                            lasers.push(Position { x: laser.x -1, y: laser.y, direction: 'l' });
                            laser.x += 1;
                        }
                        _ => {}
                    }
                },
                '.' => {
                    match laser.direction {
                        'r' => {
                            laser.x += 1;
                        },
                        'l' => {
                            laser.x -= 1;
                        },
                        'u' => {
                            laser.y -= 1;
                        },
                        'd' => {
                            laser.y += 1;
                        },
                        _ => {}
                    }
                },
                _ => {}
            }

            lasers[i] = laser;
        }
    }
    visited_indexes.sort();
    visited_indexes.dedup();

    visited_indexes.len() as i32
}

fn main() {
    // let mut stdout = stdout();

    // for (y, line) in CONTENTS.lines().enumerate() {
    //     // Move the cursor to the start of the line
    //     stdout.execute(cursor::MoveTo(0, y as u16));

    //     // Print the line
    //     write!(stdout, "{}", style::Print(line));
    // }

    println!("Part 1: {}", run(Position { x: 0, y: 0 as i32, direction: 'r' }));
    
    let mut results: Vec<i32> = Vec::new();
    
    for y in 0..CONTENTS.lines().count() {
        results.push(run(Position { x: 0, y: y as i32, direction: 'r' }));
        results.push(run(Position { x: *LINE_LEN - 1, y: y as i32, direction: 'l' }));
    }

    for x in 0..*LINE_LEN {
        results.push(run(Position { x: x as i32, y: 0, direction: 'd' }));
        results.push(run(Position { x: x as i32, y: CONTENTS.lines().count() as i32 - 1, direction: 'u' }));
    }

    println!("Part 2: {}", results.iter().max().unwrap());
}
