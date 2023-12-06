use std::ops::Range;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let input: Vec<_> = input.split("\n\n").collect();
    let seeds = numbers_from_str(input[0].strip_prefix("seeds:").unwrap());
    let seed_mapping = SeedMapping::from_input(&input[1..]);

    seeds
        .iter()
        .map(|&seed| seed_mapping.seed_to_location(seed))
        .min()
        .unwrap()
}

fn numbers_from_str(s: &str) -> Vec<usize> {
    s.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

#[derive(Debug, Default)]
struct SeedMapping {
    seed_to_soil: Vec<(Range<usize>, Range<usize>)>,
    soil_to_fertilizer: Vec<(Range<usize>, Range<usize>)>,
    fertilizer_to_water: Vec<(Range<usize>, Range<usize>)>,
    water_to_light: Vec<(Range<usize>, Range<usize>)>,
    light_to_temperature: Vec<(Range<usize>, Range<usize>)>,
    temperature_to_humidity: Vec<(Range<usize>, Range<usize>)>,
    humidity_to_location: Vec<(Range<usize>, Range<usize>)>,
}

impl SeedMapping {
    fn parse_mapping(input: &str) -> Vec<(Range<usize>, Range<usize>)> {
        let mut mapping = Vec::new();
        for line in input.lines().skip(1) {
            let nums = numbers_from_str(line);
            let (dst_range_start, src_range_start, range_len) = (nums[0], nums[1], nums[2]);
            let dst_range = dst_range_start..(dst_range_start + range_len);
            let src_range = src_range_start..(src_range_start + range_len);
            mapping.push((dst_range, src_range));
        }
        mapping
    }

    pub fn from_input(input: &[&str]) -> Self {
        Self {
            seed_to_soil: Self::parse_mapping(input[0]),
            soil_to_fertilizer: Self::parse_mapping(input[1]),
            fertilizer_to_water: Self::parse_mapping(input[2]),
            water_to_light: Self::parse_mapping(input[3]),
            light_to_temperature: Self::parse_mapping(input[4]),
            temperature_to_humidity: Self::parse_mapping(input[5]),
            humidity_to_location: Self::parse_mapping(input[6]),
        }
    }

    fn map_value_on_ranges(
        ranges: &Vec<(Range<usize>, Range<usize>)>,
        value: usize,
    ) -> Option<usize> {
        for (dst_range, src_range) in ranges {
            if src_range.contains(&value) {
                let offset = value - src_range.start;
                return Some(dst_range.start + offset);
            }
        }
        None
    }

    pub fn seed_to_location(&self, seed: usize) -> usize {
        let mut mapped = seed;
        mapped = Self::map_value_on_ranges(&self.seed_to_soil, mapped).unwrap_or(mapped);
        mapped = Self::map_value_on_ranges(&self.soil_to_fertilizer, mapped).unwrap_or(mapped);
        mapped = Self::map_value_on_ranges(&self.fertilizer_to_water, mapped).unwrap_or(mapped);
        mapped = Self::map_value_on_ranges(&self.water_to_light, mapped).unwrap_or(mapped);
        mapped = Self::map_value_on_ranges(&self.light_to_temperature, mapped).unwrap_or(mapped);
        mapped = Self::map_value_on_ranges(&self.temperature_to_humidity, mapped).unwrap_or(mapped);
        Self::map_value_on_ranges(&self.humidity_to_location, mapped).unwrap_or(mapped)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 35);
    }
}
