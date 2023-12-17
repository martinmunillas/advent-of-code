use std::{cmp::min, collections::HashMap};

fn main() {
    run_tests();
}

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
    heatmap: &Vec<Vec<i64>>,
    previous: Option<(usize, usize)>,
    start: (usize, usize),
    memo: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    if start.0 == heatmap.len() - 1 && start.1 == heatmap[start.1].len() - 1 {
        return heatmap[start.1][start.0];
    }
    if start.0 >= heatmap.len() || start.1 >= heatmap[start.1].len() {
        return i64::MAX;
    }
    if memo.contains_key(&start) {
        return *memo.get(&start).unwrap();
    }

    memo.insert(start, i64::MAX);

    let direction = match previous {
        Some(previous) => Some(get_direction(previous, start)),
        None => None,
    };

    let mut result = i64::MAX;

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
                    if left < i64::MAX {
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
                    if right < i64::MAX {
                        result = min(result, right + current);
                    }
                }
            }
        }
        Some(Direction::Down) => {
            let mut current = 0;
            for i in 1..=MAX_STRAIGHT_DISTANCE {
                if start.1 + i < heatmap.len() {
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
                    if left < i64::MAX {
                        result = min(result, left + current);
                    }
                }
                let right = find_least_heat(
                    heatmap,
                    Some((start.0, start.1 + i)),
                    (start.0 + 1, start.1 + i),
                    memo,
                );
                if right < i64::MAX {
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
                    if up < i64::MAX {
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
                    if down < i64::MAX {
                        result = min(result, down + current);
                    }
                }
            }
        }
        Some(Direction::Right) => {
            let mut current = 0;
            for i in 1..=MAX_STRAIGHT_DISTANCE {
                if start.0 + i < heatmap[0].len() {
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
                    if up < i64::MAX {
                        result = min(result, up + current);
                    }
                }
                let down = find_least_heat(
                    heatmap,
                    Some((start.0 + i, start.1)),
                    (start.0 + i, start.1 + 1),
                    memo,
                );
                if down < i64::MAX {
                    result = min(result, down + current);
                }
            }
        }
        None => {
            result = min(
                result,
                find_least_heat(heatmap, Some(start), (start.0, start.1 + 1), memo),
            );
            result = min(
                result,
                find_least_heat(heatmap, Some(start), (start.0 + 1, start.1), memo),
            );
        }
    }
    if result < i64::MAX {
        result += heatmap[start.1][start.0];
    }
    memo.insert(start, result);
    result
}

fn parse_heatmap(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|num| num.to_digit(10).unwrap() as i64)
                .collect()
        })
        .collect()
}

fn run_tests() {
    let example = include_str!("./input/example.txt");

    let example_heatmap = parse_heatmap(example);
    assert_eq!(
        find_least_heat(&example_heatmap, None, (0, 0), &mut HashMap::new()),
        102
    );
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
