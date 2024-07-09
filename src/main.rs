use crate::states::cloud::QiniuClient;
use crate::states::local::Config;
use crate::states::mysql::Book;
use crate::tasks::action::{
    category_list, chapter_content, comic_info, daily_sign, daily_work, pay_chapter, register_user,
    snapshot_list, task_list, user_info,
};

mod handlers;
mod services;
mod states;
mod tasks;
mod utils;

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
async fn test_xxmh()-> Result<(), String> {
    let qiniu = QiniuClient::construct()?;
    let config = Config::load().await?;
    let options = config.get_options().await?;
    let options = &options;
    let user = register_user(options).await.unwrap();
    println!("{:?}", user);
    let categories = category_list(options).await.unwrap();
    let mut category_id = 0;
    for i in &categories {
        if i.title == "韩漫" {
            category_id = i.id;
        }
    }
    println!("{:?}", categories);
    let tasks = task_list(user.id, options).await.unwrap();
    println!("{:?}", tasks);
    let task_list = tasks
        .iter()
        .filter(|x| {
            ["观看漫画30分钟", "阅读3本漫画", "点赞3次", "收藏3本漫画"].contains(&&*x.task_name)
        })
        .collect::<Vec<_>>();
    let comics = snapshot_list(category_id, 3, 8, options).await.unwrap();
    let mut max = 0;
    let mut comic = Book {
        id: "".to_string(),
        title: "".to_string(),
        author: "".to_string(),
        note: "".to_string(),
        pic: "".to_string(),
        big_pic: "".to_string(),
        praise_count: 0,
        click_count: 0,
        favorite_count: 0,
        over_type: "".to_string(),
        category_id: 0,
        sort: 0,
        tags: "".to_string(),
        chapters: vec![],
    };
    for i in comics {
        if i.click_count > max {
            max = i.click_count;
            comic = i.clone();
        }
    }
    println!("{:?}", comic);
    let comic = comic_info(comic.id.parse().unwrap(), 3, options)
        .await
        .unwrap();
    println!("{:?}", comic);
    let chapter_content = chapter_content(
        comic.chapters[0].id.clone().parse().unwrap(),
        user.id,
        options,
    )
    .await
    .unwrap();
    println!("{:?}", chapter_content);
    daily_sign(user.id, options).await.unwrap();
    for i in task_list {
        daily_work(i.task_no, user.id, options).await.unwrap();
    }
    let user = user_info(user.id, options).await.unwrap();
    println!("{:?}", user);
    pay_chapter(
        user.id,
        comic.id.parse().unwrap(),
        comic.chapters[9].id.clone().parse().unwrap(),
        options,
    )
    .await
    .unwrap();
    let chapter_content = crate::tasks::action::chapter_content(
        comic.chapters[9].id.clone().parse().unwrap(),
        user.id,
        options,
    )
    .await
    .unwrap();
    println!("{:?}", chapter_content);
    for item in &chapter_content {
        let client = reqwest::Client::new();
        let res = client.get(item).send().await.map_err(|_|"pic连接失败")?;
        let bytes = res.bytes().await.map_err(|_|"pic请求失败")?;
        let its: Vec<&str> = item.split("/").collect::<Vec<_>>();
        let name = its[its.len() - 2];
        qiniu.post_bytes(&bytes, name).await?;
        panic!("end")
    }
    let user = user_info(user.id, options).await.unwrap();
    println!("{:?}", user);
    Ok(())
}

#[tokio::test]
async fn test_xxmh_api()-> Result<(), String> {
    let config = Config::load().await?;
    let options = config.get_options().await?;
    let options = &options;
    let user = register_user(options).await.unwrap();
    println!("{:?}", user);
    Ok(())
}
