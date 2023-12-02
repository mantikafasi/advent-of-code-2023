use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Exploded for some reason.");
    const WORDS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut sum: usize = 0;

    struct Word {
        number: usize,
        index: usize,
    }

    contents.lines().for_each(| line | {
        let mut indexes: Vec<Word> = Vec::new();

        let chars: Vec<char> = line.chars().collect();
        for i in 0..line.len() {
            if chars[i].is_ascii_digit() {
                indexes.push(Word { number: chars[i].to_digit(10).unwrap() as usize, index: i });
            }
        }
        
        // comment if part 1
        for i in 0..WORDS.len() {
            let j = line.find(WORDS[i]);
            if j != None {
                indexes.push(Word { number: i + 1, index: j.unwrap() });
            }

            let k = line.rfind(WORDS[i]);
            if k != None && k != j {
                indexes.push(Word { number: i + 1, index: k.unwrap() });
            }
        }

        indexes.sort_by(|a, b| a.index.cmp(&b.index));
        sum += indexes[0].number * 10 + indexes.last().unwrap().number;
    });

    println!("Day 01: {}", sum)
}
