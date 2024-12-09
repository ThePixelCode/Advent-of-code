fn main() {
    use clap::Parser;
    let arg = aoc_day_01::Arg::parse();

    let file = aoc_day_01::get_file_contents(arg.file).unwrap();

    let pairs = file.lines().map(|line|aoc_day_01::from_str(line).unwrap()).collect::<Vec<(u32,u32)>>();

    let result = if arg.part == 1 {
        let mut pairs = aoc_day_01::decompress(pairs);

        pairs.0.sort();
        pairs.1.sort();

        aoc_day_01::compress(pairs).unwrap().into_iter().map(|(a, b)| a.abs_diff(b)).sum::<u32>()
    } else {
        let pairs = aoc_day_01::decompress(pairs);

        pairs.0.into_iter().map(|l| l * (pairs.1.iter().filter(|r| l == **r).count() as u32)).sum::<u32>()
    };

    println!("RESULT: {}", result);
}
