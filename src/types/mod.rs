pub mod author;
pub mod event;
pub mod message;
pub mod poll;

#[derive(Debug)]
pub struct StreamDetails {
    pub title: String,
    pub description: Option<String>,
    pub video_id: String,
    pub channel_id: String,
    #[cfg(feature = "jiff")]
    pub start_timestamp: i64,
}
