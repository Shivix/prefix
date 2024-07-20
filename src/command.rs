use clap::{arg, Arg, ArgAction, Command};

pub fn make_command() -> Command {
    Command::new("prefix")
        .about("A customizable pretty printer for FIX messages")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::new("message").num_args(1..).help(
            "FIX message to be parsed, if not provided will look for a message piped through stdin",
        ))
        .arg(
            arg!(-d --delimiter <delimiter> "Set delimiter string to print after each FIX field.")
                .default_value("\n")
        )
        .arg(
            arg!(-v --value "Translate the values of some tags. (for Side: 1 -> Buy)")
                .action(ArgAction::SetTrue)
        )
        .arg(
            arg!(-s --strip "Strip the whitespace around the = in each field. Less human readable but closer to real FIX.")
                .action(ArgAction::SetTrue)
        )
        .arg(
            arg!(-t --tag "Translate all numbers to tag names whether part of a message or not.")
                .action(ArgAction::SetTrue)
        )
        .arg(
            arg!(-z --summarise [template] "Summarise each fix message based on an optional template.")
                .default_missing_value("")
        )
}
