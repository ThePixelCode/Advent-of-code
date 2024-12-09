#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Arg {
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    pub part: u8,
    #[arg(short, long, value_name = "INPUT_FILE")]
    pub file: std::path::PathBuf,
}

pub fn from_str(line: &str) -> Result<(u32, u32), ()> {
    let (left, right) = line.split_once("   ").ok_or(())?;
    Ok((left.parse().or(Err(()))?, right.parse().or(Err(()))?))
}

pub fn compress<T>(vecs: (Vec<T>, Vec<T>)) -> Result<Vec<(T, T)>, ()> {
    if vecs.0.len() != vecs.1.len() {
        return Err(());
    }

    Ok(vecs.0.into_iter().zip(vecs.1.into_iter()).collect())
}

pub fn decompress<T>(vecs: Vec<(T, T)>) -> (Vec<T>, Vec<T>) {
    vecs.into_iter().collect()
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
