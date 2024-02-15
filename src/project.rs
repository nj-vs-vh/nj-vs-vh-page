use slugify::slugify;
use std::{fs::File, io, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Metadata {
    pub title: String,
    pub slug: Option<String>,
    pub subtitle: Option<String>,
}

#[derive(Clone)]
pub struct Project {
    pub metadata: Metadata,
    body_md: String,
    pub body_html: String,
}

impl std::fmt::Debug for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Project{{ metadata: {:?}, body: <{} bytes> }}",
            self.metadata,
            self.body_md.len()
        ))
    }
}

impl Project {
    pub fn load(dir: &Path) -> io::Result<Project> {
        log::info!("loading project from {:?}", dir);
        if !dir.is_dir() {
            return Err(io::Error::other("path must be a directory".to_owned()));
        }
        let mut metadata: Metadata = serde_yaml::from_reader(File::open(dir.join("meta.yaml"))?)
            .map_err(|e| io::Error::other(e.to_string()))?;
        if metadata.slug == None {
            metadata.slug = Some(slugify!(&metadata.title, max_length = 64));
        }
        let body_md = std::io::read_to_string(File::open(dir.join("body.md"))?)?;
        let body_html = markdown::to_html(&body_md);
        Ok(Project {
            metadata,
            body_md,
            body_html,
        })
    }
    pub fn dummy() -> Project {
        let body_md = "## Hello world\n\nThis is *markdown* **text**\n\n__cool!__".to_owned();
        return Project {
            metadata: Metadata {
                title: "Project title".to_owned(),
                slug: Some("example-project-slug".to_owned()),
                subtitle: None,
            },
            body_html: markdown::to_html(&body_md),
            body_md,
        };
    }
}

#[derive(Debug, Clone)]
pub struct ProjectCatalog {
    projects: Vec<Project>,
}

impl std::fmt::Display for ProjectCatalog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "ProjectCatalog {{ {} items }}",
            self.projects.len()
        ))
    }
}

impl ProjectCatalog {
    pub fn load(projects_dir: &Path) -> io::Result<ProjectCatalog> {
        log::info!("loading project catalog from {:?}", projects_dir);
        let projects: Vec<Project> = projects_dir
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
            .collect();
        let mut slugs: Vec<String> = projects
            .iter()
            .filter_map(|p| p.metadata.slug.clone())
            .collect();
        slugs.sort();
        slugs.dedup();
        if slugs.len() != projects.len() {
            return Err(io::Error::other("Project catalog contains duplicate slugs"));
        }
        Ok(ProjectCatalog { projects })
    }

    pub fn count(&self) -> usize {
        self.projects.len()
    }

    pub fn find(&self, slug: &str) -> Option<&Project> {
        self.projects.iter().find(|&p| {
            if let Some(project_slug) = p.metadata.slug.as_deref() {
                project_slug == slug
            } else {
                false
            }
        })
    }
}
