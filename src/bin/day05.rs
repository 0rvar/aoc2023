use aoc2023::initialize_aoc;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let mut aoc = initialize_aoc();
    let input = aoc.input();

    // let input = include_str!("day05_ex.txt");

    aoc.measure("Parsing");

    let (goal, rest) = input.split_once("\n\n").unwrap();
    let (_, goal_seeds_line) = goal.split_once(": ").unwrap();
    let goal_seeds = goal_seeds_line
        .split_whitespace()
        .map(|x| ("seed", x.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();

    let transforms = rest
        .split("\n\n")
        .map(|group| {
            let (header, body) = group.trim().split_once("\n").unwrap();
            let (from, to) = header
                .split_once(" ")
                .unwrap()
                .0
                .split_once("-to-")
                .unwrap();
            let transforms = body
                .trim()
                .lines()
                .map(|line| {
                    let mut numbers = line
                        .trim()
                        .split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap());
                    let range_from_start = numbers.next().unwrap();
                    let range_to_start = numbers.next().unwrap();
                    let range_length = numbers.next().unwrap();
                    (range_from_start, range_to_start, range_length)
                })
                .collect::<Vec<_>>();
            Transform {
                from,
                to,
                steps: transforms,
            }
        })
        .collect::<Vec<_>>();

    aoc.measure("Part 1");

    let transformed = transform_resources(&transforms, goal_seeds);
    let part1 = transformed.iter().min_by_key(|(_, x)| x).unwrap();

    tracing::info!("Part 1: {:?}", part1);

    aoc.measure("Part 2: parse");

    let goal_seeds = goal_seeds_line
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .flat_map(|nums| {
            let start = nums[0];
            let len = nums[1];
            (start..(start + len)).map(|id| ("seed", id))
        })
        .collect::<Vec<_>>();
    tracing::debug!("Number of seeds: {}", goal_seeds.len());

    aoc.measure("Part 2");
    let transformed = transform_resources(&transforms, goal_seeds);
    let part2 = transformed.iter().min_by_key(|(_, x)| x).unwrap();
    tracing::info!("Part 2: {:?}", part2);
}

struct Transform<'a> {
    from: &'a str,
    to: &'a str,
    steps: Vec<(u32, u32, u32)>,
}

fn transform_resources<'a>(
    transforms: &'a [Transform],
    initial_resources: Vec<(&'a str, u32)>,
) -> Vec<(&'a str, u32)> {
    initial_resources
        .par_iter()
        .map(|resource| {
            transforms
                .iter()
                .fold(*resource, |(resource_name, id), transform| {
                    assert_eq!(resource_name, transform.from);
                    let next_resource_name = transform.to;
                    for (transform_to_start, transform_from_start, len) in &transform.steps {
                        if *transform_from_start <= id && id < (transform_from_start + len) {
                            let offset = id - transform_from_start;
                            return (next_resource_name, transform_to_start + offset);
                        }
                    }
                    (next_resource_name, id)
                })
        })
        .collect::<Vec<_>>()
}
