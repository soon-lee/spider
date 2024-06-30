use std::collections::HashMap;

use reqwest::Error;
use serde_json::{json, Value};

use crate::states::local::Config;
use crate::states::mysql::{Book, Category, Task, User};
use crate::tasks::dto::{CategoryInfo, SnapshotInfo, TaskInfo, UserInfo};
use crate::utils::crypt::{aes_decrypt, aes_encrypt, auth_path};

pub(crate) async fn post_client(
    path: &str,
    data: &str,
    options: &HashMap<&str, &str>,
) -> Result<String, Error> {
    // 更安全地获取必要选项，避免unwrap可能导致的panic
    let origin = options.get("origin").ok_or("缺少origin参数")?;
    let app_id = options.get("app_id").ok_or("缺少app_id参数")?;
    let template = options.get("template_str").ok_or("缺少template_str参数")?;
    let aes_key = options.get("aes_key").ok_or("缺少aes_key参数")?;

    // 加密数据
    let encrypted_data = aes_encrypt(aes_key, data)?;

    // 初始化客户端，这里可以在外部初始化一次复用，但为了示例清晰放在此处
    let client = reqwest::Client::new();

    // 构建完整URL
    let url = format!("{}{}", origin, auth_path(path, template));

    // 发送POST请求
    let response = client
        .post(&url)
        .header("Appid", app_id)
        .body(format!("{{\"data\":\"{}\"}}", encrypted_data))
        .send()
        .await?;

    // 处理响应
    let json = response.json::<Value>().await?;

    // 根据success字段判断请求结果
    match json["success"].as_bool() {
        Some(true) => {
            // 解密返回的数据
            let decrypted_data = json["data"].as_str().ok_or("返回数据中未找到data字段")?;
            aes_decrypt(aes_key, decrypted_data).map_err(Into::into)
        },
        _ => {
            eprintln!("请求{}时发生异常: 请求失败", url);
            Err("请求失败".into())
        }
    }
}

/**- name: "注册用户"
  method: "POST"
  path: "/api/user/regUser"
  data:
    "devType": ""
    "timeStamp": ""
*/
pub(crate) async fn register_user(options: &HashMap<&str, &str>) -> Result<User, String> {
    // Improved error handling with a helper function
    let dev_type = options
        .get("dev_type")
        .ok_or_else(|| "Missing 'dev_type' in options");

    let data = json!({
        "devType": dev_type,
        "timeStamp": timestamp_str()
    })
    .to_string();

    // Use constant for API path
    let response_text = post_client("/api/user/regUser" ,&data, options)
        .await
        .map_err(|err| err.to_string())?;

    // Deserialize JSON and handle potential errors
    let user_info: UserInfo = serde_json::from_str(&response_text)
        .map_err(|err| format!("Failed to deserialize user info: {}", err))?;

    // Parse user ID and handle parsing errors
    let user_id = user_info
        .id
        .parse::<u64>()
        .map_err(|_| "Failed to parse user id to u64".to_string())?;

    // Create and return User instance
    let user = User::new(user_id, user_info.account, user_info.pwd, 0);
    Ok(user)
}


/*
- name: "用户信息"
  method: "POST"
  path: "/api/user/getUserInfo"
  data:
    "userId": ""
    "timeStamp": ""
    */
pub(crate) async fn user_info(user_id: u64, options: &HashMap<&str, &str>) -> Result<User, String> {
    let data = json!({
        "userId": user_id,
        "timeStamp": timestamp_str()
    })
    .to_string();

    // 使用 ? 运算符传递错误，替代 unwrap
    let response_text = post_client("/api/user/getUserInfo", &data, options)
        .await?;

    // 使用 map_err 转换错误类型，并提供清晰的错误信息
    let user_info: UserInfo = serde_json::from_str(&response_text)
        .map_err(|e| format!("Failed to parse JSON response: {}", e))?;

    // 确保 id 字段可以转换为 u64，使用 map_err 处理可能的错误
    let parsed_user_id = user_info.id.parse::<u64>()
        .map_err(|_| "Failed to parse user id from string".to_string())?;

    // 确认解析的用户ID与传入的用户ID匹配，这里假设这是必要的逻辑验证
    if parsed_user_id != user_id {
        return Err("User ID mismatch between request and response".to_string());
    }

    // 创建并返回 User 实例
    let user = User::new(parsed_user_id, user_info.account, user_info.pwd, user_info.balance);
    Ok(user)
}

/*
- name: "任务列表"
  method: "POST"
  path: "/api/user/getTaskList"
  data:
    "userId": ""
    "timeStamp": ""*/
