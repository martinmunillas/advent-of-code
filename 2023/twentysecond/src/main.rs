use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");

    println!(
        "Result A: {}",
        get_safe_to_desintegrate(&mut collect_bricks(input))
    );
}

type Brick = Vec<usize>;

fn collect_bricks(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            line.split(|c| ",~".contains(c))
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Brick>>()
}

fn overlap_xy(a: &Brick, b: &Brick) -> bool {
    max(a[0], b[0]) <= min(a[3], b[3]) && max(a[1], b[1]) <= min(a[4], b[4])
}

fn get_safe_to_desintegrate(bricks: &mut Vec<Brick>) -> i32 {
    sort_by_z(bricks);
    apply_gravity(bricks);
    sort_by_z(bricks);

    let mut supporting = HashMap::new();
    let mut supported_by = HashMap::new();
    for brick in bricks.clone() {
        supporting.insert(brick, HashSet::<&Brick>::new());
    }
    for (i, up) in bricks.iter().enumerate() {
        let mut current_supported_by = HashSet::new();
        for down in &bricks[0..i] {
            if overlap_xy(up, down) && down[5] + 1 == up[2] {
                current_supported_by.insert(down);
                supporting.get_mut(down).unwrap().insert(up);
            }
        }
        supported_by.insert(up, current_supported_by);
    }

    let mut safe_to_desintegrate = 0;
    for brick in bricks.clone() {
        let mut safe = true;
        for supported in supporting.get(&brick).unwrap() {
            if supported_by.get(supported).unwrap().len() == 1 {
                safe = false;
                break;
            }
        }
        if safe {
            safe_to_desintegrate += 1;
        }
    }

    safe_to_desintegrate
}

fn sort_by_z(bricks: &mut Vec<Brick>) {
    bricks.sort_by(|a, b| a[2].cmp(&b[2]));
}

fn apply_gravity(bricks: &mut Vec<Brick>) {
    for (i, brick) in bricks.clone().iter().enumerate() {
        let mut new_z = 1;
        for other in &bricks[0..i] {
            if overlap_xy(brick, other) {
                new_z = max(new_z, other[5] + 1);
            }
        }
        let brick_height = brick[5] - brick[2];
        bricks[i][2] = new_z;
        bricks[i][5] = brick_height + new_z;
    }
}

fn run_tests() {
    let example = include_str!("./input/example.txt");

    assert_eq!(get_safe_to_desintegrate(&mut collect_bricks(example)), 5);
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
