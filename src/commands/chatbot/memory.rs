use poise::{
    CreateReply,
    serenity_prelude::{
        self as serenity, ButtonStyle, CreateActionRow, CreateButton, CreateEmbed,
        CreateEmbedFooter, EditMessage,
    },
};

use crate::{ai, translation::tr, utils::*};

const PAGE_SIZE: usize = 15;

fn load_memory(guild_id: u64) -> Vec<String> {
    let Some(json) = ai::core::get_guild_history(guild_id) else {
        return vec![];
    };

    json.get("history")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn format_message(msg: &str) -> String {
    if let Some((prefix, text)) = msg.split_once(">: ")
        && let Some(id) = prefix.split(':').next_back()
    {
        let id = id.trim_matches(&['<', '>'][..]);
        return format!("<@{}>: {}", id, text);
    }

    msg.to_string()
}

fn build_page(history: &[String], page: usize) -> String {
    let start = page * PAGE_SIZE;
    let end = (start + PAGE_SIZE).min(history.len());

    history[start..end]
        .iter()
        .enumerate()
        .map(|(i, msg)| format!("{}. {}", start + i + 1, format_message(msg)))
        .collect::<Vec<_>>()
        .join("\n")
}

fn create_embed(ctx: Context<'_>, desc: &str, page: usize, max_page: usize) -> CreateEmbed {
    CreateEmbed::new()
        .title(tr!(ctx, "memory-title"))
        .description(desc)
        .footer(CreateEmbedFooter::new(
            tr!(ctx, "memory-footer", page: page + 1, max_page: max_page + 1),
        ))
}

#[poise::command(slash_command, prefix_command, guild_only, aliases("mem"), category = "Chatbot")]
pub async fn memory(ctx: Context<'_>) -> Result<(), Error> {
    let uuid = ctx.id();
    let guild_id = ctx.guild_id().unwrap().get();
    let history = load_memory(guild_id);

    if history.is_empty() {
        reply_without_ping(ctx, tr!(ctx, "memory-empty")).await?;
        return Ok(());
    }

    let max_page = (history.len() - 1) / PAGE_SIZE;
    let mut page = 0;

    let page_text = build_page(&history, page);
    let embed = create_embed(ctx, &page_text, page, max_page);

    let buttons = vec![CreateActionRow::Buttons(vec![
        CreateButton::new(format!("{uuid}_first"))
            .label("⏪")
            .style(ButtonStyle::Secondary),
        CreateButton::new(format!("{uuid}_prev"))
            .label("◀️")
            .style(ButtonStyle::Secondary),
        CreateButton::new(format!("{uuid}_next"))
            .label("▶️")
            .style(ButtonStyle::Secondary),
        CreateButton::new(format!("{uuid}_last"))
            .label("⏩")
            .style(ButtonStyle::Secondary),
    ])];

    ctx.send(CreateReply::default().embed(embed).components(buttons))
        .await?;

    while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx)
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .timeout(std::time::Duration::from_secs(120))
        .await
    {
        let cid = mci.data.custom_id.as_str();

        match cid {
            id if id == format!("{uuid}_first") => page = 0,
            id if id == format!("{uuid}_prev") => {
                page = page.saturating_sub(1);
            }
            id if id == format!("{uuid}_next") => {
                if page < max_page {
                    page += 1;
                }
            }
            id if id == format!("{uuid}_last") => page = max_page,
            _ => continue,
        }

        let new_text = build_page(&history, page);
        let new_embed = create_embed(ctx, &new_text, page, max_page);

        let mut msg = mci.message.clone();
        msg.edit(ctx, EditMessage::new().embed(new_embed)).await?;

        mci.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge)
            .await?;
    }

    Ok(())
}
