use poise::serenity_prelude as serenity;

pub struct AiRequest {
    pub guild: (u64, String),
    pub author: serenity::User,
    pub content: String,
    pub attachments: Vec<serenity::Attachment>,
}

impl AiRequest {
    pub fn new(
        guild: (u64, String),
        author: serenity::User,
        content: String,
        attachments: Vec<serenity::Attachment>,
    ) -> Self {
        Self {
            guild,
            author,
            content,
            attachments,
        }
    }
}
