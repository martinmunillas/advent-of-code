use std::collections::HashMap;

#[derive(Debug)]
struct Player {
    bet: i32,
    pts: i32,
}

fn main() {
    let input = include_str!("input.txt");
    let mut players: Vec<Player> = input
        .lines()
        .map(|line| {
            let cards: &str = &line[..5];
            let bet = line[6..].parse().unwrap();
            let pts = cards_pts(cards);

            Player { bet, pts }
        })
        .collect();

    sort_players(&mut players);
    let mut result = 0;
    for (i, p) in players.iter().enumerate() {
        result += (i as i32 + 1) * p.bet;
    }

    println!("Result: {}", result);
}

fn sort_players(players: &mut Vec<Player>) {
    players.sort_by(|a, b| a.pts.cmp(&b.pts));
}

fn cards_pts(cards: &str) -> i32 {
    let kind_variants = HashMap::from([
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('J', 10),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ]);
    let mut kind: HashMap<char, i8> = HashMap::new();
    let mut pts = 0;
    for (i, k) in cards.chars().enumerate() {
        if kind.contains_key(&k) {
            let current = kind.get(&k);
            kind.insert(k, current.unwrap() + 1);
        } else {
            kind.insert(k, 1);
        }
        pts += (14 as i32).pow(6 - i as u32) * kind_variants.get(&k).unwrap();
    }
    let kind_multiplier = (14 as i32).pow(7);
    if kind.len() == 1 {
        pts += kind_multiplier * 6;
    }
    if kind.len() == 2 && kind.iter().any(|(_, k)| *k == 4) {
        pts += kind_multiplier * 5
    }
    if kind.len() == 2 && kind.iter().any(|(_, k)| *k > 1 && *k < 4) {
        pts += kind_multiplier * 4
    }
    if kind.len() == 3 && kind.iter().any(|(_, k)| *k == 3) {
        pts += kind_multiplier * 3
    }
    if kind.len() == 3 && kind.iter().any(|(_, k)| *k == 2) {
        pts += kind_multiplier * 2
    }
    if kind.len() == 4 {
        pts += kind_multiplier * 1
    }

    pts
}
