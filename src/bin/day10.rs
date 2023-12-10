use std::collections::HashMap;

use aoc2023::{initialize_aoc, create_adjacent_positions};

fn main() {
    let mut aoc = initialize_aoc();
    let input = aoc.input();

    let input = "
    .....
    .F-S.
    .|-|
    .L-J.
    .....";

    aoc.measure("Parse");
    let mut map = HashMap::new();
    let mut animal_position = (0, 0);
    for (row, line) in input.trim().lines().enumerate() {
        for (column, char) in line.trim().chars().enumerate() {
            if char == 'S' {
                animal_position = (row, column);
            }
            map.insert((row, column), char);
        }
    }

    aoc.measure("P1");
    tracing::debug!("Pos: {animal_position:?} ({})", map.get(&animal_position).unwrap());
    let mut prev_position = animal_position.clone();
    let mut steps = 1;
    let mut position = walk(&animal_position, &prev_position, &map);
    tracing::debug!("Pos: {position:?} ({})", map.get(&position).unwrap());
    while position != animal_position {
        let prev = position;
        position = walk(&position, &prev_position, &map);
        prev_position = prev;
        steps += 1;
        tracing::debug!("Pos: {position:?} ({})", map.get(&position).unwrap())
    }

    aoc.done();

    tracing::info!("Part 1: {}", steps);
}

fn walk(position: &(usize, usize), previous: &(usize, usize), map: &HashMap<(usize, usize), char>) -> (usize, usize) {
    let adjacent = [
        ((position.0.saturating_sub(1), position.1), "|F7"),
        ((position.0 + 1, position.1), "|LJ"),
        ((position.0, position.1.saturating_sub(1)), "-FL"),
        ((position.0, position.1 + 1), "-J7"),
        ((position.0.saturating_sub(1), position.1), "S"),
        ((position.0 + 1, position.1), "S"),
        ((position.0, position.1.saturating_sub(1)), "S"),
        ((position.0, position.1 + 1), "S"),
    ].iter().filter_map(|(p, accepted_chars)| {
        if p == previous {
            return None;
        }
        let Some(char) = map.get(p) else {
            return None;
        };
        if !accepted_chars.contains(*char) {
            return None;
        }
        return Some(*p)
    }).next().unwrap();
    return adjacent
}

