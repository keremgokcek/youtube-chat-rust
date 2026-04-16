use crate::internal::common::SimpleText;

use super::common::{Runs, Thumbnails};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Poll {
    pub choices: Vec<PollChoice>,
    pub header: PollHeaderWrapper,
    pub live_chat_poll_id: String,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PollWrapper {
    pub poll_renderer: Poll,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PollChoice {
    pub selected: bool,
    pub text: Runs,
    pub vote_ratio: Option<f64>,
    pub vote_percentage: Option<SimpleText>,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PollHeaderWrapper {
    pub poll_header_renderer: PollHeader,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PollHeader {
    pub live_chat_poll_type: String,
    pub metadata_text: Runs,
    pub poll_question: Runs,
    pub thumbnail: Thumbnails,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionPanelWrapper {
    pub live_chat_action_panel_renderer: ActionPanel,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionPanel {
    pub contents: PollWrapper,
    pub id: String,
}
