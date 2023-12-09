use aoc2023::initialize_aoc;

fn main() {
    let mut aoc = initialize_aoc();
    let input = aoc.input();

    aoc.measure("Parse");
    let sequences = input
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    aoc.measure("P1");
    let next_values = sequences
        .iter()
        .map(|sequence| next_value(&sequence))
        .collect::<Vec<_>>();

    aoc.measure("P2");
    let prev_values = sequences
        .iter()
        .map(|sequence| {
            let reversed = sequence.iter().copied().rev().collect::<Vec<_>>();
            next_value(&reversed)
        })
        .collect::<Vec<_>>();

    aoc.done();

    tracing::info!("Part 1: {}", next_values.iter().sum::<isize>());
    tracing::info!("Part 2: {}", prev_values.iter().sum::<isize>());
}

fn next_value(sequence: &[isize]) -> isize {
    let last = *sequence.last().unwrap();
    if sequence.iter().all(|x| *x == 0) {
        return last;
    }

    let mut last_differences = vec![];
    let mut prev = sequence[0];
    for current in sequence.iter().skip(1) {
        last_differences.push(*current - prev);
        prev = *current;
    }
    let next_difference = next_value(&last_differences);
    last + next_difference
}
