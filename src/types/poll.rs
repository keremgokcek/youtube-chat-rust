use super::message::MessagePart;
use crate::internal::poll::PollWrapper;

#[derive(Debug)]
pub struct Poll {
    pub poll_id: String,
    pub question: Vec<MessagePart>,
    pub choices: Vec<PollChoice>,
    pub total_votes: u32,
}
#[derive(Debug)]
pub struct PollChoice {
    pub name: String,
    pub votes: u32,
}

impl From<&PollWrapper> for Poll {
    fn from(value: &PollWrapper) -> Self {
        let poll_renderer = &value.poll_renderer;

        let poll_id = poll_renderer.live_chat_poll_id.clone();
        let question: Vec<MessagePart> = poll_renderer.header.poll_header_renderer.poll_question.runs.iter().map(MessagePart::from).collect();
        let total_votes: u32 = poll_renderer
            .header
            .poll_header_renderer
            .metadata_text
            .runs
            .get(4)
            .and_then(|r| match MessagePart::from(r) {
                MessagePart::Text(a) => a.split_once(" ")?.0.parse().ok(),
                _ => None,
            })
            .expect(&format!("Malformed poll JSON: {value:#?}"));
        let choices: Vec<PollChoice> = poll_renderer
            .choices
            .iter()
            .map(|c| {
                let name = c
                    .text
                    .runs
                    .get(0)
                    .and_then(|r| match MessagePart::from(r) {
                        MessagePart::Text(a) => Some(a),
                        _ => None,
                    })
                    .unwrap();
                let votes: u32 = (c.vote_ratio.unwrap_or(0f64) * total_votes as f64).round() as u32;
                PollChoice { name, votes }
            })
            .collect();

        Poll {
            poll_id,
            question,
            choices,
            total_votes,
        }
    }
}
