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

    /// Parses a [`Game`] from a description string.
    /// Example: `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green`
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

    /// Returns `true` if the given [`ColourSet`] is valid for this [`Game`].
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

    /// Returns the generate minimal set of this [`Game`].
    pub fn generate_minimal_set(&self) -> ColourSet {
        let mut minimal_set = ColourSet::new(0, 0, 0);

        for subset in &self.subsets {
            minimal_set.num_blue = std::cmp::max(minimal_set.num_blue, subset.num_blue);
            minimal_set.num_green = std::cmp::max(minimal_set.num_green, subset.num_green);
            minimal_set.num_red = std::cmp::max(minimal_set.num_red, subset.num_red);
        }

        minimal_set
    }
}
