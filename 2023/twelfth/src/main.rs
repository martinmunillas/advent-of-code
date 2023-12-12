fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");
    println!("Result 1: {}", sum_possible_spring_conditions(input));
}

fn sum_possible_spring_conditions(damaged_records: &str) -> i32 {
    let mut sum = 0;
    for damaged_record in damaged_records.lines() {
        let chunks = damaged_record.split(" ").collect::<Vec<&str>>();
        let damaged_record = chunks[0];
        let groups = chunks[1];
        sum += record_arrangements(damaged_record, groups);
    }

    sum
}

fn record_arrangements(damaged_record: &str, groups: &str) -> i32 {
    let mut arrangements = 0;
    let possible = possible_strs(damaged_record);
    for record in possible {
        if str_groups(&record) == groups {
            arrangements += 1;
        }
    }

    arrangements
}

fn possible_strs(damaged_record: &str) -> Vec<String> {
    let mut unknown = Vec::new();
    for (i, c) in damaged_record.chars().enumerate() {
        if c == '?' {
            unknown.push(i);
        }
    }

    let mut possible: Vec<String> = Vec::from(["".to_owned()]);
    for _ in 0..unknown.len() {
        possible = possible
            .iter()
            .flat_map(|s| {
                let mut current = Vec::new();
                for c in ['.', '#'].iter() {
                    let mut new_s = s.clone();
                    new_s.push(*c);
                    current.push(new_s);
                }
                current
            })
            .collect();
    }
    for (i, p) in possible.clone().iter().enumerate() {
        let mut new = damaged_record.chars().collect::<Vec<char>>();
        for (j, c) in p.chars().enumerate() {
            new[unknown[j]] = c;
        }
        possible[i] = new.iter().collect();
    }

    possible
}

fn str_groups(record: &str) -> String {
    let mut groups = Vec::new();
    let mut current = 0;
    for c in record.chars() {
        if c == '#' {
            current += 1;
        } else {
            if current > 0 {
                groups.push(current.to_string());
                current = 0;
            }
        }
    }
    if current > 0 {
        groups.push(current.to_string());
    }

    groups.join(",")
}

fn run_tests() {
    let example = include_str!("./input/example.txt");

    assert_eq!(sum_possible_spring_conditions(example), 21);
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
