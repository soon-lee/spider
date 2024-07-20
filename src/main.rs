use crate::accessors::cloud::QiniuClient;
use crate::accessors::db::MySqlClient;
use crate::accessors::local::Config;
use crate::accessors::net::{category_list, comic_info, snapshot_list};

mod handlers;
mod services;
mod utils;
mod routers;
mod models;
mod accessors;

#[tokio::main]
async fn main() {
    let config = Config::load().await.unwrap();
    let options = config.get_options().await.unwrap();
    let mysql = MySqlClient::new().await.unwrap();
    let qiniu = QiniuClient::construct().unwrap();

    let categories = category_list(&options).await.unwrap();
    let snapshots = snapshot_list(categories[0].id(), &3, &10, &options).await.unwrap();
    let book = comic_info(&snapshots[0].id(), &10, &options).await.unwrap();
    println!("{:?}", book);

    // let app = routes::routes();
    // let host = std::env::var("AXUM_HOST").unwrap_or("0.0.0.0".to_string());
    // let port = std::env::var("AXUM_PORT").unwrap_or("8000".to_string());
    // let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
    //     .await
    //     .unwrap();
    // axum::serve(listener, app).await.unwrap();
}