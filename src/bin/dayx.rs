use std::collections::{HashMap, HashSet};

use aoc2023::initialize_aoc;

fn main() {
    let mut aoc = initialize_aoc();
    let input = aoc.input();

    aoc.measure("Parse");
    for (row, line) in input.trim().lines().enumerate() {}

    aoc.measure("P1");

    let part1 = 10 / 2;

    aoc.done();

    tracing::info!("Part 1: {}", part1);
}
