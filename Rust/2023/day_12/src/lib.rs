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
    pub fn get_info(self) -> CommonError<(std::fs::File, u8)> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(false)
            .create_new(false)
            .open(self.file)?;
        Ok((file, self.method as u8))
    }
}
