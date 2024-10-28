use crate::config::Config;
use askama::Template;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use futures_util::FutureExt;
use std::sync::Arc;
use axum::http::header::CONTENT_LENGTH;
use tokio::sync::RwLock;

pub async fn home_handler() -> impl IntoResponse {
    HtmlTemplate(HomeTemplate {
        host: "?".to_string(),
        id: "amoghus".to_string(),
    })
}

pub async fn discord_seafile_transformer(
    Path(id): Path<String>,
    State(state): State<Arc<RwLock<Config>>>,
) -> impl IntoResponse {
    let state = state.read().await;
    let host = &state.host;

    let resource = format!("https://{host}/f/{id}/?dl=1");
    let client = reqwest::Client::new();
    let response = client.head(resource).send().await.unwrap();
    let redirect_loc = response.url().to_string();
    let extension = redirect_loc.rsplit_once(".").unwrap().1.to_string();

    HtmlTemplate(EmbedTemplate {
        id,
        host: host.clone(),
        ext: extension,
    })
}

pub async fn discord_seafile_redirect(
    Path((id, _ext)): Path<(String, String)>,
    State(state): State<Arc<RwLock<Config>>>,
) -> impl IntoResponse {
    let state = state.read().await;
    let host = &state.host;

    let uri = format!("https://{}/f/{}/?dl=1", host, id).as_str().to_string();
    let client = reqwest::Client::new();
    let response = client.get(uri).send().await.unwrap();

    Body::from_stream(response.bytes().into_stream())
}

pub async fn discord_seafile_redirect_head(
    Path((_id, _ext)): Path<(String, String)>,
    State(_state): State<Arc<RwLock<Config>>>,
) -> impl IntoResponse {
    Response::builder()
        .header(CONTENT_LENGTH, 500)
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap()
}


pub async fn handler_404() -> impl IntoResponse {
    HtmlTemplate(Error404Template {})
}

/// Error 404 page template
#[derive(Default, Template)]
#[template(path = "error_404.html")]
struct Error404Template {}

/// Home page template
#[derive(Default, Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    id: String,
    host: String,
}

/// Embed template
#[derive(Default, Template)]
#[template(path = "embed.html")]
struct EmbedTemplate {
    id: String,
    host: String,
    ext: String,
}

/// A wrapper type that we'll use to encapsulate HTML parsed
/// by askama into valid HTML for axum to serve.
struct HtmlTemplate<T>(T);

/// Allows us to convert Askama HTML templates into valid HTML
/// for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}