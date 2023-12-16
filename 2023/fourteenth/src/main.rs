use std::cmp::Ordering;

fn main() {
    run_tests();

    // let input = include_str!("./input/input.txt");
    // let rows = collect_rows(input);
    // println!("Result A: {}", count_tilt_north(&rows));
}

fn count_tilt_north(rows: &Vec<Vec<char>>) -> i32 {
    let mut y = transpose(rows);

    y.reverse();
    print_matrix(&y);
    apply_gravity(&mut y, false);
    print_matrix(&y);

    let support = count_beam_support(&y);
    y.reverse();
    let x = transpose(&y);
    print_matrix(&x);
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

fn bubble_sort<F>(row: &mut Vec<char>, cmp: F)
where
    F: Fn(char, char) -> Ordering,
{
    let mut had_mutations = false;
    for i in 0..row.len() - 1 {
        if cmp(row[i], row[i + 1]) == Ordering::Greater {
            let tmp = row[i];
            row[i] = row[i + 1];
            row[i + 1] = tmp;
            had_mutations = true;
        }
    }
    if had_mutations {
        bubble_sort(row, cmp);
    }
}

fn apply_gravity(matrix: &mut Vec<Vec<char>>, negative: bool) {
    for row in matrix {
        bubble_sort(row, |a, b| {
            if a == '#' || b == '#' {
                std::cmp::Ordering::Equal
            } else if a == 'O' {
                if negative {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            } else if b == 'O' {
                if negative {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            } else {
                std::cmp::Ordering::Equal
            }
        })
    }
}

fn cycle(rows: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut x = transpose(rows);
    apply_gravity(&mut x, false);
    let mut y = transpose(&x);
    apply_gravity(&mut y, false);
    let mut x = transpose(&y);
    apply_gravity(&mut x, true);
    let mut y = transpose(&x);
    apply_gravity(&mut y, true);
    y
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
    result
}

fn tilt_cycles(rows: &Vec<Vec<char>>, cycles: usize) -> Vec<Vec<char>> {
    let mut result = rows.clone();
    for _ in 0..cycles {
        result = cycle(&result);
    }
    result
}

fn collect_rows(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
    println!("");
}

fn run_tests() {
    let example = include_str!("./input/example.txt");

    let mut a  = vec![".......O......#...OO.#...O#..#.#..O...........OO.O.#O...#O#.....#.#....##.O.#...##..#O......#.O.#.O.".chars().collect::<Vec<char>>()];
    apply_gravity(&mut a, true);
    print_matrix(&a);

    assert_eq!(count_tilt_north(&collect_rows(example)), 136);
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
