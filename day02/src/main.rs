use std::fs;
use regex::Regex;

struct Bag {
    green: i32,
    red: i32,
    blue: i32,
}

pub fn main() {
    let contents = fs::read_to_string("input.txt").expect("Exploded for some reason.");
    let groups = Regex::new(r"([0-9]{1,2}) ([a-z]{1,8})").unwrap();
    let mut part1 = 0;
    let mut part2 = 0;

    for i in 0..contents.lines().count() {
        let line = contents.lines().nth(i).unwrap();
        let mut minimum = Bag { green: 0, red: 0, blue: 0 };
        for round in line.split(";") {
            let mut bag = Bag { green: 13, red: 12, blue: 14 };

            for f in groups.captures_iter(round) {
                let number = f.get(1).unwrap().as_str().parse::<i32>().unwrap();
                match f.get(2).unwrap().as_str() {
                    "green" => {
                        bag.green -= number;
                        minimum.green = minimum.green.max(number);
                    },
                    "red" => {
                        bag.red -= number;
                        minimum.red = minimum.red.max(number);
                    },
                    "blue" => {
                        bag.blue -= number;
                        minimum.blue = minimum.blue.max(number);
                    },
                    _ => {},
                }
            }


            // part 1
            /*
            if bag.green < 0 || bag.red < 0 || bag.blue < 0 {
                part1 += i + 1;
                break;
            }
            */
        }

        print!("{} ", minimum.green * minimum.red * minimum.blue);
        print!(":{} {} {}:", minimum.green , minimum.red, minimum.blue);
        part2 += minimum.red * minimum.green * minimum.blue;
    };
    print!("Day 02: {},{}", 100*101/2 - part1, part2)
}
