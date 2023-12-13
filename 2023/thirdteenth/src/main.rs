use std::cmp::min;
fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");
    println!("Result A: {}", sum_mirrors(&collect_mazes(input), 0));
    println!("Result B: {}", sum_mirrors(&collect_mazes(input), 1));
}

fn sum_mirrors(mazes: &Vec<Vec<Vec<char>>>, expected_differences: i32) -> i32 {
    let mut sum = 0;
    for maze in mazes {
        let xresult = find_x_mirror(maze, expected_differences);
        sum += xresult;
        let yresult = find_y_mirror(maze, expected_differences);
        sum += yresult * 100;
    }
    sum
}

fn collect_mazes(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|maze| maze.lines().map(|l| l.chars().collect()).collect())
        .collect()
}

fn find_x_mirror(maze: &Vec<Vec<char>>, expected_differences: i32) -> i32 {
    let line_lenght = maze[0].len();

    for possible in 1..line_lenght {
        let mut differences = 0;
        for y in 0..maze.len() {
            let search_length = min(line_lenght - possible + 1, possible + 1);
            for x in 1..search_length {
                if maze[y][possible - x] != maze[y][possible + x - 1] {
                    differences += 1;
                }
            }
        }
        if differences == expected_differences {
            return possible as i32;
        }
    }

    0
}

fn find_y_mirror(maze: &Vec<Vec<char>>, expected_differences: i32) -> i32 {
    let line_lenght = maze[0].len();
    let lines = maze.len();

    for possible in 1..lines {
        let mut differences = 0;
        for x in 0..line_lenght {
            let search_length = min(lines - possible + 1, possible + 1);
            for y in 1..search_length {
                if maze[possible - y][x] != maze[possible + y - 1][x] {
                    differences += 1;
                }
            }
        }
        if differences == expected_differences {
            return possible as i32;
        }
    }

    0
}

fn run_tests() {
    let example = include_str!("./input/example.txt");

    assert_eq!(sum_mirrors(&collect_mazes(example), 0), 405);
    println!("Test passed!");
    assert_eq!(sum_mirrors(&collect_mazes(example), 1), 400);
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
