use std::{env::args, fs::OpenOptions, io::Read, process::exit, rc::Rc};

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

#[derive(Debug, PartialEq, Eq)]
struct Number {
    value: u32,
    first_pos: usize,
    last_pos: usize,
    row: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Symbol {
    row: usize,
    col: usize,
}

impl Number {
    fn is_touching_a_symbol(self: &Self, symbol: &Symbol) -> bool {
        let col_from = match self.first_pos {
            0 => self.first_pos,
            i => i - 1,
        };
        let col_to = self.last_pos + 1;
        let row_from = match self.row {
            0 => self.row,
            i => i - 1,
        };
        let row_to = self.row + 1;
        col_from <= symbol.col
            && symbol.col <= col_to
            && row_from <= symbol.row
            && symbol.row <= row_to
    }
}

fn get_numbers_and_symbols(text: &str) -> (Rc<[Symbol]>, Rc<[Number]>) {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    for (row, line) in text.lines().enumerate() {
        let numbers_and_symbols: Vec<(usize, char)> = line
            .chars()
            .enumerate()
            .filter_map(|x| {
                if x.1 == '.' {
                    return None;
                }
                Some(x)
            })
            .collect();
        let mut num = String::new();
        let mut first_num = None;
        let mut last_num = None;
        for i in numbers_and_symbols {
            match i.1 {
                c if c.is_digit(10) => {
                    if first_num.is_none() {
                        first_num = Some(i.0);
                    } else if last_num.unwrap() + 1 != i.0 {
                        numbers.push(Number {
                            value: num.parse::<u32>().unwrap(),
                            first_pos: first_num.unwrap(),
                            last_pos: last_num.unwrap(),
                            row,
                        });
                        first_num = Some(i.0);
                        num.clear();
                    }
                    num.push(c);
                    last_num = Some(i.0);
                }
                _ => {
                    if first_num.is_some() {
                        numbers.push(Number {
                            value: num.parse::<u32>().unwrap(),
                            first_pos: first_num.unwrap(),
                            last_pos: last_num.unwrap(),
                            row,
                        });
                        num.clear();
                        first_num = None;
                        last_num = None;
                    }
                    symbols.push(Symbol { row, col: i.0 });
                }
            }
        }
        if first_num.is_some() {
            numbers.push(Number {
                value: num.parse::<u32>().unwrap(),
                first_pos: first_num.unwrap(),
                last_pos: last_num.unwrap(),
                row,
            });
        }
    }
    return (symbols.into(), numbers.into());
}

fn solve_part_1(file_content: String) {
    let (symbols, numbers) = get_numbers_and_symbols(&file_content);
    let sum: u32 = numbers
        .iter()
        .filter_map(|number| {
            if symbols
                .iter()
                .any(|symbol| number.is_touching_a_symbol(&symbol))
            {
                return Some(number.value);
            }
            None
        })
        .sum();
    println!("Sum: {}", sum)
}

fn solve_part_2(file_content: String) {
    todo!()
}
