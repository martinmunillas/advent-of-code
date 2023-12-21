use std::collections::{HashSet, VecDeque};

fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");

    println!("Result A: {}", visited_after_steps(&collect_map(input), 64));
}

fn visited_after_steps(map: &Vec<Vec<char>>, steps: usize) -> i32 {
    let mut start = (0, 0);
    'outer: for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                start = (x, y);
                break 'outer;
            }
        }
    }

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut answer = HashSet::new();
    let odd = steps % 2;
    queue.push_back((start.0, start.1, 0));
    while queue.len() > 0 {
        let (x, y, i) = queue.pop_front().unwrap();
        if map[y][x] == '#' {
            continue;
        }
        if seen.contains(&(x, y)) {
            continue;
        }
        seen.insert((x, y));
        if i <= steps && i % 2 == odd {
            answer.insert((x, y));
        }
        if x > 0 {
            queue.push_back((x - 1, y, i + 1));
        }
        if x < map[y].len() - 1 {
            queue.push_back((x + 1, y, i + 1));
        }
        if y > 0 {
            queue.push_back((x, y - 1, i + 1));
        }
        if y < map.len() - 1 {
            queue.push_back((x, y + 1, i + 1));
        }
    }

    print_map(map, &answer, start);
    answer.len() as i32
}

fn print_map(map: &Vec<Vec<char>>, visited: &HashSet<(usize, usize)>, start: (usize, usize)) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if visited.contains(&(x, y)) {
                print!("O");
            } else if start.0 == x && start.1 == y {
                print!("S");
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
    input.lines().map(|line| line.chars().collect()).collect()
}

fn run_tests() {
    let example = include_str!("./input/example.txt");

    assert_eq!(visited_after_steps(&collect_map(example), 1), 2);
    println!("Test passed!");
    assert_eq!(visited_after_steps(&collect_map(example), 2), 4);
    println!("Test passed!");
    assert_eq!(visited_after_steps(&collect_map(example), 3), 6);
    println!("Test passed!");
    assert_eq!(visited_after_steps(&collect_map(example), 6), 16);
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
