use std::collections::HashMap;

use aoc2023::{create_adjacent_positions, create_adjacent_positions_limited, initialize_aoc};

fn main() {
    let mut aoc = initialize_aoc();
    let input = aoc.input();

    aoc.measure("Parsing map");
    let mut map = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.as_bytes().iter().enumerate() {
            width = width.max(x);
            map.insert((x, y), character);
        }
        height = height.max(y);
    }

    aoc.measure("Find runs");
    let mut number_runs: Vec<Vec<((usize, usize), u32)>> = Vec::new();
    for y in 0..=height {
        let mut current_run: Option<Vec<((usize, usize), u32)>> = None;
        for x in 0..=width {
            let char = map.get(&(x, y)).unwrap();
            if let Some(digit) = byte_to_digit(**char) {
                if let Some(current_run) = &mut current_run {
                    current_run.push(((x, y), digit))
                } else {
                    current_run = Some(vec![((x, y), digit)]);
                }
            } else {
                if let Some(current_run) = current_run.take() {
                    number_runs.push(current_run);
                }
            }
        }
        if let Some(current_run) = current_run.take() {
            number_runs.push(current_run);
        }
    }

    aoc.measure("part_number_sum");
    let part_number_sum: u32 = number_runs
        .clone()
        .into_iter()
        .filter_map(|run| {
            let mut has_adjacent_symbol = false;
            for ((x, y), _) in &run {
                let found_adjacent = create_adjacent_positions_limited((*x, *y), (width, height))
                    .iter()
                    .any(|(ax, ay)| is_symbol(**map.get(&(*ax as usize, *ay as usize)).unwrap()));
                if found_adjacent {
                    has_adjacent_symbol = true;
                    break;
                }
            }
            if !has_adjacent_symbol {
                return None;
            }
            let parsed = parse_digits(&run);
            Some(parsed)
        })
        .sum();

    tracing::info!("Part 1: {:?}", part_number_sum);

    aoc.measure("gear_ratio_sum");
    let mut gear_ratio_sum = 0;
    for y in 0..=height {
        for x in 0..=width {
            let char = **map.get(&(x, y)).unwrap();
            if char != b'*' {
                continue;
            }
            let adjacent_runs = number_runs
                .iter()
                .filter(|run| {
                    run.iter().any(|((rx, ry), _)| {
                        create_adjacent_positions((*rx, *ry))
                            .iter()
                            .any(|(ax, ay)| *ax == x && *ay == y)
                    })
                })
                .collect::<Vec<_>>();
            if adjacent_runs.len() == 2 {
                let gear_ratio = parse_digits(adjacent_runs[0]) * parse_digits(adjacent_runs[1]);
                gear_ratio_sum += gear_ratio;
            }
        }
    }

    tracing::info!("Part 2: {}", gear_ratio_sum);
}

fn parse_digits(vec: &Vec<((usize, usize), u32)>) -> u32 {
    let mut sum = 0;
    for (_, digit) in vec.iter() {
        sum = sum * 10 + digit
    }
    sum
}

fn is_symbol(char: u8) -> bool {
    byte_to_digit(char).is_none() && char != b'.'
}
#[test]
fn test_is_symbol() {
    assert!(is_symbol(b'#'));
    assert!(is_symbol(b'$'));
    assert!(is_symbol(b'+'));
    assert!(is_symbol(b'/'));
    assert!(is_symbol(b'&'));
    assert!(is_symbol(b'-'));
    assert!(is_symbol(b'='));
    assert!(is_symbol(b'%'));
    assert!(!is_symbol(b'.'));
    assert!(!is_symbol(b'6'));
}

fn byte_to_digit(byte: u8) -> Option<u32> {
    if byte >= b'0' && byte <= b'9' {
        Some((byte - b'0').into())
    } else {
        None
    }
}
