use rayon::prelude::*;
use std::str::Lines;

struct Map {
    dest_start: u64,
    source_start: u64,
    range: u64,
}

struct MapCollection {
    seed_to_soil: Vec<Map>,
    soil_to_fertilizer: Vec<Map>,
    fertilizer_to_water: Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temperature: Vec<Map>,
    temperature_to_humidity: Vec<Map>,
    humidity_to_location: Vec<Map>,
}

impl MapCollection {
    fn new(lines: &mut Lines<'_>) -> Self {
        lines.next(); // Throw away blank line

        let seed_to_soil = map_vec(lines);
        let soil_to_fertilizer = map_vec(lines);
        let fertilizer_to_water = map_vec(lines);
        let water_to_light = map_vec(lines);
        let light_to_temperature = map_vec(lines);
        let temperature_to_humidity = map_vec(lines);
        let humidity_to_location = map_vec(lines);
        Self {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    fn lowest_location(&self, seeds: &[u64]) -> u64 {
        let min_location = seeds
            .par_iter()
            .map(|seed| find_dest(*seed, &self.seed_to_soil))
            .map(|soil| find_dest(soil, &self.soil_to_fertilizer))
            .map(|fertilizer| find_dest(fertilizer, &self.fertilizer_to_water))
            .map(|water| find_dest(water, &self.water_to_light))
            .map(|light| find_dest(light, &self.light_to_temperature))
            .map(|temperature| find_dest(temperature, &self.temperature_to_humidity))
            .map(|humidity| find_dest(humidity, &self.humidity_to_location))
            .min()
            .unwrap();

        min_location
    }
}

impl Map {
    fn new(input: &str) -> Option<Self> {
        if input.is_empty() {
            return None;
        }
        let mut nums = input.split(' ');
        let dest_start = nums.next().unwrap().parse().unwrap();
        let source_start = nums.next().unwrap().parse().unwrap();
        let range = nums.next().unwrap().parse().unwrap();
        Some(Self {
            dest_start,
            source_start,
            range,
        })
    }

    fn get_dest(&self, source: u64) -> u64 {
        assert!(source - self.source_start <= self.range);
        let dest_range = source - self.source_start;

        assert!(self.dest_start + dest_range <= self.dest_start + self.range);

        self.dest_start + dest_range
    }

    fn source_in_range(&self, source: u64) -> bool {
        (self.source_start..(self.source_start + self.range)).contains(&source)
    }
}

fn map_vec(lines: &mut Lines<'_>) -> Vec<Map> {
    lines.next(); // Throw away label
    let mut maps = vec![];

    for line in lines {
        if let Some(map) = Map::new(line) {
            maps.push(map);
        } else {
            break;
        }
    }

    maps
}

fn find_dest(source: u64, dests: &[Map]) -> u64 {
    dests
        .iter()
        .filter_map(|map| {
            if map.source_in_range(source) {
                Some(map.get_dest(source))
            } else {
                None
            }
        })
        .next()
        .unwrap_or(source)
}

pub fn part1(input: &str) {
    let mut lines = input.lines();

    let seed_line = lines.next().unwrap();
    let seeds: Vec<u64> = seed_line
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .map(|num| num.parse().unwrap())
        .collect();

    let maps = MapCollection::new(&mut lines);

    let min_location = maps.lowest_location(&seeds);

    println!("{min_location}")
}

pub fn part2(input: &str) {
    let mut lines = input.lines();

    let seed_ranges: Vec<u64> = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .map(|num| num.parse().unwrap())
        .collect();

    let maps = MapCollection::new(&mut lines);

    let min_location = seed_ranges
        .chunks(2)
        .enumerate()
        .map(|(idx, c)| {
            let base = c[0];
            let ceiling = base + c[1];
            let seeds: Vec<u64> = (base..ceiling).collect::<Vec<_>>();
            println!("Range {idx}, {} seeds", seeds.len());
            maps.lowest_location(&seeds)
        })
        .min()
        .unwrap();

    println!("{min_location}")
}
