use std::sync::Arc;
use askama::Template;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use tokio::sync::RwLock;
use crate::config::Config;

pub async fn home_handler(
) -> impl IntoResponse {
    HtmlTemplate(HomeTemplate {
        host: "?".to_string(),
        id: "amoghus".to_string(),
    })
}

pub async fn discord_seafile_transformer(
    Path(id): Path<String>,
    State(state): State<Arc<RwLock<Config>>>
) -> impl IntoResponse {

    let state = state.read().await;
    let host = &state.host;

    HtmlTemplate(HomeTemplate {
        id,
        host: host.clone()
    })
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
    host: String
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