use clap::{Command, Arg};

pub fn build_cli() -> Command<'static> {
    Command::new("prefix")
        .about("A customizable pretty printer for FIX messages")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::new("message")
            .help("FIX message to be parsed. If not provided, will look for a message piped through stdin"))
        .arg(Arg::new("delimiter")
            .long("delimiter")
            .short('d')
            .takes_value(true)
            .default_value("\n")
            .help("Set delimiter string to print after each FIX field"))
        .arg(Arg::new("value")
            .long("value")
            .short('v')
            .help("Translate common FIX values. (for Side: 1 -> Buy)"))
}