pub(crate) async fn task_list(user_id: u64, options: &HashMap<&str, &str>) -> Result<Vec<Task>, String> {
    let data = json!({
        "userId": user_id,
        "timeStamp": timestamp_str()
    })
    .to_string();

    // 使用 ? 运算符来处理错误，替代 unwrap
    let response_text = post_client("/api/user/getTaskList", &data, options)
        .await?;

    // 使用 map_err 转换错误类型，提供更明确的错误信息
    let task_info_list: Vec<TaskInfo> = serde_json::from_str(&response_text)
        .map_err(|e| format!("Failed to parse task list: {}", e))?;

    // 使用 map 而不是 iter().map().collect()，以链式处理错误
    let task_list = task_info_list.into_iter()
        .map(|task_info| {
            Task::new(
                task_info.taskNo,
                task_info.giveCoin,
                task_info.taskName.clone(),
            )
        })
        // 收集转换过程中的所有结果，并处理可能的错误
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Error creating tasks: {}", e))?;

    Ok(task_list)
}

/*
- name: "漫画分类"
  method: "POST"
  path: "/api/h5/getCategory"
  data:
    "c": "yml"
    "timeStamp": ""
*/
pub(crate) async fn category_list(options: &HashMap<&str, &str>) -> Result<Vec<Category>, String> {
    let data = json!({
        "c": "yml",
        "timeStamp": timestamp_str()
    })
    .to_string();

    let response_text = post_client("/api/h5/getCategory", &data, options)
        .await.map_err(|err| format!("Failed to fetch category list: {}", err))?;

    let category_info_list: Vec<CategoryInfo> = serde_json::from_str(&response_text)
        .map_err(|err| format!("Failed to parse category info: {}", err))?;

    let category_list = category_info_list.into_iter()
        .map(|category_info| {
            let id = category_info.id.parse::<u64>()
                .map_err(|err| format!("Error parsing category ID: {}", err))?;
            let sort = category_info.sort.parse::<u32>()
                .map_err(|err| format!("Error parsing category sort: {}", err))?;

            Ok(Category::new(id, category_info.title.clone(), sort))
        })
        .collect::<Result<Vec<_>, String>>()?;

    Ok(category_list)
}

/*
- name: "分类查询"
  method: "POST"
  path: "/api/h5/getComicByCategoryId"
  data:
    "page": ""
    "limit": ""
    "categoryId": ""
    "timeStamp": ""*/
pub(crate) async fn snapshot_list(
    category_id: u64,
    page: u32,
    limit: u32,options: &HashMap<&str, &str>
) -> Result<Vec<Book>, ()> {
    let data = json!({
        "page": page,
        "limit": limit,
        "categoryId": category_id,
        "timeStamp": timestamp_str()
    })
        .to_string();

    let response_text = post_client("/api/h5/getCategory", &data, options)
        .await.unwrap();
    let snapshot_info: SnapshotInfo = serde_json::from_str(
        &response_text
    )
        .unwrap();
    let book_list = snapshot_info
        .records
        .iter()
        .map(|snapshot_info| {
            Book::new(
                snapshot_info.id.clone(),
                snapshot_info.title.clone(),
                snapshot_info.author.clone(),
                snapshot_info.note.clone(),
                snapshot_info.pic.clone(),
                snapshot_info.bigPic.clone(),
                0,
                snapshot_info.clickCount,
                0,
                snapshot_info.overType_dictText.clone(),
                snapshot_info.categoryId.parse::<u64>().unwrap(),
                0,
                snapshot_info.tags.clone(),
                Vec::new(),
            )
        })
        .collect::<Vec<_>>();
    Ok(book_list)
}
/*
- name: "漫画信息"
  method: "POST"
  path: "/api/h5/getComicInfo"
  data:
    "comicId": ""
    "limit": ""
    "timeStamp": ""*/
