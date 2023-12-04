use std::env::args;
use std::fs::OpenOptions;
use std::io::Read;

fn found_digits(input: &str) -> u32 {
    let mut digits = input.chars().filter_map(|x| x.to_digit(10));
    let first = digits.nth(0).unwrap();
    if let Some(last) = digits.last() {
        return (first * 10) + last;
    }
    (first * 10) + first
}

fn main() {
    let args: Vec<String> = args().collect();
    let file_name = &args[1];
    let mut file = OpenOptions::new().read(true).open(file_name).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", &input);
    let mut sum = 0;
    for line in input.lines() {
        let found = found_digits(line);
        println!("found: {}", &found);
        sum += found;
    }
    println!("Sum: {}", sum)
}
