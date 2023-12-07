use std::cmp::Ordering;

use aoc2023::initialize_aoc;
use itertools::Itertools;

fn main() {
    let mut aoc = initialize_aoc();
    let input = aoc.input();

    aoc.measure("Parse");
    let bet_list = input
        .lines()
        .map(|line| {
            let (cards, bet) = line.split_once(' ').unwrap();
            let hand = parse_hand(cards.as_bytes());
            let interesting_hand = parse_interesting_hand(cards.as_bytes());
            let bet = bet.parse::<u32>().unwrap();
            (cards, bet, hand, interesting_hand)
        })
        .collect::<Vec<_>>();

    aoc.measure("Part 1");
    let mut ranked = bet_list.clone();
    ranked.sort_by(|a, b| {
        let hand_cmp = (a.2 as u8).cmp(&(b.2 as u8));
        if !hand_cmp.is_eq() {
            return hand_cmp;
        }

        cmp_cards(a.0, b.0, false)
    });
    let ranked = ranked.into_iter().enumerate().collect::<Vec<_>>();

    let part1 = ranked
        .into_iter()
        .map(|(rank, (_, bid, _, _))| (rank as u32 + 1) * bid)
        .sum::<u32>();

    aoc.measure("Part 2");
    let mut ranked = bet_list;
    ranked.sort_by(|a, b| {
        let hand_cmp = (a.3 as u8).cmp(&(b.3 as u8));
        if !hand_cmp.is_eq() {
            return hand_cmp;
        }

        cmp_cards(a.0, b.0, true)
    });
    let ranked = ranked.into_iter().enumerate().collect::<Vec<_>>();

    let part2 = ranked
        .into_iter()
        .map(|(rank, (_, bid, _, _))| (rank as u32 + 1) * bid)
        .sum::<u32>();

    aoc.done();

    tracing::info!("Part 1: {part1:#?}");
    tracing::info!("Part 2: {part2:#?}");
}

fn cmp_cards(a: &str, b: &str, interesting: bool) -> Ordering {
    for (a, b) in a.as_bytes().iter().zip(b.as_bytes()) {
        if a == b {
            continue;
        }
        let ordering = card_value(*a, interesting).cmp(&card_value(*b, interesting));
        return ordering;
    }
    panic!("Equal cards");
}

fn card_value(c: u8, interesting: bool) -> u8 {
    match c {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' => {
            if interesting {
                0
            } else {
                11
            }
        }
        b'T' => 10,
        digit => digit - b'0',
    }
}

#[derive(Debug, Clone, Copy)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAkind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn parse_hand(hand: &[u8]) -> HandType {
    let counts = hand.iter().copied().counts();
    let card_counts = counts.into_values().sorted().collect::<Vec<_>>();

    counts_to_hand(&card_counts[..])
}

fn parse_interesting_hand(hand: &[u8]) -> HandType {
    let joker_count = hand.iter().filter(|c| **c == b'J').count();
    let counts = hand.iter().copied().filter(|c| *c != b'J').counts();
    let mut card_counts = counts.into_values().sorted().collect::<Vec<_>>();

    if card_counts.len() == 0 {
        card_counts.push(joker_count);
    } else {
        let max_index = card_counts.iter().position_max().unwrap_or(0);
        card_counts[max_index] += joker_count;
        card_counts.sort();
    }

    counts_to_hand(&card_counts[..])
}

fn counts_to_hand(counts: &[usize]) -> HandType {
    match &counts[..] {
        [5] => HandType::FiveOfAKind,
        [1, 4] => HandType::FourOfAKind,
        [2, 3] => HandType::FullHouse,
        [1, 1, 3] => HandType::ThreeOfAkind,
        [1, 2, 2] => HandType::TwoPair,
        [1, 1, 1, 2] => HandType::OnePair,
        [1, 1, 1, 1, 1] => HandType::HighCard,
        what => panic!("Did not parse {what:?}"),
    }
}
