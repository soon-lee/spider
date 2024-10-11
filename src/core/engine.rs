use getset::Getters;
use headless_chrome::Browser;
use reqwest::header::{HeaderMap, HeaderName};
use reqwest::{Client, RequestBuilder};
use scraper::{ElementRef, Html, Selector};
use serde::Deserialize;
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub(crate)")]
pub(crate) struct Mapping {
    key: String,
    name: String,
    field: String,
    sequential: bool,
    expression: String,
    operation: String,
    regexp: Option<String>,
}
impl Mapping {
    pub(crate) fn new(
        key: String,
        name: String,
        field: String,
        sequential: bool,
        expression: String,
        operation: String,
        regexp: Option<String>,
    ) -> Self {
        Mapping {
            key,
            name,
            field,
            sequential,
            expression,
            operation,
            regexp,
        }
    }
    pub(crate) fn handle_regexp(&self, value: String) {
        match &self.regexp {
            Some(regexp) => {
                let regex = regex::Regex::new(regexp).unwrap();
                let captures = regex.captures(&value).unwrap();
                let result = captures.get(1).unwrap().as_str();
                println!("{}", result);
            }
            None => {}
        }
    }
    fn handler_operation(&self, element: &ElementRef) -> Result<String, String> {
        match self.operation.as_str() {
            "text" => {
                let text = element.text().collect::<String>();
                Ok(text)
            }
            _ => {
                let action = self.operation.replace("@", "");
                let text = element
                    .attr(action.as_str())
                    .ok_or(format!("获取属性失败：{}", action))?;
                Ok(text.to_string())
            }
        }
    }
    pub(crate) fn mapping(&self, element: &ElementRef) -> Result<Value, String> {
        let selector = Selector::parse(&self.expression)
            .map_err(|err| format!("解析CSS表达式失败：{}", err))?;
        match self.sequential {
            true => {
                let text_refs = element.select(&selector).collect::<Vec<_>>();
                let mut text_items = Vec::<Value>::new();
                for text_ref in text_refs {
                    let text = self.handler_operation(&text_ref)?;
                    text_items.push(Value::String(text));
                }
                Ok(Value::Array(text_items))
            }
            false => {
                let text_ref = element
                    .select(&selector)
                    .next()
                    .ok_or(format!("获取元素失败：{}", self.expression))?;
                let text = self.handler_operation(&text_ref)?;
                Ok(Value::String(text))
            }
        }
    }
}
#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub(crate)")]
pub(crate) struct Mapper {
    key: String,
    name: String,
    field: String,
    sequential: bool,
    expression: String,
    mappings: Option<Vec<Mapping>>,
}
impl Mapper {
    pub(crate) fn new(
        key: String,
        name: String,
        field: String,
        sequential: bool,
        expression: String,
        mappings: Option<Vec<Mapping>>,
    ) -> Self {
        Mapper {
            key,
            name,
            field,
            sequential,
            expression,
            mappings,
        }
    }
    pub(crate) fn map(&self, element: &Html) -> Result<Map<String, Value>, String> {
        let selector = Selector::parse(&self.expression)
            .map_err(|err| format!("解析CSS表达式失败：{}", err))?;
        let mut values = Map::<String, Value>::new();
        match self.sequential {
            true => {
                let element_refs = element.select(&selector).collect::<Vec<_>>();
                let mut items = Vec::<Value>::new();
                for element_ref in element_refs {
                    let mut fields = Map::<String, Value>::new();
                    if let Some(mappings) = &self.mappings {
                        for mapping in mappings {
                            let value = mapping.mapping(&element_ref)?;
                            fields.insert(mapping.field.clone(), value);
                        }
                        items.push(Value::Object(fields));
                    }
                }
                values.insert(self.field.clone(), Value::Array(items));
            }
            false => {
                let element_ref = element
                    .select(&selector)
                    .next()
                    .ok_or(format!("获取元素失败：{}", self.expression))?;
                if let Some(mappings) = &self.mappings {
                    for mapping in mappings {
                        let value = mapping.mapping(&element_ref)?;
                        values.insert(format!("{}.{}", &self.field, &mapping.field), value);
                    }
                }
            }
        }
        Ok(values)
    }
}
#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub(crate)")]
pub(crate) struct Extractor {
    key: String,
    name: String,
    urls: Vec<String>,
    mappers: Vec<Mapper>,
    headers: Option<HashMap<String, String>>,
    bodies: Option<HashMap<String, String>>,
    method: String,
}
impl Extractor {
    pub(crate) fn new(
        key: String,
        name: String,
        urls: Vec<String>,
        mappers: Vec<Mapper>,
        headers: Option<HashMap<String, String>>,
        bodies: Option<HashMap<String, String>>,
        method: String,
    ) -> Self {
        Extractor {
            key,
            name,
            urls,
            mappers,
            headers,
            bodies,
            method,
        }
    }
    fn deal_headers(&self, request_builder: RequestBuilder) -> RequestBuilder {
        match &self.headers {
            Some(headers) => {
                let mut header_map = HeaderMap::new();
                headers.iter().for_each(|(key, value)| {
                    header_map.insert(key.parse::<HeaderName>().unwrap(), value.parse().unwrap());
                });
                request_builder.headers(header_map)
            }
            None => request_builder,
        }
    }
    pub(crate) fn deal_body_params(&self, request_builder: RequestBuilder) -> RequestBuilder {
        match &self.bodies {
            Some(body_params) => {
                let body = body_params
                    .iter()
                    .map(|(key, value)| format!("{}={}", key, value))
                    .collect::<Vec<String>>()
                    .join("&");
                request_builder.body(body)
            }
            None => request_builder,
        }
    }

