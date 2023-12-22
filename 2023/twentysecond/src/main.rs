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

fn get_safe_to_desintegrate(bricks: &mut Vec<Brick>) -> i32 {
    bricks.sort_by(|a, b| a[2].cmp(&b[2]));

    println!("Bricks: {:?}", bricks);
    0
}

fn run_tests() {
    let example = include_str!("./input/example.txt");

    assert_eq!(get_safe_to_desintegrate(&mut collect_bricks(example)), 5);
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
