#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum InfoType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl InfoType {
    pub fn from_str(text: &str) -> Self {
        match text.to_lowercase().as_str() {
            "seed" => InfoType::Seed,
            "soil" => InfoType::Soil,
            "fertilizer" => InfoType::Fertilizer,
            "water" => InfoType::Water,
            "light" => InfoType::Light,
            "temperature" => InfoType::Temperature,
            "humidity" => InfoType::Humidity,
            "location" => InfoType::Location,
            _ => panic!("Invalid info type: {}", text),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct InfoMapping {
    pub ranges: Vec<(usize, usize, usize)>,
}

impl InfoMapping {
    pub fn transform(&self, value: usize) -> usize {
        for (dst_range_start, src_range_start, range_len) in &self.ranges {
            if (value < *src_range_start) || (value >= *src_range_start + *range_len) {
                continue;
            }

            let index = value - src_range_start;

            return dst_range_start + index;
        }

        value
    }
}
