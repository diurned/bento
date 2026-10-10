mod args;
mod config;
mod container;
mod image;

use clap::Parser;

fn main() {
    config::init::fs();

    let argv = args::Args::parse();

    match argv.cmd {
        args::Commands::Pull { img } => image::pull(img),
        args::Commands::Remove { img } => image::remove(img),
        args::Commands::Images {} => image::list_image(),
        args::Commands::Container { version: _ } => container::version::version(),
        args::Commands::Ps {} => container::list::container(),
        args::Commands::Run { name, img } => container::create::start(name, img),
    };
}
