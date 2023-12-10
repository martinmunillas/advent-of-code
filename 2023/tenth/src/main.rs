use std::collections::HashMap;

fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");

    let maze = collect_maze(input);

    println!("Result 1: {}", furthest_from_start(maze_loop(&maze)));
    println!("Result 2: {}", tiles_inside(&maze, &maze_loop(&maze)));
}

const LEFT_CHARS: &[char] = &['-', 'F', 'L'];
const RIGHT_CHARS: &[char] = &['-', '7', 'J'];
const UP_CHARS: &[char] = &['|', 'F', '7'];
const DOWN_CHARS: &[char] = &['|', 'L', 'J'];

fn is_continuing_same_line(x: usize, line: &Vec<char>) -> bool {
    if x == 0 {
        return false;
    }

    let mut previous = x - 1;
    while line[previous] == '-' {
        if x == 0 {
            return false;
        }
        previous -= 1;
    }

    match line[x] {
        'J' => line[previous] == 'F',
        '7' => line[previous] == 'L',
        _ => false,
    }
}

fn tiles_inside(maze: &Vec<Vec<char>>, maze_loop: &Vec<(i32, i32)>) -> i32 {
    let mut bounds = HashMap::new();
    for bound in maze_loop {
        bounds.insert(*bound, true);
    }
    let mut count = 0;

    for y in 0..maze.len() {
        let mut bounds_found = 0;
        for x in 0..maze[0].len() {
            let x32 = x as i32;
            let y32 = y as i32;
            let char = maze[y][x];
            if bounds.contains_key(&(x32, y32)) {
                if char != '-'
                    && match x {
                        0 => true,
                        _ => !is_continuing_same_line(x, &maze[y]),
                    }
                {
                    bounds_found += 1;
                }
            } else if bounds_found > 0 && bounds_found % 2 == 1 {
                count += 1;
            }
        }
    }

    count
}

fn maze_loop(maze: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut history = Vec::new();
    let mut start = (0, 0);
    'outer: for (y, line) in maze.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            let x32 = x as i32;
            let y32 = y as i32;
            if c == 'S' {
                history.push((x32, y32));
                if x > 0 && LEFT_CHARS.contains(&maze[y][x - 1]) {
                    start = (x32 - 1, y32)
                } else if x < maze[0].len() - 1 && RIGHT_CHARS.contains(&maze[y][x + 1]) {
                    start = (x32 + 1, y32)
                } else if y > 0 && UP_CHARS.contains(&maze[y - 1][x]) {
                    start = (x32, y32 - 1)
                } else if y < maze.len() - 1 && DOWN_CHARS.contains(&maze[y + 1][x]) {
                    start = (x32, y32 + 1)
                } else {
                    panic!("Invalid start, no path to follow")
                }
                break 'outer;
            }
        }
    }

    solve(maze, start, &mut history);
    return history.clone();
}

fn furthest_from_start(maze_loop: Vec<(i32, i32)>) -> i32 {
    return (maze_loop.len() as f32 / 2.0).ceil() as i32;
}

fn solve(maze: &Vec<Vec<char>>, start_point: (i32, i32), history: &mut Vec<(i32, i32)>) {
    let mut xy = start_point;

    loop {
        let (x, y) = xy;
        let current = maze[y as usize][x as usize];
        let (previous_x, previous_y) = *history.last().unwrap_or(&(0, 0));
        let (new_x, new_y) = match current {
            '-' => (x * 2 - previous_x, y),
            '|' => (x, y * 2 - previous_y),
            'L' => {
                if previous_x == x {
                    (x + 1, y)
                } else {
                    (x, y - 1)
                }
            }
            'J' => {
                if previous_x == x {
                    (x - 1, y)
                } else {
                    (x, y - 1)
                }
            }
            'F' => {
                if previous_x == x {
                    (x + 1, y)
                } else {
                    (x, y + 1)
                }
            }
            '7' => {
                if previous_x == x {
                    (x - 1, y)
                } else {
                    (x, y + 1)
                }
            }
            'S' => return,
            _ => panic!("Invalid char `{}`", current),
        };
        if x < 0 || y < 0 || x >= maze[0].len() as i32 || y >= maze.len() as i32 {
            panic!("Out of bounds")
        }
        history.push((x, y));
        xy = (new_x, new_y);
    }
}

fn run_tests() {
    let example1 = include_str!("./input/example1.txt");
    let example2 = include_str!("./input/example2.txt");

    assert_eq!(furthest_from_start(maze_loop(&collect_maze(example1))), 4);
    println!("Passed part A example 1");
    assert_eq!(furthest_from_start(maze_loop(&collect_maze(example2))), 8);
    println!("Passed part A example 2");
    println!("Passed all part A tests");
    println!("");

    let example3 = include_str!("./input/example3.txt");
    let example4 = include_str!("./input/example4.txt");
    let example5 = include_str!("./input/example5.txt");

    let maze3 = collect_maze(example3);
    let maze4 = collect_maze(example4);
    let maze5 = collect_maze(example5);

    assert_eq!(tiles_inside(&maze3, &maze_loop(&maze3)), 4);
    println!("Passed part B example 1");
    assert_eq!(tiles_inside(&maze4, &maze_loop(&maze4)), 8);
    println!("Passed part B example 2");
    assert_eq!(tiles_inside(&maze5, &maze_loop(&maze5)), 10);
    println!("Passed part B example 3");

    println!("Passed all part B tests");
    println!("");

    println!("Passed all tests");
    println!("");
}

fn collect_maze(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
