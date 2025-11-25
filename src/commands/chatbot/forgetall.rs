use crate::{ai, translation::tr, utils::*};

#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    required_permissions = "ADMINISTRATOR",
    category = "Chatbot"
)]
pub async fn forgetall(ctx: Context<'_>, confirmation: String) -> Result<(), Error> {
    if confirmation.to_lowercase() != "yes" {
        reply_without_ping(ctx, tr!(ctx, "forgetall-confirm")).await?;
        return Ok(());
    }

    let guild_id = ctx.guild_id().unwrap().get();

    match ai::core::remove_prompt_file(guild_id) {
        Ok(()) => reply_without_ping(ctx, tr!(ctx, "forgetall-forgot")).await?,
        Err(e) => reply_without_ping(ctx, tr!(ctx, "error", error: e.to_string())).await?,
    };

    Ok(())
}
