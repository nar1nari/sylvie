use crate::translation::tr;
use crate::utils::*;

#[poise::command(slash_command, prefix_command, aliases(""), category = "Info")]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    reply_without_ping(ctx, tr!(ctx, "ping-pong")).await?;
    Ok(())
}
