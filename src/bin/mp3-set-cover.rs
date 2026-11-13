#![allow(unused)]
use std::{fs, io::Cursor};

use clap::Parser;
use fern::colors::{Color, ColoredLevelConfig};
use id3::{Tag, TagLike, Version};
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

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "mp3-set-cover - sets the cover front of the album in an MP3 file"
)]
pub struct Cli {
    #[arg()]
    mp3_file: Path,

    #[arg()]
    cover_image_file: Path,

    #[arg(short, long)]
    description: Option<String>,
}
impl Cli {
    pub fn mp3_file(&self) -> Path {
        self.mp3_file.clone()
    }
    pub fn cover_image_file(&self) -> Path {
        self.cover_image_file.clone()
    }
}
impl ParserDispatcher<Error> for Cli {
    fn dispatch(&mut self) -> Result<()> {
        set_cover(self.mp3_file(), self.cover_image_file(), self.description.clone())?;
        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
