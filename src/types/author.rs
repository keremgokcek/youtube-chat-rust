use crate::internal::common::{LiveChatAuthorBadgeRendererWrapper, SimpleText, Thumbnails};

#[derive(Debug)]
pub struct Author {
    pub channel_id: String,
    pub picture_url: String,
    pub username: String,
    pub is_owner: bool,
    pub is_moderator: bool,
    pub is_verified: bool,
    pub have_membership: bool,
}

pub fn convert_to_author(
    author_external_channel_id: &String,
    author_photo: &Thumbnails,
    author_name: &SimpleText,
    author_badges: &Option<Vec<LiveChatAuthorBadgeRendererWrapper>>,
) -> Author {
    let channel_id = author_external_channel_id.clone();
    let picture_url = author_photo.thumbnails.last().unwrap().url.clone();
    let username = author_name.simple_text.clone();

    let mut is_owner = false;
    let mut is_moderator = false;
    let mut is_verified = false;
    let mut have_membership = false;

    if let Some(badges) = author_badges {
        for badge in badges {
            if badge.live_chat_author_badge_renderer.custom_thumbnail.is_some() {
                have_membership = true;
            } else if let Some(icon) = &badge.live_chat_author_badge_renderer.icon {
                match icon.icon_type.as_str() {
                    "MODERATOR" => is_moderator = true,
                    "VERIFIED" => is_verified = true,
                    "OWNER" => is_owner = true,
                    _ => println!("Badge: {badge:#?}"),
                }
            }
        }
    }

    Author {
        channel_id,
        picture_url,
        username,
        is_owner,
        is_moderator,
        is_verified,
        have_membership,
    }
}
