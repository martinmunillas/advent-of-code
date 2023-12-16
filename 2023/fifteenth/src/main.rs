use std::collections::HashMap;

fn main() {
    run_tests();
    let input = include_str!("./input/input.txt")
        .split(",")
        .collect::<Vec<&str>>();

    println!("Result A: {}", sum_hashes(&input));
    println!("Result B: {}", count_focusing_power(&operate_boxes(&input)));
}

fn hash(input: &str) -> i32 {
    let mut result: i32 = 0;

    for c in input.chars() {
        result += c as i32;
        result *= 17;
        result %= 256;
    }

    result
}

fn operate_boxes(instructions: &Vec<&str>) -> HashMap<i32, Vec<(String, i32)>> {
    let mut boxes: HashMap<i32, Vec<(String, i32)>> = HashMap::new();

    for instruction in instructions {
        let mut label = "".to_string();
        let mut operation = '_';
        let mut length = "".to_string();
        for c in instruction.chars() {
            if operation == '_' {
                if c == '=' || c == '-' {
                    operation = c;
                } else {
                    label += &c.to_string();
                }
            } else {
                length += &c.to_string();
            }
        }
        let box_number = hash(&label);

        match boxes.get(&box_number) {
            Some(b) => {
                let mut new_b = b.clone();
                let mut exists_at = -1;
                for (i, (l, _)) in b.iter().enumerate() {
                    if l == &label {
                        exists_at = i as i32;
                        break;
                    }
                }
                if operation == '=' {
                    if exists_at != -1 {
                        new_b[exists_at as usize].1 = length.parse().unwrap();
                    } else {
                        new_b.push((label, length.parse().unwrap()));
                    }
                } else {
                    if exists_at != -1 {
                        new_b.remove(exists_at as usize);
                    }
                }
                boxes.insert(box_number, new_b);
            }
            None => {
                if operation == '=' {
                    boxes.insert(box_number, vec![(label, length.parse().unwrap())]);
                }
            }
        }
    }

    boxes
}

fn count_focusing_power(boxes: &HashMap<i32, Vec<(String, i32)>>) -> i32 {
    let mut result: i32 = 0;

    for (n, lens) in boxes {
        for (i, l) in lens.iter().enumerate() {
            result += (n + 1) * (i as i32 + 1) * l.1;
        }
    }

    result
}

fn sum_hashes(steps: &Vec<&str>) -> i32 {
    let mut result: i32 = 0;

    for step in steps {
        result += hash(step);
    }

    result
}

fn run_tests() {
    let example = include_str!("./input/example.txt")
        .split(",")
        .collect::<Vec<&str>>();
    assert_eq!(sum_hashes(&example), 1320);
    println!("Test 1 passed!");
    assert_eq!(count_focusing_power(&operate_boxes(&example)), 145);
    println!("Test 2 passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
