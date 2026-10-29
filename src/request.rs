use crate::client::Client;
use crate::internal::{ChatContinuation, Continuation, ContinuationContents, GetChatResponse};
use regex::Regex;
use reqwest::StatusCode;
use serde::Serialize;
use std::fs::write;

#[derive(Debug)]
pub(crate) struct LivePageData {
    pub(crate) video_id: String,
    pub(crate) channel_id: String,
    #[cfg(feature = "cookies")]
    pub(crate) datasync_id: Option<String>,
    pub(crate) api_key: String,
    pub(crate) client_version: String,
    pub(crate) continuation: String,
}

pub(crate) enum Error {
    NetworkError(reqwest::Error),
    StreamEnded,
}
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::NetworkError(err)
    }
}

impl Client {
    pub(crate) async fn get_options_from_live_page(&mut self) -> Result<Option<()>, reqwest::Error> {
        let body = self.session.get(&self.live_stream_url).send().await?.text().await?;

        // DEBUG
        write("last_live_page.html", &body).unwrap();

        let video_id_regex = Regex::new(r#""videoId":"([A-Za-z0-9_-]{11})""#).unwrap();
        let channel_id_regex = Regex::new(r#""channelId":"([A-Za-z0-9_-]{24})""#).unwrap();
        #[cfg(feature = "cookies")]
        let datasync_id_regex = Regex::new(r#""datasyncId":"([0-9]{21})"#).unwrap();
        let replay_regex = Regex::new(r#"['"]isReplay['"]:\s*(true)"#).unwrap();
        let api_key_regex = Regex::new(r#"['"]INNERTUBE_API_KEY['"]:\s*['"](.+?)['"]"#).unwrap();
        let client_version_regex = Regex::new(r#"['"]clientVersion['"]:\s*['"]([\d.]+?)['"]"#).unwrap();
        let continuation_regex = Regex::new(r#"['"]continuation['"]:\s*['"](.+?)['"]"#).unwrap();

        let video_id = match video_id_regex.captures(&body).and_then(|s| s.get(1)) {
            Some(v) => v.as_str().to_owned(),
            None => return Ok(None),
        };
        let channel_id = channel_id_regex.captures(&body).and_then(|s| s.get(1)).expect("Live stream was not found").as_str().to_owned();
        #[cfg(feature = "cookies")]
        // TODO: If not found, provided cookies are invalid or expired.
        let datasync_id = if self.sapisid.is_some() {
            Some(datasync_id_regex.captures(&body).and_then(|s| s.get(1)).unwrap().as_str().to_owned())
        } else {
            None
        };

        if replay_regex.is_match(&body) {
            panic!("{video_id} is finished live stream");
        }

        let api_key = api_key_regex.captures(&body).and_then(|s| s.get(1)).expect("API key was not found").as_str().to_owned();
        let client_version = client_version_regex.captures(&body).and_then(|s| s.get(1)).expect("Client version was not found").as_str().to_owned();
        let continuation = continuation_regex.captures(&body).and_then(|s| s.get(1)).expect("Continuation was not found").as_str().to_owned();

        self.live_page_data = Some(LivePageData {
            video_id,
            channel_id,
            #[cfg(feature = "cookies")]
            datasync_id,
            api_key,
            client_version,
            continuation,
        });

        Ok(Some(()))
    }

    pub(crate) async fn fetch_chat(&mut self) -> Result<ContinuationContents, Error> {
        let page_data = self.live_page_data.as_mut().expect("PageData doesn't exist, run get_options_from_live_page first");

        let url = format!("https://www.youtube.com/youtubei/v1/live_chat/get_live_chat?key={}", page_data.api_key);
        let body = LiveChatBody {
            context: ContextBody {
                client: ClientBody {
                    client_name: "WEB".to_string(),
                    client_version: page_data.client_version.clone(),
                    hl: "en".to_string(),
                },
            },
            continuation: page_data.continuation.clone(),
        };

        let req = self.session.post(url).json(&body);

        #[cfg(feature = "cookies")]
        let req = if let (Some(sapisid), Some(datasync_id)) = (&self.sapisid, &page_data.datasync_id) {
            use sha1_smol::Sha1;
            use std::time::{SystemTime, UNIX_EPOCH};

            const ORIGIN: &str = "https://www.youtube.com";

            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
            let hash = Sha1::from(format!("{} {} {} {}", datasync_id, timestamp, sapisid, ORIGIN)).hexdigest();

            req.header("Authorization", format!("SAPISIDHASH {timestamp}_{hash}_u")).header("X-Goog-AuthUser", "0").header("X-Origin", ORIGIN)
        } else {
            req
        };

        let resp = match req.send().await {
            Ok(r) if r.status() == StatusCode::OK => r,
            _ => {
                return Ok(ContinuationContents {
                    live_chat_continuation: ChatContinuation {
                        continuations: Vec::new(),
                        actions: None,
                    },
                });
            }
        };

        let bytes = resp.bytes().await?;

        // DEBUG
        write("last_live_chat.json", &bytes).unwrap();

        let json: GetChatResponse = match serde_json::from_slice(&bytes) {
            Ok(json) => json,
            Err(e) => {
                println!("This is the error object: \"{e:#?}\"");
                todo!()
            }
        };

        let continuation_contents = match json.continuation_contents {
            Some(c) => c,
            None => return Err(Error::StreamEnded),
        };

        let continuation = match &continuation_contents.live_chat_continuation.continuations.first().unwrap() {
            Continuation::Invalidation { continuation } => continuation.clone(),
            Continuation::Timed { continuation } => continuation.clone(),
        };
        page_data.continuation = continuation;

        Ok(continuation_contents)
    }
}

#[derive(Serialize)]
struct LiveChatBody {
    context: ContextBody,
    continuation: String,
}
#[derive(Serialize)]
struct ContextBody {
    client: ClientBody,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ClientBody {
    client_name: String,
    client_version: String,
    hl: String,
}
