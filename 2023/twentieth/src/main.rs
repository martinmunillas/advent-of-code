use core::panic;
use std::collections::{HashMap, VecDeque};

fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");

    println!("Result A: {}", push_button(collect_nodes(input), 1000));
}

fn push_button(nodes: HashMap<String, Node>, times: i32) -> i32 {
    let mut lows = 0;
    let mut highs = 0;
    let mut state = nodes.clone();
    for _ in 0..times {
        let mut queue = VecDeque::new();
        queue.push_back(("button".to_owned(), false, "broadcaster".to_owned()));
        while queue.len() > 0 {
            let (from, high, name) = queue.pop_front().unwrap();

            if high {
                highs += 1;
            } else {
                lows += 1;
            };

            let current = state.get(&name);
            if current.is_none() {
                continue;
            }
            let current = current.unwrap().clone();
            match current.operator {
                None => {
                    for next in current.next {
                        queue.push_back((name.clone(), high, next))
                    }
                }
                Some('%') => {
                    if !high {
                        for next in current.next.clone() {
                            queue.push_back((name.clone(), !current.flip_flop, next))
                        }
                        state.insert(
                            name.to_string(),
                            Node {
                                operator: current.operator,
                                flip_flop: !current.flip_flop,
                                conjunction: current.conjunction,
                                next: current.next,
                            },
                        );
                    }
                }
                Some('&') => {
                    let mut conjunction = current.conjunction.clone();
                    conjunction.insert(from, high);
                    let mut to_send = false;

                    for (nn, n) in &nodes {
                        if n.next.contains(&name) {
                            match conjunction.get(nn) {
                                None | Some(false) => {
                                    to_send = true;
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    for next in current.next.clone() {
                        queue.push_back((name.clone(), to_send, next))
                    }
                    state.insert(
                        name.to_string(),
                        Node {
                            operator: current.operator,
                            flip_flop: current.flip_flop,
                            conjunction: conjunction,
                            next: current.next.clone(),
                        },
                    );
                }
                _ => panic!("Unknown operator {}", current.operator.unwrap()),
            }
        }
    }
    lows * highs
}

#[derive(Debug, Clone)]
struct Node {
    operator: Option<char>,
    flip_flop: bool,
    conjunction: HashMap<String, bool>,
    next: Vec<String>,
}

fn collect_nodes(input: &str) -> HashMap<String, Node> {
    let mut nodes = HashMap::new();

    for line in input.lines() {
        let chunks = line.split(" -> ").collect::<Vec<&str>>();
        let first_char = chunks[0].chars().nth(0).unwrap();
        let operator = if "%&".contains(first_char) {
            Some(first_char)
        } else {
            None
        };
        let name = if operator.is_none() {
            chunks[0]
        } else {
            &chunks[0][1..chunks[0].len()]
        };
        let next = chunks[1]
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let node = Node {
            operator: operator,
            flip_flop: false,
            conjunction: HashMap::new(),
            next: next,
        };
        nodes.insert(name.to_string(), node);
    }

    nodes
}

fn run_tests() {
    let example1 = include_str!("./input/example1.txt");
    let example2 = include_str!("./input/example2.txt");

    assert_eq!(push_button(collect_nodes(example1), 1), 32);
    println!("Test passed!");
    assert_eq!(push_button(collect_nodes(example1), 1000), 32000000);
    println!("Test passed!");
    assert_eq!(push_button(collect_nodes(example2), 1000), 11687500);
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
