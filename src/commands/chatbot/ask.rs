use std::{fs::File, io::Write};

use poise::{
    CreateReply,
    serenity_prelude::{self as serenity, CreateAllowedMentions, CreateAttachment, CreateMessage},
};

use crate::{
    ai::{self, request::AiRequest},
    translation::tr,
    utils::*,
};

#[derive(Debug)]
enum Answer {
    Text(String),
    File(CreateAttachment),
    None,
}

impl Answer {
    async fn build(request: &AiRequest) -> Self {
        let response = match ai::core::ask(request).await {
            Some(res) => res,
            None => return Self::None,
        };

        if response.len() >= 2000 {
            let filename =
                std::env::temp_dir().join(format!("response_{}.txt", uuid::Uuid::new_v4()));

            let mut file = match File::create(&filename) {
                Ok(f) => f,
                Err(_) => return Self::None,
            };
            if file.write_all(response.as_bytes()).is_err() {
                return Self::None;
            }

            let attachment = match CreateAttachment::path(&filename).await {
                Ok(a) => a,
                Err(_) => return Self::None,
            };

            std::fs::remove_file(&filename).unwrap_or(());

            return Self::File(attachment);
        }

        Self::Text(response)
    }
}

pub async fn ask_from_message(
    ctx: &serenity::Context,
    message: &serenity::Message,
) -> Result<(), Error> {
    let guild = match message.guild(&ctx.cache) {
        Some(guild) => (guild.id.get(), guild.name.clone()),
        None => return Ok(()),
    };

    let typing = message.channel_id.start_typing(&ctx.http);

    let request = AiRequest::new(
        guild,
        message.author.clone(),
        message.content.clone(),
        message.attachments.clone(),
    );

    let builder = match Answer::build(&request).await {
        Answer::None => CreateMessage::new().content(":skull_crossbones: No response received. The response may have been filtered, or a network error occurred."),
        Answer::Text(t) => CreateMessage::new().content(t),
        Answer::File(f) => CreateMessage::new().add_file(f),
    }
    .allowed_mentions(CreateAllowedMentions::new());

    typing.stop();

    if message
        .channel_id
        .send_message(&ctx.http, builder.clone().reference_message(message))
        .await
        .is_err()
    {
        message.channel_id.send_message(&ctx.http, builder).await?;
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command, guild_only, category = "Chatbot")]
pub async fn ask(ctx: Context<'_>, #[rest] text: Option<String>) -> Result<(), Error> {
    ctx.defer().await?;

    let guild = match ctx.guild() {
        Some(guild) => (guild.id.get(), guild.name.clone()),
        None => return Ok(()),
    };

    let ref_message: Option<&serenity::Message> = match &ctx {
        poise::Context::Prefix(p) => {
            if let Some(m) = &p.msg.referenced_message {
                Some(m)
            } else {
                Some(p.msg)
            }
        }
        _ => None,
    };

    let (author, content, attachments) = match (&text, &ctx) {
        (Some(t), poise::Context::Prefix(p)) => (
            ctx.author().to_owned(),
            t.clone(),
            p.msg.attachments.clone(),
        ),

        (Some(t), _) => (ctx.author().to_owned(), t.clone(), vec![]),

        (None, poise::Context::Prefix(p)) if p.msg.referenced_message.is_some() => {
            let r = p.msg.referenced_message.as_ref().unwrap();
            (
                r.author.to_owned(),
                r.content.clone(),
                r.attachments.clone(),
            )
        }

        (None, poise::Context::Prefix(p)) => (
            ctx.author().to_owned(),
            String::new(),
            p.msg.attachments.clone(),
        ),

        _ => (ctx.author().to_owned(), String::new(), vec![]),
    };

    let typing = ctx.channel_id().start_typing(&ctx.serenity_context().http);

    let request = AiRequest::new(guild, author, content, attachments);
    let response = Answer::build(&request).await;

    match ref_message {
        Some(m) => {
            let builder = match response {
                Answer::None => CreateMessage::new().content(tr!(ctx, "ask-error")),
                Answer::Text(t) => CreateMessage::new().content(t),
                Answer::File(f) => CreateMessage::new().add_file(f),
            }
            .allowed_mentions(CreateAllowedMentions::new());

            if ctx
                .channel_id()
                .send_message(ctx.http(), builder.clone().reference_message(m))
                .await
                .is_err()
            {
                ctx.channel_id().send_message(ctx.http(), builder).await?;
            };
        }
        None => {
            let builder = match response {
                Answer::None => CreateReply::default().content(tr!(ctx, "ask-error")),
                Answer::Text(t) => CreateReply::default().content(t),
                Answer::File(f) => CreateReply::default().attachment(f),
            }
            .allowed_mentions(CreateAllowedMentions::new());

            if ctx.send(builder.clone().reply(true)).await.is_err() {
                ctx.send(builder).await?;
            }
        }
    }

    typing.stop();

    Ok(())
}
