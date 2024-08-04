mod command;
mod prefix;

use std::io;

fn main() {
    let matches = command::make_command().get_matches();

    let fix_message: Vec<String> = match matches.get_many::<String>("message") {
        Some(msg) => msg.map(|elem| elem.to_owned()).collect(),
        None => io::stdin().lines().map(|line| line.unwrap()).collect(),
    };

    let flags = prefix::matches_to_flags(&matches);
    prefix::run(&fix_message, flags);
}
