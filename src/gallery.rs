use exif;
use image::{GenericImageView, Pixel, Rgb};
use jiff::civil::DateTime;
use std::{
    cmp::Reverse,
    fmt::Display,
    fs::File,
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

use crate::colorpalette::{extract_palette, PaletteExtractionAlgorithm};

#[derive(Clone, Debug)]
pub struct GalleryImage {
    pub filename: String,
    pub title: Option<String>,
    pub timestamp: DateTime,
    pub colorpalette: Vec<String>,
}

impl GalleryImage {
    pub fn load(
        filepath: &PathBuf,
        stdmedia_dir: &Path,
        thumbnails_dir: &Path,
        ignore_cache: bool,
    ) -> io::Result<GalleryImage> {
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
        let standard_media_path = stdmedia_dir.join(&filename);
        let thumb_path = thumbnails_dir.join(&filename);
        let colorpalette_path = filepath.with_file_name(format!(".{}.colors", &filename));
        if ignore_cache
            || !standard_media_path.exists()
            || !thumb_path.exists()
            || !colorpalette_path.exists()
        {
            tracing::info!("Loading and processing image: {:?}", filepath);

            let full_img = image::open(filepath).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to read image file {:?}: {}", filepath, e),
                )
            })?;

            // producing the main image to be displayed on the web
            let max_display_width: u32 = 2000;
            let max_display_height: u32 = 1000;
            let standard_img = full_img.resize(
                max_display_width,
                max_display_height,
                image::imageops::FilterType::Lanczos3,
            );
            if let Err(e) = standard_img.save(standard_media_path) {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "Failed to save standard-size image from {:?}: {}",
                        filepath, e
                    ),
                ));
            };

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

            let thumb_width: u32 = 300;
            let thumb_height = 3 * thumb_width / 4;
            let thumb_img = cropped_img.thumbnail(thumb_width, thumb_height);

            if let Err(e) = thumb_img.save(thumb_path) {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to save tumbnail from {:?}: {}", filepath, e),
                ));
            }

            let mut pixels: Vec<[u8; 3]> = thumb_img
                .pixels()
                .map(|(_, _, color)| [color.0[0], color.0[1], color.0[2]])
                .collect();
            let colorpalette = extract_palette(
                pixels.as_mut_slice(),
                3,
                &PaletteExtractionAlgorithm::ModeBisect,
            )
            .unwrap();
            let lumas: Vec<u8> = colorpalette
                .iter()
                .map(|rgb| Rgb(rgb.to_owned()).to_luma().0[0])
                .collect();
            let luma_min: u8 = 60;
            let colorpalette_codes: Vec<String> = colorpalette
                .iter()
                .zip(lumas.iter())
                .filter(|(_, luma)| **luma > luma_min)
                .map(|(color, _)| color.map(|value| format!("{:02x}", value)).join(""))
                .collect();
            tracing::info!("Extracted color palette: {}", colorpalette_codes.join(" "));
            write!(
                File::create(&colorpalette_path)?,
                "{}",
                colorpalette_codes.join("\n")
            )?;
        };

        let mut contents = String::new();
        File::open(&colorpalette_path)?.read_to_string(&mut contents)?;
        let colorpalette: Vec<String> = contents
            .lines()
            .filter(|s| s.len() == 6)
            .map(|s| s.to_owned())
            .collect();

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
            colorpalette,
        })
    }

    pub fn month_year(&self) -> String {
        self.timestamp.date().strftime("%B %Y").to_string()
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
    pub fn load(
        src_dir: &Path,
        stdmedia_dir: &Path,
        thumbnails_dir: &Path,
        ignore_cache: bool,
    ) -> io::Result<Gallery> {
        tracing::info!("Loading gallery from {:?}", src_dir);
        if !src_dir.is_dir() {
            return Err(io::Error::other(
                "gallery dir must be a directory".to_owned(),
            ));
        }
        let mut images: Vec<GalleryImage> = src_dir
            .read_dir()?
            .filter_map(|maybe_dir_entry| match maybe_dir_entry {
                Ok(entry) => {
                    if entry.file_name().to_string_lossy().chars().next() == Some('.') {
                        return None;
                    }

                    match GalleryImage::load(
                        &entry.path(),
                        stdmedia_dir,
                        thumbnails_dir,
                        ignore_cache,
                    ) {
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

    pub fn find<'a>(&'a self, slug: &str) -> Option<FoundGalleryImage<'a>> {
        self.images
            .iter()
            .enumerate()
            .find(|&(_, img)| img.filename == slug)
            .map(|(index, img)| FoundGalleryImage {
                image: img,
                prev: if index > 0 {
                    self.images.get(index - 1)
                } else {
                    None
                },
                next: self.images.get(index + 1),
            })
    }

    pub fn size(&self) -> usize {
        self.images.len()
    }

    pub fn total_pages(&self, pagesize: usize) -> usize {
        self.size() / pagesize + usize::from(self.size() % pagesize > 0)
    }
}

#[derive(Clone)]
pub struct FoundGalleryImage<'a> {
    pub image: &'a GalleryImage,
    pub prev: Option<&'a GalleryImage>,
    pub next: Option<&'a GalleryImage>,
}
