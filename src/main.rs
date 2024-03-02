#![forbid(unsafe_code, unstable_features)]

use crate::arguments::Command;
use clap::Parser;

mod arguments;
mod fetch_config;
mod serve_config;

fn main() {
    let args = arguments::Arguments::parse();

    let exit_code = match args.command {
        Command::Fetch(fetch_args) => fetch_config::run(&fetch_args),
        Command::Serve(serve_args) => serve_config::run(&serve_args),
    };

    std::process::exit(exit_code);
}
