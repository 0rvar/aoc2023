use aoc2023::initialize_aoc;

fn main() {
    let mut aoc = initialize_aoc();
    let input = aoc.input();

    let input = include_str!("day05_ex.txt");

    aoc.measure("Parsing");

    let (goal, rest) = input.split_once("\n\n").unwrap();
    let (_, goal_seeds) = goal.split_once(": ").unwrap();
    let goal_seeds = goal_seeds
        .split_whitespace()
        .map(|x| ("seed", x.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();

    struct Transform<'a> {
        from: &'a str,
        to: &'a str,
        steps: Vec<(u32, u32, u32)>,
    }

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

    let transformed = transforms.iter().fold(goal_seeds, |resources, transform| {
        assert_eq!(resources.first().unwrap().0, transform.from);
        let next_resource_name = transform.to;
        let next_resources = resources
            .iter()
            .map(|(_, id)| {
                for (transform_to_start, transform_from_start, len) in &transform.steps {
                    if transform_from_start <= id && *id < (transform_from_start + len) {
                        let offset = id - transform_from_start;
                        return (next_resource_name, transform_to_start + offset);
                    }
                }
                (next_resource_name, *id)
            })
            .collect::<Vec<_>>();
        next_resources
    });

    let part1 = transformed.iter().min_by_key(|(_, x)| x).unwrap();

    tracing::info!("{:?}", part1);
}
