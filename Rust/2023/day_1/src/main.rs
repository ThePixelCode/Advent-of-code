use std::env::args;
use std::fs::OpenOptions;
use std::io::Read;
use std::process::exit;

const DIGITS: [(u32, &str); 9] = [
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
];

fn found_digits(input: &str) -> u32 {
    let mut digits = input.chars().filter_map(|x| x.to_digit(10));
    let first = digits.nth(0).unwrap();
    if let Some(last) = digits.last() {
        return (first * 10) + last;
    }
    (first * 10) + first
}

fn found_digits_on_string(input: &str) -> u32 {
    let mut numbers = Vec::new();
    if input.len() >= 5 {
        for i in 0..(input.len() - 4) {
            let slice = &input[i..(i + 5)];
            for (return_value, digit) in DIGITS {
                if slice.starts_with(digit) {
                    numbers.push(return_value)
                }
            }
            match slice.chars().nth(0).unwrap().to_digit(10) {
                Some(number) => numbers.push(number),
                None => (),
            }
        }
    }
    if input.len() >= 4 {
        for i in (input.len() - 4)..(input.len() - 3) {
            let slice = &input[i..(i + 4)];
            for (return_value, digit) in DIGITS {
                if slice.starts_with(digit) {
                    numbers.push(return_value)
                }
            }
            match slice.chars().nth(0).unwrap().to_digit(10) {
                Some(number) => numbers.push(number),
                None => (),
            }
        }
    }
    if input.len() >= 3 {
        for i in (input.len() - 3)..(input.len() - 2) {
            let slice = &input[i..(i + 3)];
            for (return_value, digit) in DIGITS {
                if slice.starts_with(digit) {
                    numbers.push(return_value)
                }
            }
            match slice.chars().nth(0).unwrap().to_digit(10) {
                Some(number) => numbers.push(number),
                None => (),
            }
        }
    }
    if input.len() >= 2 {
        for i in (input.len() - 2)..input.len() {
            match input.chars().nth(i).unwrap().to_digit(10) {
                Some(number) => numbers.push(number),
                None => (),
            }
        }
    }
    if input.len() == 1 {
        match input.chars().nth(0).unwrap().to_digit(10) {
            Some(number) => numbers.push(number),
            None => panic!(),
        }
    }
    if numbers.is_empty() {
        panic!()
    } else if numbers.len() == 1 {
        let digit = numbers.first().unwrap();
        return (digit * 10) + digit;
    } else {
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();
        return (first * 10) + last;
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        println!("Usage {} [input file] [method]", &args[0]);
        exit(1);
    }
    let file_name = &args[1];
    let method = args[2].parse::<u32>().unwrap();
    let mut file = OpenOptions::new().read(true).open(file_name).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", &input);
    let mut sum = 0;
    match method {
        1 => {
            for line in input.lines() {
                let found = found_digits(line);
                println!("found: {}", &found);
                sum += found;
            }
            println!("Sum: {}", sum);
        }
        2 => {
            for line in input.lines() {
                let found = found_digits_on_string(line);
                println!("{}", &found);
                sum += found;
            }
            println!("Sum: {}", sum);
        }
        _ => {
            println!("Invalid method only 1 or 2 are valid");
            exit(1);
        }
    }
}
