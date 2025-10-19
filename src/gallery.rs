use exif;
use jiff::civil::DateTime;
use std::{
    cmp::Reverse,
    fmt::Display,
    io,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug)]
pub struct GalleryImage {
    pub filename: String,
    pub title: Option<String>,
    pub timestamp: DateTime,
}

impl GalleryImage {
    pub fn load(filepath: &PathBuf, thumbnails_dir: &Path) -> io::Result<GalleryImage> {
        let filename = filepath
            .file_name()
            .ok_or(io::Error::new(
                io::ErrorKind::Other,
                "Failed to get filename from image path",
            ))?
            .to_str()
            .ok_or(io::Error::new(
                io::ErrorKind::Other,
                "Filename contains non-unicode characters",
            ))?
            .to_owned();

        // reading image contents and generating thumbnail

        let thumb_path = thumbnails_dir.join(&filename);
        if !thumb_path.exists() {
            let full_img = image::open(filepath).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to read image file {:?}: {}", filepath, e),
                )
            })?;

            // thumbnail aspect ratio is always 4:3 for gallery layout, so we crop image first
            let cropped_height = full_img.width() * 3 / 4;
            let cropped_img = if cropped_height <= full_img.height() {
                full_img.crop_imm(
                    0,
                    (full_img.height() - cropped_height) / 2,
                    full_img.width(),
                    cropped_height,
                )
            } else {
                let cropped_width = full_img.height() * 4 / 3;
                full_img.crop_imm(
                    (full_img.width() - cropped_width) / 2,
                    0,
                    cropped_width,
                    full_img.height(),
                )
            };

            let thumb_width: u32 = 230;
            let thumb_height = 3 * thumb_width / 4;
            let thumb_img = cropped_img.thumbnail(thumb_width, thumb_height);

            if let Err(e) = thumb_img.save(thumb_path) {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to save tumbnail from {:?}: {}", filepath, e),
                ));
            }

            // todo: generate colors and cache them in a text file
        };

        // reading image metadata from EXIF
        let rawfile = std::fs::File::open(filepath)?;
        let mut bufreader = std::io::BufReader::new(&rawfile);
        let exifreader = exif::Reader::new();
        let exif_data = exifreader
            .read_from_container(&mut bufreader)
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "Failed to parse EXIF metadata from the image {:?}: {}",
                        filepath, e
                    ),
                )
            })?;

        Ok(GalleryImage {
            filename,
            title: exif_data
                .get_field(exif::Tag::ImageDescription, exif::In::PRIMARY)
                .map(|f| f.display_value().to_string().trim_matches('"').to_string()),
            timestamp: exif_data
                .get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY)
                .ok_or(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "EXIF metadata for {:?} misses DateTimeOriginal field ",
                        filepath
                    ),
                ))?
                .display_value()
                .to_string()
                .parse()
                .map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!(
                            "Failed to parse DateTimeOriginal from the image {:?}: {}",
                            filepath, e
                        ),
                    )
                })?,
        })
    }

    pub fn year(&self) -> i16 {
        return self.timestamp.date().year();
    }
}

#[derive(Clone, Debug)]
pub struct Gallery {
    pub images: Vec<GalleryImage>,
}

impl Display for Gallery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Gallery {{ {} images }}", self.images.len(),))
    }
}

impl Gallery {
    pub fn load(gallery_dir: &Path, thumbnails_dir: &Path) -> io::Result<Gallery> {
        tracing::info!("Loading gallery from {:?}", gallery_dir);
        if !gallery_dir.is_dir() {
            return Err(io::Error::other(
                "gallery dir must be a directory".to_owned(),
            ));
        }
        let mut images: Vec<GalleryImage> = gallery_dir
            .read_dir()?
            .filter_map(|maybe_dir_entry| match maybe_dir_entry {
                Ok(entry) => {
                    if entry.file_name().to_string_lossy().chars().next() == Some('.') {
                        return None;
                    }

                    match GalleryImage::load(&entry.path(), thumbnails_dir) {
                        Ok(image) => Some(image),
                        Err(e) => {
                            tracing::warn!("Failed to load gallery image from {:?}: {}", entry, e);
                            None
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("Not a valid dir entry: {}", e);
                    None
                }
            })
            .collect();

        images.sort_by_key(|img| Reverse(img.timestamp));

        Ok(Gallery { images })
    }

    pub fn find<'a>(&'a self, slug: &str) -> Option<&'a GalleryImage> {
        self.images.iter().find(|&img| img.filename == slug)
    }
}
