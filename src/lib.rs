use std::time::Instant;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct Aoc {
    start: Instant,
    input: String,
}
impl Aoc {
    pub fn input(&self) -> String {
        self.input.clone()
    }
}
impl Drop for Aoc {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        tracing::info!("Elapsed: {}", humantime::format_duration(elapsed));
    }
}
pub fn initialize_aoc() -> Aoc {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug,reqwest=warn,hyper=warn".into()),
        ))
        .with(
            tracing_subscriber::fmt::layer()
                .with_span_events(
                    tracing_subscriber::fmt::format::FmtSpan::NEW
                        | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
                )
                .pretty(),
        )
        .init();

    let binary_name = std::env::args().next().unwrap();
    // Match 01 from target/debug/day01
    let day = binary_name
        .split('/')
        .last()
        .expect("Unknown binary name")
        .strip_prefix("day")
        .expect("Unknown binary name");

    let day_number = day
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u8>()
        .expect("Binary is not named dayNN");

    let now = Instant::now();

    tracing::info!("Advent of Code, day {}", day_number);

    Aoc {
        start: now,
        input: fetch_input(day_number),
    }
}

fn fetch_input(day: u8) -> String {
    std::fs::create_dir_all("tmp/input").expect("Failed to create tmp/ directory");
    let input_file = format!("tmp/input/day{:02}.txt", day);

    // If it already exists, return the contents
    if let Ok(input) = std::fs::read_to_string(&input_file) {
        return input;
    }

    tracing::info!("Fetching input for day {:02}", day);

    dotenv::dotenv().ok();
    let session_token = std::env::var("AOC_SESSION").expect("AOC_SESSION not set");

    let url = format!("https://adventofcode.com/2023/day/{}/input", day);
    let client = reqwest::blocking::Client::new();
    let input = client
        .get(&url)
        .header("Cookie", format!("session={}", session_token))
        .send()
        .expect("Failed to send request")
        .text()
        .expect("Failed to get response text");
    if input.contains("Please don't repeatedly request this endpoint before it unlocks!") {
        panic!("Input not available yet");
    }

    std::fs::write(&input_file, &input).expect("Failed to write input file");

    input
}

pub fn create_adjacent_positions<T: num_traits::Num + PartialOrd + Copy>(
    (x, y): (T, T),
) -> Vec<(T, T)> {
    let mut positions = Vec::with_capacity(8);
    if x > T::zero() {
        if !y.is_zero() {
            positions.push((x - T::one(), y - T::one()))
        }
        positions.push((x - T::one(), y));
        positions.push((x - T::one(), y + T::one()));
    }
    if !y.is_zero() {
        positions.push((x, y - T::one()));
        positions.push((x + T::one(), y - T::one()));
    }
    positions.push((x, y + T::one()));
    positions.push((x + T::one(), y));
    positions.push((x + T::one(), y + T::one()));

    positions
}

pub fn create_adjacent_positions_limited<T: num_traits::Num + PartialOrd + Copy>(
    (x, y): (T, T),
    (max_x, max_y): (T, T),
) -> Vec<(T, T)> {
    create_adjacent_positions((x, y))
        .into_iter()
        .filter(|(x, y)| *x <= max_x && *y <= max_y)
        .collect()
}
