use askama::Template;
use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
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

    let app = Router::new()
        .route("/", get(index))
        .route("/project/:slug", get(project_page))
        .nest_service(
            "/static",
            SetResponseHeader::if_not_present(
                ServeDir::new(static_dir),
                header::CACHE_CONTROL,
                header::HeaderValue::from_static("max-age=300"),
            ),
        )
        .nest_service(
            "/project/media",
            SetResponseHeader::if_not_present(
                ServeDir::new(&project_media_dir),
                header::CACHE_CONTROL,
                header::HeaderValue::from_static("max-age=300"),
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

struct ProjectHref {
    title: String,
    slug: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    project_hrefs: Vec<ProjectHref>,
}

async fn index(State(state): State<AppState>) -> Index {
    let mut project_hrefs: Vec<ProjectHref> = state
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
    project_hrefs.shuffle(&mut rng);
    Index { project_hrefs }
}

// project page

#[derive(Template)]
#[template(path = "project.html")]
struct ProjectPage {
    project: Project,
}

async fn project_page(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<ProjectPage, StatusCode> {
    let project_match = state.project_catalog.find(&slug);
    if let Some(project) = project_match {
        Ok(ProjectPage {
            // this is stupid!!! clone less!!!
            project: project.clone(),
        })
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
