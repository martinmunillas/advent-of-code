fn main() {
    println!("Result A {}", calibrate(line_value));
    println!("Result B {}", calibrate(line_value_with_spelled));
}

fn calibrate(func: fn(&str) -> i32) -> i32 {
    let input = include_str!("input.txt");
    let lines = input.lines();
    let mut result = 0;
    for l in lines {
        result += func(l);
    }
    result
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

fn line_value_with_spelled(x: &str) -> i32 {
    let digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut value = 0;
    'outer: for (i, c) in x.char_indices() {
        match c.to_digit(10) {
            Some(n) => {
                value += (n as i32) * 10;
                break;
            }
            None => {
                for (j, digit) in digits.iter().enumerate() {
                    for (k, cd) in digit.char_indices() {
                        let current_idx = i + k;
                        if current_idx >= x.len() {
                            break;
                        }
                        if cd == x.chars().nth(current_idx).unwrap() {
                            if k == digit.len() - 1 {
                                value += (j as i32) * 10;
                                break 'outer;
                            }
                            continue;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    'outer: for (i, c) in x.char_indices().rev() {
        match c.to_digit(10) {
            Some(n) => {
                value += n as i32;
                break;
            }
            None => {
                for (j, digit) in digits.iter().enumerate() {
                    for (k, cd) in digit.char_indices().rev() {
                        let current_idx = i + k;
                        if current_idx >= x.len() {
                            break;
                        }
                        if cd == x.chars().nth(current_idx).unwrap() {
                            if k == 0 {
                                value += j as i32;
                                break 'outer;
                            }
                            continue;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    value
}