pub(crate) async fn comic_info(config: &Config, comic_id: u64) -> Result<Book, ()> {
    let Config { crawl, actions } = config;
    let action_name = "漫画信息";
    let mut data = actions
        .group
        .iter()
        .find(|action| action.name == action_name)
        .unwrap()
        .data
        .clone();
    data.insert("comicId".to_string(), comic_id.to_string());
    data.insert("limit".to_string(), "5".to_string());
    data.insert("timeStamp".to_string(), timestamp_str());
    let data = crawl.encrypt(&serde_json::to_string(&data).unwrap());
    let response = config.client_post(action_name, &data).await;
    if response.status().is_success() {
        let json = &response.json::<Value>().await.unwrap();
        if json["code"] == 0 {
            let book_info: BookInfo = serde_json::from_str(
                crawl
                    .decrypt(&json["result"].to_string().trim_matches('"').to_string())
                    .as_str(),
            )
            .unwrap();
            let book = Book::new(
                book_info.id.clone(),
                book_info.title.clone(),
                book_info.author.clone(),
                book_info.note.clone(),
                book_info.pic.clone(),
                book_info.bigPic.clone(),
                book_info.praiseCount,
                book_info.clickCount,
                book_info.favCount,
                "".parse().unwrap(),
                book_info.categoryId.parse::<u64>().unwrap(),
                book_info.sort,
                book_info.tags.clone(),
                book_info
                    .ext
                    .iter()
                    .map(|chapter_info| {
                        Chapter::new(
                            chapter_info.id.clone(),
                            chapter_info.title.clone(),
                            chapter_info.pic.clone(),
                            chapter_info.sort,
                            chapter_info.price,
                            vec![],
                        )
                    })
                    .collect::<Vec<_>>(),
            );
            Ok(book)
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}
/*
- name: "章节内容"
  method: "POST"
  path: "/api/h5/getChapterContent"
  data:
    "chapterId": ""
    "userId": ""
    "timeStamp": ""*/
pub(crate) async fn chapter_content(
    config: &Config,
    chapter_id: u64,
    user_id: u64,
) -> Result<Vec<String>, ()> {
    let Config { crawl, actions } = config;
    let action_name = "章节内容";
    let mut data = actions
        .group
        .iter()
        .find(|action| action.name == action_name)
        .unwrap()
        .data
        .clone();
    data.insert("chapterId".to_string(), chapter_id.to_string());
    data.insert("userId".to_string(), user_id.to_string());
    data.insert("timeStamp".to_string(), timestamp_str());
    let response = config
        .client_post(
            action_name,
            &crawl.encrypt(&serde_json::to_string(&data).unwrap()),
        )
        .await;
    if response.status().is_success() {
        let json = &response.json::<Value>().await.unwrap();
        if json["code"] == 0 {
            let item_info: ItemInfo = serde_json::from_str(
                crawl
                    .decrypt(&json["result"].to_string().trim_matches('"').to_string())
                    .as_str(),
            )
            .unwrap();
            Ok(item_info.content)
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}
/**- name: "购买章节"
 method: "POST"
 path: "/api/user/coinPay"
 data:
   "userId": ""
   "comicId": ""
   "chapterId": ""
   "timeStamp": ""
*/
pub(crate) async fn pay_chapter(
    config: &Config,
    user_id: u64,
    comic_id: u64,
    chapter_id: u64,
) -> Result<(), ()> {
    let Config { crawl, actions } = config;
    let action_name = "购买章节";
    let mut data = actions
        .group
        .iter()
        .find(|action| action.name == action_name)
        .unwrap()
        .data
        .clone();
    data.insert("userId".to_string(), user_id.to_string());
    data.insert("comicId".to_string(), comic_id.to_string());
    data.insert("chapterId".to_string(), chapter_id.to_string());
    data.insert("timeStamp".to_string(), timestamp_str());
    let response = config
        .client_post(
            action_name,
            &crawl.encrypt(&serde_json::to_string(&data).unwrap()),
        )
        .await;
    if response.status().is_success() {
        let json = &response.json::<Value>().await.unwrap();
        if json["code"] == 0 {
            Ok(())
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}
/*
- name: "每日签到"
  method: "POST"
  path: "/api/user/checkSign"
  data:
    "userId": ""
    "timeStamp": ""*/
pub(crate) async fn daily_sign(config: &Config, user_id: u64) -> Result<(), ()> {
    let Config { crawl, actions } = config;
    let action_name = "每日签到";
    let mut data = actions
        .group
        .iter()
        .find(|action| action.name == action_name)
        .unwrap()
        .data
        .clone();
    data.insert("userId".to_string(), user_id.to_string());
    data.insert("timeStamp".to_string(), timestamp_str());
    let response = config
        .client_post(
            action_name,
            &crawl.encrypt(&serde_json::to_string(&data).unwrap()),
        )
        .await;
    if response.status().is_success() {
        let json = &response.json::<Value>().await.unwrap();
        if json["code"] == 0 {
            Ok(())
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}
/*
- name: "任务奖励"
  method: "POST"
  path: "/api/user/getTaskReward"
  data:
    "userId": ""
    "taskNo": ""
    "timeStamp": ""*/
pub(crate) async fn daily_work(config: &Config, task_no: u8, user_id: u64) -> Result<(), String> {
    let Config { crawl, actions } = config;
    let action_name = "任务奖励";
    let mut data = actions
        .group
        .iter()
        .find(|action| action.name == action_name)
        .unwrap()
        .data
        .clone();
    data.insert("taskNo".to_string(), task_no.to_string());
    data.insert("userId".to_string(), user_id.to_string());
    data.insert("timeStamp".to_string(), timestamp_str());
    let response = config
        .client_post(
            action_name,
            &crawl.encrypt(&serde_json::to_string(&data).unwrap()),
        )
        .await;
    if response.status().is_success() {
        let json = &response.json::<Value>().await.unwrap();
        println!("{}", json["result"]);
        println!("{}", json);
        if json["code"] == 0 {
            Ok(())
        } else {
            Err("return error".to_string())
        }
    } else {
        Err("post error".to_string())
    }
}
