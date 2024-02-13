fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        println!("Usage {} [input file] [method]", &args[0]);
        std::process::exit(1);
    }
    let path_file = &args[1];
    let method = args[2].parse::<u32>().unwrap();
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .open(path_file)
        .unwrap();
    let mut file_content = String::new();
    std::io::Read::read_to_string(&mut file, &mut file_content).unwrap();
    let result = match method {
        1 => solve_part_1(file_content),
        2 => solve_part_2(file_content),
        _ => {
            println!("Invalid method, only 1 or 2 are valid methods");
            std::process::exit(1);
        }
    };
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_method_1() {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open("example_1.txt")
            .unwrap();

        let mut buffer = String::new();
        std::io::Read::read_to_string(&mut file, &mut buffer).unwrap();

        let result = solve_part_1(buffer);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_example_2_method_1() {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open("example_2.txt")
            .unwrap();

        let mut buffer = String::new();
        std::io::Read::read_to_string(&mut file, &mut buffer).unwrap();

        let result = solve_part_1(buffer);
        assert_eq!(result, 6);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Moves {
    Left,
    Right,
}

impl Moves {
    fn try_from_str(value: &str) -> Result<std::rc::Rc<[Self]>, ()> {
        value.chars().map(|c| Self::try_from(c)).collect()
    }
}

impl TryFrom<char> for Moves {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

type Map = std::collections::HashMap<String, (String, String)>;

fn navigate_map(moves: &[Moves], map: &Map) -> u32 {
    let mut pointer = "AAA";
    let mut step = 0_usize;
    let mut navigated = 0_u32;
    loop {
        if pointer == "ZZZ" {
            return navigated;
        }
        if moves.len() <= step {
            step = 0;
        }

        let present_move = moves[..][step];
        let (left, right) = map.get(pointer).unwrap();

        match present_move {
            Moves::Left => {
                pointer = left;
                navigated += 1;
                step += 1;
            }
            Moves::Right => {
                pointer = right;
                navigated += 1;
                step += 1;
            }
        }
    }
}

fn solve_part_1(file_content: String) -> u32 {
    let mut path = None;
    let mut nodes = Map::new();
    for line in file_content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if path.is_none() {
            path = Some(String::from(line));
            continue;
        }
        static RE: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
            regex::Regex::new(r"(?P<org>[A-Z]{3}) = \((?P<sl>[A-Z]{3}), (?P<sr>[A-Z]{3})\)")
                .unwrap()
        });
        let matches = RE.captures(line).unwrap();
        let org = &matches["org"];
        let sonl = &matches["sl"];
        let sonr = &matches["sr"];
        nodes.insert(String::from(org), (String::from(sonl), String::from(sonr)));
    }
    let path = path.unwrap();
    let moves = Moves::try_from_str(&path).unwrap();
    navigate_map(moves.as_ref(), &nodes)
}

fn solve_part_2(file_content: String) -> u32 {
    todo!()
}
