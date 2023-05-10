#![forbid(unsafe_code)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

use ushift::{app::App, spider::Spider, web::WebClient, Error};

pub struct MyLocationSpider;

impl Spider for MyLocationSpider {
    async fn crawl(&self, client: impl WebClient) -> Result<(), Error> {
        let resp = client.fetch("https://api.ipify.org?format=json").await?;
        let ip: String = resp.json()?.query("$.ip")?.get()?;
        println!("Your IP is: {}", ip);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    App::new(MyLocationSpider).run().await
}
