use std::collections::HashMap;

use aoc2023::initialize_aoc;

fn main() {
    let mut aoc = initialize_aoc();
    let input = aoc.input();

    aoc.measure("Parse");
    let mut map = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    let mut animal_position = (0, 0);
    for (row, line) in input.trim().lines().enumerate() {
        for (column, char) in line.trim().chars().enumerate() {
            if char == 'S' {
                animal_position = (row, column);
            }
            map.insert((row, column), char);
            width = width.max(column + 1);
        }
        height = height.max(row + 1);
    }

    aoc.measure("P1");
    let mut prev_position = animal_position.clone();
    let mut steps = 1;
    let mut pipe_positions = Vec::new();
    let mut position: (usize, usize) = walk(&animal_position, &prev_position, &map);
    pipe_positions.push(animal_position);
    pipe_positions.push(position);
    while position != animal_position {
        let prev = position;
        position = walk(&position, &prev_position, &map);
        pipe_positions.push(position);
        prev_position = prev;
        steps += 1;
    }

    let part1 = steps / 2;

    aoc.measure("P2");

    let mut raycast_map = map
        .iter()
        .map(|(pos, c)| {
            if pipe_positions.contains(pos) {
                ((pos.0 * 2, pos.1 * 2), *c)
            } else {
                ((pos.0 * 2 + 1, pos.1 * 2 + 1), '.')
            }
        })
        .collect::<HashMap<_, _>>();

    let translated_pipe_positions = pipe_positions
        .into_iter()
        .map(|(a, b)| (a * 2, b * 2))
        .collect::<Vec<_>>();

    let mut prev_pos = *translated_pipe_positions.last().unwrap();
    for pos in &translated_pipe_positions {
        let difference = (
            pos.0 as isize - prev_pos.0 as isize,
            pos.1 as isize - prev_pos.1 as isize,
        );
        let step = (difference.0 / 2, difference.1 / 2);
        let insert_char = if step.0 == 0 { '-' } else { '|' };
        raycast_map.insert(
            (
                (pos.0 as isize - step.0) as usize,
                (pos.1 as isize - step.1) as usize,
            ),
            insert_char,
        );

        prev_pos = *pos;
    }

    let enclosed_tiles = raycast_map
        .keys()
        .filter(|pos| {
            *raycast_map.get(pos).unwrap() == '.'
                && raycast(*pos, &raycast_map, width * 2, height * 2) % 2 == 1
        })
        .collect::<Vec<_>>();
    let part2 = enclosed_tiles.len();

    // for row in 0..height * 2 {
    //     for column in 0..width * 2 {
    //         let c = raycast_map.get(&(row, column)).unwrap_or(&' ');
    //         let char = if *c == '.' {
    //             if enclosed_tiles.contains(&&(row, column)) {
    //                 'X'
    //             } else {
    //                 '0'
    //             }
    //         } else {
    //             match c {
    //                 '.' => '•',
    //                 '-' => '═',
    //                 'F' => '╔',
    //                 '|' => '║',
    //                 'L' => '╚',
    //                 '7' => '╗',
    //                 'J' => '╝',
    //                 'S' => '?',
    //                 ' ' => ' ',
    //                 _ => panic!("Unknown {c}"),
    //             }
    //             // }
    //         };
    //         print!("{char}");
    //     }
    //     println!();
    // }

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

fn raycast(
    point: &(usize, usize),
    map: &HashMap<(usize, usize), char>,
    width: usize,
    height: usize,
) -> u32 {
    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let rays = directions
        .iter()
        .map(|(d_row, d_col)| {
            let mut hits = 0;
            let mut point = *point;
            loop {
                let char = *map.get(&point).unwrap_or(&' ');
                if (*d_col == 0 && char == '-') || (*d_col != 0 && char == '|') {
                    hits += 1;
                }
                let new_row = point.0 as isize + d_row;
                let new_col = point.1 as isize + d_col;
                if new_row < 0
                    || new_col < 0
                    || new_col >= width as isize
                    || new_row >= height as isize
                {
                    break;
                }
                point = (new_row as usize, new_col as usize);
            }
            hits
        })
        .collect::<Vec<_>>();
    rays.into_iter().min().unwrap()
}
