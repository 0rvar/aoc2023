use std::collections::HashMap;

use aoc2023::initialize_aoc;

fn main() {
    let aoc = initialize_aoc();
    let input = aoc.input();

    let mut map = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            width = width.max(x);
            map.insert((x, y), character);
        }
        height = height.max(y);
    }

    let mut number_runs: Vec<Vec<((usize, usize), u32)>> = Vec::new();
    for y in 0..=height {
        let mut current_run: Option<Vec<((usize, usize), u32)>> = None;
        for x in 0..=width {
            let foo = map.get(&(x, y)).unwrap();
            if let Some(digit) = foo.to_digit(10) {
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

    let part_number_sum: u32 = number_runs
        .clone()
        .into_iter()
        .filter_map(|run| {
            let mut has_adjacent_symbol = false;
            for ((x, y), _) in &run {
                let found_adjacent = adjacent((*x, *y))
                    .iter()
                    .filter(|(x, y)| {
                        *x >= 0 && *y >= 0 && *x <= width as isize && *y <= height as isize
                    })
                    .any(|(ax, ay)| is_symbol(*map.get(&(*ax as usize, *ay as usize)).unwrap()));
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

    let mut gear_ratio_sum = 0;
    for y in 0..=height {
        for x in 0..=width {
            let char = map.get(&(x, y)).unwrap();
            if *char != '*' {
                continue;
            }
            let adjacent_runs = number_runs
                .iter()
                .filter(|run| {
                    run.iter().any(|((rx, ry), _)| {
                        adjacent((*rx, *ry))
                            .iter()
                            .any(|(ax, ay)| *ax == x as isize && *ay == y as isize)
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

fn is_symbol(char: char) -> bool {
    !char.is_ascii_digit() && char != '.'
}
#[test]
fn test_is_symbol() {
    assert!(is_symbol('#'));
    assert!(is_symbol('$'));
    assert!(is_symbol('+'));
    assert!(!is_symbol('.'));
}

fn adjacent((x, y): (usize, usize)) -> Vec<(isize, isize)> {
    let x = x as isize;
    let y = y as isize;
    vec![
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ]
}
