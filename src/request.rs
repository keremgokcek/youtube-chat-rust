use crate::{
    client::Client,
    types::{Continuation, GetChatResponse},
};
use regex::Regex;
use serde::Serialize;
use std::fs::write;

static LIVE_URL: &str = "https://www.youtube.com/@TugayAloglu/live";

#[derive(Debug)]
pub struct LivePageData {
    pub video_id: String,
    pub channel_id: String,
    pub api_key: String,
    pub client_version: String,
    pub continuation: String,
}

fn dump_to_file(text: &str) {
    let _ = write("error.json", text);
}

impl Client {
    pub async fn get_options_from_live_page(&mut self) -> Result<(), reqwest::Error> {
        let body = self.session.get(LIVE_URL).send().await?.text().await?;

        let video_id_regex = Regex::new(r#"is_popout=1\\u0026v=([A-Za-z0-9_-]{11})"#).unwrap();
        let channel_id_regex = Regex::new(r#""channelId":"([A-Za-z0-9_]{24})""#).unwrap();
        let replay_regex = Regex::new(r#"['"]isReplay['"]:\s*(true)"#).unwrap();
        let api_key_regex = Regex::new(r#"['"]INNERTUBE_API_KEY['"]:\s*['"](.+?)['"]"#).unwrap();
        let client_version_regex = Regex::new(r#"['"]clientVersion['"]:\s*['"]([\d.]+?)['"]"#).unwrap();
        let continuation_regex = Regex::new(r#"['"]continuation['"]:\s*['"](.+?)['"]"#).unwrap();

        let video_id = match video_id_regex.captures(&body).and_then(|s| s.get(1)) {
            Some(v) => v.as_str(),
            None => {
                let _ = write("error.html", &body);
                panic!("Live stream was not found")
            }
        };
        let channel_id = channel_id_regex.captures(&body).and_then(|s| s.get(1)).expect("Live stream was not found").as_str();

        if replay_regex.is_match(&body) {
            panic!("{video_id} is finished live stream");
        }

        let api_key = api_key_regex.captures(&body).and_then(|s| s.get(1)).expect("API key was not found").as_str();
        let client_version = client_version_regex.captures(&body).and_then(|s| s.get(1)).expect("Client version was not found").as_str();
        let continuation = continuation_regex.captures(&body).and_then(|s| s.get(1)).expect("Continuation was not found").as_str();

        self.live_page_data = Some(LivePageData {
            video_id: video_id.to_string(),
            channel_id: channel_id.to_string(),
            api_key: api_key.to_string(),
            client_version: client_version.to_string(),
            continuation: continuation.to_string(),
        });

        Ok(())
    }

    pub async fn fetch_chat(&mut self) -> Result<GetChatResponse, Box<dyn std::error::Error>> {
        let page_data = self.live_page_data.as_ref().expect("PageData doesn't exist, run get_options_from_live_page first");
        let url = format!("https://www.youtube.com/youtubei/v1/live_chat/get_live_chat?key={}", page_data.api_key);
        let body = LiveChatBody {
            context: ContextBody {
                client: ClientBody {
                    client_name: "WEB".to_string(),
                    client_version: page_data.client_version.clone(),
                    hl: "tr".to_string(),
                },
            },
            continuation: page_data.continuation.clone(),
        };

        let resp = self.session.post(url).json(&body).send().await?;
        let bytes = resp.bytes().await?;
        // let bytes = std::fs::read("error.json").unwrap();

        let text = String::from_utf8_lossy(&bytes);
        let json: GetChatResponse = match serde_json::from_slice(&bytes) {
            Ok(json) => json,
            Err(e) => {
                dump_to_file(&text);
                println!("This is the error object: \"{e:#?}\"");
                todo!()
            }
        };

        let continuation = match &json.continuation_contents.live_chat_continuation.continuations.first().unwrap() {
            Continuation::Invalidation { continuation } => continuation.clone(),
            Continuation::Timed { continuation } => continuation.clone(),
        };
        self.live_page_data.as_mut().map(|p| p.continuation = continuation);

        Ok(json)
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
