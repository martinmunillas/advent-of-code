fn main() {
    let input = include_str!("input.txt");
    let usable = input
        .lines()
        .map(|x| {
            x.split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    let times = usable.first().unwrap();
    let distances = usable.last().unwrap();

    let mut result = 1;

    for race in 0..times.len() {
        let mut valid = 0;
        for second in 1..times[race] {
            if second * (times[race] - second) > distances[race] {
                valid += 1;
            }
        }
        result *= valid;
    }

    println!("Result: {}", result);
}
