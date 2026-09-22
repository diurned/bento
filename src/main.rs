mod args;
mod config;
mod image;

use clap::Parser;

fn main() {
    config::init::fs();

    let argv = args::Args::parse();

    match argv.cmd {
        args::Commands::Pull { img } => image::pull(img),
        args::Commands::Remove { img } => image::remove(img),
    };
}
