use std::{collections::HashMap, env::args, fs::OpenOptions, io::Read, process::exit, rc::Rc};

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() != 3 {
        println!("Usage {} [input file] [method]", &args[0]);
        exit(1);
    }
    let path_file = &args[1];
    let method = args[2].parse::<u32>().unwrap();
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
    destination_start: u64,
    source_start: u64,
    lenght: u64,
}
impl Range {
    fn new(souce: u64, destination: u64, lenght: u64) -> Self {
        Range {
            source_start: souce,
            destination_start: destination,
            lenght,
        }
    }
}

impl Into<HashMap<u64, u64>> for Range {
    fn into(self) -> HashMap<u64, u64> {
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
    lenght: u64,
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
}

impl Into<HashMap<u64, u64>> for Map {
    fn into(self) -> HashMap<u64, u64> {
        let mut entire_map = HashMap::new();
        for partial_map in self.maps.iter() {
            entire_map.extend(Into::<HashMap<u64, u64>>::into(*partial_map))
        }
        for index in 0..self.lenght {
            entire_map.entry(index).or_insert(index);
        }
        entire_map
    }
}

fn parse_map_from_str(data: &str) -> (Vec<u64>, Map, Map, Map, Map, Map, Map, Map) {
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
                .map(|x| x.parse::<u64>().unwrap())
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
                        Some(x.trim().parse::<u64>().unwrap())
                    })
                    .collect::<Rc<[u64]>>();
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
    let seed_to_soil_map = Into::<HashMap<u64, u64>>::into(seed_to_soil_map);
    let soil_to_fertilizer_map = Into::<HashMap<u64, u64>>::into(soil_to_fertilizer_map);
    let fertilizer_to_water_map = Into::<HashMap<u64, u64>>::into(fertilizer_to_water_map);
    let water_to_light_map = Into::<HashMap<u64, u64>>::into(water_to_light_map);
    let light_to_temperature_map = Into::<HashMap<u64, u64>>::into(light_to_temperature_map);
    let temperature_to_humidity_map = Into::<HashMap<u64, u64>>::into(temperature_to_humidity_map);
    let humidity_to_location_map = Into::<HashMap<u64, u64>>::into(humidity_to_location_map);
    let solution = seeds
        .iter()
        .map(|x| seed_to_soil_map.get(x).unwrap())
        .map(|x| soil_to_fertilizer_map.get(x).unwrap())
        .map(|x| fertilizer_to_water_map.get(x).unwrap())
        .map(|x| water_to_light_map.get(x).unwrap())
        .map(|x| light_to_temperature_map.get(x).unwrap())
        .map(|x| temperature_to_humidity_map.get(x).unwrap())
        .map(|x| humidity_to_location_map.get(x).unwrap())
        .min()
        .unwrap();
    println!("Solution: {}", solution)
}

fn solve_part_2(file_content: String) {
    todo!()
}
