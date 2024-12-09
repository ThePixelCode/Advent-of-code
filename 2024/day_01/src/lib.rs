#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Arg {
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    pub part: u8,
    #[arg(short, long, value_name = "INPUT_FILE")]
    pub file: std::path::PathBuf,
}
