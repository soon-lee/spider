use getset::Getters;
use playwright::api::Page;
use playwright::Playwright;
use scraper::{ElementRef, Html, Selector};
use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Deserialize, Debug)]
pub struct Transformer {
    key: String,
    name: String,
    expression: String,
    pattern: Option<String>,
}
impl Transformer {
    pub fn new(key: String, name: String, expression: String, pattern: Option<String>) -> Self {
        Transformer {
            key,
            name,
            expression,
            pattern,
        }
    }
    fn parse(value: &Value, expression: &String) -> Result<Value, String> {
        match expression.find(".") {
            Some(index) => {
                let field = expression[..index].to_string();
                let sub_expression = expression[index + 1..].to_string();
                match field.starts_with("[") && field.ends_with("]") {
                    true => {
                        let index = field[1..field.len() - 1].to_string();
                        let item = value
                            .get(index)
                            .ok_or(format!("属性表达式错误：{}", expression))?;
                        let arr = item
                            .as_array()
                            .ok_or(format!("属性表达式错误：{}", expression))?;
                        let mut items = Vec::<Value>::new();
                        for a in arr.iter() {
                            let v = Self::parse(a, &sub_expression)?;
                            items.push(v);
                        }
                        Ok(Value::Array(items))
                    }
                    false => {
                        let item = value
                            .get(&field)
                            .ok_or(format!("属性表达式错误：{}", expression))?;
                        Self::parse(item, &sub_expression)
                    }
                }
            }
            None => {
                let item = value
                    .get(&expression)
                    .ok_or(format!("属性表达式错误：{}", expression))?;
                Ok(item.clone())
            }
        }
    }
    pub fn transform(&self, value: &Value, origin: &String) -> Result<Vec<String>, String> {
        let value = Self::parse(value, &self.expression)?;
        match &self.pattern {
            Some(pattern) => {
                let count = value
                    .as_i64()
                    .ok_or(format!("不合适的属性表达式：{}", self.expression))?;
                let mut urls = Vec::<String>::new();
                for i in 0..count {
                    urls.push(
                        pattern
                            .replace("{ORIGIN}", origin.as_str())
                            .replace("{PATTERN}", (i + 1).to_string().as_str()),
                    );
                }
                Ok(urls)
            }
            None => {
                let items = value
                    .as_array()
                    .ok_or(format!("不合适的属性表达式：{}", self.expression))?;
                let mut urls = Vec::<String>::new();
                for item in items {
                    let url = item
                        .as_str()
                        .ok_or(format!("不合适的属性表达式：{}", self.expression))?;
                    let url = String::from(url);
                    if url.starts_with(origin) {
                        urls.push(url);
                    } else if url.starts_with("//") {
                        urls.push(format!("https{}", url))
                    } else if url.starts_with("/") {
                        urls.push(format!("{}{}", origin, url))
                    } else {
                        urls.push(format!("{}/{}", origin, url));
                    }
                }
                Ok(urls)
            }
        }
    }
}
#[derive(Deserialize, Debug)]
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
                match regex.captures(&result) {
                    None => Ok(result),
                    Some(captures) => {
                        let item = captures
                            .get(1)
                            .ok_or(String::from("正则提取失败，请使用\"()\"包裹来提取。"))?;
                        Ok(String::from(item.as_str()))
                    }
                }
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
#[derive(Deserialize, Debug)]
pub struct Mapper {
    key: String,
    name: String,
    field: String,
    expression: String,
    sequential: bool,
    transformer: Option<Transformer>,
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
        transformer: Option<Transformer>,
        extractors: Option<Vec<Extractor>>,
        children: Option<Vec<Mapper>>,
    ) -> Self {
        Mapper {
            key,
            name,
            field,
            expression,
            sequential,
            transformer,
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
                    for child in children {
                        let mut values = Vec::<Value>::new();
                        for element_ref in &element_refs {
                            let value = child.mapping(element_ref)?;
                            values.push(Value::Object(value));
                        }
                        obj.insert(child.field.clone(), Value::Array(values));
                    }
                }
                false => {
                    let element_ref = element
                        .select(&selector)
                        .next()
                        .ok_or(format!("获取元素失败：{}", self.expression))?;
                    for child in children {
                        let value = child.mapping(&element_ref)?;
                        for (k, v) in value.iter() {
                            obj.insert(k.clone(), v.clone());
                        }
                    }
                }
            }
        }
        Ok(obj)
    }
}
#[derive(Deserialize, Debug, Getters)]
#[getset(get = "pub")]
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
    pub async fn crawling(&self, page: &Page, origin: &String) -> Result<Vec<Value>, String> {
        let mut result = Vec::<Value>::new();
        for url in &self.urls {
            page.goto_builder(url)
                .goto()
                .await
                .map_err(|err| format!("页面跳转失败{}", err))?;
            let content = page
                .content()
                .await
                .map_err(|err| format!("获取页面内容失败{}", err))?;
            let document = Html::parse_document(&content);
            let mut obj = Map::<String, Value>::new();
            for mapper in &self.mappers {
                let value = mapper.mapping(&document.root_element())?;
                for (k, v) in value.iter() {
                    obj.insert(k.clone(), v.clone());
                }
                let result = Value::Object(value);
                if let Some(transformer) = &mapper.transformer {
                    let urls = transformer.transform(&result, origin)?;
                    println!(
                        "{:?}",
                        urls.iter()
                            .map(|x| x.replace("yys.163.com", "yys.163.com/shishen"))
                            .collect::<Vec<_>>()
                    );
                }
            }
            result.push(Value::Object(obj));
        }
        Ok(result)
    }
}
#[derive(Deserialize, Debug, Getters)]
#[getset(get = "pub")]
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
    pub async fn context() -> Result<std::sync::Arc<Page>, String> {
        let playwright = Playwright::initialize()
            .await
            .map_err(|err| format!("Playwright初始化失败{}", err))?;
        playwright
            .install_chromium()
            .map_err(|err| format!("Playwright安装Chrome失败{}", err))?;
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
        let page = std::sync::Arc::new(page);
        Ok(page)
    }
    pub async fn run(&self) -> Result<(), String> {
        let playwright = Playwright::initialize()
            .await
            .map_err(|err| format!("Playwright初始化失败{}", err))?;
        playwright
            .install_chromium()
            .map_err(|err| format!("Playwright安装Chrome失败{}", err))?;
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
            let value = crawler.crawling(&page, &self.origins[0]).await?;
            std::fs::write(
                format!("{}.json", crawler.name),
                &serde_json::to_string_pretty(&Value::Array(value)).unwrap(),
            )
            .unwrap()
        }
        Ok(())
    }
}
