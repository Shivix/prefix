mod prefix;
use std::io::{self, Read};

fn main() {
    // stdin is used to allow piping with other commands
    let mut stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).expect("Could not read input");

    match prefix::run(input) {
        Err(x) => eprintln!("{}", x),
        _ => {}
    };
}

