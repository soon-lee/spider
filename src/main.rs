use std::collections::HashMap;
use std::time::Duration;

use base64::Engine;
use chrono::{DateTime, Local};

use crate::states::cloud::QiniuClient;
use crate::states::local::Config;
use crate::states::mysql::{Book, MySqlClient, User};
use crate::tasks::action::{
    category_list, chapter_content, comic_info, daily_sign, daily_work, pay_chapter, register_user,
    snapshot_list, task_list, user_info,
};

mod handlers;
mod services;
mod states;
mod tasks;
mod utils;
mod routers;

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
async fn test_xxmh() -> Result<(), String> {
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
        id: 0,
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
    let comic = comic_info(comic.id, 3, options)
        .await
        .unwrap();
    println!("{:?}", comic);
    let chapter_content = chapter_content(
        comic.chapters[0].id,
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
        comic.id,
        comic.chapters[9].id,
        options,
    )
        .await
        .unwrap();
    let chapter_content = crate::tasks::action::chapter_content(
        comic.chapters[9].id,
        user.id,
        options,
    )
        .await
        .unwrap();
    println!("{:?}", chapter_content);
    for item in &chapter_content {
        let client = reqwest::Client::new();
        let res = client.get(item).send().await.map_err(|_| "pic连接失败")?;
        let bytes = res.bytes().await.map_err(|_| "pic请求失败")?;
        let magic = &bytes[2..4];
        println!("magic: {:?}", magic);
        if magic == &[0xff, 0xd8] {
            println!("jpg");
        } else if magic == &[0x89, 0x50] {
            println!("png");
        } else if magic == &[0x47, 0x49] {
            println!("gif");
        } else if magic == &[0x42, 0x4d] {
            println!("bmp");
        } else if magic == &[0xff, 0xfe] {
            println!("utf-16");
        }
        let l = bytes.len();
        let bytes = &bytes[2..l];
        println!("data:image/jpeg;base64,{}", base64::engine::general_purpose::STANDARD.encode(&bytes));
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
async fn test_xxmh_book() {
    async fn pay_user(options: &HashMap<String, String>) -> User {
        let user = register_user(&options).await.unwrap();
        let tasks = task_list(user.id, &options).await.unwrap();
        let task_list = tasks
            .iter()
            .filter(|x| {
                ["观看漫画30分钟", "阅读3本漫画", "点赞3次", "收藏3本漫画"].contains(&&*x.task_name)
            })
            .collect::<Vec<_>>();
        daily_sign(user.id, &options).await.unwrap();
        for i in task_list {
            daily_work(i.task_no, user.id, &options).await.unwrap();
        }
        user
    }
    fn object_from_url(url: &str, id: u64) -> Result<String, String> {
        let offset = url.find(format!("/{}", id).as_str()).ok_or(format!("{}中，无法定位{}", url, id))?;
        let url = &url[offset..];
        Ok(url.parse().unwrap())
    }
    let mysql = MySqlClient::new().await.unwrap();
    let qiniu = QiniuClient::construct().unwrap();
    let config = Config::load().await.unwrap();
    let options = config.get_options().await.unwrap();
    let mut comic = comic_info(1982, 3, &options).await.unwrap();
    let object = format!("comic/xxmh{}", object_from_url(&comic.pic, comic.id).unwrap());
    let result = qiniu.post_bytes(&comic.pic.as_bytes(), object.as_str()).await.unwrap();
    comic.pic = result.get("key").unwrap().to_string().trim_matches('"').parse().unwrap();
    let object = format!("comic/xxmh{}", object_from_url(&comic.big_pic, comic.id).unwrap());
    let result = qiniu.post_bytes(&comic.big_pic.as_bytes(), object.as_str()).await.unwrap();
    comic.big_pic = result.get("key").unwrap().to_string().trim_matches('"').parse().unwrap();
    mysql.add_book(comic.clone()).await.unwrap();
    let mut count = 0;
    let mut user = User {
        id: 0,
        username: "".to_string(),
        password: "".to_string(),
        balance: 0,
    };
    for mut chapter in comic.chapters.clone() {
        let object = format!("comic/xxmh/{}{}", &comic.id, object_from_url(&chapter.pic, chapter.id).unwrap());
        let result = qiniu.post_bytes(&chapter.pic.as_bytes(), object.as_str()).await.unwrap();
        chapter.pic = result.get("key").unwrap().to_string().trim_matches('"').parse().unwrap();
        if count % 3 == 0 {
            user = pay_user(&options).await;
        }
        count += 1;
        pay_chapter(user.id, comic.id, chapter.id, &options).await.unwrap();

        let content = chapter_content(chapter.id, user.id, &options).await.unwrap();
        let mut items = vec![];
        for item in content {
            let object = format!("comic/xxmh/{}/{}{}", &comic.id, &chapter.id, object_from_url(&item, chapter.id).unwrap());
            let result = qiniu.post_bytes(&chapter.pic.as_bytes(), object.as_str()).await.unwrap();
            items.push(result.get("key").unwrap().to_string().trim_matches('"').parse().unwrap());
        }
        chapter.items = items;
        // mysql.add_chapter(chapter).await.unwrap();
    }
}
#[tokio::test]
async fn test_xxmh_user() {
    let config = Config::load().await.unwrap();
    let options = config.get_options().await.unwrap();
    let mut run = true;
    let mut delay = 100;
    let mut begin = std::time::SystemTime::now();
    let mut count = 1u64;
    let local_now: DateTime<Local> = DateTime::from(begin);
    let formatted = local_now.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("开始时间: {}", formatted);
    while run {
        tokio::time::sleep(Duration::from_millis(delay)).await;
        match register_user(&options).await {
            Ok(user) => {
                let now = std::time::SystemTime::now();
                let local_now: DateTime<Local> = DateTime::from(now);
                let formatted = local_now.format("%Y-%m-%d %H:%M:%S").to_string();
                println!("{:<5}在{:?},注册成功: {:?} step:{:?}秒", count, formatted, user, now.duration_since(begin).unwrap().as_secs());
                begin = now;
            }
            Err(e) => {
                let now = std::time::SystemTime::now();
                let local_now: DateTime<Local> = DateTime::from(now);
                let formatted = local_now.format("%Y-%m-%d %H:%M:%S").to_string();
                println!("{:<5} 在{},注册失败 step:{:?}秒", count, formatted, now.duration_since(begin).unwrap().as_secs());
                begin = now;
                delay += 100;
            }
        }
        count += 1;
        if count > 50 {
            run = false;
        }
    }
}