use aoc2023::initialize_aoc;

fn main() {
    initialize_aoc().bench_split(
        |input| parse(input.as_bytes(), 2),
        sum_distances,
        |input| parse(input.as_bytes(), 1_000_000),
        sum_distances,
    );
}

// For ferris elf benchmarker bot
pub fn run(input: &str) -> i64 {
    solve(input, 2) as i64
    // solve(input, 1_000_000) as i64
}

fn solve(input: &str, expansion: usize) -> u64 {
    let input = input.as_bytes();
    let galaxies = parse(&input, expansion);
    sum_distances(galaxies) as u64
}

fn sum_distances(mut galaxies: Vec<(usize, usize)>) -> usize {
    let num_galaxies = galaxies.len();
    let mut previous = galaxies.last().unwrap();
    let row_sum = previous.0
        + galaxies
            .iter()
            .enumerate()
            .rev()
            .skip(1)
            .map(|(index, galaxy)| {
                let dist_to_next = previous.0 - galaxy.0;
                // Contribution of this point to the total sum of distances
                previous = galaxy;
                galaxy.0 + dist_to_next * index * (num_galaxies - index - 1)
            })
            .sum::<usize>();

    galaxies.sort_unstable_by_key(|x| x.1);
    let mut previous = galaxies.last().unwrap();
    let column_sum = previous.1
        + galaxies
            .iter()
            .enumerate()
            .rev()
            .skip(1)
            .map(|(index, galaxy)| {
                let dist_to_next = previous.1 - galaxy.1;
                previous = galaxy;
                // Contribution of this point to the total sum of distances
                galaxy.1 + dist_to_next * index * (num_galaxies - index - 1)
            })
            .sum::<usize>();

    row_sum + column_sum
}

fn parse(input: &[u8], expansion: usize) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::with_capacity(input.len() / 8);
    let mut column_offsets = [0; 150];
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
            max_column = max_column.max(column_index);
            column_index = 0;
            continue;
        }
        if *char == b'#' {
            galaxies.push((row_index, column_index));
            column_offsets[column_index] = 0;
            all_empty = false;
        }
        column_index += 1;
    }

    // Columnar expansion
    let num_columns = max_column + 1;
    for column_index in 1..num_columns {
        if column_offsets[column_index] == 0 {
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
