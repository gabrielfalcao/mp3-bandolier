use id3::{
    Frame,
    Tag,
    TagLike,
    frame::{Content, Picture, PictureType},
};
use image::ImageFormat;
use iocore::Path;

use crate::{Error, Result};

pub fn set_cover(
    mp3_path: Path,
    cover_path: Path,
    description: Option<String>,
) -> Result<(), Error> {
    // Read an existing tag or create a new one
    let mut tag = Tag::read_from_path(&mp3_path).unwrap_or_else(|_| Tag::new());

    let data = cover_path.read_bytes()?;
    let description = description.unwrap_or_else(|| "Cover".to_string());
    let mime_type = if let Ok(guessed) = image::guess_format(&data) {
        guessed.to_mime_type()
    } else {
        log::warn!("could not guess mime type, falling back to image/jpeg");
        "image/jpeg"
    };
    let picture = Picture {
        mime_type,
        picture_type: PictureType::CoverFront, // Front cover type
        description,
        data,
    };

    // Add the picture frame to the tag
    tag.add_frame(Frame::with_content("APIC", Content::Picture(picture)));

    // Save the changes back to the MP3 file
    tag.write_to_path(&mp3_path, id3::Version::Id3v24)?;

    Ok(())
}
