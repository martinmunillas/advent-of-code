use std::collections::HashMap;

#[derive(Debug)]
struct Player<'a> {
    cards: &'a str,
    bet: i32,
}

fn main() {
    let input = include_str!("input.txt");
    let mut players: Vec<Player> = input
        .lines()
        .map(|line| {
            let cards: &str = &line[..5];
            let bet = line[6..].parse().unwrap();

            Player { bet, cards }
        })
        .collect();

    players.sort_by(|a, b| cards_pts(a.cards).cmp(&cards_pts(b.cards)));
    println!("Result A: {}", gains(&players));
    players.sort_by(|a, b| cards_pts_with_jokers(a.cards).cmp(&cards_pts_with_jokers(b.cards)));
    println!("Result B: {}", gains(&players))
}

fn gains(players: &Vec<Player>) -> i32 {
    let mut result = 0;
    for (i, p) in players.iter().enumerate() {
        result += (i as i32 + 1) * p.bet;
    }
    result
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
    } else if kind.len() == 2 && kind.iter().any(|(_, k)| *k == 4) {
        pts += kind_multiplier * 5
    } else if kind.len() == 2 && kind.iter().any(|(_, k)| *k == 3) {
        pts += kind_multiplier * 4
    } else if kind.len() == 3 && kind.iter().any(|(_, k)| *k == 3) {
        pts += kind_multiplier * 3
    } else if kind.len() == 3 && kind.iter().any(|(_, k)| *k == 2) {
        pts += kind_multiplier * 2
    } else if kind.len() == 4 {
        pts += kind_multiplier * 1
    }

    pts
}

fn cards_pts_with_jokers(cards: &str) -> i32 {
    let kind_variants = HashMap::from([
        ('J', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
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
    let j = match kind.get(&'J') {
        Some(j) => *j as usize,
        None => 0,
    };
    let kinds_without_joker = match j {
        0 => kind.len(),
        _ => kind.len() - 1,
    };

    if kinds_without_joker <= 1 {
        pts += kind_multiplier * 6;
    } else if kinds_without_joker <= 2
        && kind.iter().any(|(k, u)| match k {
            'J' => *u == 4,
            _ => *u + j as i8 == 4,
        })
    {
        pts += kind_multiplier * 5
    } else if kinds_without_joker == 2
        && match j {
            0 => kind.iter().any(|(_, u)| *u == 3),
            _ => kind.iter().any(|(_, u)| *u == 2),
        }
    {
        pts += kind_multiplier * 4
    } else if kind.iter().any(|(_, u)| *u + j as i8 == 3) {
        pts += kind_multiplier * 3
    } else if kind.iter().any(|(_, u)| *u + j as i8 == 2) {
        if kind.len() == 3 {
            pts += kind_multiplier * 2
        } else {
            pts += kind_multiplier * 1
        }
    } else {
    }

    pts
}
