use axum::body::Body;
use axum::error_handling::HandleError;
use axum::extract::Query;
use axum::handler::Handler;
use axum::http::Request;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use axum::{Extension, Router, Server};
use clap::Parser;
use once_cell::sync::Lazy;
use simple_ssr::app_api::AppApi;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio_util::task::LocalPoolHandle;
use tower::ServiceExt;
use tower_http::services::ServeDir;

// We spawn a local pool that is as big as the number of cpu threads.
static LOCAL_POOL: Lazy<LocalPoolHandle> = Lazy::new(|| LocalPoolHandle::new(num_cpus::get()));

/// A basic example
#[derive(Parser, Debug)]
struct Opt {
    /// the "dist" created by trunk directory to be served for hydration.
    #[clap(short, long, parse(from_os_str))]
    dir: PathBuf,
}

async fn render(
    Extension(index_content): Extension<IndexContent>,
    url: Request<Body>,
    Query(queries): Query<HashMap<String, String>>,
) -> Html<String> {
    // let url = url.uri().to_string();

    let content = LOCAL_POOL
        .spawn_pinned(move || async move {
            // let server_app_props = ServerAppProps {
            //     url: url.into(),
            //     queries,
            // };

            let renderer = yew::ServerRenderer::<AppApi>::new();

            // ::with_props(server_app_props);

            renderer.render().await
        })
        .await
        .expect("the task has failed.");

    // Good enough for an example, but developers should avoid the replace and extra allocation
    // here in an actual app.
    Html(
        index_content
            .0
            .replace("<body>", &format!("<body>{}", content)),
    )
}

#[derive(Clone)]
struct IndexContent(String);

#[tokio::main]
async fn main() {
    // env_logger::init();

    let opts = Opt::parse();

    let index_html_s = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    let handle_error = |e| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("error occurred: {}", e),
        )
    };

    let index_content = IndexContent(index_html_s.clone());

    let app = Router::new()
        .route("/api/test", get(|| async move { "Hello World" }))
        // needed because https://github.com/tower-rs/tower-http/issues/262
        .route("/", get(render))
        .layer(Extension(index_content.clone()))
        .fallback(HandleError::new(
            ServeDir::new(opts.dir)
                .append_index_html_on_directories(false)
                .fallback(
                    render
                        .layer(Extension(index_content))
                        .into_service()
                        .map_err(|err| -> std::io::Error { match err {} }),
                ),
            handle_error,
        ));

    println!("You can view the website at: http://localhost:8080/");

    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
