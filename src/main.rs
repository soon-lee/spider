mod handlers;
mod routes;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    let app = routes::routes();
    let host = std::env::var("AXUM_HOST").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("AXUM_PORT").unwrap_or("8000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[tokio::test]
async fn test_config() {
    let config = utils::crawl::Config::load().await;
    let user = utils::crawl::register_user(&config).await.expect("TODO: panic message");
    println!("{:?}", user);
}