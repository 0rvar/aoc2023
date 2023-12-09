use aoc2023::initialize_aoc;

fn main() {
    let mut aoc = initialize_aoc();
    let input = aoc.input();

    // let input = "
    //     0 3 6 9 12 15
    //     1 3 6 10 15 21
    //     10 13 16 21 30 45
    // "
    // .trim()
    // .to_string();
    // let input = include_str!("message.txt");

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

    tracing::debug!("{sequences:?}");
    let next_values = sequences
        .iter()
        .map(|sequence| next_value(&sequence))
        .collect::<Vec<_>>();

    aoc.done();

    tracing::info!("Part 1: {}", next_values.iter().sum::<isize>());
    // tracing::info!("Part 2: {part2}");
}

fn next_value(sequence: &[isize]) -> isize {
    let last = *sequence.last().unwrap();
    if last == 0 {
        return last;
    }

    let mut last_differences = vec![];
    let mut prev = sequence[0];
    for current in sequence.iter().skip(1) {
        last_differences.push(*current - prev);
        prev = *current;
    }
    let next_difference = next_value(&last_differences);

    tracing::debug!("Sequence: {sequence:?}");
    tracing::debug!(
        "{} + {} = {}",
        last,
        next_difference,
        last + next_difference,
    );
    last + next_difference
}
