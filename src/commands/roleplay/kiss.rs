use poise::serenity_prelude as serenity;

use crate::{
    commands::roleplay::{Action, run_rp_action},
    utils::*,
};

#[poise::command(slash_command, prefix_command, category = "Roleplay")]
pub async fn kiss(ctx: Context<'_>, target: Option<serenity::User>) -> Result<(), Error> {
    let action = Action::new("kiss", true);

    ctx.defer().await?;

    run_rp_action(ctx, &action, &target).await?;

    Ok(())
}
