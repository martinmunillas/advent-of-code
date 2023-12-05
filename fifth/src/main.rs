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
    println!("Result A: {}", lowest_location_from_seeds(&seeds, &maps));
    println!(
        "Result B: {}",
        lowest_location_from_locations(&seeds, &maps)
    );
}

fn lowest_location_from_locations(seeds: &Vec<i64>, maps: &Vec<Vec<Vec<i64>>>) -> i64 {
    let mut min_location = 0;
    let mut seed_found = false;
    while !seed_found {
        seed_found = location_has_seeds(min_location, seeds, &maps);
        if !seed_found {
            min_location += 1;
        } else {
            break;
        }
    }

    min_location
}

fn location_has_seeds(location: i64, seeds: &Vec<i64>, maps: &Vec<Vec<Vec<i64>>>) -> bool {
    let mut value = location;
    for map in maps.iter().rev() {
        for section in map {
            if value < section[0] + section[2] && value >= section[0] {
                value = value - section[0] + section[1];
                break;
            }
        }
    }
    for i in (0..seeds.len()).step_by(2) {
        if value < seeds[i + 1] + seeds[i] && value >= seeds[i] {
            return true;
        }
    }
    false
}

fn lowest_location_from_seeds(seeds: &Vec<i64>, maps: &Vec<Vec<Vec<i64>>>) -> i64 {
    let mut min_location = std::i64::MAX;
    for seed in seeds {
        let seed_value = seed_location(*seed, maps);
        if seed_value < min_location {
            min_location = seed_value;
        }
    }
    min_location
}

fn seed_location(seed: i64, maps: &Vec<Vec<Vec<i64>>>) -> i64 {
    let mut seed_value = seed;
    for map in maps {
        for section in map {
            if seed_value < section[1] + section[2] && seed_value >= section[1] {
                seed_value = seed_value + section[0] - section[1];
                break;
            }
        }
    }
    seed_value
}
