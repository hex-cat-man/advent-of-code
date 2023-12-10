use std::{
    error, io,
    str::{self, Lines},
};

#[cfg(not(feature = "part1"))]
use std::ops;

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
struct MapRange {
    dest_start: usize,
    src_start: usize,
    len: usize,
}

#[derive(Debug)]
struct MapRanges(Vec<MapRange>);

#[derive(Debug)]
struct Almanac {
    #[cfg(feature = "part1")]
    seeds: Vec<usize>,
    #[cfg(not(feature = "part1"))]
    seeds: Vec<ops::Range<usize>>,
    seed_to_soil: MapRanges,
    soil_to_fertilizer: MapRanges,
    fertilizer_to_water: MapRanges,
    water_to_light: MapRanges,
    light_to_temperature: MapRanges,
    temperature_to_humidity: MapRanges,
    humidity_to_location: MapRanges,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let seed = io::stdin()
        .lines()
        .map_while(Result::ok)
        .fold(String::new(), |s, l| s + &l + "\n")
        .parse::<Almanac>()?
        .lowest_location();

    println!("Lowest location number: {}", seed.location);

    Ok(())
}

impl MapRanges {
    fn get(&self, source: usize) -> usize {
        for range in &self.0 {
            let MapRange {
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
    #[cfg(feature = "part1")]
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

    #[cfg(feature = "part1")]
    fn lowest_location(&self) -> Seed {
        let mut lseed = None;
        for seed in self.seeds() {
            if lseed.is_none() || lseed.is_some_and(|s: Seed| s.location > seed.location) {
                lseed = Some(seed);
            }
        }

        lseed.unwrap()
    }

    #[cfg(not(feature = "part1"))]
    fn lowest_location(&self) -> Seed {
        let mut lseed = None;

        for seeds in &self.seeds {
            for seed in seeds.clone() {
                let soil = self.seed_to_soil.get(seed);
                let fertilizer = self.soil_to_fertilizer.get(soil);
                let water = self.fertilizer_to_water.get(fertilizer);
                let light = self.water_to_light.get(water);
                let temperature = self.light_to_temperature.get(light);
                let humidity = self.temperature_to_humidity.get(temperature);
                let location = self.humidity_to_location.get(humidity);

                if lseed.is_some_and(|seed: Seed| seed.location > location) || lseed.is_none() {
                    lseed = Some(Seed {
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

        #[cfg(feature = "part1")]
        while chars.peek().is_some() {
            seeds.push(
                (&mut chars)
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<usize>()?,
            );
        }

        #[cfg(not(feature = "part1"))]
        while chars.peek().is_some() {
            let start = (&mut chars)
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<usize>()?;

            let len = (&mut chars)
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<usize>()?;

            seeds.push(start..(start + len));
        }

        // Maps.
        lines.next().ok_or("expected empty line")?;
        fn parse_seed_map(lines: &mut Lines) -> Result<MapRanges, Box<dyn error::Error>> {
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

                ranges.push(MapRange {
                    dest_start: destination_range_start,
                    src_start: source_range_start,
                    len: range_length,
                });
            }

            Ok(MapRanges(ranges))
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
    #[cfg(feature = "part1")]
    fn example_part_1() {
        let almanac: Almanac = include_str!("../../examples/05.txt").parse().unwrap();
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

    #[test]
    #[cfg(not(feature = "part1"))]
    fn example_part_2() {
        let almanac: Almanac = include_str!("../../examples/05.txt").parse().unwrap();

        #[rustfmt::skip]
        assert_eq!(
            almanac.lowest_location(), 
            Seed { seed: 82, soil: 84, fertilizer: 84, water: 84, light: 77, temperature: 45, humidity: 46, location: 46 },
        );
    }
}
