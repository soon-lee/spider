use std::collections::HashMap;
use std::env::var;
use std::path::PathBuf;
use std::time::Duration;

use getset::Getters;
use qiniu_sdk::credential::Credential;
use qiniu_sdk::download::{DownloadManager, EndpointsUrlGenerator, UrlsSigner};
use qiniu_sdk::download::apis::http_client::BucketDomainsQueryer;
use qiniu_sdk::upload::{UploadManager, UploadTokenSigner};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{MySql, MySqlPool, Pool};
use sqlx::mysql::MySqlConnectOptions;

use crate::utils::crypt::{aes_decrypt, aes_encrypt, auth_path};

#[derive(Getters)]
#[getset(get = "pub")]
pub(crate) struct MySqlClient {
    host: String,
    port: String,
    username: String,
    password: String,
    database: String,
}

impl MySqlClient {
    pub(crate) async fn new() -> Result<Self, String> {
        let host =
            var("DATABASE_HOST").map_err(|err| format!("缺少DATABASE_HOST环境变量:{}", err))?;
        let port =
            var("DATABASE_PORT").map_err(|err| format!("缺少DATABASE_PORT环境变量:{}", err))?;
        let username =
            var("DATABASE_USER").map_err(|err| format!("缺少DATABASE_USER环境变量:{}", err))?;
        let password = var("DATABASE_PASSWORD")
            .map_err(|err| format!("缺少DATABASE_PASSWORD环境变量:{}", err))?;
        let database =
            var("DATABASE_NAME").map_err(|err| format!("缺少DATABASE_NAME环境变量:{}", err))?;

        Ok(MySqlClient {
            host,
            port,
            username,
            password,
            database,
        })
    }
    pub(crate) async fn get_connection(&self) -> Result<Pool<MySql>, String> {
        let connect_options = MySqlConnectOptions::new()
            .host(self.host())
            .port(self.port().parse::<u16>().unwrap())
            .username(self.username())
            .password(self.password())
            .database(self.database());
        MySqlPool::connect_with(connect_options)
            .await
            .map_err(|err| format!("数据库连接失败:{}", err))
    }
}

#[derive(Debug)]
pub(crate) struct QiniuClient {
    access_key: String,
    secret_key: String,
    bucket_name: String,
    aes_key: String,
}
impl QiniuClient {
    pub(crate) fn load() -> Result<Self, String> {
        let access_key = var("QINIU_ACCESS_KEY").map_err(|_| "缺失环境变量：QINIU_ACCESS_KEY！")?;
        let secret_key = var("QINIU_SECRET_KEY").map_err(|_| "缺失环境变量：QINIU_SECRET_KEY！")?;
        let bucket_name =
            var("QINIU_BUCKET_NAME").map_err(|_| "缺失环境变量：QINIU_BUCKET_NAME！")?;
        let aes_key = var("QINIU_AES_KEY").map_err(|_| "缺失环境变量：QINIU_AES_KEY！")?;
        Ok(Self {
            access_key,
            secret_key,
            bucket_name,
            aes_key,
        })
    }
    pub(crate) fn get_access_key(&self) -> &str {
        &self.access_key
    }
    pub(crate) fn get_upload_manager(&self) -> UploadManager {
        let upload_credential = Credential::new(&self.access_key, &self.secret_key);
        let token_signer = UploadTokenSigner::new_credential_provider(
            upload_credential,
            &self.bucket_name,
            Duration::from_secs(3600),
        );
        UploadManager::builder(token_signer).build()
    }
    pub(crate) fn get_download_manager(&self) -> DownloadManager {
        let query_credential = Credential::new(&self.access_key, &self.secret_key);
        let domain_query = BucketDomainsQueryer::new().query(query_credential, &self.bucket_name);
        let url_generator = EndpointsUrlGenerator::builder(domain_query)
            .use_https(false)
            .build();
        let download_credential = Credential::new(&self.access_key, &self.secret_key);
        let url_signer = UrlsSigner::new(download_credential, url_generator);
        DownloadManager::builder(url_signer).build()
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct WebConfig {
    origins: Vec<String>,
    options: HashMap<String, String>,
}

impl WebConfig {
    pub(crate) async fn get_scripts(&self) -> Vec<String> {
        let mut scripts = Vec::<String>::new();
        let client = reqwest::Client::new();
        for origin in self.origins.iter().as_ref() {
            let response = client.get(origin).send().await;
            if response.is_err() {
                continue;
            }
            let html = response.unwrap().text().await.unwrap();
            let root = scraper::Html::parse_document(&html);
            let script_selector = scraper::Selector::parse("script").unwrap();
            root.select(&script_selector)
                .filter(|script| script.value().attr("src").is_some())
                .for_each(|script| {
                    scripts.push(format!(
                        "{}{}",
                        &origin,
                        script.value().attr("src").unwrap()
                    ))
                });
            break;
        }
        scripts
    }
    pub(crate) async fn extract_options_from_scripts(&self) -> Result<HashMap<String, String>, String> {
        let mut configs = HashMap::<String, String>::new();
        let scripts = self.get_scripts().await;
        for script in scripts {
            let client = reqwest::Client::new();
            let response = client.get(script).send().await.map_err(|err| format!("请求失败: {}", err))?;
            let html = response.text().await.map_err(|err| format!("获取响应文本失败: {}", err))?;
            self.options.iter().for_each(|(key, value)| {
                let pattern = regex::Regex::new(value).unwrap();
                match pattern.captures(&html) {
                    Some(captures) => {
                        configs.insert(key.clone(), captures[1].parse().unwrap());
                    }
                    None => {}
                }
            });
        }
        Ok(configs)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ApiConfig {
    origin: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    web: WebConfig,
    api: ApiConfig,
}
impl Config {
    pub(crate) async fn load() -> Result<Self, String> {
        let manifest_dir =
            var("CARGO_MANIFEST_DIR").map_err(|_| "不存在环境变量：CARGO_MANIFEST_DIR，无法定位配置文件！")?;
        let config_path = PathBuf::from(&manifest_dir).join(r#"configs\crawl.yaml"#);
        let yaml = std::fs::read_to_string(config_path).map_err(|_| r#"找不到配置文件：configs\crawl.yaml，无法加载配置！"#)?;
        let config: Config = serde_yaml::from_str(&yaml).map_err(|_| "解析配置文件失败！")?;
        Ok(config)
    }
    pub(crate) async fn get_options(&self) -> Result<HashMap<String, String>, String> {
        let mut configs = self.web.extract_options_from_scripts().await?;
        configs.insert("origin".to_string(), self.api.origin.clone());
        Ok(configs)
    }
}
pub(crate) struct NetClient {
    pub(crate) options: HashMap<String, String>,
}
impl NetClient {
    pub(crate) fn new(options: HashMap<String, String>) -> Self {
        NetClient { options }
    }
    pub(crate) async fn post_client(
        &self,
        path: &str,
        data: &str,
    ) -> Result<String, String> {
        let origin = self.options.get("origin").ok_or("缺少origin参数")?;
        let app_id = self.options.get("app_id").ok_or("缺少app_id参数")?;
        let template = self.options.get("template_str").ok_or("缺少template_str参数")?;
        let default_str = self.options.get("default_str").ok_or("缺少default_str参数")?;
        let aes_key = self.options.get("aes_key").ok_or("缺少aes_key参数")?;

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
}