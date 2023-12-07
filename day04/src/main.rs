mod scratchcard;

use scratchcard::ScratchCard;

fn main() {
    let puzzle_input = include_str!("../input.txt");

    let mut cards = puzzle_input
        .lines()
        .map(ScratchCard::parse_from_text)
        .collect::<Vec<_>>();

    // Part 1
    let total_score = cards.iter().map(ScratchCard::calculate_score).sum::<u32>();
    println!("Total score: {}", total_score);

    // Part 2
    let total_cards = ScratchCard::calculate_total_scratchcards(&mut cards);
    println!("Total cards after processing winners: {}", total_cards);
}
