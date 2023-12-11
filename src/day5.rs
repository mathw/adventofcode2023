use crate::daylib::{DayResult, Parts, Result};
use std::{collections::HashSet, ops::Range, str::FromStr};

pub fn day5(input: &str) -> DayResult {
    let r = part1(input)?;
    let r2 = part2(input)?;

    Ok(Parts {
        part1: Box::new(format!("The smallest location is {}", r)),
        part2: Box::new(format!("The smallest expanded location is {}", r2)),
    })
}

#[derive(Clone, Debug)]
struct WeirdMap {
    ranges: Vec<MappedRange>,
}

impl WeirdMap {
    fn new(ranges: Vec<MappedRange>) -> Self {
        Self { ranges }
    }

    fn map(&self, source: Range<u64>) -> HashSet<Range<u64>> {
        let set: HashSet<Range<u64>> = self
            .ranges
            .iter()
            .filter_map(|r| r.map(source.clone()))
            .flatten()
            .collect();

        if set.is_empty() {
            HashSet::from([source])
        } else {
            set
        }
    }
}

#[test]
fn test_weirdmap_mapping() {
    let map = WeirdMap::new(vec![
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
    ]);

    assert_eq!(map.map(3..4), HashSet::from([3..4]));
    assert_eq!(map.map(4..5), HashSet::from([10..11]));
    assert_eq!(map.map(10..11), HashSet::from([20..21]));
    assert_eq!(map.map(15..16), HashSet::from([15..16]));
    assert_eq!(map.map(12..13), HashSet::from([30..31, 22..23]));
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct MappedRange {
    source_start: u64,
    dest_start: u64,
    source_end: u64,
}

impl MappedRange {
    fn map_value(&self, v: u64) -> Option<u64> {
        Some(match self.source_start.cmp(&self.dest_start) {
            std::cmp::Ordering::Less => v + (self.dest_start - self.source_start),
            std::cmp::Ordering::Equal => v,
            std::cmp::Ordering::Greater => v - (self.source_start - self.dest_start),
        })
    }

    fn map(&self, source: Range<u64>) -> Option<HashSet<Range<u64>>> {
        let mut did_map = false;
        let mut ranges = HashSet::new();

        // any unmapped range which exists before us
        if source.start < self.source_start {
            let end = u64::min(source.end, self.source_start);
            ranges.insert(source.start..end);
        }

        // any range where we overlap
        let start = u64::max(source.start, self.source_start);
        let end = u64::min(source.end, self.source_end);
        if start < end {
            did_map = true;
            // now we need to map it into the destination range
            match self.source_start.cmp(&self.dest_start) {
                std::cmp::Ordering::Less => {
                    let offset = self.dest_start - self.source_start;
                    let start = start + offset;
                    let end = end + offset;
                    ranges.insert(start..end);
                }
                std::cmp::Ordering::Equal => {
                    ranges.insert(start..end);
                }
                std::cmp::Ordering::Greater => {
                    let offset = self.source_start - self.dest_start;
                    let start = start - offset;
                    let end = end - offset;
                    ranges.insert(start..end);
                }
            }
        }

        // any unmapped range which exists after us
        if source.end > self.source_end {
            let start = u64::max(self.source_end, source.start);
            ranges.insert(start..source.end);
        }

        if ranges.is_empty() || !did_map {
            None
        } else {
            Some(ranges)
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
    assert_eq!(range.map(3..4), None);
    assert_eq!(range.map(4..5), Some(HashSet::from([10..11])));
    assert_eq!(range.map(7..8), Some(HashSet::from([13..14])));
    assert_eq!(range.map(8..9), None);
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
    fn locations_for_seed(&self, seed: Range<u64>) -> HashSet<Range<u64>> {
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

fn parse_input(input: &str) -> Result<(Vec<u64>, WeirdMaps)> {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let seeds = first_line
        .trim_start_matches("seeds: ")
        .split_ascii_whitespace()
        .map(u64::from_str)
        .collect::<std::result::Result<Vec<u64>, _>>()?;
    lines.next();
    lines.next();

    let mut current_ranges = Vec::new();
    let mut current_maps = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.ends_with(" map:") {
            current_maps.push(WeirdMap::new(current_ranges));
            current_ranges = Vec::new();
            continue;
        }
        let range = parse_range(line)?;
        current_ranges.push(range);
    }

    current_maps.push(WeirdMap::new(current_ranges));

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
    let ranges = seeds
        .into_iter()
        .flat_map(|s| weirdmaps.locations_for_seed(s..s + 1));
    let first_range = ranges.min_by_key(|r| r.start).map(|r| r.start);

    Ok(first_range.ok_or("No location mapped".to_owned())?)
}

fn part2(input: &str) -> Result<u64> {
    let (seeds, weirdmaps) = parse_input(input)?;
    let ranges: Vec<Range<u64>> = seeds
        .chunks_exact(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect();
    println!("{:?}", ranges);
    let range = ranges
        .into_iter()
        .flat_map(|s| weirdmaps.locations_for_seed(s))
        .min_by_key(|r| r.start);
    Ok(range
        .expect("There should at least be a range, right?")
        .start)
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
    let l = weirdmaps.locations_for_seed(79..80);
    assert_eq!(l.len(), 1);
    assert_eq!(l.into_iter().next().unwrap(), 82..83);
}

#[test]
fn test_part2() {
    let r = part2(TEST_INPUT).unwrap();
    assert_eq!(r, 46);
}
