use std::collections::HashMap;

use aoc2023::initialize_aoc;

fn main() {
    let aoc = initialize_aoc();
    let input = aoc.input();

    #[derive(Debug)]
    struct GameBall {
        color: String,
        count: u32,
    }
    #[derive(Debug)]
    struct Game {
        game_number: u32,
        balls: Vec<GameBall>,
    }

    let games = input
        .lines()
        .map(|line| {
            let parts = line.split_once(": ").unwrap();
            let game_number: u32 = parts.0.split(" ").last().unwrap().parse().unwrap();
            let mut balls = Vec::with_capacity(50);
            for specifier in parts.1.split(|c| c == ',' || c == ';') {
                let parts = specifier.trim().split_once(" ").unwrap();
                let count: u32 = parts.0.parse().unwrap();
                let color = parts.1.to_string();
                balls.push(GameBall { color, count })
            }
            Game { game_number, balls }
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
            for ball in &game.balls {
                let maximum_for_color = maximums.get(&ball.color).unwrap();
                if ball.count > *maximum_for_color {
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
            for ball in &game.balls {
                let maximum = maximums.entry(&ball.color).or_default();
                *maximum = (*maximum as u32).max(ball.count);
            }
            let power: u64 = maximums.iter().map(|(_, v)| *v as u64).product();
            power
        })
        .collect::<Vec<_>>();

    tracing::info!("Part 2: {:?}", powers.iter().sum::<u64>());
}
