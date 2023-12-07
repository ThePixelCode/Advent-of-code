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
    let games = Regex::new(r"^Game \d+: (?P<sets>.+)$").expect("regex doesn't work :(");
    let sets =
        Regex::new(r"(?P<set>\d+) (?P<color>(?:blue|red|green))").expect("regex doesn't work :(");
    let mut sum = 0_u32;
    for line in file_content.lines() {
        let caps = games.captures(line).unwrap();
        let game: Vec<(u32, u32, u32)> = caps["sets"]
            .split("; ")
            .map(|set| {
                let set: Vec<(u32, CubeColors)> = set
                    .split(", ")
                    .map(|cube| {
                        let cube = sets.captures(cube).unwrap();
                        (
                            String::from(&cube["set"])
                                .parse::<u32>()
                                .expect("number expected on set"),
                            CubeColors::try_from(&cube["color"]).expect("expected RGB color"),
                        )
                    })
                    .collect();
                let mut red: Option<(u32, CubeColors)> = None;
                let mut green: Option<(u32, CubeColors)> = None;
                let mut blue: Option<(u32, CubeColors)> = None;
                for i in set {
                    match i.1 {
                        CubeColors::Red => red = Some(i),
                        CubeColors::Green => green = Some(i),
                        CubeColors::Blue => blue = Some(i),
                    }
                }
                let red = red.unwrap_or((0, CubeColors::Red));
                let green = green.unwrap_or((0, CubeColors::Green));
                let blue = blue.unwrap_or((0, CubeColors::Blue));
                (red.0, green.0, blue.0)
            })
            .collect();
        let mut max_red = 0_u32;
        let mut max_green = 0_u32;
        let mut max_blue = 0_u32;
        for i in game {
            if i.0 > max_red {
                max_red = i.0
            }
            if i.1 > max_green {
                max_green = i.1
            }
            if i.2 > max_blue {
                max_blue = i.2
            }
        }
        sum += max_red * max_green * max_blue;
    }
    println!("Sum: {}", sum);
}
