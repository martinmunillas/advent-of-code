fn main() {
    let input = include_str!("input.txt");
    let histories: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.split(" ").map(|n| n.parse().unwrap()).collect())
        .collect();

    let mut result = 0;
    for history in histories {
        result += history_prediction(&history);
    }
    println!("Result: {}", result);
}

fn history_prediction(history: &Vec<i32>) -> i32 {
    let mut are_the_same = true;
    let mut previous_history: Vec<i32> = vec![];
    for i in 1..history.len() {
        if history[i] != history[i - 1] {
            are_the_same = false;
        }
        previous_history.push(history[i] - history[i - 1]);
    }
    if are_the_same {
        return history[0];
    }
    let previous_prediction = history_prediction(&previous_history);

    history[history.len() - 1] + previous_prediction
}
