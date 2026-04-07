use super::common::{SimpleText, Thumbnails};
use super::message::MessageItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowItemWrapper {
    pub show_live_chat_item_endpoint: ShowItem,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowItem {
    pub renderer: MessageItem,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all_fields = "camelCase")]
pub enum TickerItem {
    #[serde(rename = "liveChatTickerSponsorItemRenderer")]
    Sponsor {
        author_external_channel_id: String,
        detail_text_color: u32,
        duration_sec: u32,
        end_background_color: u32,
        full_duration_sec: u32,
        id: String,
        sponsor_photo: Thumbnails,
        show_item_endpoint: ShowItemWrapper,
        start_background_color: u32,
    },
    #[serde(rename = "liveChatTickerPaidMessageItemRenderer")]
    PaidMessage {
        amount_text_color: u32,
        author_external_channel_id: String,
        author_photo: Thumbnails,
        author_username: SimpleText,
        duration_sec: u32,
        end_background_color: u32,
        full_duration_sec: u32,
        id: String,
        show_item_endpoint: ShowItemWrapper,
        start_background_color: u32,
    },
    #[serde(untagged)]
    Unknown(serde_json::Value),
}
