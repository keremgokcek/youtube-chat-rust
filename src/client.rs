use crate::request::LivePageData;

pub struct ClientBuilder {
    live_stream_url: String,
    #[cfg(feature = "cookies")]
    cookies: Option<reqwest::cookie::Jar>,
}
impl ClientBuilder {
    #[cfg(feature = "cookies")]
    pub fn cookies(mut self, cookies: reqwest::cookie::Jar) -> Self {
        self.cookies = Some(cookies);
        self
    }
    pub fn build(self) -> Client {
        #[cfg(feature = "cookies")]
        let sapisid = if let Some(cookies) = &self.cookies {
            use reqwest::cookie::CookieStore;
            let cookie_string = cookies.cookies(&reqwest::Url::parse("https://youtube.com").unwrap()).expect("No youtube.com cookies provided");
            Some(cookie_string.to_str().unwrap().split(";").find(|c| c.contains("SAPISID")).unwrap().split_once("=").unwrap().1.to_string())
        } else {
            None
        };

        let session_builder = reqwest::Client::builder();
        #[cfg(feature = "cookies")]
        let session_builder = if let Some(cookies) = self.cookies {
            session_builder.cookie_provider(std::sync::Arc::new(cookies))
        } else {
            session_builder
        };
        let session = session_builder.build().unwrap();

        Client {
            live_stream_url: self.live_stream_url,
            session,
            live_page_data: None,
            #[cfg(feature = "cookies")]
            sapisid,
        }
    }
}

pub struct Client {
    pub(crate) session: reqwest::Client,
    pub(crate) live_stream_url: String,
    pub(crate) live_page_data: Option<LivePageData>,
    #[cfg(feature = "cookies")]
    pub(crate) sapisid: Option<String>,
}

impl Client {
    pub fn builder(live_stream_url: &str) -> ClientBuilder {
        ClientBuilder {
            live_stream_url: live_stream_url.to_owned(),
            #[cfg(feature = "cookies")]
            cookies: None,
        }
    }
}
