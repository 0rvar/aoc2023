use std::collections::HashMap;

use aoc2023::initialize_aoc;

fn main() {
    let aoc = initialize_aoc();
    let input = aoc.input();

    #[derive(Debug)]
    struct GameHand {
        color: String,
        count: u32,
    }
    #[derive(Debug)]
    struct Game {
        game_number: u32,
        hands: Vec<GameHand>,
    }

    let games = input
        .lines()
        .map(|line| {
            let parts = line.split_once(": ").unwrap();
            let game_number: u32 = parts.0.split(" ").last().unwrap().parse().unwrap();
            let mut hands = Vec::with_capacity(50);
            for hand in parts.1.split(|c| c == ',' || c == ';') {
                let parts = hand.trim().split_once(" ").unwrap();
                let count: u32 = parts.0.parse().unwrap();
                let color = parts.1.to_string();
                hands.push(GameHand { color, count })
            }
            Game { game_number, hands }
        })
        .collect::<Vec<_>>();

    let maximums: HashMap<String, u32> = HashMap::from_iter([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);

    let possible_ids = games
        .iter()
        .filter(|game| {
            for hand in &game.hands {
                let maximum_for_color = maximums.get(&hand.color).unwrap();
                if hand.count > *maximum_for_color {
                    return false;
                }
            }
            return true;
        })
        .map(|x| x.game_number)
        .collect::<Vec<_>>();

    tracing::info!("Part 1: {:?}", possible_ids.iter().sum::<u32>());

    let powers = games
        .iter()
        .map(|game| {
            let mut maximums: HashMap<&String, u32> = HashMap::new();
            for hand in &game.hands {
                let maximum = maximums.entry(&hand.color).or_default();
                *maximum = (*maximum as u32).max(hand.count);
            }
            let power: u64 = maximums.iter().map(|(_, v)| *v as u64).product();
            power
        })
        .collect::<Vec<_>>();

    tracing::info!("Part 2: {:?}", powers.iter().sum::<u64>());
}
