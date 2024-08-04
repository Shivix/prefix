use clap_complete::{generate_to, Shell};
use std::env;
use std::io::Error;
use Shell::*;

include!("src/command.rs");

fn main() -> Result<(), Error> {
    let mut cmd = make_command();
    let completion_path = "completion";
    for shell in [Bash, Fish, PowerShell, Zsh] {
        let completion_path = generate_to(shell, &mut cmd, "prefix", completion_path)?;
        println!(
            "cargo:warning=completion file is generated: {:?}",
            completion_path
        );
    }

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    let man_path = "man/prefix.1";
    std::fs::write(man_path, buffer)?;
    println!("cargo:warning=man file is generated: {:?}", man_path);

    Ok(())
}
