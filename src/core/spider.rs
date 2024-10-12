use playwright::api::Page;
use playwright::Playwright;
use scraper::{ElementRef, Html, Selector};
use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Deserialize)]
pub struct Extractor {
    key: String,
    name: String,
    field: String,
    expression: String,
    sequential: bool,
    operation: String,
    regexp: Option<String>,
}
impl Extractor {
    pub fn new(
        key: String,
        name: String,
        field: String,
        expression: String,
        sequential: bool,
        operation: String,
        regexp: Option<String>,
    ) -> Self {
        Extractor {
            key,
            name,
            field,
            expression,
            sequential,
            operation,
            regexp,
        }
    }
    fn handler_operation(&self, element: &ElementRef) -> Result<String, String> {
        let result = match self.operation.as_str() {
            "text" => {
                let text = element.text().collect::<String>();
                text
            }
            _ => {
                let action = self.operation.replace("@", "");
                let text = element
                    .attr(action.as_str())
                    .ok_or(format!("获取属性失败：{}", action))?;
                String::from(text)
            }
        };
        match &self.regexp {
            Some(regexp) => {
                let regex = regex::Regex::new(regexp).unwrap();
                let captures = regex.captures(&result).unwrap();
                let item = captures
                    .get(1)
                    .ok_or(String::from("正则提取失败，请使用\"()\"包裹来提取。"))?;
                Ok(String::from(item.as_str()))
            }
            None => Ok(result),
        }
    }
    pub fn extract(&self, element: &ElementRef) -> Result<Value, String> {
        let selector = Selector::parse(&self.expression)
            .map_err(|err| format!("解析CSS表达式失败：{}", err))?;
        match self.sequential {
            true => {
                let element_refs = element.select(&selector).collect::<Vec<_>>();
                let mut values = Vec::<Value>::new();
                for element_ref in element_refs {
                    let value = self.handler_operation(&element_ref)?;
                    values.push(Value::String(value))
                }
                Ok(Value::Array(values))
            }
            false => {
                let element_ref = element
                    .select(&selector)
                    .next()
                    .ok_or(format!("获取元素失败：{}", self.expression))?;
                let value = self.handler_operation(&element_ref)?;
                Ok(Value::String(value))
            }
        }
    }
}
#[derive(Deserialize)]
pub struct Mapper {
    key: String,
    name: String,
    field: String,
    expression: String,
    sequential: bool,
    extractors: Option<Vec<Extractor>>,
    children: Option<Vec<Mapper>>,
}
impl Mapper {
    pub fn new(
        key: String,
        name: String,
        field: String,
        expression: String,
        sequential: bool,
        extractors: Option<Vec<Extractor>>,
        children: Option<Vec<Mapper>>,
    ) -> Self {
        Mapper {
            key,
            name,
            field,
            expression,
            sequential,
            extractors,
            children,
        }
    }

