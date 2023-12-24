use std::{collections::HashMap, env::args, fs::OpenOptions, io::Read, process::exit, rc::Rc};

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() != 3 {
        println!("Usage {} [input file] [method]", &args[0]);
        exit(1);
    }
    let path_file = &args[1];
    let method = args[2].parse::<i64>().unwrap();
    let mut file = OpenOptions::new().read(true).open(path_file).unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();
    match method {
        1 => solve_part_1(file_content),
        2 => solve_part_2(file_content),
        _ => {
            println!("Invalid method, only 1 or 2 are valid methods");
            exit(1);
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Range {
    destination_start: i64,
    source_start: i64,
    lenght: i64,
}
impl Range {
    fn new(souce: i64, destination: i64, lenght: i64) -> Self {
        Range {
            source_start: souce,
            destination_start: destination,
            lenght,
        }
    }

    fn search_on_range(&self, source: &i64) -> Option<i64> {
        if (self.source_start..(self.source_start + self.lenght)).contains(source) {
            return Some(*source + (self.destination_start - self.source_start));
        }
        None
    }

    fn process_range_search(&self, ranges: &mut Vec<(i64, i64)>) -> Vec<(i64, i64)> {
        let mut new_ranges = Vec::new();
        let mut pending_ranges = Vec::new();
        let (range_start, range_end) = (self.source_start, (self.source_start + self.lenght));
        for range in &mut *ranges {
            match range {
                (start, end) if *start >= range_start && *end < range_end => {
                    let range = (
                        self.search_on_range(start).unwrap(),
                        self.search_on_range(end).unwrap(),
                    );
                    new_ranges.push(range)
                }
                (start, end) if *start >= range_start && *end == range_end => {
                    let range = (
                        self.search_on_range(start).unwrap(),
                        (self.destination_start + self.lenght),
                    );
                    new_ranges.push(range)
                }
                (start, end) if *start >= range_start && *start < range_end && *end > range_end => {
                    let range = (
                        self.search_on_range(start).unwrap(),
                        (self.destination_start + self.lenght),
                    );
                    new_ranges.push(range);
                    pending_ranges.push((range_end, *end))
                }
                (start, _) if *start >= range_end => pending_ranges.push(*range),
                (_, end) if *end <= range_start => pending_ranges.push(*range),
                (start, end) if *start < range_start && *end < range_end => {
                    let range = (self.destination_start, self.search_on_range(end).unwrap());
                    new_ranges.push(range);
                    pending_ranges.push((*start, self.source_start))
                }
                (start, end) if *start < range_start && *end == range_end => {
                    let range = (
                        self.destination_start,
                        (self.destination_start + self.lenght),
                    );
                    new_ranges.push(range);
                    pending_ranges.push((*start, self.source_start))
                }
                (start, end) if *start < range_start && *end > range_end => {
                    let range = (
                        self.destination_start,
                        (self.destination_start + self.lenght),
                    );
                    new_ranges.push(range);
                    pending_ranges.push((*start, self.source_start));
                    pending_ranges.push(((self.source_start + self.lenght), *end))
                }
                _ => {
                    dbg!((range_start, range_end));
                    dbg!(range);
                    unreachable!()
                }
            }
        }
        ranges.clear();
        for ele in pending_ranges {
            ranges.push(ele);
        }
        new_ranges
    }
}

impl Into<HashMap<i64, i64>> for Range {
    fn into(self) -> HashMap<i64, i64> {
        let mut map = HashMap::new();
        for index in 0..self.lenght {
            map.entry(self.source_start + index)
                .or_insert(self.destination_start + index);
        }
        map
    }
}

#[derive(Debug)]
struct Map {
    maps: Rc<[Range]>,
    lenght: i64,
}

impl Map {
    fn new(maps: Rc<[Range]>) -> Self {
        let lenght = maps
            .clone()
            .iter()
            .map(|x| x.source_start + x.lenght)
            .max()
            .unwrap_or(0)
            .max(
                maps.clone()
                    .iter()
                    .map(|x| x.destination_start + x.lenght)
                    .max()
                    .unwrap_or(0),
            );
        Map { maps, lenght }
    }

    fn search_on_table(&self, source: i64) -> i64 {
        let map_search = self
            .maps
            .iter()
            .filter_map(|x| x.search_on_range(&source))
            .collect::<Vec<i64>>();
        if map_search.len() == 1 {
            return map_search[0];
        }
        return source;
    }

    fn search_range_on_table(&self, mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
        let mut results = Vec::new();
        for map in self.maps.iter() {
            results.push(map.process_range_search(&mut ranges));
        }
        results.push(ranges);
        results.concat()
    }
}

impl Into<HashMap<i64, i64>> for Map {
    fn into(self) -> HashMap<i64, i64> {
        let mut entire_map = HashMap::new();
        for partial_map in self.maps.iter() {
            entire_map.extend(Into::<HashMap<i64, i64>>::into(*partial_map))
        }
        for index in 0..self.lenght {
            entire_map.entry(index).or_insert(index);
        }
        entire_map
    }
}

fn parse_map_from_str(data: &str) -> (Vec<i64>, Map, Map, Map, Map, Map, Map, Map) {
    enum Fase {
        None,
        SeedToSoil,
        SoilToFertilizer,
        FertilizerToWater,
        WaterToLight,
        LightToTemperature,
        TemperatureToHumidity,
        HumidityToLocation,
    }
    let mut fase = Fase::None;
    let mut seeds = Vec::new();
    let mut seed_to_soil_map = Vec::new();
    let mut soil_to_fertilizer_map = Vec::new();
    let mut fertilizer_to_water_map = Vec::new();
    let mut water_to_light_map = Vec::new();
    let mut light_to_temperature_map = Vec::new();
    let mut temperature_to_humidity_map = Vec::new();
    let mut humidity_to_location_map = Vec::new();
    for line in data.lines() {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("seeds:") {
            line.split_whitespace()
                .map(|x| x.trim())
                .filter_map(|x| {
                    if x.is_empty() {
                        return None;
                    }
                    if x == "seeds:" {
                        return None;
                    }
                    Some(x)
                })
                .map(|x| x.parse::<i64>().unwrap())
                .for_each(|x| seeds.push(x));
            continue;
        }
        match line {
            "seed-to-soil map:" => fase = Fase::SeedToSoil,
            "soil-to-fertilizer map:" => fase = Fase::SoilToFertilizer,
            "fertilizer-to-water map:" => fase = Fase::FertilizerToWater,
            "water-to-light map:" => fase = Fase::WaterToLight,
            "light-to-temperature map:" => fase = Fase::LightToTemperature,
            "temperature-to-humidity map:" => fase = Fase::TemperatureToHumidity,
            "humidity-to-location map:" => fase = Fase::HumidityToLocation,
            map => {
                let map = map
                    .split_whitespace()
                    .filter_map(|x| {
                        if x.trim().is_empty() {
                            return None;
                        }
                        Some(x.trim().parse::<i64>().unwrap())
                    })
                    .collect::<Rc<[i64]>>();
                match fase {
                    Fase::None => panic!(),
                    Fase::SeedToSoil => seed_to_soil_map.push(Range::new(map[1], map[0], map[2])),
                    Fase::SoilToFertilizer => {
                        soil_to_fertilizer_map.push(Range::new(map[1], map[0], map[2]))
                    }
                    Fase::FertilizerToWater => {
                        fertilizer_to_water_map.push(Range::new(map[1], map[0], map[2]))
                    }
                    Fase::WaterToLight => {
                        water_to_light_map.push(Range::new(map[1], map[0], map[2]))
                    }
                    Fase::LightToTemperature => {
                        light_to_temperature_map.push(Range::new(map[1], map[0], map[2]))
                    }
                    Fase::TemperatureToHumidity => {
                        temperature_to_humidity_map.push(Range::new(map[1], map[0], map[2]))
                    }
                    Fase::HumidityToLocation => {
                        humidity_to_location_map.push(Range::new(map[1], map[0], map[2]))
                    }
                }
            }
        }
    }
    let mut seed_to_soil_map = Map::new(seed_to_soil_map.iter().map(|x| *x).collect());
    let mut soil_to_fertilizer_map = Map::new(soil_to_fertilizer_map.iter().map(|x| *x).collect());
    let mut fertilizer_to_water_map =
        Map::new(fertilizer_to_water_map.iter().map(|x| *x).collect());
    let mut water_to_light_map = Map::new(water_to_light_map.iter().map(|x| *x).collect());
    let mut light_to_temperature_map =
        Map::new(light_to_temperature_map.iter().map(|x| *x).collect());
    let mut temperature_to_humidity_map =
        Map::new(temperature_to_humidity_map.iter().map(|x| *x).collect());
    let mut humidity_to_location_map =
        Map::new(humidity_to_location_map.iter().map(|x| *x).collect());
    let binding = [
        seed_to_soil_map.lenght,
        soil_to_fertilizer_map.lenght,
        fertilizer_to_water_map.lenght,
        water_to_light_map.lenght,
        light_to_temperature_map.lenght,
        temperature_to_humidity_map.lenght,
        humidity_to_location_map.lenght,
    ];
    let max_lenght = binding.iter().max().unwrap();
    seed_to_soil_map.lenght = *max_lenght;
    soil_to_fertilizer_map.lenght = *max_lenght;
    fertilizer_to_water_map.lenght = *max_lenght;
    water_to_light_map.lenght = *max_lenght;
    light_to_temperature_map.lenght = *max_lenght;
    temperature_to_humidity_map.lenght = *max_lenght;
    humidity_to_location_map.lenght = *max_lenght;
    return (
        seeds,
        seed_to_soil_map,
        soil_to_fertilizer_map,
        fertilizer_to_water_map,
        water_to_light_map,
        light_to_temperature_map,
        temperature_to_humidity_map,
        humidity_to_location_map,
    );
}

fn solve_part_1(file_content: String) {
    let (
        seeds,
        seed_to_soil_map,
        soil_to_fertilizer_map,
        fertilizer_to_water_map,
        water_to_light_map,
        light_to_temperature_map,
        temperature_to_humidity_map,
        humidity_to_location_map,
    ) = parse_map_from_str(&file_content);
    let solution = seeds
        .iter()
        .map(|x| seed_to_soil_map.search_on_table(*x))
        .map(|x| soil_to_fertilizer_map.search_on_table(x))
        .map(|x| fertilizer_to_water_map.search_on_table(x))
        .map(|x| water_to_light_map.search_on_table(x))
        .map(|x| light_to_temperature_map.search_on_table(x))
        .map(|x| temperature_to_humidity_map.search_on_table(x))
        .map(|x| humidity_to_location_map.search_on_table(x))
        .min()
        .unwrap();
    println!("Solution: {}", solution)
}

fn convert_seeds_from_problem_one_to_two(seeds: Vec<i64>) -> Vec<(i64, i64)> {
    if seeds.len() % 2 != 0 {
        panic!("The lenght of the seeds has no sense");
    }
    let mut new_seeds = Vec::new();
    let mut seed = None;
    for seed_of_range in seeds {
        if let Some(seed_start) = seed {
            new_seeds.push((seed_start, (seed_start + seed_of_range)));
            seed = None;
        } else {
            seed = Some(seed_of_range);
        }
    }
    new_seeds
}

fn solve_part_2(file_content: String) {
    let (
        seeds,
        seed_to_soil_map,
        soil_to_fertilizer_map,
        fertilizer_to_water_map,
        water_to_light_map,
        light_to_temperature_map,
        temperature_to_humidity_map,
        humidity_to_location_map,
    ) = parse_map_from_str(&file_content);
    let seeds = Vec::from([convert_seeds_from_problem_one_to_two(seeds)]);
    let solution = seeds
        .iter()
        .map(|x| seed_to_soil_map.search_range_on_table(x.clone()))
        .map(|x| soil_to_fertilizer_map.search_range_on_table(x))
        .map(|x| fertilizer_to_water_map.search_range_on_table(x))
        .map(|x| water_to_light_map.search_range_on_table(x))
        .map(|x| light_to_temperature_map.search_range_on_table(x))
        .map(|x| temperature_to_humidity_map.search_range_on_table(x))
        .map(|x| humidity_to_location_map.search_range_on_table(x))
        .flat_map(|range| range.iter().map(|(x, _)| *x).collect::<Vec<i64>>())
        .min()
        .unwrap();
    println!("Solution: {}", solution)
}
