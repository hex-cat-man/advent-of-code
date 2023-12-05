use std::{
    error,
    str::{self, Lines},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Seed {
    seed: usize,
    soil: usize,
    fertilizer: usize,
    water: usize,
    light: usize,
    temperature: usize,
    humidity: usize,
    location: usize,
}

#[derive(Debug)]
struct Range {
    dest_start: usize,
    src_start: usize,
    len: usize,
}

#[derive(Debug)]
struct Ranges(Vec<Range>);

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: Ranges,
    soil_to_fertilizer: Ranges,
    fertilizer_to_water: Ranges,
    water_to_light: Ranges,
    light_to_temperature: Ranges,
    temperature_to_humidity: Ranges,
    humidity_to_location: Ranges,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let seed = include_str!("../input.txt")
        .parse::<Almanac>()?
        .lowest_location();
    println!("Lowest location number: {}", seed.location);
    Ok(())
}

impl Ranges {
    fn get(&self, source: usize) -> usize {
        for range in &self.0 {
            let Range {
                len,
                src_start,
                dest_start,
            } = *range;

            // Much faster than looping the range.
            if source >= src_start && src_start + len > source {
                return if dest_start > src_start {
                    source + (dest_start - src_start)
                } else {
                    source - (src_start - dest_start)
                };
            }
        }

        source
    }
}

impl Almanac {
    fn seeds(&self) -> Vec<Seed> {
        let mut seeds = Vec::with_capacity(self.seeds.len());

        for seed in self.seeds.iter().cloned() {
            let soil = self.seed_to_soil.get(seed);
            let fertilizer = self.soil_to_fertilizer.get(soil);
            let water = self.fertilizer_to_water.get(fertilizer);
            let light = self.water_to_light.get(water);
            let temperature = self.light_to_temperature.get(light);
            let humidity = self.temperature_to_humidity.get(temperature);
            let location = self.humidity_to_location.get(humidity);

            seeds.push(Seed {
                seed,
                soil,
                fertilizer,
                water,
                light,
                temperature,
                humidity,
                location,
            });
        }

        seeds
    }

    fn lowest_location(&self) -> Seed {
        let mut lseed = None;
        for seed in self.seeds() {
            if lseed.is_none() || lseed.is_some_and(|s: Seed| s.location > seed.location) {
                lseed = Some(seed);
            }
        }

        lseed.unwrap()
    }
}

impl str::FromStr for Almanac {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        // Seed numbers.
        let mut seeds = Vec::new();
        let mut chars = lines
            .next()
            .ok_or("missing seeds")?
            .chars()
            .skip("seeds: ".len())
            .peekable();

        while chars.peek().is_some() {
            seeds.push(
                (&mut chars)
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<usize>()?,
            );
        }

        // Maps.
        lines.next().ok_or("expected empty line")?;
        fn parse_seed_map(lines: &mut Lines) -> Result<Ranges, Box<dyn error::Error>> {
            let mut ranges = Vec::new();
            for line in lines {
                if line.is_empty() {
                    break;
                }

                let mut chars = line.chars();

                let destination_range_start = (&mut chars)
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<usize>()?;

                let source_range_start = (&mut chars)
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<usize>()?;

                let range_length = (&mut chars)
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<usize>()?;

                ranges.push(Range {
                    dest_start: destination_range_start,
                    src_start: source_range_start,
                    len: range_length,
                });
            }

            Ok(Ranges(ranges))
        }

        // Seed to Soil map.
        lines.next().ok_or("expected seed-to-soil map")?;
        let seed_to_soil = parse_seed_map(&mut lines)?;

        // Soil to fertilizer map.
        lines.next().ok_or("expected soil-to-fertilizer map")?;
        let soil_to_fertilizer = parse_seed_map(&mut lines)?;

        // Fertilizer to water map.
        lines.next().ok_or("expected fertilizer-to-water map")?;
        let fertilizer_to_water = parse_seed_map(&mut lines)?;

        // Water to light map.
        lines.next().ok_or("expected water-to-light map")?;
        let water_to_light = parse_seed_map(&mut lines)?;

        // Light to temperature map.
        lines.next().ok_or("expected light-to-temperature map")?;
        let light_to_temperature = parse_seed_map(&mut lines)?;

        // Temperature to humidity map.
        lines.next().ok_or("expected temperature-to-humidity map")?;
        let temperature_to_humidity = parse_seed_map(&mut lines)?;

        // Humidity to Location map.
        lines.next().ok_or("expected humidity-to-location map")?;
        let humidity_to_location = parse_seed_map(&mut lines)?;

        Ok(Almanac {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let almanac: Almanac = include_str!("../example.txt").parse().unwrap();
        let seeds = almanac.seeds();

        #[rustfmt::skip]
        assert_eq!(seeds, vec![
            Seed { seed: 79, soil: 81, fertilizer: 81, water: 81, light: 74, temperature: 78, humidity: 78, location: 82 },
            Seed { seed: 14, soil: 14, fertilizer: 53, water: 49, light: 42, temperature: 42, humidity: 43, location: 43 },
            Seed { seed: 55, soil: 57, fertilizer: 57, water: 53, light: 46, temperature: 82, humidity: 82, location: 86 },
            Seed { seed: 13, soil: 13, fertilizer: 52, water: 41, light: 34, temperature: 34, humidity: 35, location: 35 },
        ]);

        #[rustfmt::skip]
        assert_eq!(
            almanac.lowest_location(), 
            Seed { seed: 13, soil: 13, fertilizer: 52, water: 41, light: 34, temperature: 34, humidity: 35, location: 35 },
        );
    }
}
