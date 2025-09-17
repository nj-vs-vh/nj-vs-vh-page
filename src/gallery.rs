use std::{
    fmt::Display,
    io,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug)]
pub struct GalleryImage {
    pub filepath: PathBuf,
    pub filename: String,
}

impl GalleryImage {
    pub fn load(filepath: PathBuf) -> io::Result<GalleryImage> {
        let filename = filepath
            .file_name()
            .ok_or(io::Error::new(
                io::ErrorKind::Other,
                "Failed to filename from path",
            ))?
            .to_str()
            .ok_or(io::Error::new(
                io::ErrorKind::Other,
                "Filename contains non-unicode characters",
            ))?
            .to_owned();

        // reading image metadata from EXIF
        let file = std::fs::File::open(&filepath)?;
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader
            .read_from_container(&mut bufreader)
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to parse EXIF metadata from the image: {}", e),
                )
            })?;
        // for f in exif.fields() {
        //     println!(
        //         "{} | {} | {}",
        //         f.tag,
        //         f.ifd_num,
        //         f.display_value().with_unit(&exif)
        //     );
        // }

        Ok(GalleryImage { filepath, filename })
    }
}

#[derive(Clone, Debug)]
pub struct Gallery {
    images: Vec<GalleryImage>,
}

impl Display for Gallery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Gallery {{ {} images }}", self.images.len(),))
    }
}

impl Gallery {
    pub fn load(gallery_dir: &Path) -> io::Result<Gallery> {
        tracing::info!("Loading gallery from {:?}", gallery_dir);
        if !gallery_dir.is_dir() {
            return Err(io::Error::other(
                "gallery dir must be a directory".to_owned(),
            ));
        }
        let images: Vec<GalleryImage> = gallery_dir
            .read_dir()?
            .filter_map(|maybe_dir_entry| {
                if let Ok(entry) = maybe_dir_entry {
                    if entry.file_name().to_string_lossy().chars().next() == Some('.') {
                        return None;
                    }
                    let gi_res = GalleryImage::load(entry.path());
                    if let Ok(image) = gi_res {
                        return Some(image);
                    } else {
                        tracing::warn!("Failed to load gallery image from {:?}", entry);
                    }
                }
                None
            })
            .collect();

        Ok(Gallery { images })
    }

    pub fn find<'a>(&'a self, slug: &str) -> Option<&'a GalleryImage> {
        self.images.iter().find(|&img| img.filename == slug)
    }
}
