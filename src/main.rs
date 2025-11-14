mod args;
mod formats;
mod util;

use args::{Args, Format};
use clap::Parser;
use formats::{rs, c, hex, base64, raw};

fn main() {
    util::banner();

    let args = Args::parse();

    util::info(&format!("Loading {}", args.input));
    let data = util::read_binary(&args.input);
    util::good(&format!("Loaded {} bytes", data.len()));

    util::info(&format!("Writing {} as {:?}", args.output, args.format));

    match args.format {
        Format::Rs      => rs::write(&args.output, &data),
        Format::C       => c::write(&args.output, &data),
        Format::Hex     => hex::write(&args.output, &data),
        Format::HexDump => hex::dump(&args.output, &data),
        Format::B64     => base64::write(&args.output, &data),
        Format::Raw     => raw::write(&args.output, &data),
    }

    util::good("Done");
}
