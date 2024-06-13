use crate::project::Project;
use askama::Template;

#[derive(Template)]
#[template(
    source = "
    <span>
        <a href=\"projects/{{ p.metadata.slug }}\">{{ p.metadata.title }}</a>
        {% if p.metadata.tags.len() > 0 %}
            <span style=\"font-size: smaller;\">
                {% for tag in p.metadata.tags %}
                    <a
                        style=\"color: var(--secondary-blue)\"
                        href=\"/tags?q={{tag}}\" title=\"{{tag}}\" target=\"_blank\"
                    >{{ tag.name }}</a>
                {% endfor %}
            </span>
        {% endif %}
        {{p.metadata.start}}
    </span>
    ",
    ext = "html"
)]
pub struct ProjectHyperlink<'a> {
    pub p: &'a Project,
}
