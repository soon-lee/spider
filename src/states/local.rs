use std::collections::HashMap;
use std::env::var;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/**
 * @locale: zh-CN
 * # `WebConfig` 结构体
 * - **全限定名称**：`crate::states::local::WebConfig`
 * - **功能**：此结构体用于配置Web相关设置，包含允许的源列表和额外的配置选项。
 *
 * # 特质
 * - `Debug`：为结构体提供调试信息打印能力。
 * - `Deserialize`：允许从JSON等格式反序列化实例。
 * - `Serialize`：支持将实例序列化为JSON等格式。
 *
 * # 字段
 * ## origins
 * - **类型**：`alloc::vec::Vec<String>`
 * - **描述**：允许的跨域请求源列表。
 *
 * ## options
 * - **类型**：`std::collections::HashMap<String, String>`
 * - **描述**：自定义的Web配置选项，以键值对形式存在。
 *
 * @locale: en-US
 * # `WebConfig` Structure
 * - **Full Qualified Name**: `crate::states::local::WebConfig`
 * - **Function**: This structure configures web-related settings, including a list of allowed origins and additional configuration options.
 *
 * # Trait
 * - `Debug`: Provides debugging information print capability for the structure.
 * - `Deserialize`: Enables deserialization of instances from formats like JSON.
 * - `Serialize`: Supports serialization of instances into formats such as JSON.
 *
 * # Fields
 * ## origins
 * - **Type**: `alloc::vec::Vec<String>`
 * - **Description**: A list of allowed CORS origins.
 *
 * ## options
 * - **Type**: `std::collections::HashMap<String, String>`
 * - **Description**: Custom web configuration options stored as key-value pairs.
 */
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct WebConfig {
    origins: Vec<String>,
    options: HashMap<String, String>,
}

