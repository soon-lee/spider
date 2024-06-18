use std::fmt::format;

mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let app = routes::routes();

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", env!("AXUM_HOST", "0.0.0.0"), env!("AXUM_PORT", "8000"))).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
