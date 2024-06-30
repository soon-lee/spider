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
            match response {
                Err(e) => {
                    eprintln!("Error: {}", e);
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
                                scripts.push(script.value().attr("src").unwrap().to_string())
                            });
                        break;
                    } else {
                        println!("Error: {}", response.status());
                        continue;
                    }
                }
            }
        }
        scripts
    }
    pub(crate) async fn extract_options_from_scripts(&self) -> HashMap<String, String> {
        let mut configs = HashMap::<String, String>::new();
        let scripts = self.get_scripts().await;
        for script in scripts {
            let client = reqwest::Client::new();
            let res = client.get(script).send().await.unwrap();
            let html = res.text().await.unwrap();
            self.options.iter().for_each(|(key, value)| {
                let pattern = regex::Regex::new(value).unwrap();
                if let Some(captures) = pattern.captures(&html) {
                    configs.insert(key.clone(), captures[1].to_string());
                };
            });
        }
        configs
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
    pub(crate) async fn load() -> Self {
        let manifest_dir =
            var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR environment variable not found");
        let config_path = PathBuf::from(&manifest_dir).join(r#"configs\crawl.yaml"#);
        let yaml = std::fs::read_to_string(config_path).expect("Unable to read config file");
        let mut config: Config = serde_yaml::from_str(&yaml).expect("Unable to parse config file");
        config.extract_options_from_scripts().await;
        config
    }
    pub(crate) async fn get_options(&self) -> HashMap<String, String> {
        let mut configs = self.extract_options_from_scripts().await;
        configs.insert("origin".to_string(), self.api.origin.clone());
        configs
    }
}
