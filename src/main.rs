mod prefix;
mod command;
use std::io::{self, Read};

fn main() {
    let matches = command::build_cli().get_matches();

    let fix_message = match matches.value_of("message") {
        Some(msg) => msg.to_string(),
        None => {
            // stdin is used to allow piping with other commands
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .expect("cannot read input");
            input
        }
    };
    let delimiter = matches.value_of("delimiter").unwrap();
    let value_flag = matches.is_present("value");

    if let Err(x) = prefix::run(&fix_message, value_flag, delimiter) {
        eprintln!("{}", x)
    }
}
