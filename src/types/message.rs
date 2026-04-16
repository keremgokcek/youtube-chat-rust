use super::author::{Author, convert_to_author};
use crate::internal::{
    common::{HeaderSubtext, Run},
    message::MessageItem,
};
use url::Url;

#[derive(Debug)]
pub enum Message {
    TextMessage(TextMessage),
    NewMembership(NewMembership),
    MembershipMilestone(MembershipMilestone),
    SuperChat(SuperChat),
    SuperSticker(SuperSticker),
    MembershipGift(MembershipGift),
    MembershipClaim(MembershipClaim),
    Engagement(Engagement),

    // Non moderator types
    Placeholder(Placeholder),

    // Moderator only types
    AutoModMessage(AutoModMessage),
    TimeoutMessage(TimeoutMessage),
    BanMessage(BanMessage),
    UnbanMessage(UnbanMessage),
}

#[derive(Debug)]
pub enum MessagePart {
    Text(String),
    Emoji(Emoji),
    Link(String),
}

#[derive(Debug)]
pub enum Emoji {
    Builtin(String),
    Custom { id: String, url: String, shortcut: String },
}

impl From<&MessageItem> for Message {
    fn from(value: &MessageItem) -> Self {
        match value {
            MessageItem::TextMessage {
                message,
                author_name,
                author_photo,
                id,
                timestamp_usec,
                author_badges,
                author_external_channel_id,
            } => {
                let id = id.to_string();
                let timestamp: u64 = timestamp_usec.parse().unwrap();
                let contents: Vec<MessagePart> = message.runs.iter().map(MessagePart::from).collect();
                let author = convert_to_author(author_external_channel_id, author_photo, author_name, author_badges);

                Message::TextMessage(TextMessage {
                    author,
                    timestamp,
                    contents,
                    id,
                })
            }
            MessageItem::MembershipItem {
                author_badges,
                author_external_channel_id,
                author_name,
                author_photo,
                header_primary_text,
                header_subtext,
                message,
                id,
                timestamp_usec,
            } => {
                let author = convert_to_author(author_external_channel_id, author_photo, author_name, author_badges);
                let id = id.to_string();
                let timestamp: u64 = timestamp_usec.parse().unwrap();
                match header_subtext {
                    HeaderSubtext::Single(tier) => {
                        // Milestone
                        let contents: Option<Vec<_>> = message.as_ref().map(|m| m.runs.iter().map(MessagePart::from).collect());
                        let tier = tier.simple_text.clone();
                        let title = header_primary_text.as_ref().unwrap().runs.iter().map(MessagePart::from).collect();
                        Message::MembershipMilestone(MembershipMilestone {
                            author,
                            timestamp,
                            id,
                            tier,
                            contents,
                            title,
                        })
                    }
                    HeaderSubtext::Multiple(runs) => {
                        // New membership
                        let tier = if let Some(a) = runs.runs.get(1)
                            && let Run::Text(b) = a
                        {
                            b.text.clone()
                        } else {
                            unreachable!("Tier not detected on new membership")
                        };
                        let title: Vec<_> = runs.runs.iter().map(MessagePart::from).collect();
                        Message::NewMembership(NewMembership {
                            author,
                            timestamp,
                            id,
                            tier,
                            title,
                        })
                    }
                }
            }
            MessageItem::PaidMessage {
                author_badges,
                author_external_channel_id,
                author_name,
                author_name_text_color: _,
                author_photo,
                body_background_color,
                body_text_color: _,
                header_background_color: _,
                header_text_color: _,
                id,
                message,
                purchase_amount_text,
                text_input_background_color: _,
                timestamp_color: _,
                timestamp_usec,
            } => {
                let author = convert_to_author(author_external_channel_id, author_photo, author_name, author_badges);
                let id = id.to_string();
                let timestamp: u64 = timestamp_usec.parse().unwrap();
                let contents: Option<Vec<_>> = message.as_ref().map(|r| r.runs.iter().map(MessagePart::from).collect());
                let (currency, amount): (String, f32) = purchase_amount_text
                    .simple_text
                    .split_once('\u{a0}')
                    .map(|(c, a)| (c.into(), a.parse().unwrap()))
                    .unwrap();
                let background_color = format!("#{:.2X}", body_background_color & 0xFFFFFF);

                Message::SuperChat(SuperChat {
                    author,
                    id,
                    timestamp,
                    contents,
                    currency,
                    amount,
                    background_color,
                })
            }
            MessageItem::PaidSticker {
                author_badges,
                author_external_channel_id,
                author_name,
                author_name_text_color: _,
                author_photo,
                background_color,
                money_chip_background_color: _,
                money_chip_text_color: _,
                id,
                purchase_amount_text,
                sticker,
                timestamp_usec,
            } => {
                let author = convert_to_author(author_external_channel_id, author_photo, author_name, author_badges);
                let id = id.to_string();
                let timestamp: u64 = timestamp_usec.parse().unwrap();
                let (currency, amount): (String, f32) = purchase_amount_text
                    .simple_text
                    .split_once('\u{a0}')
                    .map(|(c, a)| (c.into(), a.parse().unwrap()))
                    .unwrap();
                let background_color = format!("#{:.2X}", background_color & 0xFFFFFF);
                let sticker_url = format!("https:{}", sticker.thumbnails.last().unwrap().url.clone());

                Message::SuperSticker(SuperSticker {
                    author,
                    id,
                    timestamp,
                    currency,
                    amount,
                    background_color,
                    sticker_url,
                })
            }
            MessageItem::SponsorshipsGiftPurchase {
                author_external_channel_id,
                header,
                id,
                timestamp_usec,
            } => {
                let renderer = &header.live_chat_sponsorships_header_renderer;
                let id = id.to_string();
                let timestamp: u64 = timestamp_usec.parse().unwrap();
                let author = convert_to_author(author_external_channel_id, &renderer.author_photo, &renderer.author_name, &renderer.author_badges);
                let contents: Vec<_> = renderer.primary_text.runs.iter().map(MessagePart::from).collect();
                let amount: u16 = if let Some(a) = contents.get(1)
                    && let MessagePart::Text(b) = a
                {
                    b.parse().unwrap()
                } else {
                    unreachable!("Membership gift without gift amount");
                };
                Message::MembershipGift(MembershipGift {
                    amount,
                    author,
                    contents,
                    id,
                    timestamp,
                })
            }
            MessageItem::SponsorshipsGiftRedemption {
                author_badges,
                author_external_channel_id,
                author_name,
                author_photo,
                id,
                message,
                timestamp_usec,
            } => {
                let id = id.to_string();
                let timestamp: u64 = timestamp_usec.parse().unwrap();
                let author = convert_to_author(author_external_channel_id, author_photo, author_name, author_badges);
                let contents: Vec<MessagePart> = message.runs.iter().map(MessagePart::from).collect();
                Message::MembershipClaim(MembershipClaim {
                    author,
                    contents,
                    id,
                    timestamp,
                })
            }
            MessageItem::ViewerEngagementMessage { id, message } => {
                let id = id.to_string();
                let contents: Vec<MessagePart> = message.runs.iter().map(MessagePart::from).collect();
                Message::Engagement(Engagement { id, contents })
            }
            MessageItem::Placeholder { id, timestamp_usec } => Message::Placeholder(Placeholder {
                id: id.to_string(),
                timestamp: timestamp_usec.parse().unwrap(),
            }),
            MessageItem::AutoModMessage {
                id,
                timestamp_usec,
                auto_moderated_item,
                header_text,
            } => {
                let id = id.to_string();
                let auto_mod_item: Message = auto_moderated_item.as_ref().into();
                let timestamp: u64 = timestamp_usec.parse().unwrap();
                let title: Vec<MessagePart> = header_text.runs.iter().map(MessagePart::from).collect();

                match auto_mod_item {
                    Message::TextMessage(item) => Message::AutoModMessage(AutoModMessage { id, item, timestamp, title }),
                    _ => unreachable!("AutoModMessage without TextMessage"),
                }
            }
            MessageItem::ModerationMessage { id, message, timestamp_usec } => {
                let id = id.to_string();
                let timestamp: u64 = timestamp_usec.parse().unwrap();
                let contents: Vec<MessagePart> = message.runs.iter().map(MessagePart::from).collect();
                match contents.as_slice() {
                    // Case: Timeout (at least 5 elements, looking at index 0, 2, and 4)
                    [MessagePart::Text(target), _, MessagePart::Text(mod_name), _, MessagePart::Text(timeout), ..] => Message::TimeoutMessage(TimeoutMessage {
                        id,
                        timestamp,
                        target_username: target.into(),
                        moderator_username: mod_name.into(),
                        timeout_secs: timeout.parse().unwrap(),
                    }),
                    // Case: Unban or Ban (at least 3 elements, looking at index 0, 1, and 2)
                    [MessagePart::Text(target), MessagePart::Text(action), MessagePart::Text(mod_name), ..] => {
                        let common = (target.into(), mod_name.into());

                        if action.contains("unhidden") {
                            Message::UnbanMessage(UnbanMessage {
                                id,
                                timestamp,
                                target_username: common.0,
                                moderator_username: common.1,
                            })
                        } else {
                            Message::BanMessage(BanMessage {
                                id,
                                timestamp,
                                target_username: common.0,
                                moderator_username: common.1,
                            })
                        }
                    }
                    _ => panic!("Unexpected ModerationMessage structure"),
                }
            }
            MessageItem::Unknown(_) => todo!("Item: {value:#?}"),
        }
    }
}

