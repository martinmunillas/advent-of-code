#[derive(Debug)]
struct Card {
    ideal: Vec<i32>,
    actual: Vec<i32>,
}

fn main() {
    let input = include_str!("input.txt");
    let cards: Vec<Card> = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(line_to_card)
        .collect();
    println!("Result A: {}", total_card_pts(&cards));
    println!("Result B: {}", total_accumulative_cards(&cards));
}

fn total_accumulative_cards(cards: &[Card]) -> i32 {
    let mut accumulated = 0;
    let card_matches = cards
        .iter()
        .map(card_matching_numbers)
        .collect::<Vec<i32>>();
    let mut times: Vec<i32> = cards.iter().map(|_| 1).collect();

    for (i, matching) in card_matches.iter().enumerate() {
        for j in i + 1..=i + *matching as usize {
            times[j] += times[i];
        }
        accumulated += times[i]
    }

    accumulated
}

fn total_card_pts(cards: &[Card]) -> i32 {
    let mut pts = 0;
    for card in cards.iter() {
        pts += card_pts(card);
    }
    pts
}

fn serialize_vec(string: &str) -> Vec<i32> {
    string
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn line_to_card(line: &&str) -> Card {
    let split = line.split(":").nth(1).unwrap().split("|");
    let ideal = serialize_vec(split.clone().nth(0).unwrap());
    let actual = serialize_vec(split.clone().nth(1).unwrap());
    Card { ideal, actual }
}

fn card_pts(card: &Card) -> i32 {
    let mut pts = 0;
    for x in card.actual.clone().iter() {
        if card.ideal.contains(x) {
            if pts == 0 {
                pts = 1;
            } else {
                pts *= 2;
            }
        }
    }
    pts
}

fn card_matching_numbers(card: &Card) -> i32 {
    let mut pts = 0;
    for x in card.actual.clone().iter() {
        if card.ideal.contains(x) {
            pts += 1;
        }
    }
    pts
}
