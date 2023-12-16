use std::collections::HashMap;

fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");
    let rows = collect_rows(input);
    println!("Result A: {}", count_tilt_north(&rows));
    println!("Result B: {}", cycle(&rows, 1_000_000_000));
}

fn count_tilt_north(rows: &Vec<Vec<char>>) -> i32 {
    let y = transpose(rows);

    let tilted = apply_gravity(&y, false);

    let support = count_beam_support(&tilted);
    support
}

fn count_beam_support(matrix: &Vec<Vec<char>>) -> i32 {
    let mut result = 0;
    for row in matrix {
        for (i, c) in row.iter().enumerate() {
            if *c == 'O' {
                result += row.len() - i;
            }
        }
    }

    result as i32
}

fn apply_gravity(matrix: &Vec<Vec<char>>, negative: bool) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for row in matrix {
        let mut groups = Vec::new();
        let mut current_circles = 0;
        let mut current_spaces = 0;
        for (i, c) in row.iter().enumerate() {
            if *c == 'O' {
                current_circles += 1;
            } else if *c == '.' {
                current_spaces += 1;
            }

            if *c == '#' || i == row.len() - 1 {
                let mut group = Vec::new();
                if groups.len() > 0 {
                    group.push('#');
                }
                if negative {
                    for _ in 0..current_spaces {
                        group.push('.');
                    }
                    for _ in 0..current_circles {
                        group.push('O');
                    }
                } else {
                    for _ in 0..current_circles {
                        group.push('O');
                    }
                    for _ in 0..current_spaces {
                        group.push('.');
                    }
                }
                if *c == '#' && i == row.len() - 1 {
                    group.push('#');
                }
                groups.push(group);
                current_circles = 0;
                current_spaces = 0;
            }
        }
        result.push(groups.concat());
    }
    result
}

fn cycle(rows: &Vec<Vec<char>>, iterations: i32) -> i32 {
    let mut map = rows.clone();
    let mut seen = HashMap::new();
    let mut at: Vec<Vec<Vec<char>>> = Vec::new();
    for i in 0..iterations {
        for j in 0..4 {
            map = transpose(&map);
            map = apply_gravity(&map, j % 2 == 1);
        }
        if seen.contains_key(&map) {
            let previous = seen.get(&map).unwrap();
            let iterations_equivalent =
                ((iterations - previous - 1) % (i + 1 - previous)) + previous - 2;
            let equivalent = &at[iterations_equivalent as usize];

            return count_beam_support(&transpose(equivalent));
        }
        seen.insert(map.clone(), i);
        at.push(map.clone());
    }

    0
}

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    let row_length = matrix[0].len();

    for i in 0..row_length {
        let mut row = Vec::new();
        for j in 0..matrix.len() {
            row.push(matrix[j][i]);
        }
        result.push(row);
    }
    result.reverse();
    result
}

fn collect_rows(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn run_tests() {
    let example = collect_rows(include_str!("./input/example.txt"));

    assert_eq!(count_tilt_north(&example), 136);
    println!("Test passed!");
    assert_eq!(cycle(&example, 1_000_000), 64);
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
