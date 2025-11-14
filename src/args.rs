use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "binx",
    version = "1.0",
    author = "M4c4r0n1",
    about = "Convert binary files into code-friendly formats"
)]
pub struct Args {
    /// Input binary file
    #[arg(short, long)]
    pub input: String,

    /// Output file name
    #[arg(short, long)]
    pub output: String,

    /// Output format
    #[arg(short, long, value_enum)]
    pub format: Format,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Format {
    Rs,
    C,
    Hex,
    HexDump,
    B64,
    Raw,
}
