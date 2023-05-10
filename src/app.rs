use crate::{spider::Spider, Error};

pub struct App<S> {
    spider: S,
}

impl<S: Spider> App<S> {
    pub fn new(spider: S) -> Self {
        Self { spider }
    }

    pub async fn run(&self) -> Result<(), Error> {
        let client = crate::web::reqwest::ReqwestClient::new();
        self.spider.crawl(client).await
    }
}
