use regex::Regex;
use slugify::slugify;
use std::{fs::File, io, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectRelatedLink {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Metadata {
    pub title: String,
    pub slug: Option<String>,

    #[serde(default = "Vec::new")]
    pub links: Vec<ProjectRelatedLink>,

    pub github: Option<String>,

    #[serde(default = "Vec::new")]
    pub code_languages: Vec<String>,
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
        tracing::info!("loading project from {:?}", dir);
        if !dir.is_dir() {
            return Err(io::Error::other("path must be a directory".to_owned()));
        }

        let mut metadata: Metadata = serde_yaml::from_reader(File::open(dir.join("meta.yaml"))?)
            .map_err(|e| io::Error::other(e.to_string()))?;
        // metadata pre-processing to handle non-trivial defaults
        if metadata.slug == None {
            metadata.slug = Some(slugify!(&metadata.title, max_length = 64));
        }
        if let Some(ref github_link_url) = metadata.github {
            metadata.links.insert(
                0,
                ProjectRelatedLink {
                    name: "github".to_owned(),
                    url: github_link_url.clone(),
                },
            )
        }

        let mut body_md = std::io::read_to_string(File::open(dir.join("body.md"))?)?;

        // preprocessing Markdown
        // 1. insert a nicer typography
        body_md = body_md.replace("--", "—");

        let mut body_html = markdown::to_html(&body_md);
        // posprocessing HTML (trivially, so only regex)
        // 1. make all anchors target a blank page
        let anchor_re = Regex::new(r"<a\s+href").unwrap();
        body_html = anchor_re
            .replace_all(&body_html, "<a target=\"_blank\" href")
            .to_string();
        Ok(Project {
            metadata,
            body_md,
            body_html,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ProjectCatalog {
    pub projects: Vec<Project>,
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
        tracing::info!("loading project catalog from {:?}", projects_dir);
        let projects: Vec<Project> = projects_dir
            .read_dir()?
            .filter_map(|maybe_dir_entry| {
                if let Ok(entry) = maybe_dir_entry {
                    if entry.file_name() == "template" {
                        return None;
                    }
                    let maybe_project = Project::load(&entry.path());
                    if let Ok(project) = maybe_project {
                        return Some(project);
                    } else {
                        tracing::warn!(
                            "failed to load project from {:?}: {:?}",
                            entry,
                            maybe_project
                        );
                    }
                }
                None
            })
            .collect();

        // validating slug uniqueness
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
