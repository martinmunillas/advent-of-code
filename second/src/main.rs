fn main() {
    let input = include_str!("input.txt");
    let lines = input.lines();
    let mut result = 0;
    for (i, line) in lines.enumerate() {
        if is_possible(line) {
            result += i + 1;
        }
    }
    println!("Result {}", result);
}

fn is_possible(line: &str) -> bool {
    let start = line.split(":");
    let games = start.last().unwrap().trim().split(|c| c == ';' || c == ',');

    for game in games {
        let parts: Vec<&str> = game.trim().split_whitespace().collect();
        let (amount, color) = (parts[0], parts[1]);
        match color {
            "red" => {
                if amount.parse::<i32>().unwrap() > 12 {
                    return false;
                }
            }
            "green" => {
                if amount.parse::<i32>().unwrap() > 13 {
                    return false;
                }
            }
            "blue" => {
                if amount.parse::<i32>().unwrap() > 14 {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}
