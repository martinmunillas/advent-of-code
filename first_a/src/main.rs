fn main() {
    let input = include_str!("input.txt");
    let lines = input.lines();
    let mut result = 0;
    for l in lines {
        result += line_value(l);
    }
    println!("Result: {}", result);
}

fn line_value(x: &str) -> i32 {
    let mut value = 0;
    for c in x.chars() {
        match c.to_digit(10) {
            Some(n) => {
                value += (n as i32) * 10;
                break;
            }
            None => continue,
        }
    }
    for c in x.chars().rev() {
        match c.to_digit(10) {
            Some(n) => {
                value += n as i32;
                break;
            }
            None => continue,
        }
    }
    value
}
