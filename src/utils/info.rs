use base64::Engine;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CrawlConfig {
    origins: Vec<String>,
    scripts: Vec<String>,
    options: std::collections::HashMap<String, String>,
    configs: std::collections::HashMap<String, String>,
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
            .get("template_str").unwrap()
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
    pub(crate) fn encrypt(&self, data: &String)->String{
        let bytes = soft_aes::aes::aes_enc_ecb(data.as_bytes(), self.configs.get("aes_key").unwrap().as_bytes(), Some("PKCS7")).unwrap();
        base64::engine::general_purpose::STANDARD.encode(&bytes)
    }
    pub(crate) fn decrypt(&self, data: &String) -> String {
        let bytes =soft_aes::aes::aes_dec_ecb(data.as_bytes(), self.configs.get("aes_key").unwrap().as_bytes(), Some("PKCS7")).unwrap();
        String::from_utf8(bytes).unwrap()
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) crawl: CrawlConfig,
}
impl Config {
    pub(crate) fn load() -> Self {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR environment variable not found");
        let config_path = std::path::PathBuf::from(&manifest_dir).join(r#"configs\crawl.yaml"#);
        let yaml = std::fs::read_to_string(config_path).expect("Unable to read config file");
        serde_yaml::from_str(&yaml).expect("Unable to parse config file")
    }
    fn dump(&self) {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR environment variable not found");
        let config_path = std::path::PathBuf::from(&manifest_dir).join(r#"configs\crawl.yaml"#);
        let config = serde_yaml::to_string(&self).expect("Unable to serialize config");
        std::fs::write(config_path, config).expect("Unable to write config file");
    }
}
pub(crate) async fn register_user(config:&CrawlConfig) {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("https://wwapi1.xsjk99.com{}",config.auth_path(&"/api/h5/getChapterContent".to_string())))
        .header("Appid",config.configs.get("app_id").unwrap())
        .header("Content-Type","application/json")
        .body("{\"data\":\"1FE8heap7dhnyi2J0RV9aGfeVWwp0RC/AlfaJrI0pSzrS4si0qTjZSGbhJTyHoxHrkrPp8eCkI2WdK4Vysdn2OW2KBYDc1OaKeLoMw8+Kuo=\"}")
        // .body(format!("{{\"data\":\"{}\"}}",config.encrypt(&format!("{{\"devType\":\"3\",\"timestamp\":\"{}\"}}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()/1000))))
        .send()
        .await
        .unwrap();
    if response.status().is_success() {
        println!("Register user successfully");
        let response_text = response.text().await.unwrap();
        println!("Response: {}", response_text);
    } else {
        println!("Error: {}", response.status());
    }
}
