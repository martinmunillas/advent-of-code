use core::panic;
use std::collections::HashMap;

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
        let (a, b, new_state) = exec(&mut state.clone(), "broadcaster", true);
        lows += a;
        highs += b;
        state = new_state;
    }
    lows * highs
}

fn exec(
    state: &mut HashMap<String, Node>,
    name: &str,
    low: bool,
) -> (i32, i32, HashMap<String, Node>) {
    println!("{} {}", name, low);
    let current = state[name].clone();
    let mut lows = 0;
    let mut highs = 0;
    match current.operator {
        None => {
            for next in current.next {
                let (a, b, _) = exec(state, &next, true);
                lows += a;
                highs += b;
            }
        }
        Some('%') => {
            if low {
                for next in current.next.clone() {
                    let (a, b, _) = exec(state, &next, !current.state);
                    lows += a;
                    highs += b;
                }
                state.insert(
                    name.to_string(),
                    Node {
                        operator: current.operator,
                        state: !current.state,
                        next: current.next,
                    },
                );
            }
        }
        Some('&') => {
            for next in current.next.clone() {
                let (a, b, _) = exec(state, &next, low);
                lows += a;
                highs += b;
            }
            state.insert(
                name.to_string(),
                Node {
                    operator: current.operator,
                    state: low,
                    next: current.next.clone(),
                },
            );
        }
        _ => panic!("Unknown operator {}", current.operator.unwrap()),
    }

    (lows, highs, state.clone())
}

#[derive(Debug, Clone)]
struct Node {
    operator: Option<char>,
    state: bool,
    next: Vec<String>,
}

fn collect_nodes(input: &str) -> HashMap<String, Node> {
    let mut nodes = HashMap::new();

    for line in input.lines() {
        let chunks = line.split(" -> ").collect::<Vec<&str>>();
        let mut operator = None;
        let name = if chunks[0] == "broadcaster" {
            "broadcaster"
        } else {
            operator = Some(chunks[0].chars().nth(0).unwrap());
            &chunks[0][1..chunks[0].len()]
        };
        let next = chunks[1]
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let node = Node {
            operator: operator,
            state: false,
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
