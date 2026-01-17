use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, stdout},
    path::PathBuf,
};

use clap::Parser;
use payment_engine::process_transactions;

/// In a real project, this would have some amazing readme here.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input CSV file path.
    #[arg(value_name = "FILE")]
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let file = File::open(cli.path)?;
    let reader = BufReader::new(file);
    let output = stdout();
    let handle = output.lock();
    let writer = BufWriter::new(handle);

    process_transactions(reader, writer)?;

    Ok(())
}
