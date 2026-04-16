use super::common::{Icon, Runs, SimpleText, Thumbnails};
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
#[serde(rename_all_fields = "camelCase")]
pub enum BannerItem {
    #[serde(rename = "liveChatBannerChatSummaryRenderer")]
    BannerChatSummary { chat_summary: Runs, icon: Icon },
    #[serde(rename = "liveChatBannerRedirectRenderer")]
    Redirect { author_photo: Thumbnails, banner_message: Runs },
    #[serde(rename = "liveChatCallForQuestionsRenderer")]
    CallForQuestions {
        creator_author_name: SimpleText,
        creator_avatar: Thumbnails,
        question_message: Runs,
    },
    #[serde(untagged)]
    MessageItem(MessageItem),
}
