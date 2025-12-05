use poise::serenity_prelude as serenity;

use crate::{translation::tr, utils::*};

#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    required_permissions = "MANAGE_MESSAGES",
    category = "Admin"
)]
pub async fn clear(ctx: Context<'_>, amount: u8) -> Result<(), Error> {
    ctx.defer().await?;

    let channel = match ctx.guild_channel().await {
        Some(ch) => ch,
        None => {
            reply_without_ping(ctx, tr!(ctx, "guild-only-error")).await?;
            return Ok(());
        }
    };

    let last_message_id = match channel.last_message_id {
        Some(id) => id,
        None => {
            reply_without_ping(ctx, tr!(ctx, "clear-last-msg-fail")).await?;
            return Ok(());
        }
    };

    let builder = serenity::GetMessages::new().before(last_message_id).limit(amount);
    let messages = channel.messages(&ctx.http(), builder).await?;
    channel.delete_messages(&ctx.http(), messages).await?;

    if let poise::Context::Prefix(p) = ctx {
        p.msg.delete(&ctx.http()).await?
    }

    let msg = reply_without_ping(ctx, tr!(ctx, "clear-deleted", amount: amount)).await?;

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    msg.delete(ctx).await?;

    Ok(())
}
