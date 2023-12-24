use std::{env::args, fs::OpenOptions, io::Read, process::exit, rc::Rc};

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() != 3 {
        println!("Usage {} [input file] [method]", &args[0]);
        exit(1);
    }
    let path_file = &args[1];
    let method = args[2].parse::<u64>().unwrap();
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

fn parse_input(input: &str) -> Rc<[(u64, u64)]> {
    let mut times = None;
    let mut distances = None;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("Time:") {
            times = Some(
                line.split_whitespace()
                    .skip(1)
                    .filter_map(|x| {
                        if x.is_empty() {
                            return None;
                        }
                        if let Ok(n) = x.parse::<u64>() {
                            return Some(n);
                        }
                        unreachable!()
                    })
                    .collect::<Vec<u64>>(),
            );
            continue;
        }
        if line.starts_with("Distance:") {
            distances = Some(
                line.split_whitespace()
                    .skip(1)
                    .filter_map(|x| {
                        if x.is_empty() {
                            return None;
                        }
                        if let Ok(n) = x.parse::<u64>() {
                            return Some(n);
                        }
                        unreachable!()
                    })
                    .collect::<Vec<u64>>(),
            );
            continue;
        }
        unreachable!()
    }
    let times = times.unwrap();
    let distances = distances.unwrap();
    times
        .iter()
        .zip(distances.iter())
        .map(|(x, y)| (*x, *y))
        .collect::<Rc<[(u64, u64)]>>()
}

fn search_combinations(time_and_distance: &(u64, u64)) -> usize {
    (0..time_and_distance.0)
        .filter(|x| (x * (time_and_distance.0 - x)) > time_and_distance.1)
        .count()
}

fn solve_part_1(file_content: String) {
    let data = parse_input(&file_content);
    let product = data.iter().map(search_combinations).product::<usize>();
    println!("Products: {}", product);
}

fn parse_input_2(input: &str) -> (u64, u64) {
    let mut time = None;
    let mut distance = None;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("Time:") {
            let time_string = line
                .split_whitespace()
                .skip(1)
                .filter(|x| !x.is_empty())
                .collect::<String>();
            if let Ok(number) = time_string.parse::<u64>() {
                time = Some(number);
            }
            continue;
        }
        if line.starts_with("Distance:") {
            let distance_string = line
                .split_whitespace()
                .skip(1)
                .filter(|x| !x.is_empty())
                .collect::<String>();
            if let Ok(number) = distance_string.parse::<u64>() {
                distance = Some(number);
            }
            continue;
        }
        unreachable!()
    }
    (time.unwrap(), distance.unwrap())
}

fn solve_part_2(file_content: String) {
    let data = parse_input_2(&file_content);
    let solution = search_combinations(&data);
    println!("Solution: {}", solution);
}
