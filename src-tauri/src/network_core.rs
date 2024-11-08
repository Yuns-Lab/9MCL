use reqwest::{Client, Error};
use std::collections::HashMap;

pub struct NetworkCore {
    client: Client,
}

impl NetworkCore {
    /// 创建一个新的 `NetworkCore` 实例
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// 执行一个基本的 HTTP GET 请求
    pub async fn get_request(&self, url: &str) -> Result<String, Error> {
        let response = self.client.get(url).send().await?;
        let body = response.text().await?;
        Ok(body)
    }

    /// 执行一个带访问头的 HTTP GET 请求
    pub async fn get_request_with_headers(
        &self,
        url: &str,
        headers: HashMap<String, String>,
    ) -> Result<String, Error> {
        let mut request = self.client.get(url);

        // 为请求添加自定义头
        for (key, value) in headers {
            request = request.header(&key, &value);
        }

        let response = request.send().await?;
        let body = response.text().await?;
        Ok(body)
    }
}
