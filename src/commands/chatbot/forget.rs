use crate::{ai, translation::tr, utils::*};

#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_MESSAGES",
    guild_only,
    category = "Chatbot"
)]
pub async fn forget(ctx: Context<'_>, #[rest] indexes: String) -> Result<(), Error> {
    let indices: Vec<usize> = indexes
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let indices_str: String = indices.iter().map(|i| format!("{} ", i)).collect();
    let indices_real: Vec<usize> = indices.iter().map(|i| i.saturating_sub(1)).collect();

    let guild_id = ctx.guild_id().unwrap().get();

    match ai::core::remove_prompt_lines(guild_id, &indices_real) {
        Ok(()) => reply_without_ping(ctx, tr!(ctx, "forget-forgot", indices: indices_str)).await?,
        Err(e) => reply_without_ping(ctx, tr!(ctx, "error", error: e.to_string())).await?,
    };

    Ok(())
}
