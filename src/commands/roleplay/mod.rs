use poise::{
    CreateReply,
    serenity_prelude::{self as serenity, CreateEmbed},
};
use rand::seq::IndexedRandom;
use reqwest::StatusCode;

use crate::{translation::tr, utils::*};

pub mod bite;
pub mod hug;
pub mod kiss;
pub mod lick;
pub mod pat;
pub mod sleep;

const GIFS_URL: &str = "https://nar1nari.space/sylvie/gifs";

pub struct Action<'a> {
    pub name: &'a str,
    pub needs_target: bool,
}

impl<'a> Action<'a> {
    pub fn new(name: &'a str, needs_target: bool) -> Self {
        Self { name, needs_target }
    }

    pub async fn gifs(&self) -> Vec<String> {
        let mut urls = Vec::new();
        for i in 0.. {
            let url = format!("{}/{}/{}.gif", GIFS_URL, self.name, i);
            let resp = reqwest::Client::new().head(&url).send().await;

            if let Ok(r) = resp
                && r.status() == StatusCode::OK
            {
                urls.push(url);
            } else {
                break;
            }
        }
        urls
    }

    pub fn msg_target(&self, ctx: Context<'_>, author_name: &str, target_name: &str) -> String {
        let id = format!("{}-target", self.name);
        tr!(ctx, &id, author: author_name, target: target_name)
    }

    pub fn msg_no_target(&self, ctx: Context<'_>, author_name: &str) -> String {
        let id = format!("{}-no-target", self.name);
        tr!(ctx, &id, author: author_name)
    }
}

pub async fn run_rp_action(
    ctx: Context<'_>,
    action: &Action<'_>,
    target: &Option<serenity::User>,
) -> Result<(), Error> {
    let target = target.clone().unwrap_or_else(|| {
        if let poise::Context::Prefix(p) = ctx {
            p.msg
                .referenced_message
                .as_ref()
                .map(|m| m.author.clone())
                .unwrap_or_else(|| p.msg.author.clone())
        } else {
            ctx.author().clone()
        }
    });
    let gifs = action.gifs().await;
    let gif = match gifs.choose(&mut rand::rng()) {
        Some(g) => g,
        None => return Ok(()),
    };

    if action.needs_target && &target == ctx.author() {
        reply_with_error(ctx, tr!(ctx, "rp-need-target")).await?;
        return Ok(());
    }

    if action.name == "kiss" && target.id == ctx.framework().bot_id {
        reply_without_ping(ctx, tr!(ctx, "kiss-no")).await?;
        return Ok(());
    }

    let mut embed = CreateEmbed::new().color(DEFAULT_EMBED_COLOR).image(gif);

    embed = if &target == ctx.author() {
        embed.title(action.msg_no_target(ctx, ctx.author().display_name()))
    } else {
        embed.title(action.msg_target(ctx, ctx.author().display_name(), target.display_name()))
    };

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    subcommands(
        "hug::hug",
        "kiss::kiss",
        "bite::bite",
        "lick::lick",
        "pat::pat",
        "sleep::sleep"
    ),
    subcommand_required,
    category = "Roleplay"
)]
pub async fn rp(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
