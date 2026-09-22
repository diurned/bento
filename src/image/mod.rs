use std::fs;
use std::process;
use std::{io, path};

use clap::ValueEnum;
use reqwest;

#[derive(Clone, Debug, ValueEnum)]
pub enum Image {
    Kali,
    Parrot,
    Exegol,
}

pub fn pull(image: Image) {
    let url = match image {
        Image::Kali => "https://cdimage.kali.org/kali-2026.2/kali-linux-2026.2-qemu-amd64.7z",
        Image::Parrot => "https://deb.parrot.sh/parrot/iso/7.3/Parrot-home-7.3_amd64.qcow2.zip",
        Image::Exegol => "",
    };

    let filename = url
        .split("/")
        .last()
        .expect("failed to format filename from url");

    let mut dest = path::PathBuf::from(env!("HOME"))
        .join(".bento")
        .join("images");

    fs::create_dir_all(&dest).expect("failed to create .bento/images/ directory");
    dest = dest.join(filename);

    if fs::exists(&dest).expect("can't check existence of image") {
        eprintln!("Image {filename} already exists.");
        process::exit(1);
    }

    // TODO: Add a progressbar.
    println!("Downloading {filename}\nIt can take some times.");
    let mut response = reqwest::blocking::get(url).expect("request failed");
    let mut file = fs::File::create(dest).expect("failed to create file");
    io::copy(&mut response, &mut file).expect("failed to write file");
}

pub fn remove(image: Image) {
    let url = match image {
        Image::Kali => "https://cdimage.kali.org/kali-2026.2/kali-linux-2026.2-qemu-amd64.7z",
        Image::Parrot => "https://deb.parrot.sh/parrot/iso/7.3/Parrot-home-7.3_amd64.qcow2.zip",
        Image::Exegol => "",
    };

    let filename = url
        .split("/")
        .last()
        .expect("failed to format filename from url");

    let dest = path::PathBuf::from(env!("HOME"))
        .join(".bento")
        .join("images")
        .join(filename);

    match fs::remove_file(dest) {
        Ok(_) => println!("Image removed."),
        Err(e) => eprintln!("failed to remove image: {e}"),
    }
}
