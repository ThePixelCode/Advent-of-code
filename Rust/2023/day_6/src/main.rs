use std::{env::args, fs::OpenOptions, io::Read, process::exit, rc::Rc};

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

fn parse_input(input: &str) -> Rc<[(u32, u32)]> {
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
                        if let Ok(n) = x.parse::<u32>() {
                            return Some(n);
                        }
                        unreachable!()
                    })
                    .collect::<Vec<u32>>(),
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
                        if let Ok(n) = x.parse::<u32>() {
                            return Some(n);
                        }
                        unreachable!()
                    })
                    .collect::<Vec<u32>>(),
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
        .collect::<Rc<[(u32, u32)]>>()
}

fn search_combinations(time_and_distance: &(u32, u32)) -> usize {
    (0..time_and_distance.0)
        .filter(|x| (x * (time_and_distance.0 - x)) > time_and_distance.1)
        .count()
}

fn solve_part_1(file_content: String) {
    let data = parse_input(&file_content);
    let product = data.iter().map(search_combinations).product::<usize>();
    println!("Products: {}", product);
}

fn solve_part_2(file_content: String) {
    todo!()
}
