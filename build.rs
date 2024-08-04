use clap_complete::Shell::*;
use clap_mangen::Man;
use std::env;
use std::io::Error;

include!("src/command.rs");

fn main() -> Result<(), Error> {
    let mut cmd = make_command();
    let out_dir =
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?);

    for shell in [Bash, Fish, PowerShell, Zsh] {
        let completion_path = clap_complete::generate_to(shell, &mut cmd, "prefix", &out_dir)?;
        println!(
            "cargo:warning=completion file is generated: {:?}",
            completion_path
        );
    }

    let man = clap_mangen::Man::new(cmd);
    let man_path = Man::generate_to(&man, out_dir)?;
    println!("cargo:warning=man file is generated: {:?}", man_path);

    Ok(())
}
