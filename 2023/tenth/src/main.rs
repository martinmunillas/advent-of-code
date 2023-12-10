fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");

    let maze = collect_maze(input);

    println!("Part 1: {}", furthest_from_start(&maze));
}

const LEFT_CHARS: &[char] = &['-', 'F', 'L'];
const RIGHT_CHARS: &[char] = &['-', '7', 'J'];
const UP_CHARS: &[char] = &['|', 'F', '7'];
const DOWN_CHARS: &[char] = &['|', 'L', 'J'];

fn furthest_from_start(maze: &Vec<Vec<char>>) -> i32 {
    let mut history = Vec::new();
    let mut start = (0, 0);
    'outer: for (y, line) in maze.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == 'S' {
                history.push((x as i32, y as i32));
                if x > 0 && LEFT_CHARS.contains(&maze[y][x - 1]) {
                    start = (x as i32 - 1, y as i32)
                } else if x < maze[0].len() - 1 && RIGHT_CHARS.contains(&maze[y][x + 1]) {
                    start = (x as i32 + 1, y as i32)
                } else if y > 0 && UP_CHARS.contains(&maze[y - 1][x]) {
                    start = (x as i32, y as i32 - 1)
                } else if y < maze.len() - 1 && DOWN_CHARS.contains(&maze[y + 1][x]) {
                    start = (x as i32, y as i32 + 1)
                } else {
                    panic!("Invalid start, no path to follow")
                }
                break 'outer;
            }
        }
    }

    solve(maze, start, &mut history);
    return (history.len() as f32 / 2.0).ceil() as i32;
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

    assert_eq!(furthest_from_start(&collect_maze(example1)), 4);
    println!("Passed example 1");
    assert_eq!(furthest_from_start(&collect_maze(example2)), 8);
    println!("Passed example 2");
    println!("Passed all tests");
    println!("");
}

fn collect_maze(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
