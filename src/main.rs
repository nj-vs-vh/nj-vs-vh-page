use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use gallery::Gallery;
use project::{Project, ProjectTag, TagGroups};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{collections::HashMap, env, path};
use tower_http::trace::TraceLayer;
use tower_http::{services::ServeDir, set_header::SetResponseHeader};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use templates::ProjectHyperlink;

mod date;
mod gallery;
mod median_cut;
mod project;
mod templates;

#[derive(Clone)]
struct AppState {
    project_catalog: project::ProjectCatalog,
    gallery: gallery::Gallery,
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
    let project_media_dir = static_dir.join("project-media");
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
    let gallery_thumbnails_dir = static_dir.join("gallery-thumbnails");
    if let Err(e) = std::fs::create_dir_all(&gallery_thumbnails_dir) {
        tracing::error!(
            "Error creating thumbnails dir {:?}: {}",
            &gallery_thumbnails_dir,
            e
        );
        return;
    };
    let gallery_stdmedia_dir = static_dir.join("gallery-media");
    if let Err(e) = std::fs::create_dir_all(&gallery_stdmedia_dir) {
        tracing::error!(
            "Error creating gallery media dir {:?}: {}",
            &gallery_stdmedia_dir,
            e
        );
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

    let gallery_dir_string = env::var("GALLERY_DIR").unwrap_or("gallery".to_owned());
    let gallery_dir = std::path::Path::new(&gallery_dir_string);
    tracing::info!("Serving gallery files from {:?}", &gallery_dir);
    let gr = Gallery::load(gallery_dir, &gallery_stdmedia_dir, &gallery_thumbnails_dir);
    if let Err(e) = gr {
        tracing::error!("Failed to load gallery: {}", e);
        return;
    }
    let gallery = gr.unwrap();
    tracing::info!("Loaded gallery: {}", &gallery);

    let static_content_cache = if !is_dev { "max-age=300" } else { "no-cache" };
    let app = Router::new()
        .route("/", get(index))
        .route("/projects", get(project_list))
        .route("/projects/", get(project_list))
        .route("/projects/:slug", get(project_page))
        .route("/tags/", get(tag_list))
        .route("/tags", get(tag_list))
        .route("/music", get(music))
        .route("/gallery", get(gallery_page))
        .route("/gallery/:slug", get(gallery_image))
        .nest_service(
            "/static",
            SetResponseHeader::if_not_present(
                ServeDir::new(static_dir),
                header::CACHE_CONTROL,
                header::HeaderValue::from_static(&static_content_cache),
            ),
        )
        .nest_service(
            "/gallery/full",
            SetResponseHeader::if_not_present(
                ServeDir::new(gallery_dir),
                header::CACHE_CONTROL,
                header::HeaderValue::from_static(&static_content_cache),
            ),
        )
        .nest_service(
            "/gallery/thumbnails",
            SetResponseHeader::if_not_present(
                ServeDir::new(gallery_thumbnails_dir),
                header::CACHE_CONTROL,
                header::HeaderValue::from_static(&static_content_cache),
            ),
        )
        .nest_service(
            "/gallery/media",
            SetResponseHeader::if_not_present(
                ServeDir::new(gallery_stdmedia_dir),
                header::CACHE_CONTROL,
                header::HeaderValue::from_static(&static_content_cache),
            ),
        )
        .nest_service(
            "/projects/media",
            SetResponseHeader::if_not_present(
                ServeDir::new(project_media_dir),
                header::CACHE_CONTROL,
                header::HeaderValue::from_static(&static_content_cache),
            ),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(AppState {
            project_catalog: catalog,
            gallery,
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
    tag_filter: Option<ProjectTag>,
}

async fn project_list<'a>(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Response, StatusCode> {
    let tag_filter = match query.get("tag") {
        None => None,
        Some(t) => Some(ProjectTag::parse(t).map_err(|_| StatusCode::BAD_REQUEST)?),
    };
    Ok(ProjectList {
        project_hyperlinks: state
            .project_catalog
            .projects
            .iter()
            .filter(|p| {
                if let Some(tag_filter) = &tag_filter {
                    p.metadata.tags.contains(tag_filter)
                } else {
                    true
                }
            })
            .map(|p| ProjectHyperlink { p })
            .collect(),
        tag_filter,
    }
    .into_response())
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

#[derive(Template)]
#[template(path = "tag_list.html")]
struct TagSearchPage<'a> {
    tag_groups: &'a TagGroups,
}

async fn tag_list(State(state): State<AppState>) -> Result<Response, StatusCode> {
    Ok(TagSearchPage {
        tag_groups: &state.project_catalog.tag_groups,
    }
    .into_response())
}

#[derive(Template)]
#[template(path = "music.html")]
struct MusicPage {
    pub embeds: bool,
}

async fn music(Query(params): Query<HashMap<String, String>>) -> MusicPage {
    MusicPage {
        embeds: params
            .get("embeds")
            .map_or(true, |v| v.to_lowercase() != "false"),
    }
}

#[derive(Template)]
#[template(path = "gallery.html")]
struct GalleryPage<'a> {
    // page: usize,  // todo: pagination
    images_by_year: Vec<(String, &'a [gallery::GalleryImage])>,
}

async fn gallery_page<'a>(State(state): State<AppState>) -> Result<Response, StatusCode> {
    Ok(GalleryPage {
        // page: 0,
        images_by_year: state
            .gallery
            .images
            .chunk_by(|i1, i2| i1.month_year() == i2.month_year())
            .map(|photos| (photos[0].month_year(), photos))
            .collect(),
    }
    .into_response())
}

#[derive(Template)]
#[template(path = "gallery_image.html")]
struct GalleryImagePage<'a> {
    found: gallery::FoundGalleryImage<'a>,
}

async fn gallery_image<'a>(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Response, StatusCode> {
    if let Some(image) = state.gallery.find(&slug) {
        Ok(GalleryImagePage { found: image }.into_response())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
