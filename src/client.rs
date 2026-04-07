use crate::request::LivePageData;
use reqwest::cookie::Jar;
use std::sync::Arc;

pub struct Client {
    pub session: reqwest::Client,
    pub live_page_data: Option<LivePageData>,
}

impl Client {
    pub fn new() -> Self {
        let session = reqwest::Client::new();
        Client { session, live_page_data: None }
    }
    #[cfg(feature = "cookies")]
    pub fn new_with_cookie(cookie: Jar) -> Self {
        let session = reqwest::Client::builder().cookie_provider(Arc::new(cookie)).build().unwrap();
        Client { session, live_page_data: None }
    }
}
