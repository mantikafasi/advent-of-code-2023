use lazy_static::lazy_static;

lazy_static! {
    static ref CONTENTS: String = include_str!("input.txt").to_string();
}

#[derive(Debug,Clone, PartialEq)]
struct Range {
    start: (i64,i64),
    end: (i64,i64),
    direction: char,
}

fn main() {
    
    
    let values = CONTENTS.lines().map(|line| line.split_once(" (").unwrap().0.split_once(" ").unwrap());
    
    let mut position: (i64, i64) = ((0) as i64, (0) as i64);
    let mut ranges: Vec<Range> = Vec::new();
    let mut part2 = 0;

    for line in CONTENTS.lines() {
        let mut position_end = position.clone();

        let split = line.split_once("(#").unwrap().1.replace(")", "");

        let direction = split.chars().last().unwrap();

        let value = split.chars().take(split.len() - 1).collect::<String>();  
        
        let value = i64::from_str_radix(value.as_str(), 16).unwrap();
        println!("{}: {}", direction, value);
        let mut direction_c = ' ';

        match direction {
            '0' => {
                direction_c = 'R';
                position_end = (position.0 + value, position.1);
                // grid[position.1][position.0] = '#';
            },
            '1' => {
                direction_c = 'D';
                position_end = (position.0, position.1 + value);
                // grid[position.1][position.0] = '#';
            },
            '2' => {
                direction_c = 'L';
                position_end = (position.0 - value, position.1);
            },
            '3' => {
                direction_c = 'U';                
                position_end = (position.0, position.1 - value);
            },
            _ => {},
        }

        ranges.push(Range {
            start: position,
            end: position_end,
            direction: direction_c,
        });
        
        // let mut split = line.split(" ");

        // let direction = split.nth(0).unwrap().chars().nth(0).unwrap();
        // let value = split.nth(0).unwrap().parse::<i64>().unwrap();

        //     match direction {
        //         'R' => {
        //             position_end = (position.0 + value, position.1);
        //             // grid[position.1][position.0] = '#';
        //         },
        //         'D' => {
        //             position_end = (position.0, position.1 + value);
        //             // grid[position.1][position.0] = '#';
        //         },
        //         'L' => {
        //             position_end = (position.0 - value, position.1);
        //         },
        //         'U' => {
        //             position_end = (position.0, position.1 - value);
        //         },
        //         _ => {},
        // }
        part2 += value;

        // ranges.push(Range {
        //     start: position,
        //     end: position_end,
        //     direction: direction,
        // });
        println!("{}: {:?} -> {:?}", direction, position, position_end);
        position = position_end;
    }


    for range in ranges.clone() {
        match range.direction {
            'L' => {
                // println!("{}: {}", range.end.0 + 1 , range.start.0);
                'outer : for x in (range.end.0) .. range.start.0 + 1 {
                    let mut min_y = i64::MIN;

                    if ranges.iter().any(|f| (f.direction == 'U' || f.direction == 'D') && f.start.0 == x && (f.start.1 == range.end.1 || f.end.1 == range.start.1)) {
                        continue;
                    }

                    for range_1 in ranges.clone() {
                        if (range_1.direction == 'U' || range_1.direction == 'D') && range_1.start.0 == x {
                            if range_1.direction == 'D' {
                                // start olabilir 
                                if range_1.end.1 == range.start.1 {
                                    continue 'outer;
                                }
                            } else {
                                if range_1.start.1 == range.end.1 {
                                    continue 'outer;
                                }
                            }

                        }
                    }

                    for range_2 in ranges.clone().iter().filter(|f| f.direction != 'L') {

                        if range_2.direction == 'R' && x >= range_2.start.0 && x <= range_2.end.0 && range_2.start.1 < range.start.1 {
                            // println!("Found {} {:?}", x,  range_2);

                            if min_y == i64::MIN || range_2.start.1 > min_y {
                                min_y = range_2.start.1;
                            }
                        }
                    }
                    if min_y != i64::MIN && range.start.1 - min_y - 1 > 0 {
                        // println!("{:?}: {}", range, range.start.1 - min_y - 1);
                        part2 += range.start.1 - min_y - 1;
                    }
                }
            },
            _ => {},
        }   
    }


    println!("Part 2: {}", part2);
}

// 952407043972
// 952408144115

// 62762175165247
// 62762509300678