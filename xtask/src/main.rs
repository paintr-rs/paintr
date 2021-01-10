use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use pico_args::Arguments;
use xshell::{cmd, pushd, pushenv};

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
            Ok(())
        }
        "format" => {
            args.finish()?;
            run_rustfmt(Mode::Overwrite)
        }
        _ => {
            eprintln!(
                r#"cargo xtask
Run custom build command.
USAGE:
    cargo xtask <SUBCOMMAND>
SUBCOMMANDS:
    install
    format"#
            );
            Ok(())
        }
    }
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

#[allow(unused)]
enum Mode {
    Overwrite,
    Verify,
}

fn run_rustfmt(mode: Mode) -> Result<()> {
    let _dir = pushd(project_root())?;
    let _e = pushenv("RUSTUP_TOOLCHAIN", "stable");
    ensure_rustfmt()?;
    let check = match mode {
        Mode::Overwrite => &[][..],
        Mode::Verify => &["--", "--check"],
    };
    cmd!("cargo fmt {check...}").run()?;
    Ok(())
}

fn ensure_rustfmt() -> Result<()> {
    let out = cmd!("rustfmt --version").read()?;
    if !out.contains("stable") {
        bail!(
            "Failed to run rustfmt from toolchain 'stable'. \
             Please run `rustup component add rustfmt --toolchain stable` to install it.",
        )
    }
    Ok(())
}
