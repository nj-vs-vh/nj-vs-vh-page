use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use project::Project;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{env, path};
use tower_http::trace::TraceLayer;
use tower_http::{services::ServeDir, set_header::SetResponseHeader};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use templates::ProjectHyperlink;

mod date;
mod project;
mod templates;

#[derive(Clone)]
struct AppState {
    project_catalog: project::ProjectCatalog,
}

#[tokio::main]
async fn main() {
    let is_dev = env::var("DEV").map(|v| v.len() > 0).unwrap_or(false);
    let is_debug = env::var("DEBUG").is_ok();

    let log_level = if is_debug { Level::DEBUG } else { Level::INFO };
    let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing::info!("Debug = {}, log level = {}", is_debug, log_level);

    let static_dir_string = env::var("STATIC_DIR").unwrap_or("static".to_owned());
    let static_dir = std::path::Path::new(&static_dir_string);
    tracing::info!("Serving static files from {:?}", &static_dir);
    let project_media_dir = static_dir.join("media");
    if let Ok(dir_iter) = project_media_dir.read_dir() {
        dir_iter.for_each(|file| {
            if let Ok(file) = file {
                if let Err(e) = std::fs::remove_file(file.path()) {
                    tracing::warn!("Error deleting stale media file {:?}: {}", file.path(), e);
                }
            }
        })
    }
    if let Err(e) = std::fs::create_dir_all(&project_media_dir) {
        tracing::error!("Error creating media dir {:?}: {}", project_media_dir, e);
        return;
    };

    let projects_dir = env::var("PROJECTS_DIR").unwrap_or("projects".to_owned());
    let catalog_res =
        project::ProjectCatalog::load(path::Path::new(&projects_dir), &project_media_dir);
    if let Err(e) = catalog_res {
        tracing::error!("Failed to load project catalog: {}", e);
        return;
    }
    let catalog = catalog_res.unwrap();
    tracing::info!("Loaded project catalog: {}", &catalog);

    let cache_control = if is_dev { "max-age=300" } else { "no-cache" };
    let app = Router::new()
        .route("/", get(index))
        .route("/projects", get(project_list))
        .route("/projects/", get(project_list))
        .route("/projects/:slug", get(project_page))
        .nest_service(
            "/static",
            SetResponseHeader::if_not_present(
                ServeDir::new(static_dir),
                header::CACHE_CONTROL,
                header::HeaderValue::from_static(&cache_control),
            ),
        )
        .nest_service(
            "/projects/media",
            SetResponseHeader::if_not_present(
                ServeDir::new(&project_media_dir),
                header::CACHE_CONTROL,
                header::HeaderValue::from_static(&cache_control),
            ),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(AppState {
            project_catalog: catalog,
        });

    let port = env::var("PORT").unwrap_or("3284".to_owned());
    let host = env::var("HOST").unwrap_or("0.0.0.0".to_owned());
    let addr = format!("{}:{}", host, port);
    tracing::info!("Port: {}, host: {}, address: {}", port, host, addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// index
#[derive(Template)]
#[template(path = "index.html")]
struct Index<'a> {
    selected_project_hyperlinks: Vec<ProjectHyperlink<'a>>,
}

async fn index<'a>(State(state): State<AppState>) -> Response {
    let mut rng = thread_rng();
    Index {
        selected_project_hyperlinks: state
            .project_catalog
            .projects
            .choose_multiple(&mut rng, 5)
            .map(|p| ProjectHyperlink { p })
            .collect(),
    }
    .into_response()
}

#[derive(Template)]
#[template(path = "project_list.html")]
struct ProjectList<'a> {
    project_hyperlinks: Vec<ProjectHyperlink<'a>>,
}

async fn project_list<'a>(State(state): State<AppState>) -> Response {
    ProjectList {
        project_hyperlinks: state
            .project_catalog
            .projects
            .iter()
            .map(|p| ProjectHyperlink { p })
            .collect(),
    }
    .into_response()
}

// project page

#[derive(Template)]
#[template(path = "project.html")]
struct ProjectPage<'a> {
    project: &'a Project,
}

async fn project_page<'a>(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Response, StatusCode> {
    let project_match = state.project_catalog.find(&slug);
    if let Some(project) = project_match {
        Ok(ProjectPage { project }.into_response())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
