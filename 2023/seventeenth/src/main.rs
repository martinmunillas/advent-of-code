use std::collections::{BinaryHeap, HashSet};

fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");

    println!("Result A: {}", find_least_heat(&parse_heatmap(input), 3, 0));
    println!(
        "Result B: {}",
        find_least_heat(&parse_heatmap(input), 10, 4)
    );
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Step {
    heat: i32,
    position: (i32, i32),
    direction: (i32, i32),
    distance: i32,
}
impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.heat.cmp(&self.heat))
    }
}
impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat.cmp(&self.heat)
    }
}

fn find_least_heat(
    heatmap: &Vec<Vec<i32>>,
    max_straight_distance: i32,
    min_straight_distance: i32,
) -> i32 {
    let mut pq = BinaryHeap::new();
    let mut seen = HashSet::new();

    pq.push(Step {
        heat: 0,
        position: (0, 0),
        direction: (0, 0),
        distance: 0,
    });

    while pq.len() > 0 {
        let step = pq.pop().unwrap();
        if seen.contains(&(step.position, step.direction, step.distance)) {
            continue;
        }
        seen.insert((step.position, step.direction, step.distance));

        if step.position.0 == heatmap.len() as i32 - 1
            && step.position.1 == heatmap[0].len() as i32 - 1
            && step.distance >= min_straight_distance
        {
            return step.heat;
        }

        if step.distance < max_straight_distance && step.direction != (0, 0) {
            let new = (
                step.position.0 + step.direction.0,
                step.position.1 + step.direction.1,
            );
            if new.0 >= 0
                && new.1 >= 0
                && new.0 < heatmap.len() as i32
                && new.1 < heatmap[0].len() as i32
            {
                pq.push(Step {
                    heat: step.heat + heatmap[new.0 as usize][new.1 as usize],
                    position: new,
                    direction: step.direction,
                    distance: step.distance + 1,
                });
            }
        }
        if step.distance < min_straight_distance && step.direction != (0, 0) {
            continue;
        }
        for direction in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            if direction == step.direction || direction == (-step.direction.0, -step.direction.1) {
                continue;
            }

            let new = (step.position.0 + direction.0, step.position.1 + direction.1);
            if new.0 < 0
                || new.1 < 0
                || new.0 >= heatmap.len() as i32
                || new.1 >= heatmap[0].len() as i32
            {
                continue;
            }
            pq.push(Step {
                heat: step.heat + heatmap[new.0 as usize][new.1 as usize],
                position: new,
                direction: direction,
                distance: 1,
            })
        }
    }
    0
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

fn run_tests() {
    let example = include_str!("./input/example.txt");
    let example2 = include_str!("./input/example2.txt");

    assert_eq!(find_least_heat(&parse_heatmap(example), 3, 0), 102);
    println!("Test passed!");
    assert_eq!(find_least_heat(&parse_heatmap(example), 10, 4), 94);
    println!("Test passed!");
    assert_eq!(find_least_heat(&parse_heatmap(example2), 10, 4), 71);
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
