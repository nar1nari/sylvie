use crate::{Data, commands::chatbot, translation::tr, utils::*};
use poise::serenity_prelude as serenity;

pub async fn error_handler(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::ArgumentParse { ctx, .. } => {
            reply_with_error(
                ctx,
                format!(":broken_chain: {}", tr!(ctx, "argument-parse-error")),
            )
            .await
            .ok();
        }
        poise::FrameworkError::NsfwOnly { ctx, .. } => {
            reply_with_error(ctx, format!(":underage: {}", tr!(ctx, "nsfw-error")))
                .await
                .ok();
        }
        poise::FrameworkError::MissingUserPermissions {
            missing_permissions,
            ctx,
            ..
        } => {
            let permissions: String = missing_permissions
                .unwrap()
                .iter()
                .map(|i| format!("{}, ", i))
                .collect();
            reply_with_error(
                ctx,
                format!(
                    ":scales: {}",
                    tr!(ctx, "user-permissions-error", permissions: permissions)
                ),
            )
            .await
            .ok();
        }
        poise::FrameworkError::MissingBotPermissions {
            missing_permissions,
            ctx,
            ..
        } => {
            let permissions: String = missing_permissions
                .iter()
                .map(|i| format!("{}, ", i))
                .collect();
            reply_with_error(
                ctx,
                format!(
                    ":scales: {}",
                    tr!(ctx, "bot-permissions-error", permissions: permissions)
                ),
            )
            .await
            .ok();
        }
        poise::FrameworkError::GuildOnly { ctx, .. } => {
            reply_with_error(ctx, format!(":lock: {}", tr!(ctx, "guild-only-error")))
                .await
                .ok();
        }
        poise::FrameworkError::Command { error, ctx, .. } => {
            if let Some(serenity::Error::Http(http)) = error.downcast_ref::<serenity::Error>()
                && let serenity::HttpError::UnsuccessfulRequest(resp) = http
                && resp.status_code == 400
            {
                let body = &resp.error;
                if let Some(first) = body.errors.first()
                    && first.code == "MESSAGE_REFERENCE_UNKNOWN_MESSAGE"
                {
                    ctx.say(tr!(ctx, "error-no-reference")).await.ok();
                }
            }
        }
        other => {
            poise::builtins::on_error(other).await.ok();
        }
    }
}

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    framework: poise::FrameworkContext<'_, Data, Box<dyn std::error::Error + Send + Sync>>,
) -> Result<(), Error> {
    if let serenity::FullEvent::Message { new_message: msg } = event
        && !msg.author.bot
    {
        let content = &msg.content.to_lowercase();
        let mentions_bot = content.contains("sylvie") || content.contains("сильви");
        let has_comand = content.starts_with("s.");

        let replied_to_bot = msg
            .referenced_message
            .as_ref()
            .is_some_and(|replied| replied.author.id == framework.bot_id);

        if (replied_to_bot || mentions_bot) && !has_comand {
            chatbot::ask::ask_from_message(ctx, msg).await?;
        }
    }
    Ok(())
}