impl From<&Run> for MessagePart {
    fn from(value: &Run) -> Self {
        match value {
            Run::Text(t) => match &t.navigation_endpoint {
                Some(n) => {
                    if n.url_endpoint.url.starts_with("https://www.youtube.com/redirect") {
                        MessagePart::Link(
                            Url::parse(&n.url_endpoint.url)
                                .unwrap()
                                .query_pairs()
                                .find(|(k, _)| k == "q")
                                .map(|(_, v)| v.into_owned())
                                .unwrap(),
                        )
                    } else {
                        MessagePart::Link(n.url_endpoint.url.clone())
                    }
                }
                None => MessagePart::Text(t.text.clone()),
            },
            Run::Emoji { emoji } => MessagePart::Emoji(
                if let Some(is_custom_emoji) = emoji.is_custom_emoji
                    && is_custom_emoji
                {
                    Emoji::Custom {
                        id: emoji.emoji_id.clone(),
                        url: emoji.image.thumbnails.last().unwrap().url.clone(),
                        shortcut: emoji.shortcuts.as_ref().and_then(|e| e.last()).unwrap().into(),
                    }
                } else {
                    Emoji::Builtin(emoji.emoji_id.clone())
                },
            ),
        }
    }
}

// Individual types

#[derive(Debug)]
pub struct TextMessage {
    pub author: Author,
    pub timestamp: u64,
    pub contents: Vec<MessagePart>,
    pub id: String,
}

