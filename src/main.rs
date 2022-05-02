mod prefix;
use clap::Parser;
use std::io::{self, Read};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// FIX message to be parsed. If not provided will look for a message piped through stdin.
    message: Option<String>,
    /// Set delimiter string to print after each FIX field.
    #[clap(short, long, default_value = "\n")]
    delimiter: String,

    /// Translate common FIX values. (for Side: 1 -> Buy)
    #[clap(short, long)]
    value: bool,
}

fn main() {
    let args = Args::parse();

    let fix_message = match args.message {
        Some(msg) => msg,
        None => {
            // stdin is used to allow piping with other commands
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .expect("could not read input");
            input
        }
    };

    if let Err(x) = prefix::run(fix_message, args.value, args.delimiter) {
        eprintln!("{}", x)
    }
}
