use crate::{web::WebClient, Error};

pub trait Spider {
    async fn crawl(&self, client: impl WebClient) -> Result<(), Error>;
}
