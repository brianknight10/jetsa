use jetsa::convert;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, allow_negative_numbers = true, default_value_t = 0.0)]
    /// Magnetic variation in degrees, positive values indicate east
    pub magnetic_variation: f64,

    /// Name of the output map
    #[arg(short, long)]
    pub name: String,

    #[arg(short, long)]
    /// Path of the FAA XML file
    pub source: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    convert(args.source, args.name, args.magnetic_variation)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}