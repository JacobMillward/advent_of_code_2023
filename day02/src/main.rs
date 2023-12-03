mod game;

use game::Game;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a file path as the first argument");
        return;
    }

    let file_path = &args[1];

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let games = parse_games(&contents);
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

    #[test]
    fn test_parse_games() {
        let game_contents = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        let games = parse_games(game_contents);

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
}
