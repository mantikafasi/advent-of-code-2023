struct Step {
    source: i64,
    destination: i64,
    range: i64,
}

fn main() {
    // part 2
    let contents = include_str!("input.txt");
    
    let mut all_steps: Vec<Vec<Step>> = Vec::new();
    let mut seeds: Vec<i64> = Vec::new();

    contents.split("\n\n").for_each(|group| {
        if let Some(groups) =  group.split_once("\n")  {

            let mut steps: Vec<Step> = Vec::new();
            println!("start");
            groups.1.split("\n").for_each(|group| {
                println!("{}",group);

                let values: Vec<&str> = group.split(" ").collect();
                let source = values[1].parse::<i64>().unwrap();
                let destination = values[0].parse::<i64>().unwrap();
                let range = values[2].parse::<i64>().unwrap();
                
                steps.push(Step {
                    source: source,
                    destination: destination,
                    range: range,
                });
            });

            all_steps.push(steps);
        } else {
            group.split_once(":").unwrap().1.trim().split(" ").for_each(|seed| {
                seeds.push(seed.parse::<i64>().unwrap());
            });
        }
        
    });
    let mut min = i64::MAX;
    
    let len = seeds.len() / 2;

    for v in seeds.chunks(2).enumerate() {
        let i = v.1;

        let mut k = 0;

        for j in i[0] .. i[0] + i[1] {
            if k > 1000000 {
                k = 0;
                // println!("Progress: {}/{} ({:.2}%)", j - i[0],(i[1]) ,((j - i[0]) as f64 / (i[1]) as f64) * ((v.0 + 1) as f64 / len as f64) * 100.00);
            }
            k += 1;
            
            let mut number = j;

            for steps in all_steps.iter() {
                for step in steps {
                    if number >= step.source && number <= step.range + step.source {
                        number = number - step.source + step.destination;
                        break;
                    }
                }
            }
            if number < min {
                min = number;
            }
        }
    }

    println!("Min: {}", min);
}
