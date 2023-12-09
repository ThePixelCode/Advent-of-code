use std::{env::args, fs::OpenOptions, io::Read, process::exit};

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

fn solve_part_1(file_content: String) {
    todo!()
}

fn solve_part_2(file_content: String) {
    todo!()
}
