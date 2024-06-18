use axum::Router;
use axum::routing::get;
use crate::handlers;

pub(crate) fn routes() -> Router {
    Router::new().route("/", get(handlers::user::get_user_by_id))
}