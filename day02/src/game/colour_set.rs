pub struct ColourSet {
    pub num_blue: u32,
    pub num_green: u32,
    pub num_red: u32,
}

impl ColourSet {
    pub fn new(num_blue: u32, num_green: u32, num_red: u32) -> ColourSet {
        ColourSet {
            num_blue,
            num_green,
            num_red,
        }
    }

    pub fn parse_from_description(set_description: &str) -> ColourSet {
        let parts = set_description.split(", ");

        let mut num_blue = 0;
        let mut num_green = 0;
        let mut num_red = 0;

        for part in parts {
            let mut part = part.split(' ');
            let num = part.next().unwrap().parse::<u32>().unwrap();
            let color = part.next().unwrap();

            match color {
                "blue" => {
                    num_blue = num;
                }
                "green" => {
                    num_green = num;
                }
                "red" => {
                    num_red = num;
                }
                _ => {}
            }
        }

        ColourSet::new(num_blue, num_green, num_red)
    }

    /// Returns the power of this [`ColourSet`].
    /// The power is the product of the number of blue, green and red cubes.
    pub fn power(&self) -> u32 {
        self.num_blue * self.num_green * self.num_red
    }
}

#[cfg(test)]
mod colour_set_tests {
    use super::*;

    #[test]
    fn test_parse_from_description() {
        let set = ColourSet::parse_from_description("3 blue, 4 red");
        assert_eq!(set.num_blue, 3);
        assert_eq!(set.num_green, 0);
        assert_eq!(set.num_red, 4);

        let set = ColourSet::parse_from_description("1 red, 2 green, 6 blue");
        assert_eq!(set.num_blue, 6);
        assert_eq!(set.num_green, 2);
        assert_eq!(set.num_red, 1);

        let set = ColourSet::parse_from_description("2 green");
        assert_eq!(set.num_blue, 0);
        assert_eq!(set.num_green, 2);
        assert_eq!(set.num_red, 0);
    }

    #[test]
    fn test_power() {
        let set = ColourSet::new(3, 4, 5);
        assert_eq!(set.power(), 60);
    }
}
