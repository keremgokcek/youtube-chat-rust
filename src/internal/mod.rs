use serde::{Deserialize, Serialize};

pub mod banner;
pub mod common;
pub mod message;
pub mod poll;
pub mod ticker;

use banner::BannerWrapper;
use common::Runs;
use message::MessageItem;
use poll::PollWrapper;
use ticker::TickerItem;

use crate::internal::poll::ActionPanelWrapper;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChatResponse {
    pub continuation_contents: Option<ContinuationContents>,
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
    AddChatItem { item: MessageItem },
    #[serde(rename = "replaceChatItemAction")]
    ReplaceChatItem { target_item_id: String, replacement_item: MessageItem },
    #[serde(rename = "removeChatItemByAuthorAction")]
    RemoveChatItemByAuthor { external_channel_id: String },
    #[serde(rename = "removeChatItemAction")]
    RemoveChatItem { target_item_id: String },
    #[serde(rename = "addBannerToLiveChatCommand")]
    AddBannerToLiveChat { banner_renderer: BannerWrapper },
    #[serde(rename = "removeBannerForLiveChatCommand")]
    RemoveBannerForLiveChat { target_action_id: String },
    #[serde(rename = "addLiveChatTickerItemAction")]
    AddLiveChatTickerItem { item: TickerItem, duration_sec: String },
    #[serde(rename = "showLiveChatActionPanelAction")]
    ShowLiveChatActionPanel { panel_to_show: ActionPanelWrapper },
    #[serde(rename = "closeLiveChatActionPanelAction")]
    CloseLiveChatActionPanel { target_panel_id: String },
    #[serde(rename = "updateLiveChatPollAction")]
    UpdateLiveChatPoll { poll_to_update: PollWrapper },
    // Moderator Actions
    #[serde(rename = "markChatItemAsDeletedAction")]
    MarkChatItemAsDeleted { deleted_state_message: Runs, target_item_id: String },
    #[serde(rename = "markChatItemsByAuthorAsDeletedAction")]
    MarkChatItemsByAuthorAsDeleted { deleted_state_message: Runs, external_channel_id: String },
    #[serde(rename = "liveChatReportModerationStateCommand")]
    ReportModerationState {},
    #[serde(rename = "liveChatReportPresenceCommand")]
    ReportPresence {},
    #[serde(untagged)]
    Unknown(serde_json::Value),
}
