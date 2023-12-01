use aoc2023::initialize_aoc;
use rayon::prelude::*;

// This was a Setback.
// I'm making a note here, huge failure.
// It's hard to overstate my disappointment.
// With --release, this is slower than the unoptimized day01. Oh well.

fn main() {
    let aoc = initialize_aoc();
    let input = aoc.input();
    let calibration_numbers: u32 = input
        .trim()
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|c: char| c.to_digit(10));
            let first_digit = digits.next().unwrap();
            return first_digit * 10 + digits.next_back().unwrap_or(first_digit);
        })
        .sum();

    tracing::info!("Part1: {:?}", calibration_numbers);

    let calibration_numbers: Vec<u32> = input
        .trim()
        .lines()
        .par_bridge()
        .map(|line| {
            let mut digits = vec![];
            let chars: Vec<char> = line.chars().collect();
            let mut index = 0;
            while index < line.len() {
                if let Some(digit) = chars[index].to_digit(10) {
                    digits.push(digit);
                }
                if let Some(digit) = find_digit_word(&line[index..]) {
                    digits.push(digit);
                }
                index += 1;
            }

            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .collect();

    tracing::info!("Part2: {:?}", calibration_numbers.into_iter().sum::<u32>());
}

fn find_digit_word(s: &str) -> Option<u32> {
    if s.starts_with("one") {
        return Some(1);
    } else if s.starts_with("two") {
        return Some(2);
    } else if s.starts_with("three") {
        return Some(3);
    } else if s.starts_with("four") {
        return Some(4);
    } else if s.starts_with("five") {
        return Some(5);
    } else if s.starts_with("six") {
        return Some(6);
    } else if s.starts_with("seven") {
        return Some(7);
    } else if s.starts_with("eight") {
        return Some(8);
    } else if s.starts_with("nine") {
        return Some(9);
    }

    return None;
}
