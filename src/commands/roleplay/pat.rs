use poise::serenity_prelude::User;

use crate::{
    commands::roleplay::{Action, run_rp_action},
    utils::*,
};

#[poise::command(slash_command, prefix_command, category = "Roleplay")]
pub async fn pat(ctx: Context<'_>, target: Option<User>) -> Result<(), Error> {
    let action = Action::new("pat", false);

    ctx.defer().await?;

    run_rp_action(ctx, &action, &target).await?;

    Ok(())
}
