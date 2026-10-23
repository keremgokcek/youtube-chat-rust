use super::message::{Message, MessagePart};
use super::poll::Poll;
use crate::internal::common::Run;
use crate::internal::{Action, banner::BannerItem};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, Deserialize, PartialEq, Serialize)]
pub enum Event {
    // Normal events
    MessageSent {
        item: Message,
    },
    MessagePinned {
        action_id: String,
        item: Message,
    },
    MessageApproved {
        target_item_id: String,
        replacement_item: Message,
    },
    /// Message deleted by author or moderator (Not used when user is moderator)
    MessageDeleted {
        target_item_id: String,
    },
    BannerRemoved {
        target_action_id: String,
    },
    /// Ban or timeout
    UserRestricted {
        target_channel_id: String,
    },
    /// Returns when a moderator is active on chat
    ModeratorActive {},

    // Moderator Events
    /// Message deleted by the message author himself
    MessageDeletedByAuthor {
        target_item_id: String,
    },
    /// Message deleted by a moderator
    MessageDeletedByModerator {
        target_item_id: String,
        moderator_username: String,
    },
    /// Ban or timeout by a moderator
    MessagesDeletedByModerator {
        target_channel_id: String,
        moderator_username: String,
    },
    /// Reports presence of current user (runs every 60 seconds)
    ReportPresence {},

    // Poll Events
    PollAdded {
        poll: Poll,
    },
    PollUpdated {
        poll: Poll,
    },
    PollEnded {
        poll_id: String,
    },

    // YouTube Features
    ChatSummary {
        action_id: String,
        summary: String,
    },
    Redirect {
        action_id: String,
        profile_picture_url: String,
        username: String,
    },
}

impl From<&Action> for Event {
    fn from(value: &Action) -> Self {
        match value {
            Action::AddChatItem { item } => Event::MessageSent { item: item.into() },
            Action::ReplaceChatItem {
                target_item_id,
                replacement_item,
            } => Event::MessageApproved {
                target_item_id: target_item_id.into(),
                replacement_item: replacement_item.into(),
            },
            Action::RemoveChatItem { target_item_id } => Event::MessageDeleted {
                target_item_id: target_item_id.into(),
            },
            Action::RemoveChatItemByAuthor { external_channel_id } => Event::UserRestricted {
                target_channel_id: external_channel_id.into(),
            },
            Action::MarkChatItemAsDeleted {
                deleted_state_message,
                target_item_id,
            } => {
                let target_item_id = target_item_id.into();
                match deleted_state_message.runs.as_slice() {
                    [_] => Event::MessageDeletedByAuthor { target_item_id },
                    [_, moderator, _] => {
                        if let MessagePart::Text(username) = moderator.into() {
                            Event::MessageDeletedByModerator {
                                target_item_id,
                                moderator_username: username.into(),
                            }
                        } else {
                            unreachable!("Unexpected moderator delete action: {:#?}", deleted_state_message);
                        }
                    }
                    _ => panic!("Unexpected MarkChatItemAsDeleted structure"),
                }
            }
            Action::MarkChatItemsByAuthorAsDeleted {
                deleted_state_message,
                external_channel_id,
            } => {
                let [_, moderator, _] = deleted_state_message.runs.as_slice() else {
                    panic!("Unexpected structure: {:#?}", deleted_state_message);
                };

                let MessagePart::Text(username) = moderator.into() else {
                    unreachable!("Unexpected moderator action: {:#?}", deleted_state_message);
                };

                Event::MessagesDeletedByModerator {
                    target_channel_id: external_channel_id.into(),
                    moderator_username: username.into(),
                }
            }
            Action::AddBannerToLiveChat { banner_renderer } => {
                let action_id = banner_renderer.live_chat_banner_renderer.action_id.clone();
                match &banner_renderer.live_chat_banner_renderer.contents {
                    BannerItem::MessageItem(item) => Event::MessagePinned { action_id, item: item.into() },
                    BannerItem::BannerChatSummary { chat_summary, icon: _ } => match chat_summary.runs.last().unwrap() {
                        Run::Text(t) => Event::ChatSummary {
                            action_id,
                            summary: t.text.clone(),
                        },
                        _ => unreachable!("Unexpected non-text chat summary: {chat_summary:#?}"),
                    },
                    BannerItem::Redirect {
                        author_photo,
                        redirect_message,
                    } => Event::Redirect {
                        action_id,
                        profile_picture_url: author_photo.thumbnails.last().unwrap().url.clone(),
                        username: match redirect_message.runs.first() {
                            Some(Run::Text(t)) => t.text.clone(),
                            _ => unreachable!("Unexpected redirect message content: {redirect_message:#?}"),
                        },
                    },
                    _ => todo!("Unsupported BannerItem: {:#?}", banner_renderer.live_chat_banner_renderer.contents),
                }
            }
            Action::RemoveBannerForLiveChat { target_action_id } => Event::BannerRemoved {
                target_action_id: target_action_id.clone(),
            },
            Action::ShowLiveChatActionPanel { panel_to_show } => Event::PollAdded {
                poll: (&panel_to_show.live_chat_action_panel_renderer.contents).into(),
            },
            Action::UpdateLiveChatPoll { poll_to_update } => Event::PollUpdated { poll: poll_to_update.into() },
            Action::CloseLiveChatActionPanel { target_panel_id } => Event::PollEnded {
                poll_id: target_panel_id.into(),
            },
            Action::ReportModerationState {} => Event::ModeratorActive {},
            Action::ReportPresence {} => Event::ReportPresence {},
            _ => panic!("[TODO] Internal to public type matching not supported for this type yet: {value:#?}"),
        }
    }
}
