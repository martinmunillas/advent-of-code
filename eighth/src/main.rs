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

    println!("Result {}", i)
}
