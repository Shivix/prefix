mod command;
mod prefix;

use std::io;

fn main() {
    let matches = command::make_command().get_matches();

    let fix_message = match matches.values_of("message") {
        Some(msg) => msg.map(|elem| elem.to_owned() ).collect(),
        None => {
            io::stdin()
                .lines()
                .map(|line| line.unwrap() )
                .collect()
        }
    };
    let delimiter = matches.value_of("delimiter").unwrap();
    let flags = prefix::matches_to_flags(&matches);

    if let Err(x) = prefix::run(&fix_message, delimiter, flags) {
        eprintln!("{}", x)
    }
}
