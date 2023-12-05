use std::collections::HashMap;

fn main() {
    let contents = include_str!("input.txt");
    let mut entries: HashMap<&str, Vec<i64>> = HashMap::new();

    contents.split("\n\n").for_each(|group| {
        if let Some(groups) =  group.split_once("\n")  {
            let split = groups.0.split_once(" ").unwrap().0.split_once("-to-").unwrap();
            let mut entry: Vec<i64> = Vec::new();
            let numbers = entries.get_mut(split.0).unwrap();

            groups.1.split("\n").for_each(|group| {
                let values: Vec<&str> = group.split(" ").collect();
                let source = values[1].parse::<i64>().unwrap();
                let destination = values[0].parse::<i64>().unwrap();
                let range = values[2].parse::<i64>().unwrap();
                
                for v in numbers.clone() {
                    if v >= source && v <= range + source {
                        println!("{} - {} + {} = {}", v, source, destination, v - source + destination);
                        entry.push(v - source + destination);
                        numbers.iter().position(|&x| x == v).map(|e| numbers.remove(e));
                    }
                }
            });


            for i in numbers {
                if !(*i >= source && *i <= range + source) {
                    println!("{}", i);
                    entry.push(*i);
                }
            }

            println!("Inserting {} into {}", entry.len(), split.1);
            entries.insert(split.1, entry);
            
        } else {
            let mut entry: Vec<i64> = Vec::new();
            println!("{}", group);

            group.split_once(":").unwrap().1.trim().split(" ").for_each(|seed| {
                entry.push(seed.parse::<i64>().unwrap());
            });

            entries.insert("seed", entry);
        }
    });

    let mut min = i64::MAX;
    entries.get("location").unwrap().iter().for_each(|i| {
        //println!("{}", i)
        if *i < min {
            min = *i;
        }
    });
    println!("Min: {}", min);
}