#[derive(Debug)]
pub struct Engagement {
    pub id: String,
    /// Automated welcome message or poll result message
    pub contents: Vec<MessagePart>,
}

#[derive(Debug)]
pub struct NewMembership {
    pub author: Author,
    pub timestamp: u64,
    pub id: String,
    /// Membership tier
    pub tier: String,
    /// Welcome message sent by YouTube (Welcome to Destekçi!)
    pub title: Vec<MessagePart>,
}

#[derive(Debug)]
pub struct MembershipMilestone {
    pub author: Author,
    pub timestamp: u64,
    pub id: String,
    /// Membership tier
    pub tier: String,
    /// Milestone message if provided by user
    pub contents: Option<Vec<MessagePart>>,
    /// Automated milestone message (Member for 18 months)
    pub title: Vec<MessagePart>,
}

#[derive(Debug)]
pub struct SuperChat {
    pub author: Author,
    pub id: String,
    pub timestamp: u64,
    /// Superchat message
    pub contents: Option<Vec<MessagePart>>,
    pub currency: String,
    pub amount: f32,
    pub background_color: String,
}

#[derive(Debug)]
pub struct SuperSticker {
    pub author: Author,
    pub id: String,
    pub timestamp: u64,
    pub currency: String,
    pub amount: f32,
    pub background_color: String,
    /// Sticker image URL
    pub sticker_url: String,
}

#[derive(Debug)]
pub struct MembershipGift {
    /// Amount of membership gifts
    pub amount: u16,
    pub author: Author,
    /// Automated YouTube message (Sent 1 Tugay Aloğlu gift memberships)
    pub contents: Vec<MessagePart>,
    pub id: String,
    pub timestamp: u64,
}

#[derive(Debug)]
pub struct MembershipClaim {
    pub author: Author,
    /// Automated YouTube message (received a gift membership by @mkerem0)
    pub contents: Vec<MessagePart>,
    pub id: String,
    pub timestamp: u64,
}

/// Non moderator item (used for automod detected messages if they later get approved)
#[derive(Debug)]
pub struct Placeholder {
    pub id: String,
    pub timestamp: u64,
}

// Moderator only types

#[derive(Debug)]
pub struct AutoModMessage {
    pub id: String,
    /// Target message that is detected by AutoMod
    pub item: TextMessage,
    pub timestamp: u64,
    /// Reason (This message is held for review.)
    pub title: Vec<MessagePart>,
}
#[derive(Debug)]
pub struct TimeoutMessage {
    pub id: String,
    pub timestamp: u64,
    /// Timed out username
    pub target_username: String,
    /// Username of the moderator that timed out the target
    pub moderator_username: String,
    /// Timeout amount by seconds
    pub timeout_secs: u32,
}
#[derive(Debug)]
pub struct BanMessage {
    pub id: String,
    pub timestamp: u64,
    /// Banned username
    pub target_username: String,
    /// Username of the moderator that banned the target
    pub moderator_username: String,
}
#[derive(Debug)]
pub struct UnbanMessage {
    pub id: String,
    pub timestamp: u64,
    /// Unbanned username
    pub target_username: String,
    /// Username of the moderator that unbanned the target
    pub moderator_username: String,
}
