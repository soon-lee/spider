use axum::Router;
use axum::routing::get;

pub(crate) fn routes() -> Router {
    Router::new().route("/", get(|| {}))
}