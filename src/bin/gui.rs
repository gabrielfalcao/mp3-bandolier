#![allow(unused)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example
use std::{fs, io::Cursor};

use clap::Parser;
use eframe::egui;
use egui::widgets::ImageSource;
use fern::colors::{Color, ColoredLevelConfig};
use id3::{
    Frame,
    Tag,
    TagLike,
    Version,
    frame::{Content, Picture, PictureType},
};
use image::{DynamicImage, GenericImageView, ImageFormat, ImageReader, imageops::FilterType};
use iocore::Path;
use mp3_organizer::{
    Config,
    Error,
    Exit,
    Result,
    UNKNOWN_ARTIST,
    UNKNOWN_TITLE,
    dispatch::ParserDispatcher,
    set_cover,
};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BasicTags {
    pub artist: String,
    pub album: String,
    pub album_artist: Option<String>,
    pub title: String,
    pub cover: Vec<u8>,
    pub cover_description: Option<String>,
}
impl BasicTags {
    pub fn load(path: Path) -> Result<BasicTags> {
        let mut tag = Tag::read_from_path(&path).unwrap_or_else(|e| Tag::new());
        let artist = tag.artist().map(|s| s.to_string()).unwrap_or_default();
        let album = tag.album().map(|s| s.to_string()).unwrap_or_default();
        let album_artist = tag.album_artist().map(|s| s.to_string());
        let title = tag.title().map(|s| s.to_string()).unwrap_or_default();
        // Prefer the front cover, or fall back to the first embedded picture
        let picture = tag
            .pictures()
            .find(|pic| pic.picture_type == PictureType::CoverFront)
            .or_else(|| tag.pictures().next());

        let cover = if let Some(pic) = picture { pic.data.to_vec() } else { Vec::new() };
        Ok(BasicTags { artist, album, album_artist, title, cover, cover_description: None })
    }
    pub fn set_tags(&self, path: Path) -> Result<Path> {
        let mut tag = Tag::read_from_path(&path).unwrap_or_else(|e| Tag::new());
        tag.set_album(&self.album);
        tag.set_title(&self.title);
        tag.set_artist(&self.artist);
        if let Some(album_artist) = self.album_artist.clone() {
            tag.set_album_artist(&album_artist);
        }
        tag.write_to_path(&path, Version::Id3v24)?;
        let data = self.cover.to_vec();
        let description = self.cover_description.clone().unwrap_or_else(|| format!("Front Cover"));
        let mime_type = if let Ok(guessed) = image::guess_format(&data) {
            guessed.to_mime_type()
        } else {
            log::warn!("could not guess mime type, falling back to image/jpeg");
            "image/jpeg"
        }
        .to_string();
        let picture = Picture {
            mime_type,
            picture_type: PictureType::CoverFront, // Front cover type
            description,
            data,
        };

        // Add the picture frame to the tag
        tag.add_frame(Frame::with_content("APIC", Content::Picture(picture)));

        // Save the changes back to the MP3 file
        tag.write_to_path(&path, id3::Version::Id3v24)?;
        Ok(path)
    }
}
#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "mp3-basics - sets the cover front of the album and other basic tags"
)]
pub struct Cli {}
impl ParserDispatcher<Error> for Cli {
    fn dispatch(&mut self) -> Result<()> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([640.0, 240.0]) // wide enough for the drag-drop overlay text
                .with_drag_and_drop(true),
            ..Default::default()
        };
        eframe::run_native("Open MP3 File", options, Box::new(|_cc| Ok(Box::<MyApp>::default())));
        Ok(())
    }
}

#[derive(Default)]
struct MyApp {
    picked_path: Option<Path>,
    tags: BasicTags,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Select MP3 file");
            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(Path::from(path));
                }

                if let Some(picked_path) = self.picked_path.clone() {
                    self.tags = BasicTags::load(picked_path.clone()).unwrap();

                    ui.horizontal(|ui| {
                        ui.label("Picked file:\n");
                        ui.monospace(picked_path.to_string());
                    });
                }
            }
            egui::CentralPanel::default().show(ui, |ui| {
                ui.heading("Edit MP3 file");
                ui.horizontal(|ui| {
                    let title_label = ui.label("title: ");
                    ui.text_edit_singleline(&mut self.tags.title).labelled_by(title_label.id);
                });
                ui.horizontal(|ui| {
                    let artist_label = ui.label("artist: ");
                    ui.text_edit_singleline(&mut self.tags.artist).labelled_by(artist_label.id);
                });
                ui.horizontal(|ui| {
                    let album_label = ui.label("album: ");
                    ui.text_edit_singleline(&mut self.tags.album).labelled_by(album_label.id);
                });
                if !self.tags.cover.is_empty() {
                    ui.image(ImageSource::Bytes {
                        uri: std::borrow::Cow::Borrowed("cover"),
                        bytes: self.tags.cover.clone().into(),
                    });
                }
            });
        });
    }
}
fn main() -> Exit {
    Cli::main()
}
