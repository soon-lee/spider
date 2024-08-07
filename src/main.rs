use roxmltree::Document;
use scraper::{Html, Selector};
use serde_json::json;

mod core;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let action = json!({
        "url":"https://www.toptoon2.com/page/search/超级"
    });
    let url = "https://www.san404.com";
    let response = client
        .get(url)
        .send()
        .await
        .unwrap();
    let text = response.text().await.unwrap();
    let fragment = Html::parse_document(&text);
    let comic_selector = Selector::parse("section.container div.content-wrap div.content div.pb div.box").unwrap();
    let mut comics = vec![comic_selector];
    let pager_selector = Selector::parse("section.container div.content-wrap div.content div.pagination.pagination-multi ul li:last-child span").unwrap();
    let pager_text = fragment.select(&pager_selector).next().unwrap().text().collect::<Vec<_>>().join("#");
    let re = regex::Regex::new(r"(\d+)").unwrap();
    let pager_num = re.captures(&pager_text).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
    println!("{}", pager_num);
    // for i in 2..=pager_num {
    //     let url = format!("{}/page/{}", &url, i);
    //     let response = client
    //         .get(url)
    //         .send()
    //         .await
    //         .unwrap();
    //     let text = response.text().await.unwrap();
    //     let fragment = Html::parse_document(&text);
    //     let comic_selector = Selector::parse("section.container div.content-wrap div.content div.pb div.box").unwrap();
    //     comics.push(comic_selector);
    // }
    // for comic_selector in comics {
    //     println!("{:?}", &comic_selector);
    //     for comic in fragment.select(&comic_selector) {
    //         println!("{:?}", comic.select(&Selector::parse("header p").unwrap()).next().unwrap().text().collect::<Vec<_>>().join(" "));
    //         println!("{:?}", comic.select(&Selector::parse("header h2 a").unwrap()).next().unwrap().value().attr("href").unwrap());
    //         println!("{:?}", comic.select(&Selector::parse("header h2 a").unwrap()).next().unwrap().text().collect::<Vec<_>>().join(" "));
    //         println!("{:?}", comic.select(&Selector::parse("header span").unwrap()).next().unwrap().text().collect::<Vec<_>>().join(" "));
    //         println!("{:?}", comic.select(&Selector::parse("header small").unwrap()).next().unwrap().text().collect::<Vec<_>>().join(" "));
    //         println!("{:?}", comic.select(&Selector::parse("p.text-muted.time").unwrap()).next().unwrap().text().collect::<Vec<_>>().join(" "));
    //         println!("{:?}", comic.select(&Selector::parse("p.focus a img").unwrap()).next().unwrap().value().attr("src").unwrap());
    //         println!("{:?}", comic.select(&Selector::parse("p.note").unwrap()).next().unwrap().text().collect::<Vec<_>>().join(" "));
    //         println!("{:?}", comic.select(&Selector::parse("span.post-tags").unwrap()).next().unwrap().text().collect::<Vec<_>>().join(" "));
    //     }
    //     break;
    // }
    let num_extractor = core::engine::Extractor::new(action);
    let comic_mapper = core::engine::Mapper::new("section.container div.content-wrap div.content div.pb div.box".to_string(), "".to_string(), "comics".to_string(), true, vec![
        core::engine::Mapper::new("header p".to_string(), "text".to_string(), "title".to_string(), false, vec![]),
        core::engine::Mapper::new("header h2 a".to_string(), "@href".to_string(), "url".to_string(), false, vec![]),
        core::engine::Mapper::new("header h2 a".to_string(), "text".to_string(), "info1".to_string(), false, vec![]),
        core::engine::Mapper::new("header span".to_string(), "text".to_string(), "info2".to_string(), false, vec![]),
        core::engine::Mapper::new("header small".to_string(), "text".to_string(), "info3".to_string(), false, vec![]),
        core::engine::Mapper::new("p.text-muted.time".to_string(), "text".to_string(), "last_update".to_string(), false, vec![]),
        core::engine::Mapper::new("p.focus a img".to_string(), "@src".to_string(), "cover".to_string(), false, vec![]),
        core::engine::Mapper::new("p.note".to_string(), "text".to_string(), "note".to_string(), false, vec![]),
        core::engine::Mapper::new("span.post-tags".to_string(), "text".to_string(), "tags".to_string(), false, vec![]),
    ]);
    let comic_extractor = core::engine::Extractor::new("/".to_string(), "css".to_string());
    let spider = core::engine::Spider::new("https://www.san404.com".to_string(), vec![], "sansi".to_string());
    let comic_href = "https://www.san404.com/100895481.html";
    let response = client
        .get(comic_href)
        .send()
        .await
        .unwrap();
    let text = response.text().await.unwrap();

    let fragment = Html::parse_document(&text);
    let root = fragment.select(&Selector::parse("div.ip").unwrap());

    // let fragment = Html::parse_document(&text);
    // println!("{:?}", fragment.select(&Selector::parse("section.container div.content-wrap div.content header.article-header div.c-img img").unwrap()).next().unwrap().value().attr("src").unwrap());
    // println!("{:?}", fragment.select(&Selector::parse("section.container div.content-wrap div.content header.article-header span.dis").unwrap()).next().unwrap().text().collect::<Vec<_>>().join(" "));
    // println!("{:?}", fragment.select(&Selector::parse("section.container div.content-wrap div.content article.article-content div.article-paging a").unwrap()).into_iter().map(|x| x.value().attr("href").unwrap().to_string()).collect::<Vec<_>>());
    // println!("{:?}", fragment.select(&Selector::parse("section.container div.content-wrap div.content article.article-content p:not(.post-copyright) img").unwrap()).into_iter().map(|x| x.value().attr("src").unwrap().to_string()).collect::<Vec<_>>());
    // let pics = fragment.select(&Selector::parse("section.container div.content-wrap div.content article.article-content p:not(.post-copyright) img").unwrap()).collect::<Vec<_>>();
    // for (i, pic) in pics.iter().enumerate() {
    //     println!("{:?}", match pic.value().attr("srcset") {
    //         Some(x) => x,
    //         None => pic.value().attr("src").unwrap()
    //     });
    // }
    // let pic_url = "https://imgs.34img.com/2021/004/083552av13280.jpg";
    // let response = client
    //     .get(pic_url)
    //     .header("Referer", "https://www.san404.com/")
    //     // .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
    //     .send()
    //     .await
    //     .unwrap();
    // let bytes = response.bytes().await.unwrap();
    // std::fs::write("test.jpg", bytes).unwrap();
    // let pic_url = "https://imgs.34img.com/2021/004/083552av13280-123x300.jpg";
    // let response = client
    //     .get(pic_url)
    //     .header("Referer", "https://www.san404.com/")
    //     // .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
    //     .send()
    //     .await
    //     .unwrap();
    // let bytes = response.bytes().await.unwrap();
    // std::fs::write("test-123x300.jpg", bytes).unwrap();
}