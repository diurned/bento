use std::fs;
use std::process;
use std::{io, path};

use crate::container;

use clap::ValueEnum;
use reqwest;

#[derive(Clone, Debug, ValueEnum)]
pub enum Image {
    Kali = 0x0,
    Parrot = 0x1,
    Exegol = 0x2,
    KaliVM = 0x3,
    ParrotVM = 0x4,
}

pub fn pull(image: Image) {
    let url = match image {
        Image::Kali => "docker.io/kalilinux/kali-rolling:latest",
        Image::Parrot => "docker.io/parrotsec/security:latest",
        Image::Exegol => "docker.io/nwodtuhs/exegol:full-3.1.6",
        Image::KaliVM => "https://cdimage.kali.org/kali-2026.2/kali-linux-2026.2-qemu-amd64.7z",
        Image::ParrotVM => "https://deb.parrot.sh/parrot/iso/7.3/Parrot-home-7.3_amd64.qcow2.zip",
    };

    let discriminant = image as u8;

    if discriminant <= 0x2 {
        container::pull::pull(url);
        process::exit(0);
    } else {
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

        // TODO: Add a progressbar
        println!("Downloading {filename}\nIt can take some times.");
        let mut response = reqwest::blocking::get(url).expect("request failed");
        let mut file = fs::File::create(dest).expect("failed to create file");
        io::copy(&mut response, &mut file).expect("failed to write file");
    }
}

pub fn remove(image: Image) {
    let url = match image {
        Image::Kali => "docker.io/kalilinux/kali-rolling:latest",
        Image::Parrot => "docker.io/parrotsec/security:latest",
        Image::Exegol => "docker.io/nwodtuhs/exegol:full-3.1.6",
        Image::KaliVM => "https://cdimage.kali.org/kali-2026.2/kali-linux-2026.2-qemu-amd64.7z",
        Image::ParrotVM => "https://deb.parrot.sh/parrot/iso/7.3/Parrot-home-7.3_amd64.qcow2.zip",
    };

    let discriminant = image as u8;

    if discriminant <= 0x2 {
        // TODO: Make the remove function.
        process::exit(1);
    } else {
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
}
