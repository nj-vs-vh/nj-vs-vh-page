use exif;
use image::{GenericImageView, Pixel};
use jiff::civil::DateTime;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    cmp::Reverse,
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{self, Read, Write},
    os::unix::fs,
    path::{Path, PathBuf},
};

use crate::colorpalette::{extract_palette, PaletteExtractionAlgorithm};

pub struct GalleryConfig<'a> {
    pub stdmedia_dir: &'a Path,
    pub thumbnails_dir: &'a Path,
    pub fulls_dir: &'a Path,
    pub ignore_cache: bool,
    pub unlisted_filenames_pepper: &'a str,
}

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
        config: &GalleryConfig,
        anonymize: bool,
    ) -> io::Result<GalleryImage> {
        let mut filename = filepath
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
        let original_filename = filename.clone();
        if anonymize {
            let mut hasher = Sha256::new();
            hasher.update(filename);
            hasher.update(config.unlisted_filenames_pepper);
            filename = hex::encode(hasher.finalize()).chars().take(10).collect();
            if let Some(extension) = filepath.extension() {
                filename = [filename, extension.to_string_lossy().to_string()].join(".");
            }
        }

        // reading image contents and generating thumbnail
        let standard_media_path = config.stdmedia_dir.join(&filename);
        let thumb_path = config.thumbnails_dir.join(&filename);
        let colorpalette_path = filepath.with_file_name(format!(".{}.colors", &original_filename)); // for human-readable color pallettes
        if config.ignore_cache
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

            let luma_min: u8 = 60;
            let mut pixels: Vec<[u8; 3]> = thumb_img
                .pixels()
                .filter(|(_, _, rgb)| rgb.to_luma().0[0] > luma_min)
                .map(|(_, _, rgb)| [rgb.0[0], rgb.0[1], rgb.0[2]])
                .collect();
            let colorpalette = extract_palette(
                pixels.as_mut_slice(),
                3,
                &PaletteExtractionAlgorithm::PaletteExtractLib,
            )
            .unwrap();
            let colorpalette_codes: Vec<String> = colorpalette
                .iter()
                .map(|rgb| rgb.map(|value| format!("{:02x}", value)).join(""))
                .collect();
            tracing::info!("Extracted color palette: {}", colorpalette_codes.join(" "));
            write!(
                File::create(&colorpalette_path)?,
                "{}",
                colorpalette_codes.join("\n")
            )?;
        };

        let full_path = config.fulls_dir.join(&filename);
        if !full_path.exists() {
            fs::symlink(filepath.canonicalize()?, full_path)?;
        }

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
        f.write_str(&format!("<{} images>", self.images.len(),))
    }
}

const ALBUM_YAML: &str = "album.yaml";

impl Gallery {
    pub fn load(src_dir: &Path, config: &GalleryConfig, is_listed: bool) -> io::Result<Gallery> {
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
                    if entry.path().is_dir() {
                        return None;
                    }
                    let _fname = entry.file_name();
                    let name = _fname.to_string_lossy();
                    if name == ALBUM_YAML || name.chars().next() == Some('.') {
                        return None;
                    }

                    match GalleryImage::load(&entry.path(), config, !is_listed) {
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

#[derive(Deserialize, Debug, Clone)]
pub struct AlbumMetadata {
    pub title: String,
    pub description: String,
    pub slug: String,
    pub listed: bool,
}

#[derive(Clone, Debug)]
pub struct Album {
    pub gallery: Gallery,
    pub meta: AlbumMetadata,
}

impl Display for Album {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("'{}' ({})", self.meta.title, self.gallery))
    }
}

impl Album {
    pub fn load(dir: &Path, config: &GalleryConfig) -> io::Result<Album> {
        tracing::info!("Loading album from {:?}", dir);
        let meta: AlbumMetadata = serde_yaml::from_reader(File::open(dir.join(ALBUM_YAML))?)
            .map_err(|e| {
                io::Error::other(format!("Error loading album metadata: {}", e.to_string()))
            })?;
        Ok(Album {
            gallery: Gallery::load(dir, config, meta.listed)?,
            meta,
        })
    }

    #[allow(dead_code)]
    pub fn timestamp(&self) -> Option<DateTime> {
        self.gallery.images.iter().map(|img| img.timestamp).max()
    }
}

#[derive(Clone, Debug)]
pub struct TopGallery {
    pub root: Gallery,
    pub albums: HashMap<String, Album>,
}

impl Display for TopGallery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Main gallery: {}, albums: ", self.root))?;
        for (i, a) in self.albums.values().enumerate() {
            a.fmt(f)?;
            if i < self.albums.len() - 1 {
                f.write_str(", ")?;
            }
        }
        Ok(())
    }
}

impl TopGallery {
    pub fn load(dir: &Path, config: &GalleryConfig) -> io::Result<TopGallery> {
        tracing::info!("Loading top-level gallery from {:?}", dir);

        Ok(TopGallery {
            root: Gallery::load(dir, config, true)?,
            albums: dir
                .read_dir()?
                .filter_map(|maybe_dir_entry| match maybe_dir_entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            match Album::load(&path, config) {
                                Ok(a) => Some((a.meta.slug.clone(), a)),
                                Err(e) => {
                                    tracing::warn!(
                                        "Failed to load gallery image from {:?}: {}",
                                        entry,
                                        e
                                    );
                                    None
                                }
                            }
                        } else {
                            None
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Not a valid dir entry: {}", e);
                        None
                    }
                })
                .collect(),
        })
    }

    // pub fn find_album(&self, slug: &str) -> Option<&Album> {}
}
