use std::{env::args, fs::OpenOptions, io::Read, process::exit, rc::Rc};

use once_cell::sync::Lazy;
use regex::Regex;

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

fn extract_numbers_from_str(line: &str) -> (Rc<[u32]>, Rc<[u32]>) {
    static NUMBERS: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^Card +\d+: (?P<numbers>.+)$").expect("Regex creation has failed")
    });
    let caps = NUMBERS.captures(line).expect("No matches found");
    let numbers = caps["numbers"]
        .split("|")
        .map(|x| x.trim())
        .map(|number_list| {
            number_list
                .split(" ")
                .filter_map(|x| {
                    if !x.trim().is_empty() {
                        return Some(x.trim());
                    }
                    None
                })
                .map(|number| number.parse::<u32>().unwrap())
                .collect::<Rc<[u32]>>()
        })
        .collect::<Vec<Rc<[u32]>>>();
    let mut winning_numbers = None;
    let mut having_numbers = None;
    for i in numbers {
        if winning_numbers.is_none() {
            winning_numbers = Some(i);
        } else {
            having_numbers = Some(i);
        }
    }
    (winning_numbers.unwrap(), having_numbers.unwrap())
}

fn solve_part_1(file_content: String) {
    let sum = file_content
        .lines()
        .map(|line| extract_numbers_from_str(line))
        .map(|(winning, having)| {
            having
                .iter()
                .filter_map(|x| {
                    if winning.iter().any(|y| x == y) {
                        return Some(x);
                    }
                    None
                })
                .collect::<Rc<[&u32]>>()
                .len() as u32
        })
        .filter(|x| *x > 0)
        .map(|x| 2_u32.pow(x - 1))
        .sum::<u32>();
    println!("Sum: {}", sum);
}

fn solve_part_2(file_content: String) {
    todo!()
}
