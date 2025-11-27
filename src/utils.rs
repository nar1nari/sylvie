use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Color;
use crate::translation::tr;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, crate::Data, Error>;

pub const DEFAULT_EMBED_COLOR: Color = Color::from_rgb(0x2e, 0xcc, 0x70);
pub const HELP_EMBED_COLOR: Color = Color::from_rgb(0x3b, 0x88, 0xc3);
pub const ERROR_EMBED_COLOR: Color = Color::RED;

pub async fn reply_without_ping(
    ctx: Context<'_>,
    content: impl Into<String>,
) -> Result<poise::ReplyHandle<'_>, Error> {
    Ok(ctx
        .send(
            poise::CreateReply::default()
                .content(content.into())
                .reply(true)
                .allowed_mentions(serenity::CreateAllowedMentions::new()),
        )
        .await?)
}

pub async fn reply_with_error(
    ctx: Context<'_>,
    content: impl Into<String>,
) -> Result<poise::ReplyHandle<'_>, Error> {
    let embed = serenity::CreateEmbed::new().color(ERROR_EMBED_COLOR).title(tr!(ctx, "error")).description(content);

    Ok(ctx
        .send(
            poise::CreateReply::default()
                .embed(embed)
                .reply(true)
                .allowed_mentions(serenity::CreateAllowedMentions::new())
                .ephemeral(true),
        )
        .await?)
}
