use clap_complete::{generate_to, Shell};
use std::env;
use std::io::Error;
use Shell::*;

include!("src/command.rs");

fn main() -> Result<(), Error> {
    let outdir = match std::env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };
    let mut cmd = make_command();

    for shell in [Bash, Fish, PowerShell, Zsh] {
        let path = generate_to(shell, &mut cmd, "prefix", &outdir)?;
        println!("cargo:warning=completion file is generated: {:?}", path);
    }

    Ok(())
}
