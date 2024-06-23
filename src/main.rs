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
    let mut config = utils::info::Config::load();
    println!("{:?}", config);
    config.crawl.get_scripts().await;
    config.crawl.extract_options_from_scripts().await;
    println!("{:?}", config);
    utils::info::register_user(&config.crawl).await;
}