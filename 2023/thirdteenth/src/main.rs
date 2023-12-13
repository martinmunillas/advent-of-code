use std::cmp::min;
fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");
    println!("Result A: {}", sum_mirrors(&collect_mazes(input)));
    println!("Result B: ");
}

fn sum_mirrors(mazes: &Vec<Vec<Vec<char>>>) -> i32 {
    let mut sum = 0;
    for maze in mazes {
        let xresult = find_x_mirror(maze);
        sum += xresult;
        let yresult = find_y_mirror(maze);
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

fn find_x_mirror(maze: &Vec<Vec<char>>) -> i32 {
    let mut possible_mirrorings = Vec::new();
    let line_lenght = maze[0].len();
    'searching: for i in 1..line_lenght {
        'limits: for j in 1..line_lenght {
            if i < j || i + j - 1 >= line_lenght {
                break 'limits;
            }
            if maze[0][i - j] != maze[0][i + j - 1] {
                continue 'searching;
            }
        }
        possible_mirrorings.push(i);
    }

    'searching: for possible in possible_mirrorings {
        for y in 0..maze.len() {
            let search_length = min(line_lenght - possible + 1, possible + 1);
            for x in 1..search_length {
                if maze[y][possible - x] != maze[y][possible + x - 1] {
                    continue 'searching;
                }
            }
        }
        return possible as i32;
    }

    0
}

fn find_y_mirror(maze: &Vec<Vec<char>>) -> i32 {
    let mut possible_mirrorings = Vec::new();
    let line_lenght = maze[0].len();
    let lines = maze.len();
    'searching: for i in 1..lines {
        'limits: for j in 1..lines {
            if i < j || i + j - 1 >= lines {
                break 'limits;
            }
            if maze[i - j][0] != maze[i + j - 1][0] {
                continue 'searching;
            }
        }
        possible_mirrorings.push(i);
    }

    'searching: for possible in possible_mirrorings {
        for x in 0..line_lenght {
            let search_length = min(lines - possible + 1, possible + 1);
            for y in 1..search_length {
                if maze[possible - y][x] != maze[possible + y - 1][x] {
                    continue 'searching;
                }
            }
        }
        return possible as i32;
    }

    0
}

fn run_tests() {
    let example = include_str!("./input/example.txt");

    assert_eq!(sum_mirrors(&collect_mazes(example)), 405);
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
