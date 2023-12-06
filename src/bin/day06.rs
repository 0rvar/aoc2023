use aoc2023::initialize_aoc;

fn main() {
    let mut aoc = initialize_aoc();
    let input = aoc.input();

    //     let input = "
    // Time:      7  15   30
    // Distance:  9  40  200"
    //         .trim();

    aoc.measure("Parsing");

    let (time, distance) = input.split_once('\n').unwrap();
    let times = time
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());
    let record_distances = distance
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    let races = times.zip(record_distances).collect::<Vec<_>>();

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
}
