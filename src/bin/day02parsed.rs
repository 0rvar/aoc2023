use chumsky::{
    prelude::*,
    text::{whitespace, TextParser},
};
use std::collections::HashMap;

use aoc2023::initialize_aoc;

fn main() {
    let aoc = initialize_aoc();
    let input = aoc.input();
    let games = input.lines().map(parse_game).collect::<Vec<_>>();

    let maximums: HashMap<Color, u32> =
        HashMap::from_iter([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);

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
            let mut maximums: HashMap<Color, u32> = HashMap::new();
            for hand in &game.hands {
                let maximum = maximums.entry(hand.color.clone()).or_default();
                *maximum = (*maximum as u32).max(hand.count);
            }
            let power: u64 = maximums.iter().map(|(_, v)| *v as u64).product();
            power
        })
        .collect::<Vec<_>>();

    tracing::info!("Part 2: {:?}", powers.iter().sum::<u64>());
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}
#[derive(Debug, PartialEq, Eq)]
struct GameHand {
    count: u32,
    color: Color,
}
#[derive(Debug, PartialEq, Eq)]
struct Game {
    game_number: u32,
    hands: Vec<GameHand>,
}
fn parse_game(line: &str) -> Game {
    let int = text::int::<_, Simple<char>>(10).map(|s: String| s.parse::<u32>().unwrap());
    let color = text::ident::<_, Simple<char>>().map(|s| match s.as_str() {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => panic!("Unknown color {}", s),
    });
    let hand = int
        .then_ignore(whitespace())
        .then(color)
        .map(|(count, color)| GameHand { count, color });
    just("Game ")
        .ignore_then(int)
        .then_ignore(just(": "))
        .then(hand.padded().separated_by(just(',').or(just(';'))))
        .map(|(game_number, hands)| Game { game_number, hands })
        .parse(line)
        .unwrap()
}

#[test]
fn test_parse_game() {
    assert_eq!(
        parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green"),
        Game {
            game_number: 1,
            hands: vec![
                GameHand {
                    count: 3,
                    color: Color::Blue,
                },
                GameHand {
                    count: 4,
                    color: Color::Red,
                },
                GameHand {
                    count: 1,
                    color: Color::Red,
                },
                GameHand {
                    count: 2,
                    color: Color::Green,
                },
            ]
        }
    )
}
