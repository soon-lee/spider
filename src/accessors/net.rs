use std::collections::HashMap;

use serde_json::{json, Value};

use crate::models::entity::{Book, Category, Chapter, Task, User};
use crate::models::transfer::{BookInfo, CategoryInfo, ItemInfo, SnapshotInfo, TaskInfo, UserInfo};
use crate::utils::crypt::{aes_decrypt, aes_encrypt, auth_path};
use crate::utils::datetime::timestamp_str;

pub(crate) async fn post_client(
    path: &str,
    data: &str,
    options: &HashMap<String, String>,
) -> Result<String, String> {
    let origin = options.get("origin").ok_or("缺少origin参数")?;
    let app_id = options.get("app_id").ok_or("缺少app_id参数")?;
    let template = options.get("template_str").ok_or("缺少template_str参数")?;
    let default_str = options.get("default_str").ok_or("缺少default_str参数")?;
    let aes_key = options.get("aes_key").ok_or("缺少aes_key参数")?;

    let encrypted_data = aes_encrypt(aes_key, data)?;

    let client = reqwest::Client::new();
    let authed_path = auth_path(path, template, default_str)?;
    let url = format!("{}{}", origin, authed_path);
    let response = client
        .post(&url)
        .header("Appid", app_id)
        .header("Content-Type", "application/json")
        .body(format!("{{\"data\":\"{}\"}}", encrypted_data))
        .send()
        .await
        .map_err(|err| format!("请求失败: {}", err))?;

    let json = response
        .json::<Value>()
        .await
        .map_err(|err| format!("json解析失败: {}", err))?;
    let success = json["success"]
        .as_bool()
        .ok_or("返回数据中没有success字段")?;
    if success {
        let decrypted_data = json["result"]
            .as_str()
            .ok_or("返回数据中没有data字段")
            .unwrap();
        let text = aes_decrypt(aes_key, decrypted_data)?;
        Ok(text)
    } else {
        Err(format!("请求{}时发生异常: 请求失败{}", url, json))
    }
}

pub(crate) async fn register_user(options: &HashMap<String, String>) -> Result<User, String> {
    let dev_type = options.get("dev_type").ok_or("缺少origin参数")?;

    let data = json!({
        "devType": dev_type,
        "timeStamp": timestamp_str()?
    })
        .to_string();

    let response_text = post_client("/api/user/regUser", &data, options).await?;

    let user_info: UserInfo = serde_json::from_str(&response_text)
        .map_err(|err| format!("UserInfo序列化失败: {}", err))?;

    let user = User::new(
        user_info
            .id()
            .parse::<u64>()
            .map_err(|err| format!("解析失败，{}", err))?,
        user_info.account().clone(),
        user_info.pwd().clone(),
        0,
    );
    Ok(user)
}

pub(crate) async fn user_info(
    user_id: &u64,
    options: &HashMap<String, String>,
) -> Result<User, String> {
    let data = json!({
        "userId": &user_id,
        "timeStamp": timestamp_str()?
    })
        .to_string();

    let response_text = post_client("/api/user/getUserInfo", &data, options).await?;

    let user_info: UserInfo = serde_json::from_str(&response_text)
        .map_err(|e| format!("从Json解析UserInfo失败: {}", e))?;

    let user = User::new(
        user_info
            .id()
            .parse::<u64>()
            .map_err(|err| format!("解析失败，{}", err))?,
        user_info.account().clone(),
        user_info.pwd().clone(),
        user_info.balance().clone(),
    );
    Ok(user)
}

pub(crate) async fn task_list(
    user_id: &u64,
    options: &HashMap<String, String>,
) -> Result<Vec<Task>, String> {
    let data = json!({
        "userId": &user_id,
        "timeStamp": timestamp_str()?
    })
        .to_string();

    let response_text = post_client("/api/user/getTaskList", &data, options).await?;

    let task_info_list: Vec<TaskInfo> = serde_json::from_str(&response_text)
        .map_err(|e| format!("从Json解析TaskInfo列表失败: {}", e))?;

    let task_list = task_info_list
        .into_iter()
        .map(|task_info| {
            Task::new(
                task_info.task_no().clone(),
                task_info.give_coin().clone(),
                task_info.task_name().clone(),
            )
        })
        .collect::<Vec<_>>();

    Ok(task_list)
}

