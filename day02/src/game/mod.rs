mod colour_set;
pub use colour_set::ColourSet;

pub struct Game {
    pub id: u32,
    pub subsets: Vec<ColourSet>,
}

impl Game {
    pub fn new(id: u32) -> Game {
        Game {
            id,
            subsets: Vec::new(),
        }
    }

    pub fn add_subset(&mut self, subset: ColourSet) {
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
            let subset = ColourSet::parse_from_description(subset);
            game.add_subset(subset);
        }

        game
    }
    /**
     * Checks if this game could have been played with the given set of colours.
     */
    pub fn is_set_valid(&self, set: &ColourSet) -> bool {
        for subset in &self.subsets {
            if subset.num_blue > set.num_blue
                || subset.num_green > set.num_green
                || subset.num_red > set.num_red
            {
                return false;
            }
        }

        true
    }
}
