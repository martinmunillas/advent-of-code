use core::panic;
use std::{convert::TryInto, fmt::Display};

fn main() {
    let input = include_str!("input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let symbols = "!@#$%^&*()-+=[]{}|\\/:;'\"<>?,~_`";
    let mut current = "".to_owned();
    let mut current_start = -1;
    let mut result = 0;
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let mut should_search_symbol = false;
            match c.to_digit(10) {
                Some(_) => {
                    current.push_str(&c.to_string());
                    if current_start == -1 {
                        current_start = j as i32;
                    }
                    if j == line.len() - 1 {
                        should_search_symbol = true;
                    }
                }
                None => should_search_symbol = true,
            }
            if should_search_symbol {
                {
                    if current.len() > 0 {
                        let mut has_symbol = false;
                        for k in (current_start - 1)..(current_start + current.len() as i32 + 1) {
                            if k < 0 || k >= line.len() as i32 {
                                continue;
                            }
                            if i > 0 {
                                let ch = nth(lines[i - 1], k);
                                if symbols.contains(ch) {
                                    has_symbol = true;
                                    break;
                                }
                            }
                            if i < lines.len() - 1 {
                                let ch = nth(lines[i + 1], k);
                                if symbols.contains(ch) {
                                    has_symbol = true;
                                    break;
                                }
                            }
                        }
                        if current_start > 0 {
                            let ch = nth(line, current_start - 1);
                            if symbols.contains(ch) {
                                has_symbol = true;
                            }
                        }
                        let right = current_start + current.len() as i32;
                        if right < line.len() as i32 {
                            let ch = nth(line, right);
                            if symbols.contains(ch) {
                                has_symbol = true;
                            }
                        }

                        if has_symbol {
                            match current.parse::<i32>() {
                                Ok(v) => {
                                    // println!("Found: {}", v);
                                    result += v;
                                }
                                Err(_) => {}
                            }
                        }
                    }
                    current_start = -1;
                    current = "".to_owned()
                }
            }
        }
    }
    println!("Result: {}", result);
}

fn nth<T>(input: &str, n: T) -> char
where
    T: TryInto<usize>,
    T: Display,
    T: Copy,
{
    match n.try_into() {
        Ok(n) => input.chars().nth(n).unwrap(),
        Err(_) => {
            panic!("Invalid index {}", n)
        }
    }
}
