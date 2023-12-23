use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");

    println!("Result A: {}", find_longest_path(&collect_map(input), true));
    println!(
        "Result B: {}",
        find_longest_path(&collect_map(input), false)
    );
}

fn find_longest_path(map: &Vec<Vec<char>>, with_slopes: bool) -> i32 {
    let mut start = (0, 0);
    for (x, c) in map[0].iter().enumerate() {
        if *c == '.' {
            start = (x as i32, 0);
            break;
        }
    }
    if with_slopes {
        explore(map, start, &mut HashSet::new())
    } else {
        explore_without_slopes(map, start, &mut HashSet::new(), &mut HashMap::new())
    }
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
        return seen.len() as i32;
    }
    seen.insert(coord);
    let mut result = 0;
    match map[coord.1 as usize][coord.0 as usize] {
        '>' => {
            result = max(
                result,
                explore(map, (coord.0 + 1, coord.1), &mut seen.clone()),
            )
        }
        'v' => {
            result = max(
                result,
                explore(map, (coord.0, coord.1 + 1), &mut seen.clone()),
            )
        }
        '.' => {
            for (x, y) in DIRECTIONS {
                result = max(
                    result,
                    explore(map, (x + coord.0, y + coord.1), &mut seen.clone()),
                )
            }
        }
        _ => panic!("Unknown map symbol"),
    }

    result
}

fn explore_without_slopes(
    map: &Vec<Vec<char>>,
    coord: (i32, i32),
    seen: &mut HashSet<(i32, i32)>,
    global: &mut HashMap<((i32, i32), (i32, i32)), i32>,
) -> i32 {
    let last = match seen.iter().last() {
        Some(x) => x,
        None => &(-1, -1),
    };
    let gkey = (coord, *last);
    let g = global.contains_key(&gkey) && global[&gkey] > seen.len() as i32;
    if g {
        println!("optimization false {:?} {:?}", gkey.0, gkey.1)
    }
    if seen.contains(&coord)
        || coord.0 < 0
        || coord.1 < 0
        || coord.0 >= map[0].len() as i32
        || coord.1 >= map.len() as i32
        || map[coord.1 as usize][coord.0 as usize] == '#'
        || g
    {
        return 0;
    }
    if coord.1 as usize == map.len() - 1 {
        println!("{}", seen.len());
        return seen.len() as i32;
    }
    global.insert(gkey, seen.len() as i32);
    seen.insert(coord);
    let mut result = 0;
    for (x, y) in DIRECTIONS {
        result = max(
            result,
            explore_without_slopes(map, (x + coord.0, y + coord.1), &mut seen.clone(), global),
        )
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

    assert_eq!(find_longest_path(&collect_map(example), true), 94);
    println!("Test passed!");
    assert_eq!(find_longest_path(&collect_map(example), false), 154);
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
