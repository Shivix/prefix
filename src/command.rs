use clap::{arg, Arg, ArgAction, Command};

pub fn make_command() -> Command {
    Command::new("prefix")
        .about("A customizable pretty printer for FIX messages")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::new("message").num_args(1..).help(
            "FIX message to be parsed, if not provided will look for a message piped through stdin",
        ))
        .arg(
            arg!(-c --color <when> "Adds colour to the delimiter and = in for FIX fields, auto will colour only when printing directly into a tty")
                .alias("colour")
                .value_parser(["always", "auto", "never"])
                .default_value("auto"),
        )
        .arg(
            arg!(-d --delimiter <delimiter> "Set delimiter string to print after each FIX field")
                .default_value("\n")
        )
        .arg(
            arg!(-s --strip "Strip the whitespace around the = in each field")
                .action(ArgAction::SetTrue)
        )
        .arg(
            arg!(-S --summary [template] "Summarise each fix message based on an optional template")
                .default_missing_value("")
        )
        .arg(
            arg!(-t --tag "Translate all numbers to tag names whether part of a message or not")
                .action(ArgAction::SetTrue)
        )
        .arg(
            arg!(-v --value "Translate the values of some tags (for Side: 1 -> Buy)")
                .action(ArgAction::SetTrue)
        )
}
