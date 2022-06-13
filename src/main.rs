mod command;
mod prefix;
use std::io::{self, Read};

fn main() {
    let matches = command::make_command().get_matches();

    let fix_message = match matches.values_of("message") {
        // If multiple messages are provided, simply combine them with |
        Some(msg) => msg.map(|elem| elem.to_string() + "|").collect(),
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
    let strip_flag = matches.is_present("strip");

    if let Err(x) = prefix::run(&fix_message, value_flag, delimiter, strip_flag) {
        eprintln!("{}", x)
    }
}
