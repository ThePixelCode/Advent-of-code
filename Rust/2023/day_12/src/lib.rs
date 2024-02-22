#[derive(Debug, clap::Parser)]
#[command(version, about="Solve advent of code", long_about=None)]
pub struct Args {
    #[arg(short, long)]
    file: String,
    #[arg(short, long, value_parser(1..=2))]
    method: i64,
}

#[derive(Debug, thiserror::Error)]
pub enum CommonErrors {
    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),
}

type CommonError<T> = Result<T, CommonErrors>;

impl Args {
    pub fn get_info(self) -> CommonError<(String, u8)> {
        Ok((get_string_from_file(self.file)?, self.method as u8))
    }
}

pub fn get_string_from_file<P>(path: P) -> CommonError<String>
where
    P: AsRef<std::path::Path>,
{
    use std::io::Read;
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .create_new(false)
        .truncate(false)
        .append(false)
        .open(path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}
