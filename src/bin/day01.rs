use aoc2023::initialize_aoc;

fn main() {
    let input = initialize_aoc();
    let calibration_numbers: u32 = input
        .trim()
        .lines()
        .map(|line| {
            let digits: Vec<_> = line.chars().filter_map(|c: char| c.to_digit(10)).collect();
            return digits.first().unwrap() * 10 + digits.last().unwrap();
        })
        .sum();

    tracing::info!("Part1: {:?}", calibration_numbers);

    let calibration_numbers: Vec<u32> = input
        .trim()
        .lines()
        .map(|line| {
            let mut digits = vec![];
            let chars: Vec<char> = line.chars().collect();
            let mut index = 0;
            while index < line.len() {
                if let Some(digit) = chars[index].to_digit(10) {
                    digits.push(digit);
                }
                if line[index..].starts_with("one") {
                    digits.push(1);
                }
                if line[index..].starts_with("two") {
                    digits.push(2);
                }
                if line[index..].starts_with("three") {
                    digits.push(3);
                }
                if line[index..].starts_with("four") {
                    digits.push(4);
                }
                if line[index..].starts_with("five") {
                    digits.push(5);
                }
                if line[index..].starts_with("six") {
                    digits.push(6);
                }
                if line[index..].starts_with("seven") {
                    digits.push(7);
                }
                if line[index..].starts_with("eight") {
                    digits.push(8);
                }
                if line[index..].starts_with("nine") {
                    digits.push(9);
                }
                index += 1;
            }

            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .collect();

    tracing::info!("Part2: {:?}", calibration_numbers.into_iter().sum::<u32>());
}
