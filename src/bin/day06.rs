use aoc2023::{initialize_aoc, quadratic_formula_roots};

fn main() {
    let mut aoc = initialize_aoc();
    let input = aoc.input();

    aoc.measure("Parsing");

    let (time, distance) = input.split_once('\n').unwrap();
    let times_str = time.split_once(':').unwrap().1;
    let record_distances_str = distance.split_once(':').unwrap().1;

    let times = times_str
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap());
    let record_distances = record_distances_str
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    let races = times.zip(record_distances).collect::<Vec<_>>();

    aoc.measure("Part 1");

    let margin_of_error = races
        .iter()
        .map(|(time, record_distance)| solve(*time, *record_distance))
        .product::<u64>();
    tracing::info!("Part 1: {margin_of_error}");

    aoc.measure("Part 2 parsing");
    let race_time = times_str
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let race_record = record_distances_str
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    aoc.measure("Part 2");

    let num_winning_strategies = solve(race_time, race_record);

    tracing::info!("Part 2: {num_winning_strategies}");
}

fn solve(race_time: u64, record_distance: u64) -> u64 {
    let roots = quadratic_formula_roots(-1_f64, race_time as f64, -(record_distance as f64));
    (roots.0.max(roots.1).floor() - roots.0.min(roots.1).ceil() + 1_f64) as u64
}
