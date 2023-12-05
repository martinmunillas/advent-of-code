fn main() {
    let input = include_str!("input.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let mut maps: Vec<Vec<Vec<i64>>> = Vec::new();
    let mut seeds: Vec<i64> = Vec::new();
    let mut current: Vec<Vec<i64>> = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if lines[i] == "" {
            if i != 1 {
                maps.push(current);
                current = Vec::new();
            }
            i += 2;
            continue;
        }
        if i == 0 {
            seeds = lines[i].split(":").collect::<Vec<&str>>()[1]
                .trim()
                .split(" ")
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            i += 1;
            continue;
        }
        current.push(
            lines[i]
                .split(" ")
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
        );
        i += 1
    }
    maps.push(current);

    let mut min_location = std::i64::MAX;
    for seed in seeds {
        let mut seed_value = seed;
        for map in &maps {
            for section in map {
                if seed_value < section[1] + section[2] && seed_value >= section[1] {
                    seed_value = seed_value + section[0] - section[1];
                    break;
                }
            }
        }
        if seed_value < min_location {
            min_location = seed_value;
        }
    }
    println!("Result: {}", min_location);
}