impl WebConfig {
    /**
     * @locale: zh-CN
     * # `get_scripts` 方法
     * - **功能**：异步方法，遍历 `origins` 中的源，发送 HTTP GET 请求以获取每个源的 HTML 页面，
     *   然后解析页面中所有的 `<script>` 标签，提取带有 `src` 属性的标签的 URL，并将其与源 URL 结合后加入结果列表。
     * - **返回**：包含所有脚本 URL 的向量。
     *
     * @locale: en-US
     * # `get_scripts` Method
     * - **Function**: Asynchronously iterates through `origins`, sends HTTP GET requests to fetch the HTML of each origin,
     *   then parses all `<script>` tags from the page, extracts URLs with `src` attribute from these tags, combines them with the origin URL,
     *   and adds them to the result vector.
     * - **Returns**: A vector containing URLs of all scripts.
     */
    pub(crate) async fn get_scripts(&self) -> Vec<String> {
        let mut scripts = Vec::<String>::new();
        let client = reqwest::Client::new();
        for origin in self.origins.iter().as_ref() {
            let response = client.get(origin).send().await;
            if response.is_err(){
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
    /**
     * @locale: zh-CN
     * # `extract_options_from_scripts` 方法
     * - **功能**：异步方法，首先调用 `get_scripts` 获取脚本 URL 列表，随后对每个脚本 URL 发起请求获取其内容。
     *   使用预先定义在 `options` 字段中的正则表达式，在每个脚本的 HTML 内容中搜索匹配项，并将找到的值存入哈希映射中。
     * - **返回**：成功时返回包含配置选项的哈希映射，或在过程中遇到错误时返回错误信息。
     *
     * @locale: en-US
     * # `extract_options_from_scripts` Method
     * - **Function**: Asynchronously calls `get_scripts` to obtain a list of script URLs, then issues requests to each URL to fetch its content.
     *   Utilizes pre-defined regular expressions from the `options` field to search for matches within each script's HTML content,
     *   storing found values into a hash map.
     * - **Returns**: A hash map with configuration options on success, or an error message upon encountering errors during the process.
     */
    pub(crate) async fn extract_options_from_scripts(&self) -> Result<HashMap<String, String>,String> {
        let mut configs = HashMap::<String, String>::new();
        let scripts = self.get_scripts().await;
        for script in scripts {
            let client = reqwest::Client::new();
            let response = client.get(script).send().await.map_err(|err|format!("请求失败: {}", err))?;
            let html = response.text().await.map_err(|err|format!("获取响应文本失败: {}", err))?;
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
/**
 * @locale: zh-CN
 * # `ApiConfig` 结构体
 * - **全限定名称**：`crate::states::local::ApiConfig`
 * - **功能**：存储API配置信息，主要包含API服务器的起源地址。
 *
 * # 特质
 * - `Debug`：便于结构体的调试输出。
 * - `Deserialize`：支持从配置文件或网络响应中反序列化。
 * - `Serialize`：可将配置信息序列化输出至文件或网络。
 *
 * # 字段
 * ## origin
 * - **类型**：`String`
 * - **描述**：API服务的基URL或起源地址。
 *
 * @locale: en-US
 * # `ApiConfig` Structure
 * - **Full Qualified Name**: `crate::states::local::ApiConfig`
 * - **Function**: Stores API configuration details, primarily containing the origin address of the API server.
 *
 * # Trait
 * - `Debug`: Facilitates debugging output for the structure.
 * - `Deserialize`: Supports deserialization from configuration files or network responses.
 * - `Serialize`: Allows serialization of configuration information to files or over the network.
 *
 * # Fields
 * ## origin
 * - **Type**: `String`
 * - **Description**: The base URL or origin address of the API service.
 */
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ApiConfig {
    pub(crate) origin: String,
}

/**
 * @locale: zh-CN
 * # `Config` 结构体
 * - **全限定名称**：`crate::states::local::Config`
 * - **功能**：整合了Web配置与API配置，作为应用的总体配置实体。
 *
 * # 特质
 * - `Debug`：提供结构体的调试信息。
 * - `Deserialize`：使得配置可以从JSON、YAML等形式加载。
 * - `Serialize`：能够将配置保存为JSON、YAML等格式。
 *
 * # 字段
 * ## web
 * - **类型**：`crate::states::local::WebConfig`
 * - **描述**：Web服务的配置详情。
 *
 * ## api
 * - **类型**：`crate::states::local::ApiConfig`
 * - **描述**：API接口的配置信息。
 *
 * @locale: en-US
 * # `Config` Structure
 * - **Full Qualified Name**: `crate::states::local::Config`
 * - **Function**: Combines Web and API configurations, serving as the overall configuration entity for the application.
 *
 * # Trait
 * - `Debug`: Provides debugging information for the structure.
 * - `Deserialize`: Enables loading configuration from formats like JSON, YAML.
 * - `Serialize`: Can save configuration in formats such as JSON, YAML.
 *
 * # Fields
 * ## web
 * - **Type**: `crate::states::local::WebConfig`
 * - **Description**: Detailed configuration for the web service.
 *
 * ## api
 * - **Type**: `crate::states::local::ApiConfig`
 * - **Description**: Configuration information for the API endpoints.
 */
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) web: WebConfig,
    pub(crate) api: ApiConfig,
}
impl Config {

    /**
     * @locale: zh-CN
     * ## 方法
     *
     * ### `load`
     * - **功能**：异步加载配置文件。
     * - **参数**：无。
     * - **返回值**：成功时返回`Config`实例，失败则为错误信息字符串。
     * - **行为**：从环境变量`CARGO_MANIFEST_DIR`定位配置文件路径，默认查找`configs\crawl.yaml`，读取并反序列化为`Config`结构。
     *
     * @locale: en-US
     * ## Methods
     *
     * ### `load`
     * - **Function**: Asynchronously loads the configuration file.
     * - **Parameters**: None.
     * - **Return Value**: Returns an instance of `Config` on success, or an error message string on failure.
     * - **Behavior**: Locates the configuration file path using the `CARGO_MANIFEST_DIR` environment variable,默认查找路径为`configs\crawl.yaml`, reads, and deserializes into a `Config` structure.
     */
    pub(crate) async fn load() -> Result<Self,String> {
        let manifest_dir =
            var("CARGO_MANIFEST_DIR").map_err(|_| "不存在环境变量：CARGO_MANIFEST_DIR，无法定位配置文件！")?;
        let config_path = PathBuf::from(&manifest_dir).join(r#"configs\crawl.yaml"#);
        let yaml = std::fs::read_to_string(config_path).map_err(|_| r#"找不到配置文件：configs\crawl.yaml，无法加载配置！"#)?;
        let config: Config = serde_yaml::from_str(&yaml).map_err(|_|"解析配置文件失败！")?;
        Ok(config)
    }

    /**
     * @locale: zh-CN
     * ## 方法
     *
     * ### `get_options`
     * - **功能**：异步获取合并的配置选项。
     * - **参数**：无。
     * - **返回值**：成功时返回包含所有配置选项的`HashMap<String, String>`，失败则为错误信息字符串。
     * - **行为**：从`web`部分提取脚本配置选项，并将`api`部分的`origin`字段添加到选项中。
     *
     * @locale: en-US
     * ## Methods
     *
     * ### `get_options`
     * - **Function**: Asynchronously retrieves merged configuration options.
     * - **Parameters**: None.
     * - **Return Value**: Returns a `HashMap<String, String>` containing all configuration options on success, or an error message string on failure.
     * - **Behavior**: Extracts script configuration options from the `web` section and adds the `origin` field from the `api` section to the options.
     */
    pub(crate) async fn get_options(&self) -> Result<HashMap<String, String>,String> {
        let mut configs = self.web.extract_options_from_scripts().await?;
        configs.insert("origin".to_string(), self.api.origin.clone());
        Ok(configs)
    }
}
