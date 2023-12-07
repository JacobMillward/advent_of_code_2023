#[derive(Debug, Clone)]
pub struct ScratchCard {
    pub id: u32,
    pub winning_numbers: Vec<u8>,
    pub scratch_numbers: Vec<u8>,
    pub instances: u32,
}

impl ScratchCard {
    pub fn parse_from_text(text: &str) -> Self {
        let mut parts = text.split(": ");
        let card_id = parts
            .next()
            .unwrap()
            .trim_start_matches("Card")
            .trim()
            .parse::<u32>()
            .unwrap();

        let numbers = parts.next().unwrap().split(" | ").collect::<Vec<_>>();
        let winning_numbers = numbers[0]
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect::<Vec<_>>();
        let scratch_numbers = numbers[1]
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect::<Vec<_>>();

        ScratchCard {
            id: card_id,
            winning_numbers,
            scratch_numbers,
            instances: 1,
        }
    }

    pub fn calculate_score(&self) -> u32 {
        let mut score = 0;

        for number in &self.scratch_numbers {
            if self.winning_numbers.contains(number) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }

        score
    }

    pub fn calculate_total_scratchcards(cards: &mut [ScratchCard]) -> u32 {
        for index in 0..cards.len() {
            let number_of_winning_numbers =
                cards[index].winning_numbers.iter().fold(0, |acc, number| {
                    if cards[index].scratch_numbers.contains(number) {
                        acc + 1
                    } else {
                        acc
                    }
                });

            if number_of_winning_numbers > 0 {
                let max_index = cards.len() - 1;

                for id in
                    (index + 1..=index + number_of_winning_numbers).filter(|&id| id <= max_index)
                {
                    cards[id].instances += cards[index].instances;
                }
            }
        }

        cards.iter().map(|card| card.instances).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_from_text() {
        let text = "Card 1: 1 2 3 4 5 | 1 2 3 4 5";
        let card = ScratchCard::parse_from_text(text);

        assert_eq!(card.id, 1);
        assert_eq!(card.winning_numbers, vec![1, 2, 3, 4, 5]);
        assert_eq!(card.scratch_numbers, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_scores_cards_correctly() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        let cards = input
            .lines()
            .map(ScratchCard::parse_from_text)
            .collect::<Vec<_>>();

        assert_eq!(cards[0].calculate_score(), 8);
        assert_eq!(cards[1].calculate_score(), 2);
        assert_eq!(cards[2].calculate_score(), 2);
        assert_eq!(cards[3].calculate_score(), 1);
        assert_eq!(cards[4].calculate_score(), 0);
        assert_eq!(cards[5].calculate_score(), 0);
    }

    #[test]
    fn test_calculate_total_scratchcards() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        let mut cards = input
            .lines()
            .map(ScratchCard::parse_from_text)
            .collect::<Vec<_>>();

        let new_cards = ScratchCard::calculate_total_scratchcards(&mut cards);

        assert_eq!(new_cards, 30);
    }
}
