use std::collections::HashMap;

fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");
    let universe = collect_universe(input);

    println!("Result 1: {}", sum_galaxy_distances(universe));
}

fn sum_galaxy_distances(universe: Vec<Vec<char>>) -> i32 {
    let mut has_galaxy: HashMap<(char, usize), bool> = HashMap::new();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for (y, row) in universe.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == '#' {
                has_galaxy.insert(('x', x), true);
                has_galaxy.insert(('y', y), true);
                galaxies.push((x, y));
            }
        }
    }

    let mut x_offsets = Vec::new();
    let mut y_offsets = Vec::new();
    let mut current_offset = 0;
    for (y, _) in universe.iter().enumerate() {
        y_offsets.push(current_offset);

        if has_galaxy.get(&('y', y)).is_none() {
            current_offset += 1;
        }
    }

    current_offset = 0;
    for (x, _) in universe[0].iter().enumerate() {
        x_offsets.push(current_offset);

        if has_galaxy.get(&('x', x)).is_none() {
            current_offset += 1;
        }
    }

    let mut sum = 0;

    for (i, a) in galaxies[0..galaxies.len()].iter().enumerate() {
        for j in (i + 1)..galaxies.len() {
            let b = &galaxies[j];

            let real_a = (a.0 + x_offsets[a.0], a.1 + y_offsets[a.1]);
            let real_b = (b.0 + x_offsets[b.0], b.1 + y_offsets[b.1]);

            let distance = (real_a.0 as i32 - real_b.0 as i32).abs()
                + (real_a.1 as i32 - real_b.1 as i32).abs();

            sum += distance;
        }
    }

    sum
}

fn collect_universe(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn run_tests() {
    let example = include_str!("./input/example.txt");

    assert_eq!(sum_galaxy_distances(collect_universe(example)), 374);
    println!("Tests passed!")
}
