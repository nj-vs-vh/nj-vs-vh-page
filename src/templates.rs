use crate::project::Project;
use askama::Template;

#[derive(Template)]
#[template(
    source = "<a href=\"projects/{{ p.metadata.slug }}\">{{ p.metadata.title }} ({{p.metadata.start}})</a>",
    ext = "html"
)]
pub struct ProjectHyperlink<'a> {
    pub p: &'a Project,
}
