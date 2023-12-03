mod subset;
use subset::Subset;

pub struct Game {
    pub id: u32,
    pub subsets: Vec<Subset>,
}

impl Game {
    pub fn new(id: u32) -> Game {
        Game {
            id,
            subsets: Vec::new(),
        }
    }

    pub fn add_subset(&mut self, subset: Subset) {
        self.subsets.push(subset);
    }

    pub fn parse_from_description(game_description: &str) -> Game {
        // Split line on the colon
        let mut parts = game_description.split(": ");
        let game_id = parts
            .next()
            .unwrap()
            .trim_start_matches("Game ")
            .parse::<u32>()
            .unwrap();

        let mut game = Game::new(game_id);

        let subsets = parts.next().unwrap();
        let subsets = subsets.split("; ").collect::<Vec<_>>();

        for subset in subsets {
            let subset = Subset::parse_from_description(subset);
            game.add_subset(subset);
        }

        game
    }
}
