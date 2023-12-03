pub struct Subset {
    pub num_blue: u32,
    pub num_green: u32,
    pub num_red: u32,
}

impl Subset {
    pub fn new(num_blue: u32, num_green: u32, num_red: u32) -> Subset {
        Subset {
            num_blue,
            num_green,
            num_red,
        }
    }

    pub fn parse_from_description(subsets_description: &str) -> Subset {
        let parts = subsets_description.split(", ");

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

        Subset::new(num_blue, num_green, num_red)
    }
}
