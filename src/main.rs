use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{env, path};
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
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let projects_dir = env::var("PROJECTS_DIR").unwrap_or("projects".to_owned());
    tracing::info!("Reading project catalog from {}", &projects_dir);
    let catalog_res = project::ProjectCatalog::load(path::Path::new(&projects_dir));
    if let Err(e) = catalog_res {
        tracing::error!("Failed to load project catalog: {}", e);
        return;
    }
    let catalog = catalog_res.unwrap();
    tracing::info!("Read project catalog: {}", &catalog);

    let app = Router::new()
        .route("/", get(index))
        .route("/project/:slug", get(project_page))
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

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    number: usize, // temp
}

async fn index(State(state): State<AppState>) -> IndexTemplate {
    IndexTemplate {
        number: state.project_catalog.count(),
    }
}

#[derive(Template)]
#[template(path = "project.html")]
struct ProjectTemplate {
    title: String,
    body_html: String,
}

async fn project_page(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<ProjectTemplate, StatusCode> {
    let project_match = state.project_catalog.find(&slug);
    if let Some(project) = project_match {
        Ok(ProjectTemplate {
            title: project.metadata.title.clone(),
            body_html: project.body_html.clone(),
        })
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
