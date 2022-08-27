#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]

use cargo_compete::{shell::Shell, Context, Opt};
use eyre::{Context as _, ContextCompat as _};
use std::env;
use structopt::StructOpt as _;

fn main() -> eyre::Result<()> {
    let Opt::Compete(opt) = Opt::from_args();
    let mut shell = Shell::new();

    let cwd = env::current_dir().with_context(|| "could not get the current directory")?;

    let cookies_path = dirs_next::data_local_dir()
        .with_context(|| "could not find the local data directory")?
        .join("cargo-compete")
        .join("cookies.jsonl");

    let ctx = Context {
        cwd,
        cookies_path,
        shell: &mut shell,
    };

    cargo_compete::run(opt, ctx)
}
