use crate::core::spider::Spider;

mod core;

#[tokio::main]
async fn main() {
    let spider = Spider::load("src/spider.json".to_string()).unwrap();
    spider.run().await.expect("err");
}