    pub fn mapping(&self, element: &ElementRef) -> Result<Map<String, Value>, String> {
        let selector = Selector::parse(&self.expression)
            .map_err(|err| format!("解析CSS表达式失败：{}", err))?;
        let mut obj = Map::<String, Value>::new();
        if let Some(extractors) = &self.extractors {
            match self.sequential {
                true => {
                    let element_refs = element.select(&selector).collect::<Vec<_>>();
                    let mut values = Vec::<Value>::new();
                    for element_ref in &element_refs {
                        let mut inner = Map::<String, Value>::new();
                        for extractor in extractors {
                            let value = extractor.extract(&element_ref)?;
                            inner.insert(extractor.field.clone(), value);
                        }
                        values.push(Value::Object(inner));
                    }
                    obj.insert(self.field.clone(), Value::Array(values));
                }
                false => {
                    let element_ref = element
                        .select(&selector)
                        .next()
                        .ok_or(format!("获取元素失败：{}", self.expression))?;
                    let mut inner = Map::<String, Value>::new();
                    for extractor in extractors {
                        let value = extractor.extract(&element_ref)?;
                        inner.insert(extractor.field.clone(), value);
                    }
                    obj.insert(self.field.clone(), Value::Object(inner));
                }
            }
        }
        if let Some(children) = &self.children {
            match self.sequential {
                true => {
                    let element_refs = element.select(&selector).collect::<Vec<_>>();
                    let mut values = Vec::<Value>::new();
                    for element_ref in &element_refs {
                        let mut inner = Map::<String, Value>::new();
                        for child in children {
                            let value = child.mapping(element_ref)?;
                            for (k, v) in value.iter() {
                                inner.insert(k.clone(), v.clone());
                            }
                        }
                        values.push(Value::Object(inner));
                    }
                    obj.insert(self.field.clone(), Value::Array(values));
                }
                false => {
                    let element_ref = element
                        .select(&selector)
                        .next()
                        .ok_or(format!("获取元素失败：{}", self.expression))?;
                    let mut inner = Map::<String, Value>::new();
                    for child in children {
                        let value = child.mapping(&element_ref)?;
                        for (k, v) in value.iter() {
                            inner.insert(k.clone(), v.clone());
                        }
                    }
                    obj.insert(self.field.clone(), Value::Object(inner));
                }
            }
        }
        Ok(obj)
    }
}
#[derive(Deserialize)]
pub struct Crawler {
    key: String,
    name: String,
    mappers: Vec<Mapper>,
    urls: Vec<String>,
}
impl Crawler {
    pub fn new(key: String, name: String, mappers: Vec<Mapper>, urls: Vec<String>) -> Self {
        Crawler {
            key,
            name,
            mappers,
            urls,
        }
    }
    pub async fn crawling(&self, page: &Page) -> Result<(), String> {
        for url in &self.urls {
            page.goto_builder(url)
                .goto()
                .await
                .map_err(|err| format!("页面跳转失败{}", err))?;
            page.wait_for_selector_builder("h1");
            let content = page
                .content()
                .await
                .map_err(|err| format!("获取页面内容失败{}", err))?;
            let document = Html::parse_document(&content);
            for mapper in &self.mappers {
                let value = mapper.mapping(&document.root_element())?;
                std::fs::write(
                    format!("{}.json", mapper.name),
                    &serde_json::to_string_pretty(&Value::Object(value)).unwrap(),
                )
                .unwrap()
            }
        }
        Ok(())
    }
}
#[derive(Deserialize)]
pub struct Spider {
    key: String,
    name: String,
    origins: Vec<String>,
    addresses: Option<Vec<String>>,
    crawlers: Vec<Crawler>,
}
impl Spider {
    pub fn new(
        key: String,
        name: String,
        origins: Vec<String>,
        addresses: Option<Vec<String>>,
        crawlers: Vec<Crawler>,
    ) -> Self {
        Spider {
            key,
            name,
            origins,
            addresses,
            crawlers,
        }
    }
    pub fn load(path: String) -> Result<Self, String> {
        let serde_json =
            std::fs::read_to_string(path).map_err(|err| format!("读取文件失败：{}", err))?;
        serde_json::from_str::<Spider>(&serde_json).map_err(|err| format!("解析JSON失败：{}", err))
    }
    pub async fn run(&self) -> Result<(), String> {
        let playwright = Playwright::initialize()
            .await
            .map_err(|err| format!("Playwright初始化失败{}", err))?;
        playwright
            .prepare()
            .map_err(|err| format!("Playwright配置失败{}", err))?; // Install browsers
        let chromium = playwright.chromium();
        let browser = chromium
            .launcher()
            .headless(true)
            .launch()
            .await
            .map_err(|err| format!("浏览器启动失败{}", err))?;
        let context = browser
            .context_builder()
            .build()
            .await
            .map_err(|err| format!("创建浏览器上下文失败{}", err))?;
        let page = context
            .new_page()
            .await
            .map_err(|err| format!("创建浏览器页面失败{}", err))?;
        for crawler in &self.crawlers {
            crawler.crawling(&page).await?;
        }
        Ok(())
    }
}
