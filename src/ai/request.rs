use poise::serenity_prelude::{Attachment, User};

pub struct AiRequest {
    pub guild: (u64, String),
    pub author: User,
    pub content: String,
    pub attachments: Vec<Attachment>,
}

impl AiRequest {
    pub fn new(
        guild: (u64, String),
        author: User,
        content: String,
        attachments: Vec<Attachment>,
    ) -> Self {
        Self {
            guild,
            author,
            content,
            attachments,
        }
    }
}
