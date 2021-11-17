// hide console, disabled println!
#![windows_subsystem = "windows"]

mod client;

use client::Client;
use iced::{Application, Settings, window::{self, Icon}};
use image::{GenericImageView, io::Reader as ImReader};

pub fn main() {
    let path = "./resources/aircraft.png";
    let icon = match ImReader::open(path) {
        Ok(buffer) => match buffer.decode() {
            Ok(img) => {
                let (width, height) = img.dimensions();
                let rgba = img.into_rgba8().into_raw();

                Icon::from_rgba(rgba, width, height).ok()
            }
            _ => None,
        }
        _ => None,
    };

    Client::run(Settings {
        window: window::Settings {
            icon,
            ..Default::default()
        },
        ..Default::default()
    }).unwrap();
}
