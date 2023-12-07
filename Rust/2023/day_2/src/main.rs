use std::{env::args, fs::OpenOptions, io::Read, process::exit};

use regex::Regex;

fn main() {
    let args: Vec<String> = args().collect();
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

const PROBLEM_1_VALUES: [u32; 3] = [
    12, // Red
    13, // Green
    14, // Blue
];

#[derive(Debug, PartialEq, Eq)]
enum CubeColors {
    Red,
    Green,
    Blue,
}

impl TryFrom<&str> for CubeColors {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "red" => Ok(CubeColors::Red),
            "green" => Ok(CubeColors::Green),
            "blue" => Ok(CubeColors::Blue),
            _ => Err("Invalid Value"),
        }
    }
}

fn solve_part_1(file_content: String) {
    let games = Regex::new(r"^Game (?P<game>\d+): (?P<sets>.+)$").expect("regex doesn't work :(");
    let sets =
        Regex::new(r"(?P<set>\d+) (?P<color>(?:blue|red|green))").expect("regex doesn't work :(");
    let mut sum = 0_u32;
    for line in file_content.lines() {
        let caps = games.captures(line).unwrap();
        let game = String::from(&caps["game"])
            .parse::<u32>()
            .expect("number expected on game");
        let valid = &caps["sets"]
            .split("; ")
            .map(|set| {
                set.split(", ")
                    .map(|cube| {
                        let cube = sets.captures(cube).unwrap();
                        (
                            String::from(&cube["set"])
                                .parse::<u32>()
                                .expect("number expected on set"),
                            CubeColors::try_from(&cube["color"]).expect("expected RGB color"),
                        )
                    })
                    .all(|cube| match cube.1 {
                        CubeColors::Red => cube.0 <= PROBLEM_1_VALUES[0],
                        CubeColors::Green => cube.0 <= PROBLEM_1_VALUES[1],
                        CubeColors::Blue => cube.0 <= PROBLEM_1_VALUES[2],
                    })
            })
            .all(|set| set);
        if *valid {
            sum += game
        }
    }
    println!("Sum: {}", sum);
}

fn solve_part_2(file_content: String) {
    todo!()
}
