use super::{Error, FetchMethod, FetchOptions, Response, WebClient};

pub struct ReqwestClient {
    client: reqwest::Client,
}

impl ReqwestClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl Default for ReqwestClient {
    fn default() -> Self {
        Self::new()
    }
}

impl WebClient for ReqwestClient {
    async fn fetch<T: AsRef<str>>(
        &self,
        opts: impl Into<FetchOptions<T>>,
    ) -> Result<Response, Error> {
        let opts = opts.into();

        let resp = match opts.method {
            FetchMethod::Get => self.client.get(opts.url.as_ref()).send().await?,
            FetchMethod::Post => self.client.post(opts.url.as_ref()).send().await?,
        };

        let status = resp.status().as_u16();
        let headers = resp
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
            .collect();
        let body = resp.text().await.ok();

        Ok(Response::new(status, headers, body))
    }
}
