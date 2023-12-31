mod game;

use game::Game;

fn main() {
    let puzzle_input = include_str!("../input.txt");

    let games = parse_games(puzzle_input);

    // Part 1
    let set = game::ColourSet::new(14, 13, 12);
    let valid_game_ids_sum = games
        .iter()
        .filter(|game| game.is_set_valid(&set))
        .map(|game| game.id)
        .sum::<u32>();

    println!("Sum of valid game IDs: {}", valid_game_ids_sum);

    // Part 2
    let minimal_sets_power_sum = games
        .iter()
        .map(|game| game.generate_minimal_set().power())
        .sum::<u32>();

    println!("Sum of minimal sets' power: {}", minimal_sets_power_sum);
}

fn parse_games(contents: &str) -> Vec<Game> {
    let mut games = Vec::new();

    for line in contents.lines() {
        let game = Game::parse_from_description(line);
        games.push(game);
    }

    games
}

#[cfg(test)]
mod day02_tests {
    use super::*;

    const GAME_CONTENTS: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_parse_games() {
        let games = parse_games(GAME_CONTENTS);

        assert_eq!(games.len(), 5);

        assert_eq!(games[0].id, 1);
        assert_eq!(games[0].subsets.len(), 3);
        assert_eq!(games[0].subsets[0].num_blue, 3);
        assert_eq!(games[0].subsets[0].num_green, 0);
        assert_eq!(games[0].subsets[0].num_red, 4);
        assert_eq!(games[0].subsets[1].num_blue, 6);
        assert_eq!(games[0].subsets[1].num_green, 2);
        assert_eq!(games[0].subsets[1].num_red, 1);
        assert_eq!(games[0].subsets[2].num_blue, 0);
        assert_eq!(games[0].subsets[2].num_green, 2);
        assert_eq!(games[0].subsets[2].num_red, 0);

        assert_eq!(games[1].id, 2);
        assert_eq!(games[1].subsets.len(), 3);
        assert_eq!(games[1].subsets[0].num_blue, 1);
        assert_eq!(games[1].subsets[0].num_green, 2);
        assert_eq!(games[1].subsets[0].num_red, 0);
        assert_eq!(games[1].subsets[1].num_blue, 4);
        assert_eq!(games[1].subsets[1].num_green, 3);
        assert_eq!(games[1].subsets[1].num_red, 1);
        assert_eq!(games[1].subsets[2].num_blue, 1);
        assert_eq!(games[1].subsets[2].num_green, 1);
        assert_eq!(games[1].subsets[2].num_red, 0);

        assert_eq!(games[2].id, 3);
        assert_eq!(games[2].subsets.len(), 3);
        assert_eq!(games[2].subsets[0].num_blue, 6);
        assert_eq!(games[2].subsets[0].num_green, 8);
        assert_eq!(games[2].subsets[0].num_red, 20);
        assert_eq!(games[2].subsets[1].num_blue, 5);
        assert_eq!(games[2].subsets[1].num_green, 13);
        assert_eq!(games[2].subsets[1].num_red, 4);
        assert_eq!(games[2].subsets[2].num_blue, 0);
        assert_eq!(games[2].subsets[2].num_green, 5);
        assert_eq!(games[2].subsets[2].num_red, 1);

        assert_eq!(games[3].id, 4);
        assert_eq!(games[3].subsets.len(), 3);
        assert_eq!(games[3].subsets[0].num_blue, 6);
        assert_eq!(games[3].subsets[0].num_green, 1);
        assert_eq!(games[3].subsets[0].num_red, 3);
        assert_eq!(games[3].subsets[1].num_blue, 0);
        assert_eq!(games[3].subsets[1].num_green, 3);
        assert_eq!(games[3].subsets[1].num_red, 6);
        assert_eq!(games[3].subsets[2].num_blue, 15);
        assert_eq!(games[3].subsets[2].num_green, 3);
        assert_eq!(games[3].subsets[2].num_red, 14);

        assert_eq!(games[4].id, 5);
        assert_eq!(games[4].subsets.len(), 2);
        assert_eq!(games[4].subsets[0].num_blue, 1);
        assert_eq!(games[4].subsets[0].num_green, 3);
        assert_eq!(games[4].subsets[0].num_red, 6);
        assert_eq!(games[4].subsets[1].num_blue, 2);
        assert_eq!(games[4].subsets[1].num_green, 2);
        assert_eq!(games[4].subsets[1].num_red, 1);
    }

    #[test]
    fn test_valid_games() {
        let games = parse_games(GAME_CONTENTS);

        let set = game::ColourSet::new(14, 13, 12);

        // Only games 1, 2 and 5 are valid for this set
        assert!(games[0].is_set_valid(&set));
        assert!(games[1].is_set_valid(&set));
        assert!(!games[2].is_set_valid(&set));
        assert!(!games[3].is_set_valid(&set));
        assert!(games[4].is_set_valid(&set));
    }

    #[test]
    fn test_minimal_set_generation() {
        let games = parse_games(GAME_CONTENTS)
            .iter()
            .map(|game| game.generate_minimal_set())
            .collect::<Vec<_>>();

        assert_eq!(games[0].num_blue, 6);
        assert_eq!(games[0].num_green, 2);
        assert_eq!(games[0].num_red, 4);

        assert_eq!(games[1].num_blue, 4);
        assert_eq!(games[1].num_green, 3);
        assert_eq!(games[1].num_red, 1);

        assert_eq!(games[2].num_blue, 6);
        assert_eq!(games[2].num_green, 13);
        assert_eq!(games[2].num_red, 20);

        assert_eq!(games[3].num_blue, 15);
        assert_eq!(games[3].num_green, 3);
        assert_eq!(games[3].num_red, 14);

        assert_eq!(games[4].num_blue, 2);
        assert_eq!(games[4].num_green, 3);
        assert_eq!(games[4].num_red, 6);
    }
}
