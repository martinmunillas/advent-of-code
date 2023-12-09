use std::cmp::max;

fn main() {
    possible_points();
    fewest_possible_points()
}

fn fewest_possible_points() {
    let input = include_str!("input.txt");
    let lines = input.lines();
    let mut result = 0;
    for line in lines {
        result += fewest_possible_power(line);
    }
    println!("Result B {}", result);
}

fn fewest_possible_power(line: &str) -> i32 {
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;

    let start = line.split(":");
    let games = start.last().unwrap().trim().split(|c| c == ';' || c == ',');

    for game in games {
        let parts: Vec<&str> = game.trim().split_whitespace().collect();
        let (amount, color) = (parts[0], parts[1]);
        let new = amount.parse::<i32>().unwrap();
        match color {
            "red" => red = max(red, new),
            "green" => green = max(green, new),
            "blue" => blue = max(blue, new),
            _ => {}
        }
    }

    return red * blue * green;
}

fn possible_points() {
    let input = include_str!("input.txt");
    let lines = input.lines();
    let mut result = 0;
    for (i, line) in lines.enumerate() {
        if is_possible(line) {
            result += i + 1;
        }
    }
    println!("Result A {}", result);
}

fn is_possible(line: &str) -> bool {
    let start = line.split(":");
    let games = start.last().unwrap().trim().split(|c| c == ';' || c == ',');

    for game in games {
        let parts: Vec<&str> = game.trim().split_whitespace().collect();
        let (amount, color) = (parts[0], parts[1]);
        let new = amount.parse::<i32>().unwrap();
        match color {
            "red" => {
                if new > 12 {
                    return false;
                }
            }
            "green" => {
                if new > 13 {
                    return false;
                }
            }
            "blue" => {
                if new > 14 {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}
