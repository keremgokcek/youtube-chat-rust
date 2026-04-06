use crate::request::LivePageData;

pub struct Client {
    pub session: reqwest::Client,
    pub live_page_data: Option<LivePageData>,
}

impl Client {
    pub fn new() -> Self {
        let session = reqwest::Client::new();
        Client { session, live_page_data: None }
    }
}
