use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChatResponse {
    pub continuation_contents: ContinuationContents,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuationContents {
    pub live_chat_continuation: ChatContinuation,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatContinuation {
    pub continuations: Vec<Continuation>,
    pub actions: Option<Vec<ActionWrapper>>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum Continuation {
    #[serde(rename = "invalidationContinuationData")]
    Invalidation { continuation: String },
    #[serde(rename = "timedContinuationData")]
    Timed { continuation: String },
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionWrapper {
    pub click_tracking_params: Option<String>,
    #[serde(flatten)]
    pub action: Action,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all_fields = "camelCase")]
pub enum Action {
    #[serde(rename = "addChatItemAction")]
    AddChatItem { item: Item },
    #[serde(rename = "replaceChatItemAction")]
    ReplaceChatItem { target_item_id: String, replacement_item: Item },
    #[serde(rename = "removeChatItemByAuthorAction")]
    RemoveChatItemByAuthor { external_channel_id: String },
    #[serde(rename = "removeChatItemAction")]
    RemoveChatItem { target_item_id: String },
    #[serde(rename = "addBannerToLiveChatCommand")]
    AddBannerToLiveChat { banner_renderer: BannerWrapper },
    #[serde(rename = "addLiveChatTickerItemAction")]
    AddLiveChatTickerItem { item: TickerItem, duration_sec: String },
    #[serde(rename = "showLiveChatActionPanelAction")]
    ShowLiveChatActionPanel { panel_to_show: Item },
    #[serde(rename = "updateLiveChatPollAction")]
    UpdateLiveChatPoll { poll_to_update: PollWrapper },
    #[serde(untagged)]
    Unknown(serde_json::Value),
}

// Common types
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleText {
    pub simple_text: String,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum HeaderSubtext {
    Single(SimpleText),
    Multiple(Runs),
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Runs {
    pub runs: Vec<Run>,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum Run {
    Text(Text),
    Emoji { emoji: Emoji },
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    pub emoji_id: String,
    pub image: Thumbnails,
    pub is_custom_emoji: Option<bool>,
    pub search_terms: Option<Vec<String>>,
    pub shortcuts: Option<Vec<String>>,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub text: String,
    pub bold: Option<bool>,
    pub italics: Option<bool>,
    pub navigation_endpoint: Option<NavigationEndpoint>,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint {
    pub url_endpoint: UrlEndpoint,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlEndpoint {
    pub url: String,
    pub target: String,
    pub nofollow: bool,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnails {
    pub thumbnails: Vec<Thumbnail>,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub url: String,
    pub width: Option<u16>,
    pub height: Option<u16>,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatAuthorBadgeRendererWrapper {
    live_chat_author_badge_renderer: LiveChatAuthorBadgeRenderer,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    icon_type: String,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatAuthorBadgeRenderer {
    pub custom_thumbnail: Option<Thumbnails>,
    pub icon: Option<Icon>,
    pub tooltip: String,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeaderWrapper {
    pub live_chat_sponsorships_header_renderer: Header,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub author_badges: Option<Vec<LiveChatAuthorBadgeRendererWrapper>>,
    pub author_name: SimpleText,
    pub author_photo: Thumbnails,
    pub image: Thumbnails,
    pub primary_text: Runs,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerWrapper {
    pub live_chat_banner_renderer: Banner,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Banner {
    pub contents: Item,
    pub action_id: String,
    pub viewer_is_creator: Option<bool>,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowItemWrapper {
    pub show_live_chat_item_endpoint: ShowItem,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowItem {
    pub renderer: Item,
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

// YouTube Message Types
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all_fields = "camelCase")]
pub enum Item {
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
    #[serde(rename = "liveChatBannerChatSummaryRenderer")]
    BannerChatSummary { chat_summary: Runs, icon: Icon },
    #[serde(rename = "liveChatActionPanelRenderer")]
    LiveChatActionPanel { contents: PollWrapper, id: String },
    #[serde(rename = "liveChatViewerEngagementMessageRenderer")]
    ViewerEngagementMessage {},
    #[serde(untagged)]
    Unknown(serde_json::Value),
}

// YouTube Ticker Item Types
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
