use askama::Template;
use axum::{response::IntoResponse, routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod project;
mod template_resp;

// #[tokio::main]
// async fn main() {
//     let subscriber = FmtSubscriber::builder()
//         .with_max_level(Level::DEBUG)
//         .finish();
//     tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

//     let app = Router::new()
//         .route("/", get(index))
//         .layer(TraceLayer::new_for_http());

//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
//         .await
//         .unwrap();

//     println!("listening on {}", listener.local_addr().unwrap());

//     axum::serve(listener, app).await.unwrap();
// }

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    template_resp::HtmlTemplateResponse(template)
}

// TEMP
fn main() {
    let project = project::Project::load(&std::path::Path::new("projects/this"));
    println!("{:?}", project);
}
