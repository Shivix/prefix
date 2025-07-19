mod command;
mod prefix;

use std::io;

fn main() {
    let matches = command::make_command().get_matches();
    let flags = prefix::matches_to_flags(&matches);

    // Avoid compiling regexes multiple times.
    let msg_regex = prefix::get_msg_regex();
    let tag_regex = prefix::get_tag_regex();
    let summary_regexes = prefix::get_summary_regexes(&flags);

    if let Some(msgs) = matches.get_many::<String>("message") {
        let n_msgs = msgs.len();
        for (i, msg) in msgs.enumerate() {
            let is_last_line = i + 1 == n_msgs;
            prefix::run(
                msg,
                is_last_line,
                &msg_regex,
                &tag_regex,
                &summary_regexes,
                &flags,
            );
        }
    } else {
        let mut lines = io::stdin().lines().peekable();
        while let Some(line) = lines.next() {
            let line = line.unwrap();
            let is_last_line = lines.peek().is_none();
            prefix::run(
                &line,
                is_last_line,
                &msg_regex,
                &tag_regex,
                &summary_regexes,
                &flags,
            );
        }
    }
}
