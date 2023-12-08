use std::{cmp::Ordering, collections::HashMap};

use aoc2023::initialize_aoc;
use itertools::Itertools;
use num::Integer;

fn main() {
    let mut aoc = initialize_aoc();
    let input = aoc.input();

    aoc.measure("Parse");
    let (instructions, map) = parse(&input);

    aoc.measure("P1");
    let mut steps = 0;
    let mut cycled_instructions = instructions.chars().cycle();
    let mut position = "AAA";
    while position != "ZZZ" {
        let instruction = cycled_instructions.next().unwrap();
        let map_guide = map.get(position).unwrap();
        position = if instruction == 'L' {
            map_guide.0
        } else {
            map_guide.1
        };
        steps += 1;
    }
    let part1 = steps;

    aoc.measure("Parse");

    let (instructions, map) = parse(&input);

    aoc.measure("P2");

    let mut current_nodes = map
        .keys()
        .copied()
        .filter(|node| node.ends_with("A"))
        .collect::<Vec<_>>();
    let mut cycled_instructions = instructions.chars().cycle();
    let mut steps: u64 = 0;
    let nodes_count = current_nodes.len();
    let mut cycles = vec![];
    loop {
        steps += 1;
        let instruction = cycled_instructions.next().unwrap();
        let mut done = true;
        for index in 0..nodes_count {
            let position = current_nodes[index];
            if position == "" {
                continue;
            }
            done = false;
            let map_guide = map.get(position).unwrap();
            let position = if instruction == 'L' {
                map_guide.0
            } else {
                map_guide.1
            };
            if position.ends_with('Z') {
                cycles.push(steps);
                current_nodes[index] = "";
            } else {
                current_nodes[index] = position;
            }
        }
        if done {
            break;
        }
        if steps % 10_000_000 == 0 {
            println!("Steps {}", steps);
        }
    }

    let mut lcm = cycles[0];
    for cycle in cycles.iter().skip(1) {
        lcm = lcm.lcm(cycle);
    }

    aoc.done();
    tracing::info!("Part 1: {part1}");
    tracing::info!("Part 2: {lcm}");
}

fn parse(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().trim();
    lines.next();
    let map = lines
        .map(|line| {
            let (node, next) = line.split_once(" = (").unwrap();
            let (left, next) = next.split_once(", ").unwrap();
            let right = next.split(')').next().unwrap();
            (node, (left, right))
        })
        .collect::<_>();
    (instructions, map)
}
