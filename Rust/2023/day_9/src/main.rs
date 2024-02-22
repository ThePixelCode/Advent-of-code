use std::fmt::{Debug, Display};

fn main() {
    use clap::Parser;
    let (mut file, method) = match advent_of_code_day_9::Args::parse().get_info() {
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
    let mut lectures = Vec::new();
    for line in text.lines() {
        lectures.push(
            line.split(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        )
    }

    let prediction = lectures[0].len();

    let result = lectures
        .into_iter()
        .map(|x| {
            let func = advent_of_code_day_9::derivator::get_derived_function(x);
            func(prediction)
        })
        .sum::<i32>();

    T::from(result as u64)
}

#[allow(dead_code)]
fn solve_method_2<T>(#[allow(unused_variables)] text: String) -> T
where
    T: From<u64> + Debug + Display,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn test_example_method_1() {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open("example_1.txt")
            .unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let result = solve_method_1::<u64>(buffer);
        assert_eq!(result, 114);
    }

    #[test]
    fn test_that_problem_1_still_works() {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open("problem_input.txt")
            .unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let result = solve_method_1::<u64>(buffer);
        assert_eq!(result, 1_789_635_132);
    }
}
