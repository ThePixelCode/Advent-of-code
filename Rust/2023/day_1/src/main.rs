use std::env::args;
use std::fs::OpenOptions;
use std::io::Read;

fn main() {
    let args: Vec<String> = args().collect();
    let file_name = &args[1];
    let mut file = OpenOptions::new().read(true).open(file_name).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", &input);
}
