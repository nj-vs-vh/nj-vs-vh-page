use std::{fs::File, io, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Metadata {
    title: String,
}

pub struct Project {
    metadata: Metadata,
    body: String,
}

impl std::fmt::Debug for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Project{{ metadata: {:?} }}", self.metadata))
    }
}

impl Project {
    pub fn load(dir: &Path) -> io::Result<Project> {
        if !dir.is_dir() {
            return Err(io::Error::other("path must be a directory".to_owned()));
        }
        let metadata: Metadata = serde_yaml::from_reader(File::open(dir.join("meta.yaml"))?)
            .map_err(|e| io::Error::other(e.to_string()))?;
        let body = std::io::read_to_string(File::open(dir.join("body.md"))?)?;
        Ok(Project { metadata, body })
    }
}
