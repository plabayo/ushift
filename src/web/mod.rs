use std::str::FromStr;

use anyhow::Context;
use jsonpath_rust::{JsonPathFinder, JsonPathInst};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::Error;

pub mod reqwest;

#[derive(Debug, Default)]
pub enum FetchMethod {
    #[default]
    Get,
    Post,
}

#[derive(Debug)]
pub struct FetchOptions<T: AsRef<str>> {
    pub url: T,
    pub method: FetchMethod,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl<T: AsRef<str>> FetchOptions<T> {
    pub fn new(url: T) -> Self {
        Self {
            url,
            method: FetchMethod::Get,
            headers: Vec::new(),
            body: None,
        }
    }

    pub fn method(&mut self, method: FetchMethod) -> &mut Self {
        self.method = method;
        self
    }

    pub fn header(&mut self, key: String, value: String) -> &mut Self {
        self.headers.push((key, value));
        self
    }

    pub fn body(&mut self, body: String) -> &mut Self {
        self.body = Some(body);
        self
    }
}

impl<T: AsRef<str>> From<T> for FetchOptions<T> {
    fn from(url: T) -> Self {
        Self::new(url)
    }
}

enum JsonSelectorContent<'a> {
    Root(&'a str),
    Value(Value),
}

pub struct JsonSelector<'a> {
    content: JsonSelectorContent<'a>,
}

impl<'a> JsonSelector<'a> {
    pub fn new(body: &'a str) -> Result<Self, Error> {
        Ok(Self {
            content: JsonSelectorContent::Root(body),
        })
    }

    pub fn query(&self, path: impl AsRef<str>) -> Result<JsonSelector<'a>, Error> {
        match &self.content {
            JsonSelectorContent::Root(body) => {
                let finder = JsonPathFinder::from_str(body, path.as_ref())?;
                Ok(Self {
                    content: JsonSelectorContent::Value(finder.find()),
                })
            }
            JsonSelectorContent::Value(value) => {
                let finder = JsonPathFinder::new(
                    Box::new(value.to_owned()),
                    Box::new(JsonPathInst::from_str(path.as_ref())?),
                );
                Ok(Self {
                    content: JsonSelectorContent::Value(finder.find()),
                })
            }
        }
    }

    pub fn get<T: DeserializeOwned>(&self) -> Result<T, Error> {
        match &self.content {
            JsonSelectorContent::Root(body) => Ok(serde_json::from_str(body)?),
            JsonSelectorContent::Value(value) => Ok(serde_json::from_value(
                value
                    .as_array()
                    .and_then(|a| a.first())
                    .context("value as array")?
                    .to_owned(),
            )?),
        }
    }
}

pub struct Response {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl Response {
    pub fn new(status: u16, headers: Vec<(String, String)>, body: Option<String>) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    pub fn json(&self) -> Result<JsonSelector, Error> {
        match &self.body {
            Some(body) => JsonSelector::new(body),
            None => Err("No body".into()),
        }
    }
}

pub trait WebClient {
    async fn fetch<T: AsRef<str>>(
        &self,
        opts: impl Into<FetchOptions<T>>,
    ) -> Result<Response, Error>;
}
