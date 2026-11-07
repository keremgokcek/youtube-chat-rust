use crate::request::{Error, LivePageData};
use crate::types::event::Event;
use crate::types::message::{MembershipClaim, MembershipGift, MembershipMilestone, Message, NewMembership, SuperChat, SuperSticker, TextMessage};
use crate::types::poll::Poll;
use async_trait::async_trait;
use std::collections::HashSet;
use tokio::time::{Duration, sleep};

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn on_message(&self, _message: &TextMessage) {}
    async fn on_message_pin(&self, _action_id: &str, _message: &TextMessage) {}
    async fn on_message_unpin(&self, _action_id: &str, _message: &TextMessage) {}
    async fn on_message_approve(&self, _target_item_id: &str, _message: &TextMessage) {}
    async fn on_superchat(&self, _superchat: &SuperChat) {}
    async fn on_supersticker(&self, _supersticker: &SuperSticker) {}
    async fn on_membership(&self, _membership: &NewMembership) {}
    async fn on_membership_milestone(&self, _membership_milestone: &MembershipMilestone) {}
    async fn on_membership_gift(&self, _membership_gift: &MembershipGift) {}
    async fn on_membership_claim(&self, _membership_claim: &MembershipClaim) {}
    async fn on_poll_add(&self, _poll: &Poll) {}
    async fn on_poll_update(&self, _poll: &Poll) {}
    async fn on_poll_remove(&self, _poll: &Poll) {}
    async fn on_stream_end(&self) {}
}

pub struct ClientBuilder {
    live_stream_url: String,
    handler: Box<dyn EventHandler>,
    #[cfg(feature = "cookies")]
    cookies: Option<reqwest::cookie::Jar>,
}
impl ClientBuilder {
    #[cfg(feature = "cookies")]
    pub fn cookies(mut self, cookies: reqwest::cookie::Jar) -> Self {
        self.cookies = Some(cookies);
        self
    }
    pub fn build(self) -> Client {
        #[cfg(feature = "cookies")]
        let sapisid = if let Some(cookies) = &self.cookies {
            use reqwest::cookie::CookieStore;
            let cookie_string = cookies.cookies(&reqwest::Url::parse("https://youtube.com").unwrap()).expect("No youtube.com cookies provided");
            Some(cookie_string.to_str().unwrap().split(";").find(|c| c.contains("SAPISID")).unwrap().split_once("=").unwrap().1.to_string())
        } else {
            None
        };

        let session_builder = reqwest::Client::builder();
        #[cfg(feature = "cookies")]
        let session_builder = if let Some(cookies) = self.cookies {
            session_builder.cookie_provider(std::sync::Arc::new(cookies))
        } else {
            session_builder
        };
        let session = session_builder.build().unwrap();

        Client {
            live_stream_url: self.live_stream_url,
            session,
            live_page_data: None,
            handler: self.handler,
            message_cache: HashSet::new(),
            current_poll: None,
            current_pin: None,
            #[cfg(feature = "cookies")]
            sapisid,
        }
    }
}

pub struct Client {
    pub(crate) session: reqwest::Client,
    pub(crate) live_stream_url: String,
    pub(crate) live_page_data: Option<LivePageData>,
    pub(crate) handler: Box<dyn EventHandler>,
    pub(crate) message_cache: HashSet<Message>, // TODO: Set limit for max message cache
    pub(crate) current_poll: Option<Poll>,
    pub(crate) current_pin: Option<(String, TextMessage)>,
    #[cfg(feature = "cookies")]
    pub(crate) sapisid: Option<String>,
}

