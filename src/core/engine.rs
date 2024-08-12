use getset::Getters;
use reqwest::header::{HeaderMap, HeaderName};
use reqwest::{Client, RequestBuilder};
use scraper::{ElementRef, Html, Selector};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Getters)]
#[getset(get = "pub(crate)")]
pub(crate) struct Mapping {
    field: String,
    expression: String,
    operation: String,
    regexp: Option<String>,
}
impl Mapping {
    pub(crate) fn new(
        field: String,
        expression: String,
        operation: String,
        regexp: Option<String>,
    ) -> Self {
        Mapping {
            field,
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
    pub(crate) fn deal_css(&self, element: ElementRef) -> Result<(String, String), String> {
        match self.operation.as_str() {
            "text" => {
                let selector = Selector::parse(&self.expression)
                    .map_err(|err| format!("解析CSS表达式失败：{}", err))?;
                let text_ref = element
                    .select(&selector)
                    .next()
                    .ok_or(format!("获取元素失败：{}", self.expression))?;
                let text = text_ref.text().collect::<String>();
                Ok((self.field.clone(), text))
            }
            _ => {
                let selector = Selector::parse(&self.expression)
                    .map_err(|err| format!("解析CSS表达式失败：{}", err))?;
                let text_ref = element
                    .select(&selector)
                    .next()
                    .ok_or(format!("获取元素失败：{}", self.expression))?;
                let action = self.operation.replace("@", "");
                let text = text_ref
                    .attr(action.as_str())
                    .ok_or(format!("获取属性失败：{}", action))?;
                Ok((self.field.clone(), text.to_string()))
            }
        }
    }
    pub(crate) fn deal_json(&self, value: Value) {}
    pub(crate) fn deal_xpath(&self) {}
}
#[derive(Debug, Getters)]
#[getset(get = "pub(crate)")]
pub(crate) struct Mapper {
    field: String,
    expression: String,
    sequential: bool,
    mapping: Option<Vec<Mapping>>,
    children: Option<Vec<Mapper>>,
}
impl Mapper {
    pub(crate) fn new(
        field: String,
        expression: String,
        sequential: bool,
        mapping: Option<Vec<Mapping>>,
        children: Option<Vec<Mapper>>,
    ) -> Self {
        Mapper {
            field,
            expression,
            sequential,
            mapping,
            children,
        }
    }
    pub(crate) fn deal_css(&self, element: ElementRef) -> Result<Value, String> {
        let selector = Selector::parse(&self.expression)
            .map_err(|err| format!("解析CSS表达式失败：{}", err))?;
        let fragment = element
            .select(&selector)
            .next()
            .ok_or(format!("获取元素失败：{}", self.expression))?;
        match self.sequential {
            true => {
                let series = fragment
                    .child_elements()
                    .map(|child_element| {
                        let mut item_fields = Map::<String, Value>::new();
                        match &self.mapping {
                            Some(mappings) => {
                                for mapping in mappings {
                                    let (field, value) = mapping.deal_css(child_element)?;
                                    item_fields.insert(field, Value::String(value));
                                }
                            }
                            None => {}
                        };
                        match &self.children {
                            Some(children) => {
                                for child in children {
                                    let child_fields = child.deal_css(child_element)?;
                                    item_fields.insert(child.field.clone(), child_fields);
                                }
                            }
                            None => {}
                        };
                        Ok(Value::Object(item_fields))
                    })
                    .collect::<Result<Vec<Value>, String>>()?;
                Ok(Value::Array(series))
            }
            false => {
                let mut fields = Map::<String, Value>::new();
                match &self.mapping {
                    Some(mappings) => {
                        for mapping in mappings {
                            let (field, value) = mapping.deal_css(fragment)?;
                            fields.insert(field, Value::String(value));
                        }
                    }
                    None => {}
                };
                match &self.children {
                    Some(children) => {
                        for child in children {
                            let child_fields = child.deal_css(fragment)?;
                            fields.insert(child.field.clone(), child_fields);
                        }
                    }
                    None => {}
                }
                Ok(Value::Object(fields))
            }
        }
    }
    pub(crate) fn deal_json(&self, element: Value) {}
    pub(crate) fn deal_xpath(&self) {}
}
#[derive(Debug, Getters)]
#[getset(get = "pub(crate)")]
pub(crate) struct Extractor {
    field: String,
    parser: String,
    mappers: Vec<Mapper>,
}
impl Extractor {
    pub(crate) fn new(field: String, parser: String, mappers: Vec<Mapper>) -> Self {
        Extractor {
            field,
            parser,
            mappers,
        }
    }
    pub(crate) fn extract(&self, text: &String) -> Result<Value, String> {
        match self.parser.as_str() {
            "CSS" => {
                let fragment = Html::parse_document(&text);
                let mut fields = Map::<String, Value>::new();
                for mapper in &self.mappers {
                    let value = mapper.deal_css(fragment.root_element())?;
                    fields.insert(mapper.field.clone(), value);
                }
                Ok(Value::Object(fields))
            }
            "JSON" => Ok(Value::Null),
            "XPATH" => Ok(Value::Null),
            _ => Err("解析器类型错误".to_string()),
        }
    }
}
#[derive(Debug, Getters)]
#[getset(get = "pub(crate)")]
pub(crate) struct Action {
    method: String,
    path: String,
    headers: Option<HashMap<String, String>>,
    path_params: Option<HashMap<String, String>>,
    query_params: Option<HashMap<String, String>>,
    body_params: Option<HashMap<String, String>>,
    extractor: Arc<Extractor>,
}
impl Action {
    pub(crate) fn new(
        method: String,
        path: String,
        path_params: Option<HashMap<String, String>>,
        query_params: Option<HashMap<String, String>>,
        body_params: Option<HashMap<String, String>>,
        headers: Option<HashMap<String, String>>,
        extractor: Arc<Extractor>,
    ) -> Self {
        Action {
            method,
            path,
            path_params,
            query_params,
            body_params,
            headers,
            extractor,
        }
    }
    pub(crate) fn deal_headers(&self, request_builder: RequestBuilder) -> RequestBuilder {
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
    pub(crate) fn url(&self) -> String {
        let mut url = self.path.clone();
        url = match &self.path_params {
            Some(path_params) => {
                for (key, value) in path_params {
                    url = url.replace(format!("@{}", key).as_str(), value);
                }
                url
            }
            None => url,
        };
        match &self.query_params {
            Some(query_params) => {
                let query = query_params
                    .iter()
                    .map(|(key, value)| format!("{}={}", key, value))
                    .collect::<Vec<String>>()
                    .join("&");
                format!("{}?{}", url, query)
            }
            None => url,
        }
    }
    pub(crate) fn deal_body_params(&self, request_builder: RequestBuilder) -> RequestBuilder {
        match &self.body_params {
            Some(body_params) => {
                let body = body_params
                    .iter()
                    .map(|(key, value)| format!("{}={}", key, value))
                    .collect::<Vec<String>>()
                    .join("&");
                request_builder
                    .body(body)
                    .header("Content-Type", "application/x-www-form-urlencoded")
            }
            None => request_builder,
        }
    }

    pub(crate) async fn fetch(&self) -> Result<(), String> {
        let client = Client::new();
        let mut request_builder = match self.method.as_str() {
            "GET" => Ok(client.get(self.url())),
            "POST" => Ok(client.get(self.url())),
            _ => Err(format!("不支持的请求方法:{}", &self.method)),
        }?;
        request_builder = self.deal_headers(request_builder);
        request_builder = self.deal_body_params(request_builder);
        match request_builder.send().await {
            Ok(response) => {
                println!("{:?}", response);
                Ok(())
            }
            Err(err) => Err(format!("{:?}", err)),
        }
    }
}
#[derive(Debug, Getters)]
#[getset(get = "pub(crate)")]
pub(crate) struct Spider {
    field: String,
    origins: Vec<String>,
    addresses: Vec<String>,
    extractors: Vec<Extractor>,
    actions: Vec<Action>,
}
impl Spider {
    pub(crate) fn new(
        field: String,
        origins: Vec<String>,
        addresses: Vec<String>,
        extractors: Vec<Extractor>,
        actions: Vec<Action>,
    ) -> Self {
        Spider {
            field,
            origins,
            addresses,
            extractors,
            actions,
        }
    }
    pub(crate) fn deal(&self) -> Result<(), String> {
        for action in &self.actions {
            println!("{:?}", action);
        }
        Ok(())
    }
}
