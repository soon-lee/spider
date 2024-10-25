use playwright::api::Page;
use playwright::Playwright;
use reqwest::{Client, Response};
use serde_json::{Map, Value};
use std::time::Duration;
use tokio::time::sleep;

pub struct Retriever {
    client: Client,
    page: Page,
    retry: u8,
}
impl Retriever {
    pub async fn new() -> Result<Self, String> {
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
            .map_err(|err| format!("Chrome浏览器启动失败{}", err))?;
        let context = browser
            .context_builder()
            .build()
            .await
            .map_err(|err| format!("创建浏览器上下文失败{}", err))?;
        let page = context
            .new_page()
            .await
            .map_err(|err| format!("创建浏览器页面失败{}", err))?;
        Ok(Retriever {
            client: Client::new(),
            page,
            retry: 3,
        })
    }
    pub async fn request(
        &self,
        url: &str,
        method: &str,
        headers: &Option<Vec<(String, String)>>,
        query: &Option<Vec<(String, String)>>,
    ) -> Result<Response, String> {
        let mut request_builder = match method {
            "GET" => Ok(self.client.get(url)),
            "POST" => Ok(self.client.post(url)),
            _ => Err(String::from("不支持的请求方法")),
        }?;
        if let Some(_headers) = headers {
            let mut items = Map::new();
            for (key, value) in _headers {
                items.insert(key.to_string(), Value::String(value.to_string()));
            }
            request_builder = request_builder.json(&Value::Object(items));
        }
        if let Some(_query) = query {
            for (key, value) in _query {
                request_builder = request_builder.query(&(key, value));
            }
        }
        request_builder
            .send()
            .await
            .map_err(|err| format!("请求静态页面失败【{}】：{}", url, err))
    }
    pub async fn request_with_retry(
        &self,
        url: &str,
        method: &str,
        headers: &Option<Vec<(String, String)>>,
        query: &Option<Vec<(String, String)>>,
    ) -> Result<Response, String> {
        sleep(Duration::from_secs(1)).await;
        match self
            .request(url, method, headers, query)
            .await
            .map_err(|err| format!("首次{}", err))
        {
            Ok(response) => Ok(response),
            Err(_) => {
                let mut response_option = None;
                let mut delay = 1;
                for i in 0..self.retry {
                    sleep(Duration::from_secs(delay)).await;
                    match self
                        .request(url, method, headers, query)
                        .await
                        .map_err(|err| format!("第{}次{}", i + 1, err))
                    {
                        Ok(resp) => {
                            response_option = Some(resp);
                            break;
                        }
                        Err(_) => {
                            delay *= 2;
                            continue;
                        }
                    };
                }
                response_option.ok_or("重试请求静态页面失败".to_string())
            }
        }
    }
    pub async fn static_content(
        &self,
        url: &str,
        method: &str,
        headers: &Option<Vec<(String, String)>>,
        query: &Option<Vec<(String, String)>>,
    ) -> Result<String, String> {
        let response = self.request_with_retry(url, method, headers, query).await?;
        response
            .text()
            .await
            .map_err(|err| format!("获取静态页面内容失败{}", err))
    }
    pub async fn binary_content(
        &self,
        url: &str,
        method: &str,
        headers: &Option<Vec<(String, String)>>,
        query: &Option<Vec<(String, String)>>,
    ) -> Result<Vec<u8>, String> {
        let response = self.request_with_retry(url, method, headers, query).await?;
        let bytes = response
            .bytes()
            .await
            .map_err(|err| format!("获取静态资源字节失败{}", err))?;
        Ok(bytes.to_vec())
    }
    pub async fn dynamic_content(&self, url: &str) -> Result<String, String> {
        self.page
            .goto_builder(url)
            .goto()
            .await
            .map_err(|err| format!("打开动态页面失败{}", err))?;
        self.page
            .content()
            .await
            .map_err(|err| format!("获取动态页面内容失败{}", err))
    }
}
