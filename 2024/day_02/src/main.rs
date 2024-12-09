fn main() {
    use clap::Parser;
    let arg = aoc_day_02::Arg::parse();

    let file = aoc_day_02::get_file_contents(arg.file).unwrap();

    let reports = file.lines().map(|line| aoc_day_02::get_report(line)).collect::<Result<Vec<Vec<u32>>, ()>>().unwrap();

    let result = if arg.part == 1 {
        reports.into_iter().filter(|report| aoc_day_02::is_safe(report)).count() as u32
    } else {
        todo!()
    };

    println!("RESULT: {}", result);
}
