use poise::ReplyHandle;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Color;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, crate::Data, Error>;

pub const DEFAULT_EMBED_COLOR: Color = Color::from_rgb(0x2e, 0xcc, 0x70);
pub const HELP_EMBED_COLOR: Color = Color::from_rgb(0x3b, 0x88, 0xc3);
pub const ERROR_EMBED_COLOR: Color = Color::RED;

pub async fn reply_without_ping(
    ctx: Context<'_>,
    content: impl Into<String>,
) -> Result<ReplyHandle<'_>, Error> {
    Ok(ctx
        .send(
            poise::CreateReply::default()
                .content(content.into())
                .reply(true)
                .allowed_mentions(serenity::CreateAllowedMentions::new().replied_user(false)),
        )
        .await?)
}
