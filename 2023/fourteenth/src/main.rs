fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");
    println!("Result A: {}", count_tilt_north(input));
}

fn count_tilt_north(map: &str) -> i32 {
    let row_lenght = map.find('\n').unwrap() + 1;
    let mut columns = Vec::new();
    for (i, c) in map.chars().enumerate() {
        if i / row_lenght == 0 {
            columns.push(vec![c]);
        } else if c != '\n' {
            columns[i % row_lenght].push(c);
        }
    }

    let mut result = 0;
    for column in columns {
        let mut last_square = 0;
        let mut accumulated_circle = 0;
        for (i, c) in column.iter().enumerate() {
            if *c == '#' {
                last_square = i + 1;
                accumulated_circle = 0;
            }
            if *c == 'O' {
                result += column.len() - last_square - accumulated_circle;
                accumulated_circle += 1;
            }
        }
    }

    result as i32
}

fn run_tests() {
    let example = include_str!("./input/example.txt");

    assert_eq!(count_tilt_north(example), 136);
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
