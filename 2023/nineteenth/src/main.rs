use std::collections::HashMap;

fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");

    let (parts, instructions) = collect_input(input);
    println!("Result A: {}", run_instructions(&instructions, &parts));
    println!(
        "Result B: {}",
        find_possible_combinations(
            &instructions,
            "in",
            HashMap::from([
                ('x', (1, 4000)),
                ('m', (1, 4000)),
                ('a', (1, 4000)),
                ('s', (1, 4000)),
            ])
        )
    );
}

fn exec(
    instruction: &str,
    part: &HashMap<char, i64>,
    instructions: &HashMap<&str, Vec<Rule>>,
) -> bool {
    let rules = &instructions[instruction];
    for rule in rules {
        let mut complies = false;
        match rule.operator {
            Some('<') => {
                if part[&rule.part.unwrap()] < rule.value.unwrap() {
                    complies = true;
                }
            }
            Some('>') => {
                if part[&rule.part.unwrap()] > rule.value.unwrap() {
                    complies = true;
                }
            }
            _ => {}
        }

        if complies || rule.operator.is_none() {
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

fn run_instructions(
    instructions: &HashMap<&str, Vec<Rule>>,
    parts: &Vec<HashMap<char, i64>>,
) -> i64 {
    let mut result = 0;

    for part in parts {
        if exec("in", &part, instructions) {
            result += part[&'x'] + part[&'m'] + part[&'a'] + part[&'s'];
        }
    }

    result
}

fn find_possible_combinations(
    instructions: &HashMap<&str, Vec<Rule>>,
    instruction: &str,
    bounds: HashMap<char, (i64, i64)>,
) -> i64 {
    if instruction == "A" {
        return (bounds[&'x'].1 - bounds[&'x'].0 + 1)
            * (bounds[&'m'].1 - bounds[&'m'].0 + 1)
            * (bounds[&'a'].1 - bounds[&'a'].0 + 1)
            * (bounds[&'s'].1 - bounds[&'s'].0 + 1);
    } else if instruction == "R" {
        return 0;
    }
    let mut possible = 0;
    let mut bf = bounds.clone();
    for rule in &instructions[instruction] {
        match rule.operator {
            None => {
                possible += find_possible_combinations(instructions, rule.next_rule, bf.clone())
            }
            Some('<') => {
                let mut bt = bf.clone();
                let part = &rule.part.unwrap();
                let value = rule.value.unwrap();
                bt.insert(*part, (bt[part].0, value - 1));
                bf.insert(*part, (value, bf[part].1));
                possible += find_possible_combinations(instructions, rule.next_rule, bt);
            }
            Some('>') => {
                let mut bt = bf.clone();
                let part = &rule.part.unwrap();
                let value = rule.value.unwrap();
                bt.insert(*part, (value + 1, bt[part].1));
                bf.insert(*part, (bf[part].0, value));
                possible += find_possible_combinations(instructions, rule.next_rule, bt);
            }
            _ => {}
        }
    }

    possible
}

#[derive(Debug)]
struct Rule<'a> {
    part: Option<char>,
    operator: Option<char>,
    value: Option<i64>,
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
                        value: Some(chunks[1].parse::<i64>().unwrap()),
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

fn collect_parts(input: &str) -> Vec<HashMap<char, i64>> {
    let mut parts = Vec::new();

    for line in input.lines() {
        let chunks: Vec<i64> = line
            .split(|c: char| "{}=xmas,".contains(c))
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        parts.push(HashMap::from([
            ('x', chunks[0]),
            ('m', chunks[1]),
            ('a', chunks[2]),
            ('s', chunks[3]),
        ]));
    }
    parts
}

fn collect_input<'a>(input: &'a str) -> (Vec<HashMap<char, i64>>, HashMap<&str, Vec<Rule<'a>>>) {
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
    assert_eq!(
        find_possible_combinations(
            &instructions,
            "in",
            HashMap::from([
                ('x', (1, 4000)),
                ('m', (1, 4000)),
                ('a', (1, 4000)),
                ('s', (1, 4000)),
            ])
        ),
        167409079868000
    );
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
