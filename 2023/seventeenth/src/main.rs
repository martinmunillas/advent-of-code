use std::{cmp::min, collections::HashMap};

fn main() {
    run_tests();
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_direction(from: (usize, usize), to: (usize, usize)) -> Direction {
    if from.0 == to.0 {
        if from.1 < to.1 {
            Direction::Right
        } else {
            Direction::Left
        }
    } else {
        if from.0 < to.0 {
            Direction::Down
        } else {
            Direction::Up
        }
    }
}

const MAX_STRAIGHT_DISTANCE: usize = 3;

fn find_least_heat(
    heatmap: &Vec<Vec<i32>>,
    start: (usize, usize),
    direction: Option<Direction>,
    memo: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    println!("RECURSION {:?}", start);
    if start.0 == heatmap[0].len() - 1 && start.1 == heatmap.len() - 1 {
        println!("END");
        let last = heatmap[start.1][start.0];
        memo.insert(start, last);
        return last;
    }
    if start.0 >= heatmap[0].len() - 1 || start.1 >= heatmap.len() - 1 {
        println!("OUT OF BOUNDS");
        return i32::MAX;
    }
    if memo.contains_key(&start) {
        println!("IN MEMO");
        return *memo.get(&start).unwrap();
    }

    memo.insert(start, i32::MAX);

    let mut result = i32::MAX;
    println!("DIRECTION {:?}", direction);
    match direction {
        Some(Direction::Up) => {
            let mut current = 0;
            for i in 1..=MAX_STRAIGHT_DISTANCE {
                if start.1 < i {
                    break;
                }
                current += heatmap[start.1 - i as usize][start.0];
                if start.1 >= i && start.0 >= 1 {
                    let left = find_least_heat(
                        heatmap,
                        Some((start.0, start.1 - i)),
                        (start.0 - 1, start.1 - i),
                        memo,
                    );
                    if left < i32::MAX {
                        result = min(result, left + current);
                    }
                }
                if start.1 >= i {
                    let right = find_least_heat(
                        heatmap,
                        Some((start.0, start.1 - i)),
                        (start.0 + 1, start.1 - i),
                        memo,
                    );
                    if right < i32::MAX {
                        result = min(result, right + current);
                    }
                }
            }
        }
        Some(Direction::Down) => {
            let mut current = 0;
            for i in 1..=MAX_STRAIGHT_DISTANCE {
                if start.1 + i >= heatmap.len() {
                    break;
                }
                current += heatmap[start.1 + i as usize][start.0];
                if start.0 >= 1 {
                    let left = find_least_heat(
                        heatmap,
                        Some((start.0, start.1 + i)),
                        (start.0 - 1, start.1 + i),
                        memo,
                    );
                    if left < i32::MAX {
                        result = min(result, left + current);
                    }
                }
                let right = find_least_heat(
                    heatmap,
                    Some((start.0, start.1 + i)),
                    (start.0 + 1, start.1 + i),
                    memo,
                );
                if right < i32::MAX {
                    result = min(result, right + current);
                }
            }
        }
        Some(Direction::Left) => {
            let mut current = 0;
            for i in 1..=MAX_STRAIGHT_DISTANCE {
                if start.0 < i {
                    break;
                }
                current += heatmap[start.1][start.0 - i as usize];
                if start.0 >= i && start.1 >= 1 {
                    let up = find_least_heat(
                        heatmap,
                        Some((start.0 - i, start.1)),
                        (start.0 - i, start.1 - 1),
                        memo,
                    );
                    if up < i32::MAX {
                        result = min(result, up + current);
                    }
                }
                if start.0 >= i {
                    let down = find_least_heat(
                        heatmap,
                        Some((start.0 - i, start.1)),
                        (start.0 - i, start.1 + 1),
                        memo,
                    );
                    if down < i32::MAX {
                        result = min(result, down + current);
                    }
                }
            }
        }
        Some(Direction::Right) => {
            let mut current = 0;
            for i in 1..=MAX_STRAIGHT_DISTANCE {
                if start.0 + i >= heatmap[0].len() {
                    break;
                }
                current += heatmap[start.1][start.0 + i as usize];
                if start.1 >= 1 {
                    let up = find_least_heat(
                        heatmap,
                        Some((start.0 + i, start.1)),
                        (start.0 + i, start.1 - 1),
                        memo,
                    );
                    if up < i32::MAX {
                        result = min(result, up + current);
                    }
                }
                let down = find_least_heat(
                    heatmap,
                    Some((start.0 + i, start.1)),
                    (start.0 + i, start.1 + 1),
                    memo,
                );
                if down < i32::MAX {
                    result = min(result, down + current);
                }
            }
        }
        None => {
            for i in 1..=MAX_STRAIGHT_DISTANCE {
                result = min(
                    result,
                    find_least_heat(heatmap, Some(start), (start.0, start.1 + i), memo),
                );
                result = min(
                    result,
                    find_least_heat(heatmap, Some(start), (start.0 + i, start.1), memo),
                );
            }
        }
    }

    memo.insert(start, result);
    result
}

fn parse_heatmap(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|num| num.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn print_accumulated_heatmap(map: &Vec<Vec<i32>>, heatmap: &HashMap<(usize, usize), i32>) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            match heatmap.get(&(j, i)) {
                Some(heat) => {
                    if heat == &i32::MAX {
                        print!(" MAX ");
                    } else {
                        print!(" {:3} ", heat)
                    }
                }
                None => print!(" --- "),
            }
        }
        println!("");
    }
}

fn run_tests() {
    let example = include_str!("./input/example.txt");

    let example_heatmap = parse_heatmap(example);
    let mut hash = HashMap::new();
    let test = find_least_heat(&example_heatmap, None, (0, 0), &mut hash);
    print_accumulated_heatmap(&example_heatmap, &hash);
    assert_eq!(test, 102);
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
