use std::collections::HashMap;
use std::env::var;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

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
    pub(crate) origin: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) web: WebConfig,
    pub(crate) api: ApiConfig,
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
