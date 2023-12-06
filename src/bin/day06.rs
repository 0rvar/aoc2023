use aoc2023::initialize_aoc;

fn main() {
    let mut aoc = initialize_aoc();
    let input = aoc.input();

    aoc.measure("Parsing");

    let (time, distance) = input.split_once('\n').unwrap();
    let times_str = time.split_once(':').unwrap().1;
    let record_distances_str = distance.split_once(':').unwrap().1;

    let times = times_str
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());
    let record_distances = record_distances_str
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    let races = times.zip(record_distances).collect::<Vec<_>>();

    aoc.measure("Part 1");

    let margin_of_error = races
        .iter()
        .map(|(time, record_distance)| {
            let mut num_winning_strategies = 0;

            for hold_time in 1..*time {
                let speed = hold_time;
                if speed * (time - hold_time) > *record_distance {
                    num_winning_strategies += 1;
                }
            }

            num_winning_strategies
        })
        .product::<u64>();
    tracing::info!("Part 1: {margin_of_error}");

    aoc.measure("Part 2 parsing");
    let race_time = times_str
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let race_record = record_distances_str
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    aoc.measure("Part 2");

    let mut num_winning_strategies = 0;

    for hold_time in 1..race_time {
        let speed = hold_time;
        if speed * (race_time - hold_time) > race_record {
            num_winning_strategies += 1;
        }
    }

    tracing::info!("Part 2: {num_winning_strategies}");
}
