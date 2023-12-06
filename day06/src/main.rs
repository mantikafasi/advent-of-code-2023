fn main() {
    let mut times: Vec<i64> = Vec::new();
    let mut distances: Vec<i64> = Vec::new();

    /* part 1


    //let contents = include_str!("input.txt");
    //let lines: Vec<&str> = contents.lines().collect();


    lines[0].split_once("  ").unwrap().1.split("  ").for_each(|m|{
        if m.trim() != "" {
            times.push(m.trim().parse::<i32>().unwrap());
        }
    });

    lines[1].split_once("  ").unwrap().1.split("  ").for_each(|m|{
        if m.trim() != "" {
            distances.push(m.trim().parse::<i32>().unwrap());
        }
    });
    */

    // part 2
    // too lazy to parse
    times.push(49787980);
    distances.push(298118510661181);

    let acceleartion = 1;
    //let mut result = 1;
    let mut total_outcome_count = 0;

    for v in times.iter().enumerate() {
        let time = v.1;
        let i = v.0;
        // let mut possible_outcome_count = 0;

        for t in 0..*time {
            let distance = (time - t) * acceleartion * t;
            if distance > distances[i] {
                // possible_outcome_count += 1;
                total_outcome_count += 1;
            }
        }
        // result *= possible_outcome_count;
    }

    println!("{:?}", total_outcome_count);
}
