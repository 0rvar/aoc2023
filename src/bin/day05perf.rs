use std::ops::RangeInclusive;

use aoc2023::initialize_aoc;

fn main() {
    let mut aoc = initialize_aoc();
    let input = aoc.input();

    // let input = include_str!("day05_ex.txt");

    aoc.measure("Parsing");

    let (goal, rest) = input.split_once("\n\n").unwrap();
    let (_, goal_seeds_line) = goal.split_once(": ").unwrap();
    let goal_seeds = goal_seeds_line
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .map(|x| x..=x)
        .collect::<Vec<_>>();

    let steps = rest
        .split("\n\n")
        .map(|group| {
            let (_, body) = group.trim().split_once("\n").unwrap();
            let transforms = body
                .trim()
                .lines()
                .map(|line| {
                    let mut numbers = line
                        .trim()
                        .split_whitespace()
                        .map(|x| x.parse::<u64>().unwrap());
                    let range_to_start = numbers.next().unwrap();
                    let range_from_start = numbers.next().unwrap();
                    let range_length = numbers.next().unwrap();
                    Transform {
                        from: range_from_start..=(range_from_start + range_length - 1),
                        to: range_to_start,
                    }
                })
                .collect::<Vec<_>>();
            TransformStep(transforms)
        })
        .collect::<Vec<_>>();

    aoc.measure("Part 1");

    let transformed = transform_resources(&steps, goal_seeds);

    let part1 = transformed.iter().map(|r| r.start()).min().unwrap();

    tracing::info!("Part 1: {:?}", part1);

    aoc.measure("Part 2: parse");

    let goal_seeds = goal_seeds_line
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|nums| {
            let start = nums[0];
            let len = nums[1];
            start..=(start + len - 1)
        })
        .collect::<Vec<_>>();

    tracing::debug!("Seeds: {goal_seeds:?}");

    aoc.measure("Part 2");

    let transformed = transform_resources(&steps, goal_seeds);

    let part2 = transformed.iter().map(|x| x.start()).min().unwrap();

    tracing::info!("Part 2: {:?}", part2);
}

struct TransformStep(Vec<Transform>);
struct Transform {
    from: RangeInclusive<u64>,
    to: u64,
}

fn transform_resources(
    steps: &[TransformStep],
    initial_resources: Vec<RangeInclusive<u64>>,
) -> Vec<RangeInclusive<u64>> {
    let mut resources = initial_resources.clone();
    for TransformStep(transforms) in steps {
        let mut index = 0;
        while index < resources.len() {
            let resource = resources[index].clone();
            for Transform { from, to } in transforms {
                let adjust = |range: RangeInclusive<u64>, offset: i64| {
                    ((*range.start() as i64 + offset) as u64)
                        ..=((*range.end() as i64 + offset) as u64)
                };
                if from.contains(&resource.start()) {
                    let offset = *to as i64 - *from.start() as i64;
                    let new_resource = adjust(
                        (*resource.start())..=*resource.end().min(from.end()),
                        offset,
                    );
                    if !from.contains(&(resource.end())) {
                        resources.push((*from.end() + 1)..=*resource.end());
                    }
                    resources[index] = new_resource;
                    break;
                } else if from.contains(&resource.end()) {
                    let offset = *to as i64 - *from.start() as i64;
                    let new_resource = adjust(*from.start()..=*resource.end(), offset);
                    resources.push(*resource.start()..=(*from.start() - 1));
                    resources[index] = new_resource;
                    break;
                } else if is_overlapping(&resource, &from) {
                    let offset = *to as i64 - *from.start() as i64;
                    let new_resource = adjust((*from.start())..=*from.end(), offset);
                    resources.push((*resource.start())..=(*resource.start() - 1));
                    resources.push((*from.end() + 1)..=*resource.end());
                    resources[index] = new_resource;
                    break;
                }
            }
            index += 1;
        }
    }

    resources
}

fn is_overlapping(r1: &RangeInclusive<u64>, r2: &RangeInclusive<u64>) -> bool {
    r1.start().max(r2.start()) <= r1.end().min(r2.end())
}
#[test]
fn test_is_overlapping() {
    assert!(is_overlapping(&(1..=1), &(1..=1)));
    assert!(is_overlapping(&(1..=2), &(1..=2)));
    assert!(is_overlapping(&(1..=2), &(2..=3)));
    assert!(is_overlapping(&(2..=3), &(1..=2)));
    assert!(is_overlapping(&(1..=5), &(2..=3)));
    assert!(is_overlapping(&(2..=3), &(1..=5)));
    assert!(!is_overlapping(&(1..=2), &(3..=4)));
}
