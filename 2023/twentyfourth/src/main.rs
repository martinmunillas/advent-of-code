use std::collections::HashSet;

fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");

    println!(
        "Result A: {}",
        count_collisions_in_range(
            &collect_hailstones(input),
            (200000000000000.0, 400000000000000.0)
        )
    );
    println!(
        "Result B: {}",
        sum_rock_components(&collect_hailstones(input))
    );
}

fn count_collisions_in_range(hailstones: &Vec<Hailstone>, range: (f64, f64)) -> i64 {
    let mut collisions = 0;
    for (i, a) in hailstones.iter().enumerate() {
        for b in hailstones.iter().skip(i + 1) {
            if let Some(coords) = get_hailstones_collision(*a, *b) {
                if coords.0 >= range.0
                    && coords.0 <= range.1
                    && coords.1 >= range.0
                    && coords.1 <= range.1
                    && !is_coord_in_past(*a, coords)
                    && !is_coord_in_past(*b, coords)
                {
                    collisions += 1;
                }
            }
        }
    }
    collisions
}

fn is_coord_in_past(hailstone: Hailstone, coord: (f64, f64)) -> bool {
    let ((x, _, _), (dx, _, _)) = hailstone;
    let (cx, _) = coord;

    let is_negative = dx < 0;
    let is_past = if is_negative {
        (x as f64) < cx
    } else {
        (x as f64) > cx
    };
    is_past
}

fn get_hailstones_collision(a: Hailstone, b: Hailstone) -> Option<(f64, f64)> {
    let ((a_x, a_y, _), (a_dx, a_dy, _)) = a;
    let ((b_x, b_y, _), (b_dx, b_dy, _)) = b;

    let a_x = a_x as f64;
    let a_y = a_y as f64;
    let a_dx = a_dx as f64;
    let a_dy = a_dy as f64;
    let b_x = b_x as f64;
    let b_y = b_y as f64;
    let b_dx = b_dx as f64;
    let b_dy = b_dy as f64;

    let a_slope = a_dy / a_dx;
    let b_slope = b_dy / b_dx;

    if a_slope == b_slope {
        return None;
    }

    let x = (b_y - a_y + a_slope * a_x - b_slope * b_x) / (a_slope - b_slope);
    let y = a_y + a_slope * (x - a_x);

    Some((x, y))
}
type Hailstone = ((i64, i64, i64), (i64, i64, i64));

fn is_integer(n: f64) -> bool {
    n == n.round()
}

fn change_xy_velocity(h: Hailstone, x: i64, y: i64) -> Hailstone {
    let (pos, (dx, dy, dz)) = h;
    (pos, (dx - x, dy - y, dz))
}

fn swap_yz(h: Hailstone) -> Hailstone {
    let ((x, y, z), (dx, dy, dz)) = h;
    ((x, z, y), (dx, dz, dy))
}

fn sum_rock_components(hailstones: &Vec<Hailstone>) -> i64 {
    for i in 0..500 {
        for neg_i in [1, -1] {
            for j in 0..500 {
                'trying: for neg_j in [1, -1] {
                    let i = i * neg_i;
                    let j = j * neg_j;

                    let mut collisions = HashSet::new();
                    let mut pairs_attempted = 0;

                    let mut f = hailstones[0];
                    let mut s = hailstones[1];
                    'collisions: for (idx, first) in hailstones.iter().enumerate() {
                        for second in &hailstones[(idx + 1)..] {
                            pairs_attempted += 1;
                            let first_with_velocity = change_xy_velocity(*first, i, j);
                            let second_with_velocity = change_xy_velocity(*second, i, j);
                            if let Some((x, y)) =
                                get_hailstones_collision(first_with_velocity, second_with_velocity)
                            {
                                if is_integer(x) && is_integer(y) {
                                    collisions.insert((x as i64, y as i64));
                                    f = *first;
                                    s = *second;
                                }
                            } else {
                                continue 'trying;
                            }
                            if pairs_attempted > 5 {
                                break 'collisions;
                            }
                        }
                    }

                    if collisions.len() != 1 {
                        continue;
                    }

                    let (x, y) = collisions.iter().next().unwrap();
                    for k in 0..500 {
                        for neg_k in [1, -1] {
                            let k = k * neg_k;

                            let first_z_with_velocity = change_xy_velocity(swap_yz(f), i, k);
                            let second_z_with_velocity = change_xy_velocity(swap_yz(s), i, k);

                            if let Some((xx, z)) = get_hailstones_collision(
                                first_z_with_velocity,
                                second_z_with_velocity,
                            ) {
                                if xx as i64 == *x
                                    && hailstones.iter().all(|h| {
                                        !is_coord_in_past(
                                            change_xy_velocity(*h, i, j),
                                            (*x as f64, *y as f64),
                                        )
                                    })
                                {
                                    return x + y + z as i64;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    0
}

fn collect_hailstones(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .map(|line| {
            let numbers = line
                .split(|c| ", @".contains(c))
                .filter(|c| !c.is_empty())
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            return (
                (numbers[0], numbers[1], numbers[2]),
                (numbers[3], numbers[4], numbers[5]),
            );
        })
        .collect::<Vec<Hailstone>>()
}

fn run_tests() {
    let example = include_str!("./input/example.txt");

    assert_eq!(
        count_collisions_in_range(&collect_hailstones(example), (7.0, 27.0)),
        2
    );
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
