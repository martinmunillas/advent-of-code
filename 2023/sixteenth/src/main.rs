use std::{cmp, collections::HashSet};

fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");
    println!(
        "Result A: {}",
        count_energized(input, (0, 0), Direction::Right)
    );
    println!("Result B: {}", find_most_energized(input));
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn explore(
    map: &Vec<Vec<char>>,
    start: (usize, usize),
    direction: Direction,
    visited: &mut HashSet<(usize, usize, Direction)>,
) {
    if visited.contains(&(start.0, start.1, direction)) {
        return;
    }
    visited.insert((start.0, start.1, direction));
    let line_length = map[0].len();
    // print_map(map, start, visited);

    let (x, y) = start;
    let ch = map[y][x];
    match direction {
        Direction::Down => {
            if (ch == '|' || ch == '.') && y < map.len() - 1 {
                explore(map, (x, y + 1), direction, visited);
            } else if ch == '-' {
                explore(map, (x, y), Direction::Left, visited);
                explore(map, (x, y), Direction::Right, visited);
            } else if ch == '\\' && x < line_length - 1 {
                explore(map, (x + 1, y), Direction::Right, visited);
            } else if ch == '/' && x > 0 {
                explore(map, (x - 1, y), Direction::Left, visited);
            }
        }
        Direction::Up => {
            if (ch == '|' || ch == '.') && y > 0 {
                explore(map, (x, y - 1), direction, visited);
            } else if ch == '-' {
                explore(map, (x, y), Direction::Left, visited);
                explore(map, (x, y), Direction::Right, visited);
            } else if ch == '\\' && x > 0 {
                explore(map, (x - 1, y), Direction::Left, visited);
            } else if ch == '/' && x < line_length - 1 {
                explore(map, (x + 1, y), Direction::Right, visited);
            }
        }
        Direction::Left => {
            if (ch == '-' || ch == '.') && x > 0 {
                explore(map, (x - 1, y), direction, visited);
            } else if ch == '|' {
                explore(map, (x, y), Direction::Up, visited);
                explore(map, (x, y), Direction::Down, visited);
            } else if ch == '\\' && y > 0 {
                explore(map, (x, y - 1), Direction::Up, visited);
            } else if ch == '/' && y < map.len() - 1 {
                explore(map, (x, y + 1), Direction::Down, visited);
            }
        }
        Direction::Right => {
            if (ch == '-' || ch == '.') && x < line_length - 1 {
                explore(map, (x + 1, y), direction, visited);
            } else if ch == '|' {
                explore(map, (x, y), Direction::Up, visited);
                explore(map, (x, y), Direction::Down, visited);
            } else if ch == '\\' && y < map.len() - 1 {
                explore(map, (x, y + 1), Direction::Down, visited);
            } else if ch == '/' && y > 0 {
                explore(map, (x, y - 1), Direction::Up, visited);
            }
        }
    }
}

fn count_energized(map: &str, from: (usize, usize), direction: Direction) -> i32 {
    let mut visited = HashSet::new();
    let mut active = HashSet::new();

    explore(
        &map.lines().map(|l| l.chars().collect()).collect(),
        from,
        direction,
        &mut visited,
    );

    for v in visited {
        active.insert((v.0, v.1));
    }

    active.len() as i32
}

fn find_most_energized(map: &str) -> i32 {
    let m: Vec<Vec<char>> = map.lines().map(|l| l.chars().collect()).collect();

    let mut most = 0;

    let line_length = m[0].len();

    for i in 0..m.len() {
        most = cmp::max(most, count_energized(map, (0, i), Direction::Right));
        most = cmp::max(
            most,
            count_energized(map, (line_length - 1, i), Direction::Left),
        );
    }

    for i in 0..line_length {
        most = cmp::max(most, count_energized(map, (i, 0), Direction::Down));
        most = cmp::max(
            most,
            count_energized(map, (i, m.len() - 1), Direction::Left),
        );
    }

    most
}

fn print_map(
    map: &Vec<Vec<char>>,
    current: (usize, usize),
    visited: &HashSet<(usize, usize, Direction)>,
) {
    let (x, y) = current;
    let mut map = map.clone();
    map[y][x] = 'X';
    for (y, line) in map.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if visited.contains(&(x, y, Direction::Up))
                || visited.contains(&(x, y, Direction::Down))
                || visited.contains(&(x, y, Direction::Left))
                || visited.contains(&(x, y, Direction::Right))
            {
                print!("#");
            } else {
                print!("{}", ch);
            }
        }
        println!("");
    }
    println!("");
}

fn run_tests() {
    let example = include_str!("./input/example.txt");

    assert_eq!(count_energized(example, (0, 0), Direction::Right), 46);
    println!("Test 1 passed!");
    assert_eq!(find_most_energized(example), 51);
    println!("Test 2 passed!");

    println!("");
    println!("All tests passed!");
    println!("");
}
