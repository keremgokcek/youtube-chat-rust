use super::common::{Icon, Runs, Thumbnails};
use super::message::MessageItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerWrapper {
    pub live_chat_banner_renderer: Banner,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Banner {
    pub contents: BannerItem,
    pub action_id: String,
    pub viewer_is_creator: Option<bool>,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Poll {
    pub choices: Vec<PollChoice>,
    pub header: PollHeaderWrapper,
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
#[serde(rename_all_fields = "camelCase")]
pub enum BannerItem {
    #[serde(rename = "liveChatBannerChatSummaryRenderer")]
    BannerChatSummary { chat_summary: Runs, icon: Icon },
    #[serde(untagged)]
    MessageItem(MessageItem),
}
