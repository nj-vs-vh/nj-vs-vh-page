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
pub struct Date {
    pub year: u16,
    pub month: Option<u16>,
    pub day: Option<u16>,
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        if let Some(d) = self.day {
            parts.push(d.to_string());
        }
        if let Some(m) = self.month {
            parts.push(
                match m {
                    1 => "Jan",
                    2 => "Feb",
                    3 => "Mar",
                    4 => "Apr",
                    5 => "May",
                    6 => "Jun",
                    7 => "Jul",
                    8 => "Aug",
                    9 => "Sep",
                    10 => "Oct",
                    11 => "Nov",
                    12 => "Dec",
                    _ => "",
                }
                .to_owned(),
            );
        }
        parts.push(self.year.to_string());
        f.write_str(&parts.join(" "))
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Tag {
    pub prefix: String,
    pub name: String,
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

    #[serde(default = "default_math")]
    pub math: bool,

    pub start: Option<Date>,
    pub end: Option<Date>,

    #[serde(default = "Vec::new", alias = "tags")]
    tags_raw: Vec<String>,
    #[serde(default = "Vec::new")]
    pub tags: Vec<Tag>,
}

fn default_math() -> bool {
    false
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
    pub fn load(dir: &Path, project_media_dir: &Path) -> io::Result<Project> {
        tracing::info!("Loading project from {:?}", dir);
        if !dir.is_dir() {
            return Err(io::Error::other("path must be a directory".to_owned()));
        }

        // loading metadata
        let mut metadata: Metadata = serde_yaml::from_reader(File::open(dir.join("meta.yaml"))?)
            .map_err(|e| io::Error::other(e.to_string()))?;
        // pre-processing to handle non-trivial defaults
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
        // post-parsing tags
        for tag_raw in metadata.tags_raw.iter() {
            if let Some((prefix, body)) = tag_raw.split_once(":") {
                metadata.tags.push(Tag {
                    prefix: prefix.to_owned(),
                    name: body.to_owned(),
                })
            } else {
                return Err(io::Error::other(format!("invalid tag: {}", tag_raw)));
            }
        }

        // loading project description body
        let body_md = std::io::read_to_string(File::open(dir.join("body.md"))?)?;
        // preprocessing Markdown: insert nicer typography
        // body_md = body_md.replace("---", "—");
        let mut options = comrak::Options::default();
        options.render.unsafe_ = true;
        options.parse.smart = true;
        options.extension.strikethrough = true;
        let mut body_html = comrak::markdown_to_html(&body_md, &options);
        // posprocessing HTML (trivially, so only regex)
        // make all anchors target a blank page
        let anchor_re = Regex::new(r"<a\s+href").unwrap();
        body_html = anchor_re
            .replace_all(&body_html, "<a target=\"_blank\" href")
            .to_string();

        // copying media to a dedicated dir
        let media_dir = dir.join("media");
        if media_dir.exists() && media_dir.is_dir() {
            for file in media_dir.read_dir()? {
                if let Ok(file) = file {
                    let first_char = file
                        .file_name()
                        .to_str()
                        .unwrap()
                        .chars()
                        .take(1)
                        .next()
                        .unwrap();
                    if first_char == '.' {
                        continue;
                    }
                    let target_file = project_media_dir.join(file.file_name());
                    if target_file.exists() {
                        return Err(io::Error::other(format!("Project media {:?} name is duplicated, conflicting with an already loaded project", file.path())));
                    }
                    std::fs::copy(file.path(), &target_file)?;
                }
            }
        }
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
    pub fn load(projects_dir: &Path, project_media_dir: &Path) -> io::Result<ProjectCatalog> {
        tracing::info!(
            "Loading project catalog from {:?}, copying media into {:?}",
            projects_dir,
            project_media_dir
        );
        let projects: Vec<Project> = projects_dir
            .read_dir()?
            .filter_map(|maybe_dir_entry| {
                if let Ok(entry) = maybe_dir_entry {
                    if entry.file_name() == "template" || !entry.path().is_dir() {
                        return None;
                    }
                    let maybe_project = Project::load(&entry.path(), project_media_dir);
                    if let Ok(project) = maybe_project {
                        return Some(project);
                    } else {
                        tracing::warn!(
                            "Failed to load project from {:?}: {:?}",
                            entry.path(),
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

    pub fn find<'a>(&'a self, slug: &str) -> Option<&'a Project> {
        self.projects.iter().find(|&p| {
            if let Some(project_slug) = p.metadata.slug.as_deref() {
                project_slug == slug
            } else {
                false
            }
        })
    }
}
