use std::collections::HashMap;

use base64::Engine;
use rand::Rng;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::utils::mysql::{Book, Category, Chapter, Task, User};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UserInfo {
    id: String,
    account: String,
    pwd: String,
    nickName: String,
    devType: u8,
    devCode: String,
    lastLoginIp: String,
    lastLoginTime: String,
    createTime: String,
    appId: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TaskInfo {
    id: String,
    taskNo: u8,
    taskType: u8,
    triggerValue: u8,
    giveCoin: u8,
    giveVip: u8,
    hrefUrl: String,
    createTime: String,
    ext: u8,
    taskName: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CategoryInfo {
    id: String,
    title: String,
    status: u8,
    sort: String,
    createBy: String,
    createTime: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SnapshotBook {
    note: String,
    clickCount: u64,
    isSyn: u8,
    pic: String,
    title: String,
    overType_dictText: String,
    categoryId_dictText: String,
    bigPic: String,
    id: String,
    author: String,
    overType: u8,
    tags: String,
    categoryId: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SnapshotInfo {
    records:Vec<SnapshotBook>
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ChapterInfo {
    id: String,
    title: String,
    pic: String,
    sort: u32,
    price: u32,
    isSyn: u8,
    createTime: String,
    feel: u8,
    payMode: u8,
    formatTime: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct BookInfo {
    id: String,
    title: String,
    pic: String,
    bigPic: String,
    author: String,
    note: String,
    payMode: u8,
    feelCount: u8,
    payCoin: u8,
    praiseCount: u64,
    clickCount: u64,
    favCount: u64,
    sales: u8,
    payTotal: u8,
    overType: u8,
    categoryId: String,
    isSyn: u8,
    sort: u32,
    status: u8,
    tags: String,
    indexCol: String,
    createTime: String,
    updateTime: String,
    ext: Vec<ChapterInfo>,
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ItemInfo {
    content:Vec<String>
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CrawlConfig {
    origins: Vec<String>,
    scripts: Vec<String>,
    options: HashMap<String, String>,
    configs: HashMap<String, String>,
}

impl CrawlConfig {
    pub(crate) async fn get_scripts(&mut self) {
        for origin in self.origins.iter().as_ref() {
            let client = reqwest::Client::new();
            let response = client.get(origin).send().await;
            match response {
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
                Ok(response) => {
                    if response.status().is_success() {
                        let html = response.text().await.unwrap();
                        let root = scraper::Html::parse_document(&html);
                        let script_selector = scraper::Selector::parse("script").unwrap();
                        root.select(&script_selector)
                            .filter(|script| script.value().attr("src").is_some())
                            .for_each(|script| {
                                self.scripts
                                    .push(script.value().attr("src").unwrap().to_string())
                            });
                        break;
                    } else {
                        println!("Error: {}", response.status());
                        continue;
                    }
                }
            }
        }
    }
    pub(crate) async fn extract_options_from_scripts(&mut self) {
        for script in &self.scripts {
            let client = reqwest::Client::new();
            let res = client.get(script).send().await.unwrap();
            let html = res.text().await.unwrap();
            self.options.iter().for_each(|(key, value)| {
                let pattern = regex::Regex::new(value).unwrap();
                if let Some(captures) = pattern.captures(&html) {
                    self.configs.insert(key.clone(), captures[1].to_string());
                };
            });
        }
    }

    pub(crate) fn random_str(&self) -> String {
        self.configs
            .get("template_str")
            .unwrap()
            .chars()
            .map(|c| {
                let mut rander = rand::thread_rng();
                let n = rander.gen_range(0..16);
                match c {
                    'x' => format!("{:x}", n),
                    'y' => format!("{:x}", (n & 0x3 | 0x8)),
                    _ => c.to_string(),
                }
            })
            .collect::<Vec<_>>()
            .join("")
    }
    pub(crate) fn fill_path(path: &String) -> String {
        let mut result = path.clone();
        if !path.starts_with("/") {
            result.insert(0, '/');
        }
        result.insert_str(0, "/api");
        result
    }
    pub(crate) fn path_hash(&self, path: &String, timestamp_10: u128) -> String {
        let text = format!(
            "{}-{}-{}-0-{}",
            path.clone(),
            timestamp_10,
            self.random_str(),
            self.configs.get("template_str").unwrap()
        );
        format!("{:x}", md5::compute(text))
    }
    pub(crate) fn auth_path(&self, path: &String) -> String {
        let concat = match path.contains("?") {
            true => "&",
            false => "?",
        };
        let timestamp_10 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_millis())
            .unwrap()
            / 1000;
        let random_str = self.random_str();
        let hash = self.path_hash(path, timestamp_10);
        format!(
            "{}{}cpt_auth={}-{}-0-{}",
            path.clone(),
            concat,
            timestamp_10,
            random_str,
            hash
        )
    }
    pub(crate) fn encrypt(&self, data: &String) -> String {
        let bytes = soft_aes::aes::aes_enc_ecb(
            data.as_bytes(),
            self.configs.get("aes_key").unwrap().as_bytes(),
            Some("PKCS7"),
        )
        .unwrap();
        base64::engine::general_purpose::STANDARD.encode(&bytes)
    }
    pub(crate) fn decrypt(&self, data: &String) -> String {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(data)
            .unwrap();
        let bytes = soft_aes::aes::aes_dec_ecb(
            &*bytes,
            self.configs.get("aes_key").unwrap().as_bytes(),
            Some("PKCS7"),
        )
        .unwrap();
        String::from_utf8(bytes).unwrap()
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Action {
    name: String,
    method: String,
    path: String,
    data: HashMap<String, String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Actions {
    origin: String,
    headers: HashMap<String, String>,
    group: Vec<Action>,
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) crawl: CrawlConfig,
    pub(crate) actions: Actions,
}
impl Config {
    pub(crate) async fn load() -> Self {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR environment variable not found");
        let config_path = std::path::PathBuf::from(&manifest_dir).join(r#"configs\crawl.yaml"#);
        let yaml = std::fs::read_to_string(config_path).expect("Unable to read config file");
        let mut config: Config = serde_yaml::from_str(&yaml).expect("Unable to parse config file");
        config.crawl.get_scripts().await;
        config.crawl.extract_options_from_scripts().await;
        config
    }
    fn dump(&self) {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR environment variable not found");
        let config_path = std::path::PathBuf::from(&manifest_dir).join(r#"configs\crawl.yaml"#);
        let config = serde_yaml::to_string(&self).expect("Unable to serialize config");
        std::fs::write(config_path, config).expect("Unable to write config file");
    }
    pub(crate) async fn client_post(&self, action_name: &str, data: &String) -> Response {
        let client = reqwest::Client::new();
        let action = self
            .actions
            .group
            .iter()
            .find(|action| action.name == *action_name)
            .unwrap();
        let url = format!(
            "{}{}",
            self.actions.origin,
            self.crawl.auth_path(&action.path)
        );
        client
            .post(url)
            .header("Appid", self.crawl.configs.get("app_id").unwrap())
            .json(&json!({"data":data}))
            .send()
            .await
            .unwrap()
    }
}
fn timestamp_str() -> String {
    (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap()
        / 1000)
        .to_string()
}
pub(crate) async fn register_user(config: &Config) -> Result<User, ()> {
    let Config { crawl, actions } = config;
    let action_name = "注册用户";
    let mut data = actions
        .group
        .iter()
        .find(|action| action.name == action_name)
        .unwrap()
        .data
        .clone();
    data.insert(
        "devType".to_string(),
        crawl.configs.get("dev_type").unwrap().to_string(),
    );
    data.insert("timeStamp".to_string(), timestamp_str());
    let data = crawl.encrypt(&serde_json::to_string(&data).unwrap());
    let response = config.client_post(action_name, &data).await;
    if response.status().is_success() {
        let json = &response.json::<Value>().await.unwrap();
        if json["code"] == 0 {
            let user_info: UserInfo = serde_json::from_str(
                crawl
                    .decrypt(&json["result"].to_string().trim_matches('"').to_string())
                    .as_str(),
            )
            .unwrap();
            let user = User::new(
                user_info.id.parse::<u64>().unwrap(),
                user_info.account,
                user_info.pwd,
            );
            Ok(user)
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}
pub(crate) async fn task_list(config: &Config, user_id: &String) -> Result<Vec<Task>, ()> {
    let Config { crawl, actions } = config;
    let action_name = "任务列表";
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
            let task_info_list: Vec<TaskInfo> = serde_json::from_str(
                crawl
                    .decrypt(&json["result"].to_string().trim_matches('"').to_string())
                    .as_str(),
            )
            .unwrap();
            let task_list = task_info_list
                .iter()
                .map(|task_info| {
                    Task::new(
                        task_info.taskNo,
                        task_info.giveCoin,
                        task_info.taskName.clone(),
                    )
                })
                .collect::<Vec<_>>();
            Ok(task_list)
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}
pub(crate) async fn category_list(config: &Config) -> Result<Vec<Category>, ()> {
    let Config { crawl, actions } = config;
    let action_name = "漫画分类";
    let mut data = actions
        .group
        .iter()
        .find(|action| action.name == action_name)
        .unwrap()
        .data
        .clone();
    data.insert("c".to_string(), "yml".to_string());
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
            let category_info_list: Vec<CategoryInfo> = serde_json::from_str(
                crawl
                    .decrypt(&json["result"].to_string().trim_matches('"').to_string())
                    .as_str(),
            )
                .unwrap();
            let category_list = category_info_list
                .iter()
                .map(|category_info| {
                    Category::new(
                        category_info.id.parse::<u64>().unwrap(),
                        category_info.title.clone(),
                        category_info.sort.parse::<u32>().unwrap(),
                    )
                })
                .collect::<Vec<_>>();
            Ok(category_list)
        }else{
            Err(())
        }
    } else {
        Err(())
    }
}
pub(crate) async fn snapshot_list(
    config: &Config,
    category_id: u64,
    page: u32,
    limit: u32,
) -> Result<Vec<Book>, ()> {
    let Config { crawl, actions } = config;
    let action_name = "分类查询";
    let mut data = actions
        .group
        .iter()
        .find(|action| action.name == action_name)
        .unwrap()
        .data
        .clone();
    data.insert("categoryId".to_string(), category_id.to_string());
    data.insert("page".to_string(), page.to_string());
    data.insert("limit".to_string(), limit.to_string());
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
            let snapshot_info: SnapshotInfo = serde_json::from_str(
                crawl
                    .decrypt(&json["result"].to_string().trim_matches('"').to_string())
                    .as_str(),
            )
                .unwrap();
            let book_list = snapshot_info.records
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
                        Vec::new()
                    )
                })
                .collect::<Vec<_>>();
            Ok(book_list)
        }else{
            Err(())
        }
    } else {
        Err(())
    }
}
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
                book_info.ext.iter().map(|chapter_info| {
                    Chapter::new(
                        chapter_info.id.clone(),
                        chapter_info.title.clone(),
                        chapter_info.pic.clone(),
                        chapter_info.sort,
                        chapter_info.price,
                        vec![],
                    )
                }).collect::<Vec<_>>(),
            );
            Ok(book)
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}
pub(crate) async fn chapter_content(config: &Config, chapter_id: u64, user_id: u64) -> Result<Vec<String>, ()> {
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
        }else { Err(()) }
    } else {
        Err(())
    }
}
pub(crate) async fn pay_chapter(config: &Config, user_id: u64,comic_id:u64,chapter_id:u64) -> Result<(), ()> {
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
        }else { Err(()) }
    } else {
        Err(())
    }
}
pub(crate) async fn daily_sign(config: &Config, user_id: &String) -> Result<(), ()> {
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
    data.insert("timeStamp".to_string(),timestamp_str());
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
        }else { Err(()) }
    } else {
        Err(())
    }
}
pub(crate) async fn daily_work(config: &Config, task_no: u8, user_id: &String) -> Result<(), ()> {
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
    data.insert("timeStamp".to_string(),timestamp_str());
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
        }else { Err(()) }
    } else {
        Err(())
    }
}
