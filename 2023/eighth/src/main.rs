use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node<'a> {
    id: &'a str,
    a: &'a str,
    b: &'a str,
}

fn main() {
    let input = include_str!("./input.txt");
    let mut nodes = HashMap::new();
    let instructions: Vec<char> = input.lines().nth(0).unwrap().chars().collect();
    for line in input.lines().skip(2) {
        let id = &line[0..3];
        nodes.insert(
            id,
            Node {
                id,
                a: &line[7..10],
                b: &line[12..15],
            },
        );
    }

    println!("Result A {}", camel_path(&nodes, &instructions));
    println!("Result B {}", ghost_paths(&nodes, &instructions));
}

fn camel_path(nodes: &HashMap<&str, Node>, instructions: &Vec<char>) -> i32 {
    let mut i = 0;
    let mut current = "AAA";
    loop {
        let mut idx = i;
        if idx > 0 {
            idx = idx % instructions.len();
        }
        if instructions[idx] == 'L' {
            current = nodes.get(current).unwrap().a;
        } else {
            current = nodes.get(current).unwrap().b;
        }
        i += 1;
        if current == "ZZZ" {
            break;
        }
    }

    i as i32
}

fn ghost_paths(nodes: &HashMap<&str, Node>, instructions: &Vec<char>) -> i64 {
    let initial = nodes.iter().filter(|(id, _)| id[2..=2] == *"A");
    let lengths: Vec<i64> = initial
        .map(|(id, _)| ghost_path(nodes, instructions, id) as i64)
        .collect();

    let lcm = least_common_multiple(&lengths);

    println!("{:?}", lengths);

    lcm
}

fn ghost_path(nodes: &HashMap<&str, Node>, instructions: &Vec<char>, initial: &str) -> i32 {
    let mut i = 0;
    let mut current = initial;
    loop {
        let mut idx = i;
        if idx > 0 {
            idx = idx % instructions.len();
        }
        if instructions[idx] == 'L' {
            current = nodes.get(current).unwrap().a;
        } else {
            current = nodes.get(current).unwrap().b;
        }
        i += 1;
        if current[2..=2] == *"Z" {
            break;
        }
    }

    i as i32
}

fn greatest_common_divisor(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        greatest_common_divisor(b, a % b)
    }
}

fn least_common_multiple(nums: &Vec<i64>) -> i64 {
    nums.iter()
        .fold(1, |lcm, &num| lcm * num / greatest_common_divisor(lcm, num))
}
