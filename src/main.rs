use crate::core::engine::{Action, Extractor, Mapper, Mapping, Spider};
use std::sync::Arc;

mod core;

#[tokio::main]
async fn main() {
    let toptoon = "https://www.toptoon2.com/page/search/超级";
    let spider = Spider::new(
        "sansiyule".to_string(),
        vec!["https://www.san404.com".to_string()],
        vec![],
        vec![],
        vec![],
    );
    let comics_extractor = Arc::new(Extractor::new(
        "漫画列表".to_string(),
        "CSS".to_string(),
        vec![Mapper::new(
            "comics".to_string(),
            "section.container div.content-wrap div.content div.pb".to_string(),
            true,
            Some(vec![
                Mapping::new(
                    "title".to_string(),
                    "header p".to_string(),
                    "text".to_string(),
                    None,
                ),
                Mapping::new(
                    "url".to_string(),
                    "header h2 a".to_string(),
                    "@href".to_string(),
                    None,
                ),
                Mapping::new(
                    "info1".to_string(),
                    "header h2 a".to_string(),
                    "text".to_string(),
                    None,
                ),
                Mapping::new(
                    "info2".to_string(),
                    "header span".to_string(),
                    "text".to_string(),
                    None,
                ),
                Mapping::new(
                    "info3".to_string(),
                    "header small".to_string(),
                    "text".to_string(),
                    None,
                ),
                Mapping::new(
                    "last_update".to_string(),
                    "p.text-muted.time".to_string(),
                    "text".to_string(),
                    None,
                ),
                Mapping::new(
                    "cover".to_string(),
                    "p.focus a img".to_string(),
                    "@src".to_string(),
                    None,
                ),
                Mapping::new(
                    "note".to_string(),
                    "p.note".to_string(),
                    "text".to_string(),
                    None,
                ),
                Mapping::new(
                    "tags".to_string(),
                    "span.post-tags".to_string(),
                    "text".to_string(),
                    None,
                ),
            ]),
            None,
        )],
    ));
    let pager_extractor = Arc::new(Extractor::new("列表分页".to_string(), "CSS".to_string(), vec![Mapper::new("comics".to_string(), "section.container div.content-wrap div.content div.pagination.pagination-multi ul li:last-child".to_string(), false, Some(vec![
        Mapping::new("num".to_string(), "span".to_string(), "text".to_string(), Some(r"(\d+)".to_string())),
    ]), None)]));
    let mut comics_actions = vec![Action::new(
        "GET".to_string(),
        "/".to_string(),
        None,
        None,
        None,
        None,
        comics_extractor.clone(),
    )];
    for i in 2..=10 {
        comics_actions.push(Action::new(
            "GET".to_string(),
            format!("/page/{}", i),
            None,
            None,
            None,
            None,
            comics_extractor.clone(),
        ));
    }
    let mut pager_actions = vec![Action::new(
        "GET".to_string(),
        "/".to_string(),
        None,
        None,
        None,
        None,
        pager_extractor.clone(),
    )];

    let comic_info_extractor = Arc::new(Extractor::new("漫画信息".to_string(), "CSS".to_string(), vec![
        Mapper::new("comic_info".to_string(), "/".to_string(), true, Some(vec![
            Mapping::new("cover".to_string(), "section.container div.content-wrap div.content header.article-header div.c-img img".to_string(), "@src".to_string(), None),
            Mapping::new("title".to_string(), "section.container div.content-wrap div.content header.article-header span.dis".to_string(), "text".to_string(), None),
            Mapping::new("author".to_string(), "section.container div.content-wrap div.content article.article-content div.article-paging a".to_string(), "text".to_string(), None),
            Mapping::new("status".to_string(), "section.container div.content-wrap div.content article.article-content p:not(.post-copyright) img".to_string(), "text".to_string(), None),
        ]), None)]));
    let comic_info_actions = vec![Action::new(
        "GET".to_string(),
        "/100895481.html".to_string(),
        None,
        None,
        None,
        None,
        comic_info_extractor.clone(),
    )];

    let html = reqwest::Client::new().get("https://www.san404.com").send().await.unwrap().text().await.unwrap();
    let pager_extractor = pager_extractor.clone();
    println!("{:?}", pager_extractor.extract(&html).unwrap());
    println!("{:?}", comics_extractor.extract(&html).unwrap());
}
