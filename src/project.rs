use std::{fs::File, io, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Metadata {
    title: String,
    subtitle: Option<String>,
}

pub struct Project {
    metadata: Metadata,
    body: String,
}

impl std::fmt::Debug for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Project{{ metadata: {:?}, body: <{} bytes> }}",
            self.metadata,
            self.body.len()
        ))
    }
}

impl Project {
    pub fn load(dir: &Path) -> io::Result<Project> {
        log::info!("loading project from {:?}", dir);
        if !dir.is_dir() {
            return Err(io::Error::other("path must be a directory".to_owned()));
        }
        let metadata: Metadata = serde_yaml::from_reader(File::open(dir.join("meta.yaml"))?)
            .map_err(|e| io::Error::other(e.to_string()))?;
        let body = std::io::read_to_string(File::open(dir.join("body.md"))?)?;
        Ok(Project { metadata, body })
    }

    pub fn load_catalog(projects_dir: &Path) -> io::Result<Vec<Project>> {
        log::info!("loading project catalog from {:?}", projects_dir);
        Ok(projects_dir
            .read_dir()?
            .filter_map(|maybe_dir_entry| {
                if let Ok(entry) = maybe_dir_entry {
                    let maybe_project = Project::load(&entry.path());
                    if let Ok(project) = maybe_project {
                        return Some(project);
                    } else {
                        log::warn!(
                            "failed to load project from {:?}: {:?}",
                            entry,
                            maybe_project
                        );
                    }
                }
                None
            })
            .collect())
    }
}
