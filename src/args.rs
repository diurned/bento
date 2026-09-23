use crate::image;
use clap::builder::{
    Styles,
    styling::{AnsiColor, Effects},
};
use clap::{Parser, Subcommand};

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default())
    .error(AnsiColor::BrightRed.on_default().effects(Effects::BOLD))
    .valid(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
    .invalid(AnsiColor::BrightRed.on_default().effects(Effects::BOLD));

#[derive(Parser, Debug)]
#[command(
    name = "bento",
    version,
    about = "An all-in-one pentesting environment manager.",
    long_about = None,
    styles = STYLES
)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Download an image
    Pull {
        /// Image to download
        #[clap(name = "image")]
        img: image::Image,
    },

    /// Remove an image
    #[clap(alias = "rm")]
    Remove {
        /// Image to remove
        #[clap(name = "image")]
        img: image::Image,
    },

    /// Container relative commands
    #[clap(alias = "c")]
    Container {
        /// Retrieve CRI version
        #[arg(name = "version", long = "version", short = 'v')]
        version: bool,
    },
}
