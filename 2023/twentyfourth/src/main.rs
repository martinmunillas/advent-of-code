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

    let is_negative = dx < 0.0;
    let is_past = if is_negative { x < cx } else { x > cx };
    is_past
}

fn get_hailstones_collision(a: Hailstone, b: Hailstone) -> Option<(f64, f64)> {
    let ((a_x, a_y, _), (a_dx, a_dy, _)) = a;
    let ((b_x, b_y, _), (b_dx, b_dy, _)) = b;

    let a_slope = a_dy / a_dx;
    let b_slope = b_dy / b_dx;

    if a_slope == b_slope {
        return None;
    }

    let x = (b_y - a_y + a_slope * a_x - b_slope * b_x) / (a_slope - b_slope);
    let y = a_y + a_slope * (x - a_x);

    Some((x, y))
}
type Hailstone = ((f64, f64, f64), (f64, f64, f64));

fn collect_hailstones(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .map(|line| {
            let nums = line
                .split(|c| ", @".contains(c))
                .filter(|c| !c.is_empty())
                .map(|c| c.parse::<f64>().unwrap())
                .collect::<Vec<f64>>();

            return ((nums[0], nums[1], nums[2]), (nums[3], nums[4], nums[5]));
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
