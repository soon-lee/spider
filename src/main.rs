use std::sync::Arc;

mod handlers;
mod services;
mod utils;
mod routers;
mod models;
mod accessors;

#[tokio::main]
async fn main() {
    // let app = routes::routes();
    // let host = std::env::var("AXUM_HOST").unwrap_or("0.0.0.0".to_string());
    // let port = std::env::var("AXUM_PORT").unwrap_or("8000".to_string());
    // let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
    //     .await
    //     .unwrap();
    // axum::serve(listener, app).await.unwrap();
}
#[tokio::test]
async fn test_net() {
    let config = crate::utils::global::Config::load().await.unwrap();
    let options = config.get_options().await.unwrap();
    let net_client = crate::utils::global::NetClient::new(options);
    let user_net_accessor = crate::accessors::net::UserNetAccessor::new(std::sync::Arc::new(net_client));
    let user = user_net_accessor.user_info(&1809542822452150274u64).await.unwrap();
    println!("{:?}", user);
}
#[tokio::test]
async fn test_db() {
    let mysql = crate::utils::global::MySqlClient::new().await.unwrap();
    let connection = Arc::new(mysql.get_connection().await.unwrap());
    let user_db_accessor = crate::accessors::db::UserDbAccessor::new(connection.clone());
    let users = user_db_accessor.get_users().await.unwrap();
    println!("{:?}", users);
    let book_db_accessor = crate::accessors::db::BookDbAccessor::new(connection.clone());
    let books = book_db_accessor.get_books().await.unwrap();
    let book = book_db_accessor.get_book_by_id(&1982u64).await.unwrap();
    println!("{:?}", book);
}