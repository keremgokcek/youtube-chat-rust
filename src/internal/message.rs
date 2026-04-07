use super::common::{HeaderSubtext, HeaderWrapper, LiveChatAuthorBadgeRendererWrapper, Runs, SimpleText, Thumbnails};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all_fields = "camelCase")]
pub enum MessageItem {
    #[serde(rename = "liveChatTextMessageRenderer")]
    TextMessage {
        message: Runs,
        author_name: SimpleText,
        author_photo: Thumbnails,
        id: String,
        timestamp_usec: String,
        author_badges: Option<Vec<LiveChatAuthorBadgeRendererWrapper>>,
        author_external_channel_id: String,
    },
    #[serde(rename = "liveChatMembershipItemRenderer")]
    MembershipItem {
        author_badges: Option<Vec<LiveChatAuthorBadgeRendererWrapper>>,
        author_external_channel_id: String,
        author_name: SimpleText,
        author_photo: Thumbnails,
        header_primary_text: Option<Runs>,
        header_subtext: HeaderSubtext,
        message: Option<Runs>,
        id: String,
        timestamp_usec: String,
    },
    #[serde(rename = "liveChatPaidMessageRenderer")]
    PaidMessage {
        author_badges: Option<Vec<LiveChatAuthorBadgeRendererWrapper>>,
        author_external_channel_id: String,
        author_name: SimpleText,
        author_name_text_color: u32,
        author_photo: Thumbnails,
        body_background_color: u32,
        body_text_color: u32,
        header_background_color: u32,
        header_text_color: u32,
        id: String,
        message: Option<Runs>,
        purchase_amount_text: SimpleText,
        text_input_background_color: u32,
        timestamp_color: u32,
        timestamp_usec: String,
    },
    #[serde(rename = "liveChatSponsorshipsGiftPurchaseAnnouncementRenderer")]
    SponsorshipsGiftPurchase {
        author_external_channel_id: String,
        header: HeaderWrapper,
        id: String,
        timestamp_usec: String,
    },
    #[serde(rename = "liveChatSponsorshipsGiftRedemptionAnnouncementRenderer")]
    SponsorshipsGiftRedemption {
        author_badges: Option<Vec<LiveChatAuthorBadgeRendererWrapper>>,
        author_external_channel_id: String,
        author_name: SimpleText,
        author_photo: Thumbnails,
        id: String,
        message: Runs,
        timestamp_usec: String,
    },
    #[serde(rename = "liveChatViewerEngagementMessageRenderer")]
    ViewerEngagementMessage {},
    #[serde(untagged)]
    Unknown(serde_json::Value),
}
