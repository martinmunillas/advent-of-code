use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, VecDeque},
};

fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");

    println!(
        "Result A: {}",
        get_safe_to_desintegrate(&mut collect_bricks(input))
    );
    println!(
        "Result B: {}",
        get_sum_chain_reaction_desintegration(&mut collect_bricks(input))
    );
}

fn get_sum_chain_reaction_desintegration(bricks: &mut Vec<Brick>) -> i32 {
    sort_by_z(bricks);
    apply_gravity(bricks);
    sort_by_z(bricks);

    let (supporting, supported_by) = get_supporting_and_supported(bricks);

    let mut fallen_on_reaction = 0;
    for brick in bricks.clone() {
        let mut falling = VecDeque::new();
        let mut fallen = HashSet::new();
        fallen.insert(&brick);

        for supported in &supporting[&brick] {
            if supported_by[supported].len() == 1 {
                falling.push_back(supported);
                fallen.insert(supported);
            }
        }
        while falling.len() > 0 {
            let current = falling.pop_front().unwrap();
            for supported in &supporting[current] {
                if fallen.contains(supported) {
                    continue;
                }
                let mut all_suppports_fallen = true;
                for supporting in &supported_by[supported] {
                    if !fallen.contains(supporting) {
                        all_suppports_fallen = false;
                        break;
                    }
                }
                if all_suppports_fallen {
                    falling.push_back(supported);
                    fallen.insert(supported);
                }
            }
        }

        fallen_on_reaction += fallen.len() - 1;
    }

    fallen_on_reaction as i32
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

fn get_supporting_and_supported(
    bricks: &Vec<Brick>,
) -> (
    HashMap<&Brick, HashSet<&Brick>>,
    HashMap<&Brick, HashSet<&Brick>>,
) {
    let mut supporting = HashMap::new();
    let mut supported_by = HashMap::new();
    for brick in bricks {
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

    (supporting, supported_by)
}

fn get_safe_to_desintegrate(bricks: &mut Vec<Brick>) -> i32 {
    sort_by_z(bricks);
    apply_gravity(bricks);
    sort_by_z(bricks);

    let (supporting, supported_by) = get_supporting_and_supported(bricks);

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
    assert_eq!(
        get_sum_chain_reaction_desintegration(&mut collect_bricks(example)),
        7
    );
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
