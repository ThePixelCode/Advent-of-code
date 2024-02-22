use std::fmt::{Debug, Display};

fn main() {
    use clap::Parser;
    let (mut file, method) = match day_19::Args::parse().get_info() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(1)
        }
    };

    let mut buffer = String::new();
    match std::io::Read::read_to_string(&mut file, &mut buffer) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(1)
        }
    };

    let result = match method {
        1 => solve_method_1::<u64>(buffer),
        2 => solve_method_2::<u64>(buffer),
        _ => unreachable!(),
    };
    println!("{}", result)
}

#[allow(dead_code)]
fn solve_method_1<T>(#[allow(unused_variables)] text: String) -> T
where
    T: From<u64> + Debug + Display,
{
    todo!()
}

#[allow(dead_code)]
fn solve_method_2<T>(#[allow(unused_variables)] text: String) -> T
where
    T: From<u64> + Debug + Display,
{
    todo!()
}
