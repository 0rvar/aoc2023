use std::collections::{HashMap, HashSet};

use aoc2023::{create_adjacent_positions, initialize_aoc};

fn main() {
    let mut aoc = initialize_aoc();
    let input = aoc.input();

    // let input = "
    // ...........
    // .S-------7.
    // .|F-----7|.
    // .||.....||.
    // .||.....||.
    // .|L-7.F-J|.
    // .|..|.|..|.
    // .L--J.L--J.
    // ...........
    // ";
    // let input = "
    // ..........
    // .S------7.
    // .|F----7|.
    // .||....||.
    // .||....||.
    // .|L-7F-J|.
    // .|..||..|.
    // .L--JL--J.
    // ..........
    // ";
    let input = "
    .F----7F7F7F7F-7....
    .|F--7||||||||FJ....
    .||.FJ||||||||L7....
    FJL7L7LJLJ||LJ.L-7..
    L--J.L7...LJS7F-7L7.
    ....F-J..F7FJ|L7L7L7
    ....L7.F7||L7|.L7L7|
    .....|FJLJ|FJ|F7|.LJ
    ....FJL-7.||.||||...
    ....L---J.LJ.LJLJ...
    ";

    aoc.measure("Parse");
    let mut map = HashMap::new();
    let mut animal_position = (0, 0);
    for (row, line) in input.trim().lines().enumerate() {
        for (column, char) in line.trim().chars().enumerate() {
            if char == 'S' {
                animal_position = (row, column);
            }
            map.insert((row, column), char);
        }
    }

    aoc.measure("P1");
    let mut prev_position = animal_position.clone();
    let mut steps = 1;
    let mut pipe_positions = HashSet::new();
    let mut position: (usize, usize) = walk(&animal_position, &prev_position, &map);
    pipe_positions.insert(animal_position);
    pipe_positions.insert(position);
    while position != animal_position {
        let prev = position;
        position = walk(&position, &prev_position, &map);
        pipe_positions.insert(position);
        prev_position = prev;
        steps += 1;
    }

    let part1 = steps / 2;

    aoc.measure("P2");

    let raycast_map = map
        .iter()
        .map(|(pos, c)| {
            if pipe_positions.contains(pos) {
                (*pos, *c)
            } else {
                (*pos, ' ')
            }
        })
        .collect::<HashMap<_, _>>();

    let enclosed_tiles = map
        .keys()
        .filter(|pos| *map.get(pos).unwrap() == '.' && raycast(*pos, &raycast_map) % 2 == 1)
        .collect::<Vec<_>>();
    let part2 = enclosed_tiles.len();

    for (row, line) in input.trim().lines().enumerate() {
        let line = line
            .trim()
            .chars()
            .enumerate()
            .map(|(index, c)| {
                if c == '.' {
                    if enclosed_tiles.contains(&&(row, index)) {
                        'X'
                    } else {
                        ' '
                    }
                // } else if *raycast_map.get(&(row, index)).unwrap() != ' ' {
                // c
                } else {
                    let char = match c {
                        '.' => '•',
                        '-' => '═',
                        'F' => '╔',
                        '|' => '║',
                        'L' => '╚',
                        '7' => '╗',
                        'J' => '╝',
                        'S' => '?',
                        _ => panic!("Unknown {c}"),
                    };
                    char
                }
            })
            .collect::<String>();
        println!("{line}");
    }

    aoc.done();

    tracing::info!("Part 1: {}", part1);
    tracing::info!("Part 2: {}", part2);
}

fn walk(
    position: &(usize, usize),
    previous: &(usize, usize),
    map: &HashMap<(usize, usize), char>,
) -> (usize, usize) {
    let char = map.get(position).unwrap();
    let adjacent = match char {
        'S' => vec![
            ((position.0.saturating_sub(1), position.1), "|F7"),
            ((position.0 + 1, position.1), "|LJ"),
            ((position.0, position.1.saturating_sub(1)), "-FL"),
            ((position.0, position.1 + 1), "-J7"),
        ],
        '-' => vec![
            ((position.0, position.1.saturating_sub(1)), "*"), // left
            ((position.0, position.1 + 1), "*"),               // right
        ],
        '|' => vec![
            ((position.0.saturating_sub(1), position.1), "*"), // up
            ((position.0 + 1, position.1), "*"),               // down
        ],
        'F' => vec![
            ((position.0, position.1 + 1), "*"), // right
            ((position.0 + 1, position.1), "*"), // down
        ],
        '7' => vec![
            ((position.0, position.1.saturating_sub(1)), "*"), // left
            ((position.0 + 1, position.1), "*"),               // down
        ],
        'J' => vec![
            ((position.0, position.1.saturating_sub(1)), "*"), // left
            ((position.0.saturating_sub(1), position.1), "*"), // up
        ],
        'L' => vec![
            ((position.0, position.1 + 1), "*"),               // right
            ((position.0.saturating_sub(1), position.1), "*"), // up
        ],

        c => panic!("Unhandled {c}"),
        // ((position.0.saturating_sub(1), position.1), "S"),
        // ((position.0 + 1, position.1), "S"),
        // ((position.0, position.1.saturating_sub(1)), "S"),
        // ((position.0, position.1 + 1), "S"),
    }
    .iter()
    .filter_map(|(p, accepted_chars)| {
        if p == previous {
            return None;
        }
        let Some(char) = map.get(p) else {
            return None;
        };
        if *accepted_chars != "*" && !accepted_chars.contains(*char) {
            return None;
        }
        return Some(*p);
    })
    .next()
    .unwrap();
    return adjacent;
}

fn raycast(point: &(usize, usize), map: &HashMap<(usize, usize), char>) -> u32 {
    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let rays = directions
        .iter()
        .map(|(d_row, d_col)| {
            let mut hits = 0;
            let mut point = *point;
            while let Some(char) = map.get(&point) {
                if matches!(*char, 'F' | 'J' | '7' | 'L' | 'S') {
                    hits += 1;
                } else if (*d_col == 0 && *char == '-') || (*d_col != 0 && *char == '|') {
                    hits += 2;
                }
                let new_row = (point.0 as isize + d_row);
                let new_col = point.1 as isize + d_col;
                if new_row < 0 || new_col < 0 {
                    break;
                }
                point = (new_row as usize, new_col as usize);
            }
            hits / 2
        })
        .collect::<Vec<_>>();
    if rays.iter().any(|r| *r == 0) {
        return 0;
    }
    // rays.into_iter().max().unwrap()
    rays.into_iter().min().unwrap()
    // rays.into_iter().find(|x| x % 2 == 1).unwrap_or(0)
    // rays.into_iter().find(|x| x % 2 == 1).unwrap_or(0)
}
