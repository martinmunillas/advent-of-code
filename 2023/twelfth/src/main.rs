use std::collections::HashMap;

fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");
    println!("Result A: {}", sum_possible_spring_conditions(input, 1));
    println!("Result B: {}", sum_possible_spring_conditions(input, 5));
}

fn sum_possible_spring_conditions(damaged_records: &str, folding_times: usize) -> i32 {
    let mut sum = 0;
    for damaged_record in damaged_records.lines() {
        let chunks = damaged_record.split(" ").collect::<Vec<&str>>();
        let damaged_record = (vec![chunks[0]]).repeat(folding_times).join("?");
        let groups = (vec![chunks[1]])
            .repeat(folding_times)
            .join(",")
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        sum += record_arrangements(damaged_record, &groups, &mut HashMap::new());
    }

    sum
}

fn record_arrangements(
    damaged_record: String,
    groups: &Vec<usize>,
    memo: &mut HashMap<(String, Vec<usize>), i32>,
) -> i32 {
    println!("{} {:?}", damaged_record, groups);
    // let key = (damaged_record.to_owned(), groups.clone());
    // if memo.contains_key(&key) {
    //     return *memo.get(&key).unwrap();
    // }
    if damaged_record == "" {
        return match groups.len() {
            0 => 1,
            _ => 0,
        };
    }
    if groups.len() == 0 {
        if damaged_record.contains("#") {
            return 0;
        }
        return 1;
    }
    if groups[0] > damaged_record.len() {
        return 0;
    }

    let mut result = 0;
    if damaged_record.starts_with(".") || damaged_record.starts_with("?") {
        result += record_arrangements(
            damaged_record[1..damaged_record.len()].to_owned(),
            groups,
            memo,
        );
    }

    if damaged_record.starts_with("#") || damaged_record.starts_with("?") {
        for i in 1..groups[0] {
            if damaged_record[i..i].to_owned() == "." {
                return 0;
            }
        }
        result += record_arrangements(
            damaged_record[groups[0]..damaged_record.len()].to_owned(),
            &groups[1..groups.len()].to_vec(),
            memo,
        )
    }

    // memo.insert(key, result);

    result
}

fn run_tests() {
    let example = include_str!("./input/example.txt");

    assert_eq!(sum_possible_spring_conditions(example, 1), 21);
    println!("Test passed!");
    assert_eq!(sum_possible_spring_conditions(example, 5), 525152);
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
