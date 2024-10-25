use scraper::{ElementRef, Selector};
use serde::Deserialize;
use serde_json::{Map, Value};
use std::fmt::format;

#[derive(Deserialize, Debug)]
pub struct JsonExtractor {
    key: String,
    name: String,
    field: String,
    origin: String,
}
impl JsonExtractor {
    pub fn new(key: String, name: String, field: String, origin: String) -> Self {
        Self {
            key,
            name,
            field,
            origin,
        }
    }
    pub fn extract(&self, value: &Value) -> Result<Value, String> {
        let item = value
            .get(&self.origin)
            .ok_or(format!("获取值失败，不存在'{}'属性", self.origin))?;
        Ok(item.clone())
    }
}
#[derive(Deserialize, Debug)]
pub struct JsonMapper {
    key: String,
    name: String,
    field: String,
    expression: Option<String>,
    sequential: bool,
    extractors: Option<Vec<JsonExtractor>>,
    children: Option<Vec<JsonMapper>>,
}
impl JsonMapper {
    pub fn new(
        key: String,
        name: String,
        field: String,
        expression: Option<String>,
        sequential: bool,
        extractors: Option<Vec<JsonExtractor>>,
        children: Option<Vec<JsonMapper>>,
    ) -> Self {
        Self {
            key,
            name,
            field,
            expression,
            sequential,
            extractors,
            children,
        }
    }
    fn extract(&self, value: &Value) -> Result<Map<String, Value>, String> {
        let mut items = Map::<String, Value>::new();
        if let Some(extractors) = &self.extractors {
            for extractor in extractors {
                let item = extractor.extract(value)?;
                items.insert(extractor.field.clone(), item);
            }
        }
        Ok(items)
    }
    pub fn handle_expression(&self, value: &Value) -> Result<Vec<Value>, String> {
        if let Some(expression) = &self.expression {
            let fields = expression.split(".");
            let mut items = vec![value.clone()];
            for field in fields {
                let mut temp = vec![];
                while !items.is_empty() {
                    let current = items.pop().ok_or(String::from("获取元素失败"))?;
                    // println!("current: {:?}", current);
                    if field.starts_with("[") && field.ends_with("]") {
                        let index = &field[1..field.len() - 1];
                        let array = current
                            .get(index)
                            .ok_or(format!("表达式错误，'{}'属性不存在", index))?
                            .as_array()
                            .ok_or(format!("表达式错误，'{}'不是数组", index))?;
                        temp.extend(array.clone());
                    } else {
                        let obj = current
                            .get(field)
                            .ok_or(format!("表达式错误，'{}'属性不存在", field))?;
                        temp.push(obj.clone());
                    }
                }
                items = temp;
            }
            Ok(items)
        } else {
            Ok(vec![value.clone()])
        }
    }

    pub fn mapping(&self, value: &Value) -> Result<Value, String> {
        let objs = match self.sequential {
            true => {
                let mut array = vec![];
                let items = value
                    .as_array()
                    .ok_or(String::from("'sequential'属性错误，不是数组"))?;
                for item in items {
                    let objs = self.handle_expression(item)?;
                    array.extend(objs);
                }
                Ok(array)
            }
            false => self.handle_expression(value),
        }?;
        let mut items = vec![];
        for obj in objs {
            let mut fields = self.extract(&obj)?;
            if let Some(children) = &self.children {
                for child in children {
                    let item = child.mapping(&obj)?;
                    fields.insert(child.field.clone(), item);
                }
            }
            items.push(Value::Object(fields));
        }
        if items.is_empty() {
            Ok(Value::Array(items))
        } else if items.len() == 1 {
            Ok(items[0].clone())
        } else {
            Ok(Value::Array(items))
        }
    }
}
#[derive(Deserialize, Debug)]
pub struct HtmlExtractor {
    key: String,
    name: String,
    field: String,
    expression: String,
    sequential: bool,
    operation: String,
    regexp: Option<String>,
}
impl HtmlExtractor {
    pub fn new(
        key: String,
        name: String,
        field: String,
        expression: String,
        sequential: bool,
        operation: String,
        regexp: Option<String>,
    ) -> Self {
        Self {
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
                            .ok_or(String::from("正则提取失败，请使用'()'包裹来提取。"))?;
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
pub struct HtmlMapper {
    key: String,
    name: String,
    field: String,
    expression: String,
    sequential: bool,
    extractors: Option<Vec<HtmlExtractor>>,
    children: Option<Vec<HtmlMapper>>,
}
impl HtmlMapper {
    pub fn new(
        key: String,
        name: String,
        field: String,
        expression: String,
        sequential: bool,
        extractors: Option<Vec<HtmlExtractor>>,
        children: Option<Vec<HtmlMapper>>,
    ) -> Self {
        Self {
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
