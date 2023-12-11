use crate::daylib::{DayResult, Parts, Result};
use std::{collections::HashSet, str::FromStr};

pub fn day5(input: &str) -> DayResult {
    let r = part1(input)?;

    Ok(Parts {
        part1: Box::new(format!("The smallest location is {}", r)),
        part2: Box::new(format!("not done yet")),
    })
}
#[derive(Clone, Debug)]
struct WeirdMap {
    ranges: Vec<MappedRange>,
}

impl WeirdMap {
    fn map(&self, source: u64) -> HashSet<u64> {
        let set: HashSet<u64> = self.ranges.iter().filter_map(|r| r.map(source)).collect();
        if set.is_empty() {
            HashSet::from([source])
        } else {
            set
        }
    }
}

#[test]
fn test_weirdmap_mapping() {
    let map = WeirdMap {
        ranges: vec![
            MappedRange {
                source_start: 4,
                source_end: 8,
                dest_start: 10,
            },
            MappedRange {
                source_start: 10,
                source_end: 15,
                dest_start: 20,
            },
            MappedRange {
                source_start: 12,
                source_end: 15,
                dest_start: 30,
            },
        ],
    };

    assert_eq!(map.map(3), HashSet::from([3]));
    assert_eq!(map.map(4), HashSet::from([10]));
    assert_eq!(map.map(10), HashSet::from([20]));
    assert_eq!(map.map(15), HashSet::from([15]));
    assert_eq!(map.map(12), HashSet::from([30, 22]));
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct MappedRange {
    source_start: u64,
    dest_start: u64,
    source_end: u64,
}

impl MappedRange {
    fn map(&self, source: u64) -> Option<u64> {
        if source >= self.source_start && source < self.source_end {
            Some(self.dest_start + (source - self.source_start))
        } else {
            None
        }
    }
}

#[test]
fn test_range_mapping() {
    let range = MappedRange {
        source_start: 4,
        source_end: 8,
        dest_start: 10,
    };
    assert_eq!(range.map(3), None);
    assert_eq!(range.map(4), Some(10));
    assert_eq!(range.map(7), Some(13));
    assert_eq!(range.map(8), None);
}

fn parse_weirdmap(input: &str) -> Result<WeirdMap> {
    let ranges = input
        .lines()
        .map(parse_range)
        .collect::<Result<Vec<MappedRange>>>()?;

    Ok(WeirdMap { ranges })
}

fn parse_range(input: &str) -> Result<MappedRange> {
    let numbers: Vec<u64> = input
        .split_ascii_whitespace()
        .map(u64::from_str)
        .collect::<std::result::Result<Vec<u64>, _>>()?;

    if numbers.len() != 3 {
        Err(format!("Input line '{}' does not contain three u64s", input).into())
    } else {
        Ok(MappedRange {
            source_start: numbers[1],
            dest_start: numbers[0],
            source_end: numbers[1] + numbers[2],
        })
    }
}

#[test]
fn test_parse_range() {
    assert_eq!(
        parse_range("50 98 2").unwrap(),
        MappedRange {
            source_start: 98,
            source_end: 100,
            dest_start: 50
        }
    );
}

#[derive(Debug)]
struct WeirdMaps {
    seed_to_soil: WeirdMap,
    soil_to_fertiliser: WeirdMap,
    fertiliser_to_water: WeirdMap,
    water_to_light: WeirdMap,
    light_to_temperature: WeirdMap,
    temperature_to_humidity: WeirdMap,
    humidity_to_location: WeirdMap,
}

impl WeirdMaps {
    fn locations_for_seed(&self, seed: u64) -> HashSet<u64> {
        println!("Finding location for {}...", seed);
        self.seed_to_soil
            .map(seed)
            .into_iter()
            .flat_map(|i| self.soil_to_fertiliser.map(i))
            .flat_map(|i| self.fertiliser_to_water.map(i))
            .flat_map(|i| self.water_to_light.map(i))
            .flat_map(|i| self.light_to_temperature.map(i))
            .flat_map(|i| self.temperature_to_humidity.map(i))
            .flat_map(|i| self.humidity_to_location.map(i))
            .collect()
    }
}

fn parse_input(input: &str) -> Result<(HashSet<u64>, WeirdMaps)> {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let seeds = first_line
        .trim_start_matches("seeds: ")
        .split_ascii_whitespace()
        .map(u64::from_str)
        .collect::<std::result::Result<HashSet<u64>, _>>()?;
    lines.next();
    lines.next();

    let mut current_ranges = Vec::new();
    let mut current_maps = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.ends_with(" map:") {
            current_maps.push(WeirdMap {
                ranges: current_ranges,
            });
            current_ranges = Vec::new();
            continue;
        }
        let range = parse_range(line)?;
        current_ranges.push(range);
    }

    current_maps.push(WeirdMap {
        ranges: current_ranges,
    });

    if current_maps.len() != 7 {
        return Err(format!(
            "There should have been seven sets of ranges, but I only got {}",
            current_maps.len()
        )
        .into());
    }

    Ok((
        seeds,
        WeirdMaps {
            seed_to_soil: current_maps[0].clone(),
            soil_to_fertiliser: current_maps[1].clone(),
            fertiliser_to_water: current_maps[2].clone(),
            water_to_light: current_maps[3].clone(),
            light_to_temperature: current_maps[4].clone(),
            temperature_to_humidity: current_maps[5].clone(),
            humidity_to_location: current_maps[6].clone(),
        },
    ))
}

fn part1(input: &str) -> Result<u64> {
    let (seeds, weirdmaps) = parse_input(input)?;
    seeds
        .into_iter()
        .flat_map(|s| weirdmaps.locations_for_seed(s))
        .min()
        .ok_or("No locations found".to_owned().into())
}

#[cfg(test)]
static TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[test]
fn test_part1() {
    let r = part1(TEST_INPUT).unwrap();
    assert_eq!(r, 35);
}

#[test]
fn test_first_seed() {
    let (_seeds, weirdmaps) = parse_input(TEST_INPUT).unwrap();
    let l = weirdmaps.locations_for_seed(79);
    assert_eq!(l.len(), 1);
    assert_eq!(l.into_iter().next().unwrap(), 82);
}
