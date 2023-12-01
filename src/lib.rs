use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn initialize_aoc() -> String {
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

    fetch_input(day_number)
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
