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
            let mut parts = line.split(": ");
            let game_number: u32 = parts
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse()
                .unwrap();
            let balls = parts
                .next()
                .unwrap()
                .split(|c| c == ',' || c == ';')
                .map(|specifier| {
                    let mut specified_parts = specifier.trim().split(" ");
                    let count: u32 = specified_parts.next().unwrap().parse().unwrap();
                    let color = specified_parts.next().unwrap().to_string();
                    GameBall { color, count }
                })
                .collect::<Vec<_>>();
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
