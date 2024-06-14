use itertools::Itertools;
use regex::Regex;
use std::cmp::Reverse;
use std::{fs::File, io, path::Path};

use crate::date::Date;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectLink {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectTag {
    pub category: String,
    pub name: String,
}

impl std::fmt::Display for ProjectTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}", self.category, self.name))
    }
}

impl ProjectTag {
    pub fn parse(s: &str) -> io::Result<ProjectTag> {
        if let Some((category, body)) = s.split_once(":") {
            Ok(ProjectTag {
                category: category.to_owned(),
                name: body.to_owned(),
            })
        } else {
            Err(io::Error::other(format!("invalid tag: {:?}", s)))
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectMetadata {
    pub title: String,
    pub slug: String,

    #[serde(default = "Vec::new")]
    pub links: Vec<ProjectLink>,

    pub github: Option<String>,

    #[serde(default = "Vec::new")]
    pub code_languages: Vec<String>,

    #[serde(default = "default_math")]
    pub math: bool,

    pub start: Date,
    pub end: Option<Date>,

    #[serde(default = "Vec::new", alias = "tags")]
    tags_raw: Vec<String>,
    #[serde(default = "Vec::new")]
    pub tags: Vec<ProjectTag>,
}

fn default_math() -> bool {
    false
}

#[derive(Clone)]
pub struct Project {
    pub metadata: ProjectMetadata,
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
        let mut metadata: ProjectMetadata =
            serde_yaml::from_reader(File::open(dir.join("meta.yaml"))?)
                .map_err(|e| io::Error::other(e.to_string()))?;
        if let Some(ref github_link_url) = metadata.github {
            metadata.links.insert(
                0,
                ProjectLink {
                    name: "github".to_owned(),
                    url: github_link_url.clone(),
                },
            )
        }
        // post-parsing tags
        for tag_raw in metadata.tags_raw.iter() {
            metadata.tags.push(ProjectTag::parse(&tag_raw)?);
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

pub type TagGroups = Vec<(String, Vec<ProjectTag>)>;

#[derive(Debug, Clone)]
pub struct ProjectCatalog {
    pub projects: Vec<Project>,
    pub tag_groups: TagGroups,
}

impl std::fmt::Display for ProjectCatalog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "ProjectCatalog {{ {} projects, tags: {} }}",
            self.projects.len(),
            self.tag_groups
                .iter()
                .map(|(category, tags)| format!(
                    "{}:[{}]",
                    category,
                    tags.iter().map(|t| t.name.clone()).join("|")
                ))
                .join(", "),
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
        let mut projects: Vec<Project> = projects_dir
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

        // sorting by date newest->oldest
        projects.sort_by(|a, b| b.metadata.start.cmp(&a.metadata.start));

        // validating slug uniqueness
        let mut slugs: Vec<String> = projects.iter().map(|p| p.metadata.slug.clone()).collect();
        slugs.sort();
        slugs.dedup();
        if slugs.len() != projects.len() {
            return Err(io::Error::other("Project catalog contains duplicate slugs"));
        }

        // tag groups
        let mut tags: Vec<ProjectTag> = projects
            .iter()
            .flat_map(|p| p.metadata.tags.clone())
            .collect();
        tags.sort_by_key(|t| t.category.clone());
        let mut tag_groups = tags
            .into_iter()
            .chunk_by(|t| t.category.clone())
            .into_iter()
            .map(|(key, chunk)| {
                (
                    key,
                    chunk
                        .counts()
                        .into_iter()
                        .sorted_by_key(|(_, freq)| Reverse(*freq))
                        .map(|(tag, _)| tag)
                        .collect_vec(),
                )
            })
            .collect_vec();
        tag_groups.sort_by_key(|(_, group)| Reverse(group.len()));

        Ok(ProjectCatalog {
            projects,
            tag_groups,
        })
    }

    pub fn find<'a>(&'a self, slug: &str) -> Option<&'a Project> {
        self.projects.iter().find(|&p| p.metadata.slug == slug)
    }
}
