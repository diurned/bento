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

    /// List images
    #[clap(name = "images", alias = "i")]
    Images {},

    /// Container relative commands
    #[clap(alias = "c")]
    Container {
        /// Retrieve CRI version
        #[arg(name = "version", long = "version", short = 'v')]
        version: bool,
    },

    /// List containers
    Ps {},

    /// Create and run a new container from an image
    Run {
        /// Container name
        #[clap(name = "name")]
        name: String,

        /// Image to run
        #[clap(name = "image")]
        img: image::Image,
    },

    /// Delete a container by its ID
    #[clap(alias = "del")]
    Delete {
        /// Container ID to delete
        #[clap(name = "cid")]
        cid: String,
    },
}
