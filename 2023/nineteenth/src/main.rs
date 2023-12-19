use std::collections::HashMap;

fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");

    let (parts, instructions) = collect_input(input);
    println!("Result A: {}", run_instructions(&instructions, &parts));
}

fn exec(instruction: &str, part: &Part, instructions: &HashMap<&str, Vec<Rule>>) -> bool {
    let rules = &instructions[instruction];
    for rule in rules {
        let mut complies = false;
        match rule.operator {
            None => {
                if rule.next_rule == "A" {
                    return true;
                } else if rule.next_rule == "R" {
                    return false;
                }
                return exec(rule.next_rule, part, instructions);
            }
            Some('<') => match rule.part.unwrap() {
                'x' => {
                    if part.x < rule.value.unwrap() {
                        complies = true;
                    }
                }
                'm' => {
                    if part.m < rule.value.unwrap() {
                        complies = true;
                    }
                }
                'a' => {
                    if part.a < rule.value.unwrap() {
                        complies = true;
                    }
                }
                's' => {
                    if part.s < rule.value.unwrap() {
                        complies = true;
                    }
                }
                _ => panic!("Unknown part"),
            },
            Some('>') => match rule.part.unwrap() {
                'x' => {
                    if part.x > rule.value.unwrap() {
                        complies = true;
                    }
                }
                'm' => {
                    if part.m > rule.value.unwrap() {
                        complies = true;
                    }
                }
                'a' => {
                    if part.a > rule.value.unwrap() {
                        complies = true;
                    }
                }
                's' => {
                    if part.s > rule.value.unwrap() {
                        complies = true;
                    }
                }
                _ => panic!("Unknown part"),
            },
            Some(_) => panic!("Unknown operator"),
        }

        if complies {
            if rule.next_rule == "A" {
                return true;
            } else if rule.next_rule == "R" {
                return false;
            }
            return exec(rule.next_rule, part, instructions);
        }
    }

    false
}

fn run_instructions(instructions: &HashMap<&str, Vec<Rule>>, parts: &Vec<Part>) -> i32 {
    let mut result = 0;

    for part in parts {
        if exec("in", &part, instructions) {
            result += part.x + part.m + part.a + part.s;
        }
    }

    result
}

#[derive(Debug)]
struct Rule<'a> {
    part: Option<char>,
    operator: Option<char>,
    value: Option<i32>,
    next_rule: &'a str,
}

fn collect_instructions<'a>(input: &'a str) -> HashMap<&'a str, Vec<Rule<'a>>> {
    let mut instructions = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split(|c| "{}".contains(c)).filter(|s| !s.is_empty());
        let name = parts.next().unwrap();
        let rules = parts
            .next()
            .unwrap()
            .split(",")
            .map(|rule| {
                if rule.contains("<") || rule.contains(">") {
                    let operator = if rule.contains("<") {
                        Some('<')
                    } else {
                        Some('>')
                    };
                    let chunks = rule.split(|c| "<>:".contains(c)).collect::<Vec<&str>>();
                    Rule {
                        operator,
                        part: Some(chunks[0].chars().nth(0).unwrap()),
                        value: Some(chunks[1].parse::<i32>().unwrap()),
                        next_rule: chunks[2],
                    }
                } else {
                    Rule {
                        part: None,
                        operator: None,
                        value: None,
                        next_rule: rule,
                    }
                }
            })
            .collect();
        instructions.insert(name, rules);
    }

    instructions
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

fn collect_parts(input: &str) -> Vec<Part> {
    let mut parts = Vec::new();

    for line in input.lines() {
        let chunks: Vec<i32> = line
            .split(|c: char| "{}=xmas,".contains(c))
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        parts.push(Part {
            x: chunks[0],
            m: chunks[1],
            a: chunks[2],
            s: chunks[3],
        })
    }
    parts
}

fn collect_input<'a>(input: &'a str) -> (Vec<Part>, HashMap<&str, Vec<Rule<'a>>>) {
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    let instructions = collect_instructions(sections[0]);
    let parts = collect_parts(sections[1]);

    (parts, instructions)
}

fn run_tests() {
    let example = include_str!("./input/example.txt");
    let (parts, instructions) = collect_input(example);
    assert_eq!(run_instructions(&instructions, &parts), 19114);
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
