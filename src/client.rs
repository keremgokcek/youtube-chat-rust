use crate::request::LivePageData;
#[cfg(feature = "cookies")]
use reqwest::cookie::Jar;

pub struct Client {
    pub session: reqwest::Client,
    pub live_page_data: Option<LivePageData>,
    #[cfg(feature = "cookies")]
    pub(crate) sapisid: String,
}

impl Client {
    #[cfg(not(feature = "cookies"))]
    pub fn new() -> Self {
        let session = reqwest::Client::new();
        Client { session, live_page_data: None }
    }
    #[cfg(feature = "cookies")]
    pub fn new_with_cookie(cookie: Jar) -> Self {
        use reqwest::cookie::CookieStore;
        use std::sync::Arc;

        let cookie_string = cookie.cookies(&reqwest::Url::parse("https://youtube.com").unwrap()).expect("No youtube.com cookies provided");
        let sapisid = cookie_string.to_str().unwrap().split(";").find(|c| c.contains("SAPISID")).unwrap().split_once("=").unwrap().1.to_string();
        let session = reqwest::Client::builder().cookie_provider(Arc::new(cookie)).build().unwrap();

        Client {
            session,
            live_page_data: None,
            sapisid,
        }
    }
}
