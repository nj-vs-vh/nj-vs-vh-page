use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use project::ProjectRelatedLink;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{env, path};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod project;

#[derive(Clone)]
struct AppState {
    project_catalog: project::ProjectCatalog,
}

#[tokio::main]
async fn main() {
    let is_debug = env::var("DEBUG").is_ok();

    let log_level = if is_debug { Level::DEBUG } else { Level::INFO };
    let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing::info!("Debug = {}, log level = {}", is_debug, log_level);

    let projects_dir = env::var("PROJECTS_DIR").unwrap_or("projects".to_owned());
    tracing::info!("Reading project catalog from {}", &projects_dir);
    let catalog_res = project::ProjectCatalog::load(path::Path::new(&projects_dir));
    if let Err(e) = catalog_res {
        tracing::error!("Failed to load project catalog: {}", e);
        return;
    }
    let catalog = catalog_res.unwrap();
    tracing::info!("Read project catalog: {}", &catalog);

    let static_dir = env::var("STATIC_DIR").unwrap_or("static".to_owned());
    tracing::info!("Serving static files from {}", &static_dir);
    let app = Router::new()
        .route("/", get(index))
        .route("/project/:slug", get(project_page))
        .nest_service("/static", ServeDir::new(static_dir))
        .layer(TraceLayer::new_for_http())
        .with_state(AppState {
            project_catalog: catalog,
        });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

struct ProjectHref {
    title: String,
    slug: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    project_hrefs: Vec<ProjectHref>,
}

async fn index(State(state): State<AppState>) -> IndexTemplate {
    let mut project_links: Vec<ProjectHref> = state
        .project_catalog
        .projects
        .iter()
        .filter_map(|p| {
            if let Some(slug) = p.metadata.slug.as_deref() {
                Some(ProjectHref {
                    title: p.metadata.title.to_owned(),
                    slug: slug.to_owned(),
                })
            } else {
                None
            }
        })
        .collect();
    let mut rng = thread_rng();
    project_links.shuffle(&mut rng);
    IndexTemplate {
        project_hrefs: project_links,
    }
}

#[derive(Template)]
#[template(path = "project.html")]
struct ProjectTemplate {
    title: String,
    body_html: String,
    related_links: Vec<ProjectRelatedLink>,
}

async fn project_page(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<ProjectTemplate, StatusCode> {
    let project_match = state.project_catalog.find(&slug);
    if let Some(project) = project_match {
        Ok(ProjectTemplate {
            // TODO: less stupid cloning!!!
            title: project.metadata.title.to_owned(),
            body_html: project.body_html.to_owned(),
            related_links: project.metadata.links.to_owned(),
        })
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
