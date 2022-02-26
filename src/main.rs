mod prefix;
use std::io::{self, Read};
extern crate getopts;
use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!(
        "Usage: {} [options]
        Pretty prints fix messages in a more human readable manner
        Pipe other commands into or out of prefix to use",
        program
    );
    print!("{}", opts.usage(&brief));
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let program = argv[0].clone();

    let mut opts = Options::new();
    opts.optopt(
        "d",
        "delimiter",
        "set delimiter string to print after each FIX fields",
        "STRING",
    );
    opts.optflag(
        "v",
        "value",
        "translate common FIX values (for Side: 1 -> Buy)",
    );
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&argv[1..]) {
        Ok(m) => m,
        Err(f) => std::panic::panic_any(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let mut delimiter = String::from("\n");
    if matches.opt_present("d") {
        delimiter = matches.opt_str("d").expect("bad delimiter string");
    }

    let mut value_flag = false;
    if matches.opt_present("v") {
        value_flag = true;
    }

    // stdin is used to allow piping with other commands
    let mut stdin = io::stdin();
    let mut input = String::new();
    stdin
        .read_to_string(&mut input)
        .expect("Could not read input");

    if let Err(x) = prefix::run(input, value_flag, delimiter) {
        eprintln!("{}", x)
    }
}
