mod handlers;
mod routes;
mod services;
mod utils;
mod tasks;
mod states;

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
    let task_list = utils::crawl::task_list(&config,1806329574235979778).await.expect("TODO: panic message");
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
#[tokio::test]
async fn test_book_info() {
    let config = utils::crawl::Config::load().await;
    let book = utils::crawl::comic_info(&config,1413).await.expect("TODO: panic message");
    println!("{:?}", book);
}
#[tokio::test]
async fn test_chapter_content() {
    let config = utils::crawl::Config::load().await;
    let chapter_id_list = [228247,228246,228245];
    let content_list = utils::crawl::chapter_content(&config,228247,1806329574235979778).await.expect("TODO: panic message");
    println!("{:?}", content_list);
}
#[tokio::test]
async fn test_daily_task() {
    let config = utils::crawl::Config::load().await;

    let user = utils::crawl::register_user(&config).await.expect("TODO: panic message");
    println!("{:?}", user);
    utils::crawl::daily_sign(&config,user.id).await.expect("TODO: panic message");
    match utils::crawl::daily_work(&config,1,user.id).await{
        Ok(o)=>match utils::crawl::daily_work(&config,2,user.id).await{
            Ok(o)=>match utils::crawl::daily_work(&config,4,user.id).await{
                Ok(o)=>match utils::crawl::daily_work(&config,6,user.id).await{
                    Ok(o)=>{},
                    Err(err) => println!("6 {}",err)
                },
                Err(err) => println!("4 {}",err)
            },
            Err(err) => println!("2 {}",err)
        },
        Err(err) => println!("1 {}",err)
    }
    let user = utils::crawl::user_info(&config,user.id).await.expect("TODO: panic message");
    println!("{:?}", user);
}