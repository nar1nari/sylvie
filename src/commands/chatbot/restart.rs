use crate::{ai, translation::tr, utils::*};

#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    required_permissions = "MANAGE_MESSAGES",
    category = "Chatbot"
)]
pub async fn restart(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get();

    match ai::core::remove_agent(guild_id).await {
        Ok(_) => reply_without_ping(ctx, tr!(ctx, "restart-restarted")).await?,
        Err(_) => reply_without_ping(ctx, tr!(ctx, "restart-not")).await?,
    };

    Ok(())
}
