use std::{char, collections::HashMap, ops::Index};

struct Round {
    original_cards: Vec<char>,
    cards: Vec<char>,
    card_levels: Vec<(char, usize)>,
    level: usize,
    bid: usize,
}

fn main() {
    let contents = include_str!("input.txt");
    let strength = "AKQT98765432J";
    let mut part2 = 0;

    let rules: [[usize; 5]; 7] = [
        [5, 0, 0, 0, 0],
        [4, 1, 0, 0, 0],
        [3, 2, 0, 0, 0],
        [3, 1, 1, 0, 0],
        [2, 2, 1, 0, 0],
        [2, 1, 1, 1, 0],
        [1, 1, 1, 1, 1],
    ];

    let mut rounds: Vec<Round> = Vec::new();

    contents.lines().for_each(|line| {
        let card = line.split_once(" ").unwrap();
        let mut characters: Vec<(char, usize)> = Vec::new();

        card.0.chars().for_each(|character| {
            if characters.iter().any(|c| c.0 == character) {
                return;
            }
            characters.push((character, card.0.matches(character).count()));
        });

        characters.sort_by(|a, b| {
            if a.1 == b.1 {
                strength
                    .find(b.0)
                    .unwrap()
                    .cmp(&strength.find(a.0).unwrap())
            } else {
                b.1.cmp(&a.1)
            }
        });
        let binding = card.0.replace("J", characters[0].0.to_string().as_str());
        let j = characters.iter().position(|c| c.0 == 'J');
        if !j.is_none() {
            let count = characters.remove(j.unwrap()).1;

            if characters.len() == 0 {
                characters.push(('J', 0));
            }
            characters[0].1 += count;
        }

        rounds.push(Round {
            original_cards: card.0.chars().collect(),
            cards: binding.chars().collect(),
            card_levels: characters,
            level: 0,
            bid: card.1.parse::<usize>().unwrap(),
        })
    });

    for round in rounds.iter_mut() {
        let group: &mut Vec<(char, usize)> = &mut round.card_levels;
        println!("Group: {:?}", group);
        'type_loop: for game_type1 in rules.iter().enumerate() {
            let game_type = game_type1.1;
            for character in group.iter().enumerate() {
                let index = character.0;
                if game_type[index] != character.1 .1 {
                    continue 'type_loop;
                }
            }
            round.level = game_type1.0;
            break;
        }
    }

    rounds.sort_by(|a, b| {
        if a.level == b.level {
            for card in a.original_cards.iter().enumerate() {
                let index = card.0;
                let char1 = *card.1;
                let char2 = b.original_cards[index];
                if strength.find(char1).unwrap() != strength.find(char2).unwrap() {
                    println!(
                        "{} vs {}",
                        strength.find(char1).unwrap(),
                        strength.find(char2).unwrap()
                    );
                    return strength
                        .find(char1)
                        .unwrap()
                        .cmp(&strength.find(char2).unwrap());
                }
            }
        } else {
            return a.level.cmp(&b.level);
        };

        std::cmp::Ordering::Equal // wont happen
    });

    rounds.reverse();

    for iter in rounds.iter().enumerate() {
        part2 += iter.1.bid * (iter.0 + 1);
    }

    println!("Part 2: {}", part2)
}
