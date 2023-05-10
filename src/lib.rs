#![forbid(unsafe_code)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

pub mod app;
pub mod spider;
pub mod web;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
