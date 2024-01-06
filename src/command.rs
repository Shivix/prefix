use clap::{Arg, Command};

pub fn make_command() -> Command<'static> {
    Command::new("prefix")
        .about("A customizable pretty printer for FIX messages")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::new("message").multiple_occurrences(true).help(
            "FIX message to be parsed, if not provided will look for a message piped through stdin",
        ))
        .arg(
            Arg::new("delimiter")
                .long("delimiter")
                .short('d')
                .takes_value(true)
                .default_value("\n")
                .help("Set delimiter string to print after each FIX field."),
        )
        .arg(
            Arg::new("value")
                .long("value")
                .short('v')
                .help("Translate the values of some tags. (for Side: 1 -> Buy)"),
        )
        .arg(
            Arg::new("strip")
                .long("strip")
                .short('s')
                .help("Strip the whitespace around the = in each field. Less human readable but closer to real FIX."),
        )
        .arg(
            Arg::new("tag")
                .long("tag")
                .short('t')
                .help("Translate numeric tags."),
        )
}
