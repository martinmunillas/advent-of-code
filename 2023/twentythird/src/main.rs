use std::{cmp::max, collections::HashSet};

fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");

    println!("Result A: {}", find_longest_path(&collect_map(input)));
}

fn find_longest_path(map: &Vec<Vec<char>>) -> i32 {
    let mut start = (0, 0);
    for (x, c) in map[0].iter().enumerate() {
        if *c == '.' {
            start = (x as i32, 0);
            break;
        }
    }
    let mut seen = HashSet::new();

    explore(map, start, &mut seen) - 1
}

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn explore(map: &Vec<Vec<char>>, coord: (i32, i32), seen: &mut HashSet<(i32, i32)>) -> i32 {
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
        return 1;
    }
    seen.insert(coord);
    let mut result = 0;
    match map[coord.1 as usize][coord.0 as usize] {
        '>' => {
            result = max(
                result,
                explore(map, (coord.0 + 1, coord.1), &mut seen.clone()) + 1,
            )
        }
        'v' => {
            result = max(
                result,
                explore(map, (coord.0, coord.1 + 1), &mut seen.clone()) + 1,
            )
        }
        '.' => {
            for (x, y) in DIRECTIONS {
                result = max(
                    result,
                    explore(map, (x + coord.0, y + coord.1), &mut seen.clone()) + 1,
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

    assert_eq!(find_longest_path(&collect_map(example)), 94);
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
