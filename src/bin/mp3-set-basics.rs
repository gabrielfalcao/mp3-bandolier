#![allow(unused)]
use std::{fs, io::Cursor};

use clap::Parser;
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
use serde::{Serialize, Deserialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BasicTags {
    pub artist: String,
    pub album: String,
    pub title: String,
    pub cover: Vec<u8>,
    pub cover_description: Option<String>,
}
impl BasicTags {
    pub fn set_tags(&self, path: Path) -> Result<Path> {
        let mut tag = Tag::read_from_path(&path).unwrap_or_else(|e| Tag::new());
        tag.set_album(&self.album);
        tag.set_title(&self.title);
        tag.set_artist(&self.artist);
        tag.write_to_path(&path, Version::Id3v24)?;
        let data = self.cover.to_vec();
        let description =
            self.cover_description.clone().unwrap_or_else(|| format!("Front Cover"));
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
pub struct Cli {
    #[arg()]
    mp3_file: Path,

    #[arg(short, long)]
    title: String,

    #[arg(short, long)]
    artist: String,

    #[arg(short = 'A', long)]
    album: String,

    #[arg(short, long)]
    cover_image_file: Path,

    #[arg(short, long, default_value = "Front Cover")]
    cover_description: Option<String>,
}
impl Cli {
    pub fn basic_tags(&self) -> Result<BasicTags> {
        let artist = self.artist.to_string();
        let album = self.album.to_string();
        let title = self.title.to_string();
        let cover = self.cover_image_file.read_bytes()?;
        let cover_description = self.cover_description.clone();
        Ok(BasicTags { artist, album, title, cover, cover_description })
    }
    pub fn mp3_file(&self) -> Path {
        self.mp3_file.clone()
    }
}
impl ParserDispatcher<Error> for Cli {
    fn dispatch(&mut self) -> Result<()> {
        let tags = self.basic_tags()?;
        tags.set_tags(self.mp3_file())?;
        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
