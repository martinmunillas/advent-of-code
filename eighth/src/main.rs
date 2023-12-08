use std::collections::{HashMap, HashSet};

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

    // println!("Result A {}", camel_path(&nodes, &instructions));
    println!("Result B {}", ghost_path(&nodes, &instructions));
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

#[derive(Debug)]
struct GhostPathState<'a> {
    current: &'a str,
    previous: Option<&'a str>,
    loop_start: Option<&'a str>,
    loop_end: Option<&'a str>,
}

#[derive(Debug)]
struct GhostPathResult {
    loop_start: usize,
    loop_end: usize,
}

fn ghost_path(nodes: &HashMap<&str, Node>, instructions: &Vec<char>) -> i64 {
    let initial = nodes.iter().filter(|(id, _)| id[2..=2] == *"A");
    let mut paths = Vec::new();
    let mut seen_at: HashMap<(usize, &str), usize> = HashMap::new();
    for (id, _) in initial {
        paths.push(GhostPathState {
            current: id,
            previous: None,
            loop_start: None,
            loop_end: None,
        });
    }
    let mut i = 0;

    loop {
        let mut idx = i;
        if idx > 0 {
            idx = idx % instructions.len();
        }

        for j in 0..paths.len() {
            let current = paths[j].current;
            if paths[j].loop_end.is_some() {
                continue;
            }
            if seen_at.contains_key(&(j, current)) {
                paths[j].loop_start = Some(current);
                paths[j].loop_end = paths[j].previous;
                continue;
            }
            seen_at.insert((j, current), i);
            paths[j].previous = Some(current);
            if instructions[idx] == 'L' {
                paths[j].current = nodes.get(current).unwrap().a;
            } else {
                paths[j].current = nodes.get(current).unwrap().b;
            }
        }
        i += 1;
        if paths.iter().all(|path| path.loop_end.is_some()) {
            break;
        }
    }

    let path_results: Vec<GhostPathResult> = paths
        .iter()
        .enumerate()
        .map(|(i, path)| {
            let loop_start = seen_at.get(&(i, path.loop_start.unwrap())).unwrap().clone();
            let loop_end = seen_at.get(&(i, path.loop_end.unwrap())).unwrap().clone();

            GhostPathResult {
                loop_start,
                loop_end,
            }
        })
        .collect();

    let multipliers: HashSet<i64> = path_results
        .iter()
        .map(|path| ((path.loop_end + 1) - path.loop_start) as i64)
        .collect();

    println!("{:?}", multipliers);

    let max_offset = path_results
        .iter()
        .map(|path| path.loop_start)
        .max()
        .unwrap() as i64;

    let mut result = 1;
    for multiplier in multipliers {
        result *= multiplier;
    }
    result += max_offset - 1;

    result.pow(2)
}
