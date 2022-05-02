mod prefix;
use clap::Parser;
use std::io::{self, Read};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// set delimiter string to print after each FIX field
    #[clap(short, long, default_value = "\n")]
    delimiter: String,

    /// translate common FIX values (for Side: 1 -> Buy)
    #[clap(short, long)]
    value: bool,
}

fn main() {
    let args = Args::parse();

    // stdin is used to allow piping with other commands
    let mut stdin = io::stdin();
    let mut input = String::new();
    stdin
        .read_to_string(&mut input)
        .expect("Could not read input");

    if let Err(x) = prefix::run(input, args.value, args.delimiter) {
        eprintln!("{}", x)
    }
}
