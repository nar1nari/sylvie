use std::error::Error;

use poise::Command;
use poise::serenity_prelude::{self as serenity};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

mod ai;
mod commands;
mod handlers;
mod translation;
mod utils;
use crate::commands::*;
use crate::handlers::*;

pub struct Data {
    translations: translation::Translations,
}

fn base_commands() -> Vec<Command<Data, Box<dyn Error + Send + Sync>>> {
    vec![
        fun::cat::cat(),
        fun::coin::coin(),
        fun::rule34::rule34(),
        fun::safebooru::safebooru(),
        info::ping::ping(),
        info::help::help(),
        chatbot::ask::ask(),
        chatbot::forget::forget(),
        chatbot::forgetall::forgetall(),
        chatbot::restart::restart(),
        admin::clear::clear(),
        chatbot::memory::memory(),
        roleplay::rp(),
    ]
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::new("info,gem_rs=off"))
        .with(fmt::layer())
        .init();

    ai::init_agents().await;

    tokio::spawn(ai::core::clear_agents_task());

    let translations = translation::read_ftl().expect("failed to read translation files");
    let mut registered_commands = base_commands();
    let mut commands = base_commands();
    commands.extend([
        roleplay::hug::hug(),
        roleplay::kiss::kiss(),
        roleplay::bite::bite(),
        roleplay::lick::lick(),
        roleplay::pat::pat(),
        roleplay::sleep::sleep(),
    ]);
    translation::apply_translations(&translations, &mut registered_commands);
    translation::apply_translations(&translations, &mut commands);

    let token = match dotenvy::var("DISCORD_TOKEN") {
        Ok(t) => t,
        Err(e) => {
            println!("Failed to get token: {}", e);
            return;
        }
    };
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("s.".into()),
                case_insensitive_commands: true,
                ..Default::default()
            },
            event_handler: |ctx, event, framework, _data| {
                Box::pin(event_handler(ctx, event, framework))
            },
            on_error: |error| Box::pin(error_handler(error)),
            ..Default::default()
        })
        .setup(move |ctx, _ready, _| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &registered_commands).await?;
                Ok(Data { translations })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}
