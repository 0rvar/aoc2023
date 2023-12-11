use std::time::Instant;

use easybench::bench;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct Aoc {
    start: Instant,
    measurements: Vec<(std::time::Duration, &'static str)>,
    current_measure: Option<(Instant, &'static str)>,
    reported_already: bool,
    input: String,
}
impl Aoc {
    pub fn input(&self) -> String {
        self.input.clone()
    }
    pub fn measure(&mut self, label: &'static str) {
        self.end_measure();
        self.current_measure = Some((Instant::now(), label));
    }
    fn end_measure(&mut self) {
        if let Some((instant, label)) = self.current_measure.take() {
            self.measurements.push((instant.elapsed(), label));
        }
    }
    pub fn done(&mut self) {
        if self.reported_already {
            return;
        }
        let total_elapsed = self.start.elapsed();
        self.end_measure();
        let mut measurements = Vec::new();
        std::mem::swap(&mut self.measurements, &mut measurements);
        let mut output = String::new();
        for (duration, label) in measurements {
            output.push_str(&format!(
                "{}:{}{}\n",
                label,
                " ".repeat(15 - label.len()),
                humantime::format_duration(duration)
            ));
        }
        output.push_str(&format!(
            "Total elapsed: {}",
            humantime::format_duration(total_elapsed)
        ));
        tracing::warn!("Performance counters\n\n{output}");
        self.reported_already = true;
    }

    pub fn bench<F1, F2>(mut self, part1: F1, part2: F2)
    where
        F1: Fn(&str) -> u64,
        F2: Fn(&str) -> u64,
    {
        let input = self.input();
        self.reported_already = true;
        let part1_answer = part1(&input);
        let part1_timings = bench(|| part1(&input));
        let part2_answer = part2(&input);
        let part2_timings = bench(|| part2(&input));

        tracing::info!("Part 1: {part1_answer}, Part 2: {part2_answer}");
        tracing::warn!("Timings\n\nPart 1\n{part1_timings}\n\nPart 2\n{part2_timings}\n");
    }
}
impl Drop for Aoc {
    fn drop(&mut self) {
        self.done();
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

    tracing::info!("Advent of Code, day {}", day_number);

    let input = fetch_input(day_number);

    Aoc {
        start: Instant::now(),
        current_measure: None,
        measurements: vec![],
        reported_already: false,
        input,
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

pub trait IntegerSquareRoot {
    fn sqrt(self) -> Self;
}
impl IntegerSquareRoot for u64 {
    fn sqrt(self) -> Self {
        (self as f64).sqrt() as Self
    }
}
impl IntegerSquareRoot for i64 {
    fn sqrt(self) -> Self {
        (self as f64).sqrt() as Self
    }
}
impl IntegerSquareRoot for u32 {
    fn sqrt(self) -> Self {
        (self as f64).sqrt() as Self
    }
}
impl IntegerSquareRoot for i32 {
    fn sqrt(self) -> Self {
        (self as f64).sqrt() as Self
    }
}

#[inline]
pub fn quadratic_formula_roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let top = (b * b - 4_f64 * a * c).sqrt();
    let bottom = 2_f64 * a;
    ((-b - top) / bottom, (-b + top) / bottom)
}
