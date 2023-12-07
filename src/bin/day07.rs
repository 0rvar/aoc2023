use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use aoc2023::initialize_aoc;

fn main() {
    let mut aoc = initialize_aoc();
    let input = aoc.input();

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

    let mut ranked = bet_list.clone();
    ranked.sort_by(|a, b| {
        let hand_cmp = (a.2 as u8).cmp(&(b.2 as u8));
        if !hand_cmp.is_eq() {
            return hand_cmp;
        }

        cmp_cards(a.0, b.0, false)
    });
    let ranked = ranked.iter().enumerate().collect::<Vec<_>>();

    let part1 = ranked
        .iter()
        .map(|(rank, (_, bid, _, _))| (*rank as u32 + 1) * *bid)
        .sum::<u32>();
    tracing::info!("Part 1: {part1:#?}");

    let mut ranked = bet_list.clone();
    ranked.sort_by(|a, b| {
        let hand_cmp = (a.3 as u8).cmp(&(b.3 as u8));
        if !hand_cmp.is_eq() {
            return hand_cmp;
        }

        cmp_cards(a.0, b.0, true)
    });
    let ranked = ranked.iter().enumerate().collect::<Vec<_>>();

    let part2 = ranked
        .iter()
        .map(|(rank, (_, bid, _, _))| (*rank as u32 + 1) * *bid)
        .sum::<u32>();

    tracing::info!("Part 2: {part2:#?}");
}

fn cmp_cards(a: &str, b: &str, interesting: bool) -> Ordering {
    for (a, b) in a.as_bytes().iter().zip(b.as_bytes()) {
        let ordering = card_value(*a, interesting).cmp(&card_value(*b, interesting));
        if !ordering.is_eq() {
            return ordering;
        }
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
    let card_counts = hand.iter().fold(HashMap::new(), |mut map, card| {
        *(map.entry(card).or_insert(0)) += 1u32;
        map
    });
    let mut counts = card_counts.values().map(|x| *x).collect::<Vec<_>>();
    counts.sort();

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

fn parse_interesting_hand(hand: &[u8]) -> HandType {
    let (joker_count, card_counts) =
        hand.iter()
            .fold((0u32, HashMap::new()), |(joker_count, mut map), card| {
                if *card == b'J' {
                    (joker_count + 1, map)
                } else {
                    *(map.entry(card).or_insert(0)) += 1u32;
                    (joker_count, map)
                }
            });
    let mut counts = card_counts.values().map(|x| *x).collect::<Vec<_>>();
    counts.sort();

    match joker_count {
        0 => match &counts[..] {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAkind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            what => panic!("Did not parse {what:?}"),
        },
        1 => match &counts[..] {
            [4] => HandType::FiveOfAKind,
            [1, 3] => HandType::FourOfAKind,
            [2, 2] => HandType::FullHouse,
            [1, 1, 2] => HandType::ThreeOfAkind,
            [1, 1, 1, 1] => HandType::OnePair,
            what => panic!("Did not parse {what:?}"),
        },
        2 => match &counts[..] {
            [3] => HandType::FiveOfAKind,
            [1, 2] => HandType::FourOfAKind,
            [1, 1, 1] => HandType::ThreeOfAkind,
            what => panic!("Did not parse {what:?}"),
        },
        3 => match &counts[..] {
            [2] => HandType::FiveOfAKind,
            [1, 1] => HandType::FourOfAKind,
            what => panic!("Did not parse {what:?}"),
        },
        4 => match &counts[..] {
            [1] => HandType::FiveOfAKind,
            what => panic!("Did not parse {what:?}"),
        },
        5 => HandType::FiveOfAKind,
        _ => panic!("More than 5 Js"),
    }
}
