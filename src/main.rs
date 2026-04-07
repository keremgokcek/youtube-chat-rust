use std::collections::HashMap;
use std::thread::sleep;
use youtube_chat::client::Client;
use youtube_chat::cookie::Cookie;
use youtube_chat::internal::Action;
use youtube_chat::internal::banner::BannerItem;
use youtube_chat::internal::common::{HeaderSubtext, Run};
use youtube_chat::internal::message::MessageItem;
use youtube_chat::internal::ticker::TickerItem;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::new_with_cookie(Cookie::from_file("cookies.firefox-private.txt"));
    client.get_options_from_live_page().await.unwrap();
    loop {
        let resp = client.fetch_chat().await?;
        if let Some(actions) = &resp.live_chat_continuation.actions {
            for action_wrapper in actions {
                match &action_wrapper.action {
                    Action::AddChatItem { item } => match item {
                        MessageItem::TextMessage {
                            message,
                            author_name,
                            author_photo: _,
                            id: _,
                            timestamp_usec: _,
                            author_badges: _,
                            author_external_channel_id: _,
                        } => {
                            println!(
                                "{}: {}",
                                author_name.simple_text.clone(),
                                message
                                    .runs
                                    .iter()
                                    .map(|run| match run {
                                        Run::Text(text) => match &text.navigation_endpoint {
                                            Some(n) => {
                                                if n.url_endpoint.url.starts_with("https://www.youtube.com/redirect") {
                                                    url::Url::parse(&n.url_endpoint.url)
                                                        .unwrap()
                                                        .query_pairs()
                                                        .into_owned()
                                                        .collect::<HashMap<_, _>>()
                                                        .get("q")
                                                        .unwrap()
                                                        .clone()
                                                } else {
                                                    n.url_endpoint.url.clone()
                                                }
                                            }
                                            None => {
                                                text.text.clone()
                                            }
                                        },
                                        Run::Emoji { emoji } => emoji.emoji_id.clone(),
                                    })
                                    .collect::<Vec<String>>()
                                    .join("")
                            )
                        }
                        MessageItem::MembershipItem {
                            author_badges: _,
                            author_external_channel_id: _,
                            author_name,
                            author_photo: _,
                            header_primary_text: _,
                            header_subtext,
                            message,
                            id: _,
                            timestamp_usec: _,
                        } => match header_subtext {
                            HeaderSubtext::Single(text) => match message {
                                Some(m) => {
                                    println!(
                                        "{} [{}]: {}",
                                        author_name.simple_text,
                                        text.simple_text,
                                        m.runs
                                            .iter()
                                            .map(|run| match run {
                                                Run::Text(text) => text.text.clone(),
                                                Run::Emoji { emoji } => emoji.emoji_id.clone(),
                                            })
                                            .collect::<Vec<String>>()
                                            .join("")
                                    );
                                }
                                None => {
                                    println!("{} [{}]", author_name.simple_text, text.simple_text);
                                }
                            },
                            HeaderSubtext::Multiple(runs) => {
                                println!(
                                    "{}: {}",
                                    author_name.simple_text,
                                    runs.runs
                                        .iter()
                                        .map(|run| match run {
                                            Run::Text(text) => text.text.clone(),
                                            Run::Emoji { emoji } => emoji.emoji_id.clone(),
                                        })
                                        .collect::<Vec<String>>()
                                        .join("")
                                )
                            }
                        },
                        MessageItem::PaidMessage {
                            author_badges: _,
                            author_external_channel_id: _,
                            author_name,
                            author_name_text_color: _,
                            author_photo: _,
                            body_background_color: _,
                            body_text_color: _,
                            header_background_color: _,
                            header_text_color: _,
                            id: _,
                            message,
                            purchase_amount_text,
                            text_input_background_color: _,
                            timestamp_color: _,
                            timestamp_usec: _,
                        } => match message {
                            Some(m) => {
                                println!(
                                    "{} [{}]: {}",
                                    author_name.simple_text,
                                    purchase_amount_text.simple_text,
                                    m.runs
                                        .iter()
                                        .map(|run| match run {
                                            Run::Text(text) => text.text.clone(),
                                            Run::Emoji { emoji } => emoji.emoji_id.clone(),
                                        })
                                        .collect::<Vec<String>>()
                                        .join("")
                                )
                            }
                            None => {
                                println!("{} [{}]", author_name.simple_text, purchase_amount_text.simple_text,)
                            }
                        },
                        MessageItem::SponsorshipsGiftPurchase {
                            author_external_channel_id: _,
                            header,
                            id: _,
                            timestamp_usec: _,
                        } => {
                            println!(
                                "{}: {}",
                                header.live_chat_sponsorships_header_renderer.author_name.simple_text,
                                header
                                    .live_chat_sponsorships_header_renderer
                                    .primary_text
                                    .runs
                                    .iter()
                                    .map(|run| match run {
                                        Run::Text(text) => text.text.clone(),
                                        Run::Emoji { emoji } => emoji.emoji_id.clone(),
                                    })
                                    .collect::<Vec<String>>()
                                    .join("")
                            );
                        }
                        MessageItem::SponsorshipsGiftRedemption {
                            author_badges: _,
                            author_external_channel_id: _,
                            author_name,
                            author_photo: _,
                            id: _,
                            message,
                            timestamp_usec: _,
                        } => {
                            println!(
                                "{}: {}",
                                author_name.simple_text,
                                message
                                    .runs
                                    .iter()
                                    .map(|run| match run {
                                        Run::Text(text) => text.text.clone(),
                                        Run::Emoji { emoji } => emoji.emoji_id.clone(),
                                    })
                                    .collect::<Vec<String>>()
                                    .join("")
                            );
                        }
                        MessageItem::ViewerEngagementMessage {} => {}
                        MessageItem::Unknown(unknown) => println!("{unknown:#?}"),
                    },
                    Action::ReplaceChatItem {
                        target_item_id: _,
                        replacement_item: _,
                    } => println!("Message accepted"),
                    Action::RemoveChatItemByAuthor { external_channel_id: _ } => println!("Someone is banned or timed out."),
                    Action::RemoveChatItem { target_item_id: _ } => println!("Message deleted"),
                    Action::AddBannerToLiveChat { banner_renderer } => match &banner_renderer.live_chat_banner_renderer.contents {
                        BannerItem::MessageItem(_) => println!("Message is pinned"),
                        BannerItem::BannerChatSummary { chat_summary: _, icon: _ } => println!("=== CHAT SUMMARY ==="),
                    },
                    Action::AddLiveChatTickerItem { item, duration_sec } => match item {
                        TickerItem::Unknown(u) => println!("Unknown ticker item: {u:#?}"),
                        _ => println!("Add ticker for {duration_sec} seconds"),
                    },
                    Action::ShowLiveChatActionPanel { panel_to_show: _ } => println!("Poll added"),
                    Action::UpdateLiveChatPoll { poll_to_update: _ } => println!("Poll updated"),
                    Action::Unknown(unknown) => println!("Unknown action: {unknown:#?}"),
                }
            }
        }
        sleep(std::time::Duration::from_secs(1));
    }

    // Ok(())
}
