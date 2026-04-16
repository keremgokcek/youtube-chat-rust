use super::message::{Message, MessagePart};
use super::poll::Poll;
use crate::internal::{Action, banner::BannerItem};

#[derive(Debug)]
pub enum Event {
    // Normal events
    MessageSent {
        item: Message,
    },
    MessagePinned {
        item: Message,
    },
    MessageApproved {
        target_item_id: String,
        replacement_item: Message,
    },
    MessageDeleted {
        target_item_id: String,
    },
    /// Ban or timeout
    UserRestricted {
        target_channel_id: String,
    },

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
            Action::AddBannerToLiveChat { banner_renderer } => match &banner_renderer.live_chat_banner_renderer.contents {
                BannerItem::MessageItem(item) => Event::MessagePinned { item: item.into() },
                _ => todo!(),
            },
            Action::ShowLiveChatActionPanel { panel_to_show } => Event::PollAdded {
                poll: (&panel_to_show.live_chat_action_panel_renderer.contents).into(),
            },
            Action::UpdateLiveChatPoll { poll_to_update } => Event::PollUpdated { poll: poll_to_update.into() },
            Action::CloseLiveChatActionPanel { target_panel_id } => Event::PollEnded {
                poll_id: target_panel_id.into(),
            },
            _ => panic!("Action not supported yet"),
        }
    }
}
