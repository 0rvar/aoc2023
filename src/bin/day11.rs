use std::collections::HashSet;

use aoc2023::initialize_aoc;

fn main() {
    initialize_aoc().bench(|input| solve(input, 1), |input| solve(input, 1_000_000));
}

fn solve(input: &str, expansion: usize) -> u64 {
    let input = input.as_bytes();
    let galaxies = parse(&input, expansion);
    sum_distances(&galaxies) as u64
}

fn sum_distances(galaxies: &[(usize, usize)]) -> usize {
    let mut sum = 0;
    for (galaxy_index, galaxy) in galaxies.iter().enumerate() {
        for other_galaxy in galaxies.iter().skip(galaxy_index + 1) {
            sum += galaxy.0.abs_diff(other_galaxy.0) + galaxy.1.abs_diff(other_galaxy.1);
        }
    }
    sum
}

fn parse(input: &[u8], expansion: usize) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::with_capacity(input.len() / 8);
    let mut seen_columns = HashSet::with_capacity(input.len() / 8 / 8);
    let mut all_empty = true;
    let mut row_index = 0;
    let mut column_index = 0;
    let mut max_column = 0;

    // Parsing with built in row expansion
    for char in input {
        if *char == b'\n' {
            if all_empty {
                row_index += expansion;
            } else {
                row_index += 1;
            }

            all_empty = true;
            column_index = 0;
            continue;
        }
        if *char == b'#' {
            galaxies.push((row_index, column_index));
            seen_columns.insert(column_index);
            all_empty = false;
        }
        column_index += 1;
        max_column = max_column.max(column_index);
    }

    // Columnar expansion
    let num_columns = max_column + 1;
    let mut column_offsets = vec![0; num_columns];
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
