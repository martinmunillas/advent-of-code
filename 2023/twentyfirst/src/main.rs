use std::collections::{HashSet, VecDeque};

fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");

    let map = collect_map(input);
    println!("Result A: {}", visited_after_steps(&map, &find_s(&map), 64));
    println!(
        "Result B: {}",
        visited_after_many_steps(&collect_map(input), 26501365)
    );
}

fn find_s(map: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                return (x, y);
            }
        }
    }
    panic!("No S found");
}

fn visited_after_many_steps(map: &Vec<Vec<char>>, steps: i64) -> i64 {
    let size = map.len();
    let middle = size / 2;

    let lg_side = (size * 3 / 2) - 1;
    let sm_side = middle - 1;

    let full_odd = visited_after_steps(map, &(middle, middle), size);
    let full_even = visited_after_steps(map, &(middle, middle), size + 1);

    let n = visited_after_steps(map, &(middle, size - 1), size - 1);

    let nw_lg = visited_after_steps(map, &(0, size - 1), lg_side);
    let nw_sm = visited_after_steps(map, &(0, size - 1), sm_side);

    let w = visited_after_steps(map, &(0, middle), size - 1);

    let sw_lg = visited_after_steps(map, &(0, 0), lg_side);
    let sw_sm = visited_after_steps(map, &(0, 0), sm_side);

    let s = visited_after_steps(map, &(middle, 0), size - 1);

    let se_lg = visited_after_steps(map, &(size - 1, 0), lg_side);
    let se_sm = visited_after_steps(map, &(size - 1, 0), sm_side);

    let e = visited_after_steps(map, &(size - 1, middle), size - 1);

    let ne_lg = visited_after_steps(map, &(size - 1, size - 1), lg_side);
    let ne_sm = visited_after_steps(map, &(size - 1, size - 1), sm_side);

    let diamond_width = steps / size as i64 - 1;
    let odds = (diamond_width / 2 * 2 + 1).pow(2);
    let evens = (diamond_width + 1).pow(2);
    let smalls = diamond_width + 1;
    let larges = diamond_width;

    odds * full_odd
        + evens * full_even
        + (n + s + e + w)
        + (smalls * (nw_sm + ne_sm + sw_sm + se_sm))
        + (larges * (nw_lg + ne_lg + sw_lg + se_lg))
}

fn visited_after_steps(map: &Vec<Vec<char>>, start: &(usize, usize), steps: usize) -> i64 {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut answer = HashSet::new();
    let odd = steps % 2;
    queue.push_back((start.0, start.1, 0));
    while queue.len() > 0 {
        let (x, y, i) = queue.pop_front().unwrap();
        if map[y][x] == '#' || seen.contains(&(x, y)) {
            continue;
        } else if i > steps {
            break;
        }
        seen.insert((x, y));
        if i % 2 == odd {
            answer.insert((x, y));
        }
        if x > 0 {
            queue.push_back((x - 1, y, i + 1));
        }
        if x < map[y].len() - 1 {
            queue.push_back((x + 1, y, i + 1));
        }
        if y > 0 {
            queue.push_back((x, y - 1, i + 1));
        }
        if y < map.len() - 1 {
            queue.push_back((x, y + 1, i + 1));
        }
    }

    print_map(map, &answer, start);
    answer.len() as i64
}

fn print_map(map: &Vec<Vec<char>>, visited: &HashSet<(usize, usize)>, start: &(usize, usize)) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if visited.contains(&(x, y)) {
                print!("O");
            } else if start.0 == x && start.1 == y {
                print!("S");
            } else if map[y][x] == '#' {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

fn collect_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn run_tests() {
    let example = include_str!("./input/example.txt");

    let map = collect_map(example);
    let s = find_s(&map);
    assert_eq!(visited_after_steps(&map, &s, 1), 2);
    println!("Test passed!");
    assert_eq!(visited_after_steps(&map, &s, 2), 4);
    println!("Test passed!");
    assert_eq!(visited_after_steps(&map, &s, 3), 6);
    println!("Test passed!");
    assert_eq!(visited_after_steps(&map, &s, 6), 16);
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
