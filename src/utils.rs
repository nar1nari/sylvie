use poise::ReplyHandle;
use poise::serenity_prelude as serenity;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, crate::Data, Error>;

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