    pub(crate) async fn fetch(&self, url: &String) -> Result<String, String> {
        let client = Client::new();
        let mut request_builder = match self.method.as_str() {
            "GET" => Ok(client.get(url)),
            "POST" => Ok(client.post(url)),
            _ => Err(format!("不支持的请求方法:{}", &self.method)),
        }?;
        request_builder = self.deal_headers(request_builder);
        request_builder = self.deal_body_params(request_builder);
        match request_builder.send().await {
            Ok(response) => response
                .text()
                .await
                .map_err(|err| format!("解析响应文本失败：{:?}", err)),
            Err(err) => Err(format!("请求失败：{:?}", err)),
        }
    }
    pub(crate) async fn extract(&self) -> Result<Vec<Value>, String> {
        let mut fields = Vec::<Value>::new();
        for url in &self.urls {
            // let text = self.fetch(url).await?;
            let mut option_builder = headless_chrome::LaunchOptionsBuilder::default();
            let browser = Browser::new(option_builder.headless(true).build().unwrap()).unwrap();

            let tab = browser.new_tab().unwrap();

            tab.navigate_to(url).unwrap();
            tab.wait_until_navigated().unwrap();
            let text = tab.get_content().unwrap();
            std::fs::write(
                format!(
                    "src/{}.html",
                    url.replace("/", "").replace(":", "").replace(".", "")
                ),
                &text,
            )
            .unwrap();
            let document = Html::parse_document(&text);
            let mut new_value = Map::<String, Value>::new();
            for mapper in &self.mappers {
                let value = mapper.map(&document)?;
                for (k, v) in value.iter().collect::<Vec<_>>() {
                    new_value.insert(k.to_string(), v.clone());
                }
            }
            fields.push(Value::Object(new_value));
        }
        Ok(fields)
    }
}
#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub(crate)")]
pub(crate) struct Spider {
    key: String,
    name: String,
    origins: Vec<String>,
    addresses: Option<Vec<String>>,
    extractors: Vec<Extractor>,
}
impl Spider {
    pub(crate) fn new(
        key: String,
        name: String,
        origins: Vec<String>,
        addresses: Option<Vec<String>>,
        extractors: Vec<Extractor>,
    ) -> Self {
        Spider {
            key,
            name,
            origins,
            addresses,
            extractors,
        }
    }
    pub(crate) fn load(path: String) -> Result<Self, String> {
        let serde_json =
            std::fs::read_to_string(path).map_err(|err| format!("读取文件失败：{}", err))?;
        serde_json::from_str::<Spider>(&serde_json).map_err(|err| format!("解析JSON失败：{}", err))
    }
    pub(crate) async fn deal(&self) -> Result<(), String> {
        println!("**********************************************************************************************");
        for extractor in &self.extractors {
            let fields = extractor.extract().await?;
            std::fs::write(
                format!("src/{}.json", extractor.name),
                serde_json::to_string(&Value::Array(fields)).unwrap(),
            )
            .expect("err");
        }
        println!("**********************************************************************************************");
        Ok(())
    }
}
