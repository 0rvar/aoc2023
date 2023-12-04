use aoc2023::initialize_aoc;

fn main() {
    let mut aoc = initialize_aoc();
    let input = aoc.input();

    aoc.measure("Parsing");

    #[derive(Debug)]
    struct Scatch {
        ticket_number: u32,
        winning_numbers: Vec<u32>,
        ticket_numbers: Vec<u32>,
    }

    let games = input
        .lines()
        .map(|line| {
            let parts = line.split_once(": ").unwrap();
            let ticket_number: u32 = parts.0.split(" ").last().unwrap().parse().unwrap();
            let mut winning_numbers = Vec::new();
            let mut ticket_numbers = Vec::new();
            let (winning, ticket) = parts.1.split_once(" | ").unwrap();
            for num in winning.split_whitespace() {
                winning_numbers.push(num.parse().unwrap());
            }
            for num in ticket.split_whitespace() {
                ticket_numbers.push(num.parse().unwrap());
            }
            Scatch {
                ticket_number,
                winning_numbers,
                ticket_numbers,
            }
        })
        .collect::<Vec<_>>();

    let winning_numbers = games
        .iter()
        .map(|game| {
            let mut num_winning = 0;
            for num in &game.ticket_numbers {
                if game.winning_numbers.contains(num) {
                    num_winning += 1;
                }
            }
            num_winning
        })
        .collect::<Vec<_>>();

    tracing::info!(
        "Part 1: {:#?}",
        winning_numbers
            .iter()
            .map(|num| if *num > 0 { 2_u32.pow(*num - 1) } else { 0 })
            .sum::<u32>()
    );

    let mut number_of_tickets = vec![1u32; winning_numbers.len()];
    for (index, score) in winning_numbers.iter().enumerate() {
        let num_copies = number_of_tickets[index];
        for index in (index + 1)..=(index + (*score as usize)) {
            if index < number_of_tickets.len() {
                number_of_tickets[index] += num_copies;
            }
        }
    }

    tracing::info!("Part 2: {:#?}", number_of_tickets.iter().sum::<u32>());
}
