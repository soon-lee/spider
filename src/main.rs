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
async fn test_register_user() {
    let config = utils::crawl::Config::load().await;
    let user = utils::crawl::register_user(&config).await.expect("TODO: panic message");
    println!("{:?}", user);
}
#[tokio::test]
async fn test_task_list() {
    let config = utils::crawl::Config::load().await;
    let task_list = utils::crawl::task_list(&config,&"1806329574235979778".to_string()).await.expect("TODO: panic message");
    println!("{:?}", task_list);
}
#[tokio::test]
async fn test_category_list() {
    let config = utils::crawl::Config::load().await;
    let category_list = utils::crawl::category_list(&config).await.expect("TODO: panic message");
    println!("{:?}", category_list);
}
#[tokio::test]
async fn test_snapshot_list() {
    let config = utils::crawl::Config::load().await;
    let snapshot_list = utils::crawl::snapshot_list(&config,1487722500401532929,10,8).await.expect("TODO: panic message");
    println!("{:?}", snapshot_list);
}