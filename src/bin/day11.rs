use std::collections::HashSet;

use aoc2023::initialize_aoc;

fn main() {
    let mut aoc = initialize_aoc();
    let input = aoc.input();

    aoc.measure("Parse");
    let galaxies = parse(&input, 2);

    aoc.measure("P1");
    let mut pairs = vec![];
    for (galaxy_index, galaxy) in galaxies.iter().enumerate() {
        for other_galaxy in galaxies.iter().skip(galaxy_index + 1) {
            pairs.push(manhattan_distance(galaxy, other_galaxy));
        }
    }
    let p1 = pairs.iter().sum::<usize>();

    aoc.measure("Parse 2");
    let galaxies = parse(&input, 1_000_000);

    aoc.measure("P2");
    let mut pairs = vec![];
    for (galaxy_index, galaxy) in galaxies.iter().enumerate() {
        for other_galaxy in galaxies.iter().skip(galaxy_index + 1) {
            pairs.push(manhattan_distance(galaxy, other_galaxy));
        }
    }
    let p2 = pairs.iter().sum::<usize>();

    aoc.done();

    tracing::info!("Part 1: {}", p1);
    tracing::info!("Part 2: {}", p2);
}

fn parse(input: &str, expansion: usize) -> Vec<(usize, usize)> {
    let mut row_index: usize = 0;
    let mut galaxies = Vec::with_capacity(input.len() / 8);
    let mut seen_columns = HashSet::new();
    for line in input.trim().lines() {
        let line = line.trim();
        if line.chars().all(|c| c == '.') {
            row_index += expansion;
        } else {
            for (column_index, char) in line.chars().enumerate() {
                if char == '#' {
                    galaxies.push((row_index, column_index));
                    seen_columns.insert(column_index);
                }
            }
            row_index += 1;
        }
    }
    let num_columns = *seen_columns.iter().max().unwrap() + 1;
    let mut column_offsets = (0..num_columns).map(|_| 0).collect::<Vec<_>>();
    for column_index in 1..num_columns {
        if seen_columns.contains(&column_index) {
            column_offsets[column_index] = column_offsets[column_index - 1]
        } else {
            column_offsets[column_index] = column_offsets[column_index - 1] + (expansion - 1)
        }
    }
    for galaxy in &mut galaxies {
        galaxy.1 = galaxy.1 + column_offsets[galaxy.1];
    }
    galaxies
}

use core::ops::{Add, Sub};
#[inline]
fn abs_difference<T>(x: T, y: T) -> T
where
    T: Sub<Output = T> + Ord,
{
    if x < y {
        y - x
    } else {
        x - y
    }
}

#[inline]
fn manhattan_distance<T>((x1, y1): &(T, T), (x2, y2): &(T, T)) -> T
where
    T: Sub<Output = T> + Add<Output = T> + Ord + Copy,
{
    abs_difference(*x2, *x1) + abs_difference(*y2, *y1)
}
