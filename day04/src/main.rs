use std::collections::HashMap;

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    let contents = include_str!("input.txt").to_string();
    let mut card_count: HashMap<usize,usize> = HashMap::new();
    
    for i in 0..contents.lines().count() {
        let line = contents.lines().nth(i).unwrap();

        let groups = line.split_once(":").unwrap().1.split_once("|").unwrap();
        let entries: Vec<i32> = groups.0.split_whitespace().map(|entry| entry.parse::<i32>().unwrap()).collect();
        let mut count:u32 = 0;
        groups.1.split_whitespace().for_each(|word| {
            if entries.contains(&word.parse::<i32>().unwrap()) {
                count += 1;
            }
        });

        card_count.entry(i + 1).or_insert(1);

        if count > 0 {
            part1 += (2 as i32).pow(count - 1);
            for j in 1..count + 1 {
                let index = i + (j as usize) + 1;
                let current_card_count = *card_count.get(&(i + 1)).unwrap_or(&1);

                println!("Card {} inserting {} cards into {}", i + 1, current_card_count, index);
                card_count.entry(index).and_modify(|f| *f +=  current_card_count).or_insert(current_card_count + 1);
            }
        }
    }

    card_count.iter().for_each(|card| {
            part2 += card.1;
            println!("{}: {}", card.0, card.1);
    });

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
