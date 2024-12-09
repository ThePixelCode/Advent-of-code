#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Arg {
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    pub part: u8,
    #[arg(short, long, value_name = "INPUT_FILE")]
    pub file: std::path::PathBuf,
}

pub fn get_file_contents<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<String> {
    use std::io::Read;
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(false)
        .open(path)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    Ok(content)
}

