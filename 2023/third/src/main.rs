use core::panic;
use std::{convert::TryInto, fmt::Display};

fn main() {
    println!("Result: A {}", engine_schematic());
    println!("Result: B {}", gear_ratios());
}

fn gear_ratios() -> i32 {
    let input = include_str!("input.txt");
    let lines: Vec<&str> = input.lines().collect();

    let mut result = 0;
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '*' {
                continue;
            }
            let mut max_start = j;
            if j > 0 {
                max_start -= 1;
            }
            let mut min_end = j;
            if min_end < line.len() - 1 {
                min_end += 1;
            }
            let mut numbers: Vec<i32> = vec![];
            if i > 0 {
                let top = find_numbers_substring(lines[i - 1], max_start, min_end);
                substring_to_numbers(top)
                    .iter()
                    .for_each(|v| numbers.push(*v));
            }

            let middle = find_numbers_substring(lines[i], max_start, min_end);
            substring_to_numbers(middle)
                .iter()
                .for_each(|v| numbers.push(*v));
            if i < lines.len() - 1 {
                let bottom = find_numbers_substring(lines[i + 1], max_start, min_end);
                substring_to_numbers(bottom)
                    .iter()
                    .for_each(|v| numbers.push(*v));
            }

            if numbers.len() == 2 {
                result += numbers[0] * numbers[1];
            }
        }
    }

    result
}

fn find_numbers_substring(line: &str, max_start: usize, min_end: usize) -> &str {
    let mut idx_start = max_start;
    while match nth(line, idx_start).to_digit(10) {
        Some(_) => true,
        None => false,
    } {
        if idx_start == 0 {
            break;
        }
        idx_start -= 1;
    }

    let mut idx_end = min_end;
    while match nth(line, idx_end).to_digit(10) {
        Some(_) => true,
        None => false,
    } {
        idx_end += 1;
        if idx_end >= line.len() {
            break;
        }
    }

    &line[idx_start..idx_end]
}

fn substring_to_numbers(line: &str) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];

    line.split(|c| c == '*' || c == '.')
        .filter(|s| s.len() > 0)
        .for_each(|s| match s.parse::<i32>() {
            Ok(v) => result.push(v),
            Err(_) => {}
        });
    result
}

fn engine_schematic() -> i32 {
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
    result
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
