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

pub fn get_report(line: &str) -> Result<Vec<u32>, ()> {
    line.split(" ").map(|l| l.parse::<u32>().or(Err(()))).collect::<Result<Vec<u32>, ()>>()
}

const TOLERANCE: std::ops::RangeInclusive<u32> = 1..=3;

pub fn is_safe(report: &Vec<u32>) -> bool {
    let mut last_level = None;
    let mut increasing = None;

    for level in report {
        if let Some(last_level) = last_level {
            if !TOLERANCE.contains(&level.abs_diff(last_level)) {
                return false;
            }
            let is_increasing = *level > last_level;

            if let Some(increasing) = increasing {
                if increasing != is_increasing {
                    return false;
                }
            } else {
                increasing = Some(is_increasing);
            }
        }
        last_level = Some(*level);
    }

    true
}
