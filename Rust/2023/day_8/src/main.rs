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

    #[test]
    fn test_example_1_method_2() {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open("example_1.txt")
            .unwrap();

        let mut buffer = String::new();
        std::io::Read::read_to_string(&mut file, &mut buffer).unwrap();

        let result = solve_part_2(buffer);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_example_2_method_2() {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open("example_2.txt")
            .unwrap();

        let mut buffer = String::new();
        std::io::Read::read_to_string(&mut file, &mut buffer).unwrap();

        let result = solve_part_2(buffer);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_example_3_method_2() {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open("example_3.txt")
            .unwrap();

        let mut buffer = String::new();
        std::io::Read::read_to_string(&mut file, &mut buffer).unwrap();

        let result = solve_part_2(buffer);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_lcm() {
        let result = get_lcm(vec![2, 3, 4, 6, 8]);
        assert_eq!(result, 24);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Moves {
    Left,
    Right,
}

impl Moves {
    fn try_from_str(value: &str) -> Result<std::rc::Rc<[Self]>, ()> {
        value.chars().map(Self::try_from).collect()
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

fn navigate_map(moves: &[Moves], map: &Map) -> u64 {
    let mut pointer = "AAA";
    let mut step = 0_usize;
    let mut navigated = 0_u64;
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

fn parse(text: String) -> (std::rc::Rc<[Moves]>, Map) {
    let mut path = None;
    let mut nodes = Map::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if path.is_none() {
            path = Some(String::from(line));
            continue;
        }
        static RE: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
            regex::Regex::new(
                r"(?P<org>[0-9A-Z]{3}) = \((?P<sl>[0-9A-Z]{3}), (?P<sr>[0-9A-Z]{3})\)",
            )
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
    (moves, nodes)
}

fn solve_part_1(file_content: String) -> u64 {
    let (moves, nodes) = parse(file_content);
    navigate_map(moves.as_ref(), &nodes)
}

#[derive(Debug, Clone)]
struct NodeExplorer {
    map: std::rc::Rc<Map>,
    step: usize,
    pointer: String,
    moves: std::rc::Rc<[Moves]>,
}

impl NodeExplorer {
    pub fn new(map: std::rc::Rc<Map>, pointer: String, moves: std::rc::Rc<[Moves]>) -> Self {
        Self {
            map,
            pointer,
            moves,
            step: 0_usize,
        }
    }

    pub fn explore_a_step(mut self) -> (Self, String) {
        if self.moves.len() <= self.step {
            self.step = 0;
        }
        let present_move = self.moves[..][self.step];
        let (left, right) = self.map.get(&self.pointer).unwrap();
        match present_move {
            Moves::Left => self.pointer = left.clone(),
            Moves::Right => self.pointer = right.clone(),
        }
        self.step += 1;
        let result = self.pointer.clone();
        (self, result)
    }
}

struct NodeExplorerManager {
    explorers: Vec<NodeExplorer>,
    navigated: u64,
    lcm: Vec<u64>,
}

impl NodeExplorerManager {
    fn new(map: std::rc::Rc<Map>, moves: std::rc::Rc<[Moves]>) -> Self {
        let mut explorers = Vec::new();
        for key in map.keys() {
            if !key.ends_with('A') {
                continue;
            }
            explorers.push(NodeExplorer::new(map.clone(), key.clone(), moves.clone()));
        }
        Self {
            explorers,
            navigated: 0_u64,
            lcm: Vec::new(),
        }
    }

    pub fn explore(&mut self) -> u64 {
        loop {
            let explorers = self.explorers.clone();
            let (pending, completed) = explorers
                .into_iter()
                .map(|node| node.explore_a_step())
                .map(|(node, result)|{
                    if result.ends_with('Z') {
                        return (None, Some(node));
                    }
                    (Some(node), None)
                })
                    .unzip::<Option<NodeExplorer>, Option<NodeExplorer>, Vec<Option<NodeExplorer>>, Vec<Option<NodeExplorer>>>(
            );
            let completed = completed
                .into_iter()
                .flatten()
                .collect::<Vec<NodeExplorer>>();
            let pending = pending.into_iter().flatten().collect::<Vec<NodeExplorer>>();
            self.explorers = pending;
            for _ in completed {
                self.lcm.push(self.navigated + 1);
            }
            if !self.explorers.is_empty() {
                self.navigated += 1;
            } else {
                println!("{:?}", &self.lcm);
                return get_lcm(self.lcm.clone());
            }
        }
    }
}

pub fn get_lcm(numbers: Vec<u64>) -> u64 {
    let mut numbers = numbers;
    let mut lcm = 2_u64;
    let mut acumulated = 1_u64;
    loop {
        let mut catch = false;
        numbers = numbers
            .into_iter()
            .map(|x| {
                if x % lcm == 0 {
                    catch = true;
                    return x / lcm;
                };
                x
            })
            .filter(|x| x != &1)
            .collect::<Vec<u64>>();
        if catch {
            acumulated *= lcm;
        } else {
            lcm += 1;
        }
        if numbers.is_empty() {
            break;
        }
    }
    acumulated
}

fn solve_part_2(file_content: String) -> u64 {
    let (moves, map) = parse(file_content);
    let mut node_explorer = NodeExplorerManager::new(std::rc::Rc::new(map), moves);
    node_explorer.explore()
}