impl Client {
    pub fn builder<H: EventHandler + 'static>(live_stream_url: &str, handler: H) -> ClientBuilder {
        ClientBuilder {
            live_stream_url: live_stream_url.to_owned(),
            handler: Box::new(handler),
            #[cfg(feature = "cookies")]
            cookies: None,
        }
    }

    // TODO: Implement waiting for live streams
    // TODO: Start handlers without awaiting them to avoid blocking the main loop
    pub async fn start(mut self) {
        self.get_options_from_live_page().await.unwrap();

        loop {
            let resp = self.fetch_chat().await;
            match resp {
                Ok(resp) => {
                    if let Some(actions) = &resp.live_chat_continuation.actions {
                        for action_wrapper in actions {
                            let Ok(event) = (&action_wrapper.action).try_into() else { continue };

                            match event {
                                Event::MessageSent { item } => {
                                    if self.message_cache.contains(&item) {
                                        continue;
                                    }
                                    match item {
                                        Message::TextMessage(ref message) => self.handler.on_message(message).await,
                                        Message::SuperChat(ref superchat) => self.handler.on_superchat(superchat).await,
                                        Message::SuperSticker(ref supersticker) => self.handler.on_supersticker(supersticker).await,
                                        Message::NewMembership(ref membership) => self.handler.on_membership(membership).await,
                                        Message::MembershipMilestone(ref milestone) => self.handler.on_membership_milestone(milestone).await,
                                        Message::MembershipGift(ref gift) => self.handler.on_membership_gift(gift).await,
                                        Message::MembershipClaim(ref claim) => self.handler.on_membership_claim(claim).await,
                                        _ => println!("[TODO] No supported handler for this type yet: {item:#?}"),
                                    }
                                    self.message_cache.insert(item);
                                }
                                Event::MessageApproved {
                                    target_item_id,
                                    replacement_item,
                                } => {
                                    if self.message_cache.contains(&replacement_item) {
                                        continue;
                                    }
                                    match replacement_item {
                                        Message::TextMessage(ref message) => self.handler.on_message_approve(&target_item_id, message).await,
                                        _ => println!("[BUG] Non text message was approved"),
                                    }
                                    self.message_cache.insert(replacement_item);
                                }
                                Event::MessagePinned { action_id, item } => match item {
                                    Message::TextMessage(message) => {
                                        if let Some(old_pin) = &self.current_pin {
                                            println!("[BUG] MessagePinned while there was already a pinned message: {old_pin:#?}");
                                        }
                                        self.handler.on_message_pin(&action_id, &message).await;
                                        self.current_pin = Some((action_id, message));
                                    }
                                    _ => println!("[BUG] Non text message was pinned"),
                                },
                                Event::PollAdded { poll } => {
                                    if let Some(old_poll) = &self.current_poll {
                                        println!("[BUG] PollAdded while there was already a poll: {old_poll:#?}");
                                    }
                                    self.handler.on_poll_add(&poll).await;
                                    self.current_poll = Some(poll);
                                }
                                Event::PollUpdated { poll } => {
                                    if let Some(old_poll) = &self.current_poll {
                                        if old_poll != &poll {
                                            self.handler.on_poll_update(&poll).await;
                                            self.current_poll = Some(poll);
                                        }
                                    } else {
                                        println!("[BUG] PollUpdated while there was no poll.");
                                    }
                                }
                                Event::PollEnded { poll_id } => {
                                    if let Some(poll) = self.current_poll {
                                        if poll.poll_id != poll_id {
                                            println!("[BUG] Saved poll ID and ended poll ID doesn't match.");
                                        }
                                        self.handler.on_poll_remove(&poll).await;
                                    } else {
                                        println!("[BUG] PollEnded while there was no poll.");
                                    }
                                    self.current_poll = None;
                                }
                                // TODO: Add support for ChatSummary and Redirect handlers
                                Event::BannerRemoved { target_action_id } => {
                                    if let Some(ref pin) = self.current_pin
                                        && pin.0 == target_action_id
                                    {
                                        if let Some(pin) = self.current_pin {
                                            self.handler.on_message_unpin(&target_action_id, &pin.1).await;
                                        } else {
                                            println!("[BUG] MessageUnpinned while there was no pinned message.");
                                        }
                                        self.current_pin = None;
                                    }
                                }
                                Event::ReportPresence {} => {}
                                Event::ModeratorActive {} => {}
                                _ => println!("[TODO] No supported handler for this event yet: {event:#?}"),
                            }
                        }
                    }
                }
                Err(Error::NetworkError(e)) => panic!("{e}"),
                Err(Error::StreamEnded) => {
                    self.handler.on_stream_end().await;
                    panic!("Stream ended.")
                }
            }

            sleep(Duration::from_secs(1)).await;
        }
    }
}
