use serde::{Deserialize, Serialize};

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