pub(crate) async fn category_list(
    options: &HashMap<String, String>,
) -> Result<Vec<Category>, String> {
    let data = json!({
        "c": "yml",
        "timeStamp": timestamp_str()?
    })
        .to_string();

    let response_text = post_client("/api/h5/getCategory", &data, options).await?;

    let category_info_list: Vec<CategoryInfo> = serde_json::from_str(&response_text)
        .map_err(|err| format!("从Json解析CategoryInfo列表失败: {}", err))?;

    let category_list = category_info_list
        .into_iter()
        .map(|category_info| {
            Category::new(
                category_info
                    .id()
                    .parse::<u64>()
                    .map_err(|err| format!("解析失败，{}", err))
                    .unwrap(),
                category_info.title().clone(),
                category_info
                    .sort()
                    .parse::<u32>()
                    .map_err(|err| format!("解析失败，{}", err))
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();

    Ok(category_list)
}

pub(crate) async fn snapshot_list(
    category_id: &u64,
    page: &u32,
    limit: &u32,
    options: &HashMap<String, String>,
) -> Result<Vec<Book>, String> {
    let data = json!({
        "page": page,
        "limit": limit,
        "category_id": category_id,
        "timeStamp": timestamp_str()?
    })
        .to_string();

    let response_text = post_client("/api/h5/getComicByCategoryId", &data, options).await?;

    let snapshot_info: SnapshotInfo = serde_json::from_str(&response_text)
        .map_err(|err| format!("从Json解析SnapshotInfo失败: {}", err))?;
    let book_list = snapshot_info
        .records()
        .iter()
        .map(|snapshot_info| {
            Book::new(
                snapshot_info
                    .id()
                    .parse::<u64>()
                    .map_err(|err| format!("解析失败，{}", err))
                    .unwrap(),
                snapshot_info.title().clone(),
                snapshot_info.author().clone(),
                snapshot_info.note().clone(),
                snapshot_info.pic().clone(),
                snapshot_info.big_pic().clone(),
                0,
                snapshot_info.click_count().clone(),
                0,
                snapshot_info.over_type_dict_text().clone(),
                snapshot_info
                    .category_id()
                    .parse::<u64>()
                    .map_err(|err| format!("解析失败，{}", err))
                    .unwrap(),
                0,
                snapshot_info.tags().clone(),
                vec![],
            )
        })
        .collect::<Vec<_>>();
    Ok(book_list)
}

pub(crate) async fn comic_info(
    comic_id: &u64,
    limit: &u32,
    options: &HashMap<String, String>,
) -> Result<Book, String> {
    let data = json!({
        "comicId": comic_id,
        "limit": limit,
        "timeStamp": timestamp_str()?
    })
        .to_string();

    let response_text = post_client("/api/h5/getComicInfo", &data, options).await?;
    
    let book_info: BookInfo = serde_json::from_str(&response_text)
        .map_err(|err| format!("从Json解析SnapshotInfo失败: {}", err))?;

    let book = Book::new(
        book_info
            .id()
            .parse::<u64>()
            .map_err(|err| format!("解析失败，{}", err))?,
        book_info.title().clone(),
        book_info.author().clone(),
        book_info.note().clone(),
        book_info.pic().clone(),
        book_info.big_pic().clone(),
        book_info.praise_count().clone(),
        book_info.click_count().clone(),
        book_info.fav_count().clone(),
        "".parse().unwrap(),
        book_info
            .category_id()
            .parse::<u64>()
            .map_err(|err| format!("解析失败，{}", err))
            .unwrap(),
        book_info.sort().clone(),
        book_info.tags().clone(),
        book_info
            .ext()
            .iter()
            .map(|chapter_info| {
                Chapter::new(
                    chapter_info
                        .id()
                        .parse::<u64>()
                        .map_err(|err| format!("解析失败，{}", err))
                        .unwrap(),
                    book_info
                        .id()
                        .parse::<u64>()
                        .map_err(|err| format!("解析失败，{}", err))
                        .unwrap(),
                    chapter_info.title().clone(),
                    chapter_info.pic().clone(),
                    chapter_info
                        .sort().clone(),
                    chapter_info.price().clone(),
                    vec![],
                )
            })
            .collect::<Vec<_>>(),
    );
    Ok(book)
}

pub(crate) async fn chapter_content(
    chapter_id: &u64,
    user_id: &u64,
    options: &HashMap<String, String>,
) -> Result<Vec<String>, String> {
    let data = json!({
        "chapterId": chapter_id,
        "userId": &user_id,
        "timeStamp": timestamp_str()?
    })
        .to_string();
    let response_text = post_client("/api/h5/getChapterContent", &data, options).await?;
    let item_info: ItemInfo = serde_json::from_str(&response_text)
        .map_err(|err| format!("从Json解析SnapshotInfo失败: {}", err))?;
    Ok(item_info.content().clone())
}

pub(crate) async fn pay_chapter(
    user_id: &u64,
    comic_id: &u64,
    chapter_id: &u64,
    options: &HashMap<String, String>,
) -> Result<(), String> {
    let data = json!({
        "userId": &user_id,
        "comicId": comic_id,
        "chapterId": chapter_id,
        "timeStamp": timestamp_str()?
    })
        .to_string();
    let _ = post_client("/api/user/coinPay", &data, options)
        .await
        .map_err(|err| format!("用户{}支付失败: {}", user_id, err))?;
    Ok(())
}

pub(crate) async fn daily_sign(
    user_id: &u64,
    options: &HashMap<String, String>,
) -> Result<(), String> {
    let data = json!({
        "userId": &user_id,
        "timeStamp": timestamp_str()?
    })
        .to_string();

    let _ = post_client("/api/user/checkSign", &data, options).await?;
    Ok(())
}

pub(crate) async fn daily_work(
    task_no: &u8,
    user_id: &u64,
    options: &HashMap<String, String>,
) -> Result<(), String> {
    let data = json!({
        "userId": &user_id,
        "task_no": task_no,
        "timeStamp": timestamp_str()?
    })
        .to_string();

    let _ = post_client("/api/user/getTaskReward", &data, options).await?;
    Ok(())
}
