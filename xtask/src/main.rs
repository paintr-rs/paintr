use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Result;
use pico_args::Arguments;
use xshell::{cmd, pushd};

fn main() -> Result<()> {
    let _d = pushd(project_root());

    let mut args = Arguments::from_env();
    let subcommand = args.subcommand()?.unwrap_or_default();

    match subcommand.as_str() {
        "install" => {
            if args.contains(["-h", "--help"]) {
                println!(
                    "\
cargo xtask install
Install paintr by cargo
USAGE:
    cargo xtask install [FLAGS]
FLAGS:
    -h, --help            Prints help information
        "
                );
                return Ok(());
            }

            let cmd = cmd!("cargo install --path crates/paintr --locked --force");
            cmd.run()?;
        }
        _ => {
            eprintln!(
                r#"
cargo xtask
Run custom build command.
USAGE:
    cargo xtask <SUBCOMMAND>
SUBCOMMANDS:
    install"#
            );
        }
    }

    Ok(())
}

pub fn project_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(1)
    .unwrap()
    .to_path_buf()
}
