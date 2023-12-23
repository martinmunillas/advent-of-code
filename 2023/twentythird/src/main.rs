use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");

    println!(
        "Result A: {}",
        find_longest_path_with_slopes(&collect_map(input))
    );
    println!("Result B: {}", find_longest_path(&collect_map(input)));
}

type Graph = HashMap<Coord, HashSet<(Coord, i32)>>;
type Coord = (usize, usize);

fn find_longest_path(map: &Vec<Vec<char>>) -> i32 {
    let mut start = (0, 0);
    for (x, c) in map[0].iter().enumerate() {
        if *c == '.' {
            start = (x, 0);
            break;
        }
    }

    let mut end = (0, 0);
    for (x, c) in map[map.len() - 1].iter().enumerate() {
        if *c == '.' {
            end = (x, map.len() - 1);
            break;
        }
    }

    let mut nodes = HashSet::new();
    nodes.insert(start);
    nodes.insert(end);

    for (r, row) in map.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == '#' {
                continue;
            }
            let mut neighbors = 0;
            if r > 0 && map[r - 1][c] != '#' {
                neighbors += 1;
            }
            if r < map.len() - 1 && map[r + 1][c] != '#' {
                neighbors += 1;
            }
            if c > 0 && map[r][c - 1] != '#' {
                neighbors += 1;
            }
            if c < map[r].len() - 1 && map[r][c + 1] != '#' {
                neighbors += 1;
            }

            if neighbors > 2 {
                nodes.insert((c, r));
            }
        }
    }

    let mut graph = Graph::new();
    for node in &nodes {
        graph.insert(*node, HashSet::new());
        let mut seen = HashSet::new();
        let mut stack = Vec::new();
        stack.push((*node, 0));
        seen.insert(*node);
        while !stack.is_empty() {
            let (coord, dist) = stack.pop().unwrap();
            if dist != 0 && nodes.contains(&coord) {
                let mut neighbors = graph.get(&node).unwrap().clone();
                neighbors.insert((coord, dist));
                graph.insert(*node, neighbors);
                continue;
            }

            for (x, y) in DIRECTIONS {
                let new = (x + coord.0 as i32, y + coord.1 as i32);
                if new.0 < 0
                    || new.1 < 0
                    || new.0 >= map[0].len() as i32
                    || new.1 >= map.len() as i32
                {
                    continue;
                }
                let new = (new.0 as usize, new.1 as usize);
                if !seen.contains(&new) && map[new.1][new.0] != '#' {
                    stack.push((new, dist + 1));
                    seen.insert(new);
                }
            }
        }
    }

    explore(&graph, start, end, &mut HashSet::new())
}

fn _print_graph(graph: &Graph) {
    println!("Graph:");
    for (coord, neighbors) in graph {
        println!("{:?} -> {:?}", coord, neighbors);
    }
}

fn explore(graph: &Graph, coord: Coord, end: Coord, seen: &mut HashSet<Coord>) -> i32 {
    if coord == end {
        return 0;
    }

    seen.insert(coord);
    let mut result = -i32::MAX;
    for (neighbor, dist) in &graph[&coord] {
        if seen.contains(neighbor) {
            continue;
        }
        result = max(
            result,
            explore(graph, *neighbor, end, &mut seen.clone()) + dist,
        );
    }
    result
}

fn find_longest_path_with_slopes(map: &Vec<Vec<char>>) -> i32 {
    let mut start = (0, 0);
    for (x, c) in map[0].iter().enumerate() {
        if *c == '.' {
            start = (x as i32, 0);
            break;
        }
    }
    explore_with_slopes(map, start, &mut HashSet::new())
}

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn explore_with_slopes(
    map: &Vec<Vec<char>>,
    coord: (i32, i32),
    seen: &mut HashSet<(i32, i32)>,
) -> i32 {
    if seen.contains(&coord)
        || coord.0 < 0
        || coord.1 < 0
        || coord.0 >= map[0].len() as i32
        || coord.1 >= map.len() as i32
        || map[coord.1 as usize][coord.0 as usize] == '#'
    {
        return 0;
    }
    if coord.1 as usize == map.len() - 1 {
        return seen.len() as i32;
    }
    seen.insert(coord);
    let mut result = 0;
    match map[coord.1 as usize][coord.0 as usize] {
        '>' => {
            result = max(
                result,
                explore_with_slopes(map, (coord.0 + 1, coord.1), &mut seen.clone()),
            )
        }
        'v' => {
            result = max(
                result,
                explore_with_slopes(map, (coord.0, coord.1 + 1), &mut seen.clone()),
            )
        }
        '.' => {
            for (x, y) in DIRECTIONS {
                result = max(
                    result,
                    explore_with_slopes(map, (x + coord.0, y + coord.1), &mut seen.clone()),
                )
            }
        }
        _ => panic!("Unknown map symbol"),
    }

    result
}

fn _print_map(map: &Vec<Vec<char>>, visited: &HashSet<(i32, i32)>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if visited.contains(&(x as i32, y as i32)) {
                print!("O");
            } else if map[y][x] == '#' {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

fn collect_map(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
fn run_tests() {
    let example = include_str!("./input/example.txt");

    assert_eq!(find_longest_path_with_slopes(&collect_map(example)), 94);
    println!("Test passed!");
    assert_eq!(find_longest_path(&collect_map(example)), 154);
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
