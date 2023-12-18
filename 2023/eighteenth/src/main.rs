fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");

    println!("Result A: {}", find_area(&collect_instructions(input)));
    println!("Result B: {}", find_area(&collect_hex_instructions(input)));
}

struct Instruction {
    operation: char,
    value: i64,
}

fn find_area(instructions: &Vec<Instruction>) -> i64 {
    let mut vertices = vec![(0, 0)];
    let mut perimeter = 0;
    for instruction in instructions {
        let (previous_x, previous_y) = vertices[vertices.len() - 1];
        perimeter += instruction.value;
        match instruction.operation {
            'U' => vertices.push((previous_x, previous_y - instruction.value)),
            'D' => vertices.push((previous_x, previous_y + instruction.value)),
            'L' => vertices.push((previous_x - instruction.value, previous_y)),
            'R' => vertices.push((previous_x + instruction.value, previous_y)),
            _ => panic!("Unknown operation"),
        }
    }

    let mut area = 0;
    for i in 0..vertices.len() - 1 {
        area += (vertices[i].0 * vertices[i + 1].1) - (vertices[i + 1].0 * vertices[i].1);
    }

    area.abs() / 2 + perimeter / 2 + 1
}

fn collect_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            let operation = parts[0].chars().nth(0).unwrap();
            let value = parts[1].parse::<i64>().unwrap();

            Instruction { operation, value }
        })
        .collect()
}

fn collect_hex_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let hex = line.split(" ").collect::<Vec<&str>>()[2];
            let operation = hex[hex.len() - 2..hex.len() - 1].chars().nth(0).unwrap();
            let value = i64::from_str_radix(&hex[2..hex.len() - 2], 16).unwrap();

            Instruction {
                operation: match operation {
                    '0' => 'R',
                    '1' => 'D',
                    '2' => 'L',
                    '3' => 'U',
                    _ => panic!("Unknown operation"),
                },
                value,
            }
        })
        .collect()
}

fn run_tests() {
    let example = include_str!("./input/example.txt");

    assert_eq!(find_area(&collect_instructions(example)), 62);
    println!("Test passed!");
    assert_eq!(find_area(&collect_hex_instructions(example)), 952408144115);
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
