fn find_next_value(numbers: Vec<i32>) -> i32 {
    let mut differences: Vec<i32> = Vec::new();

    for number in numbers.iter().zip(numbers.iter().skip(1)) {
        differences.push(number.1 - number.0);
    }

    if differences.len() > 1 && !differences.iter().all(|&x| x == differences[0]) {
        let diff = find_next_value(differences);
        return numbers.last().unwrap() + diff;
    }
    return numbers.last().unwrap() + differences[0];
}

fn find_previous_value(numbers: Vec<i32>) -> i32 {
    let mut differences: Vec<i32> = Vec::new();

    for number in numbers.iter().zip(numbers.iter().skip(1)) {
        let difference = number.1 - number.0;
        differences.push(difference);
    }

    if differences.len() > 1 {
        if !differences.iter().all(|&x| x == differences[0]) {
            let diff = find_previous_value(differences.clone());
            return numbers.first().unwrap() - diff;
        }
    }
    return numbers.first().unwrap() - differences[0];
}

fn main() {
    let contents: &str = include_str!("input.txt");
    let mut part1 = 0;
    let mut part2 = 0;

    contents.lines().for_each(|line| {
        let numbers = line
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        part1 += find_next_value(numbers.clone());
        part2 += find_previous_value(numbers.clone());
    });

    println!("Part 1: {} \nPart 2: {}", part1, part2);
}
