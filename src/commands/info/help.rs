use poise::serenity_prelude::{CommandId, CreateAllowedMentions, CreateEmbed};
use poise::{Command, CreateReply};
use std::collections::HashMap;

use crate::translation;
use crate::{Data, translation::tr, utils::*};

fn format_context_menu_name(command: &Command<Data, Error>) -> Option<String> {
    let kind = match command.context_menu_action {
        Some(poise::ContextMenuCommandAction::User(_)) => "user",
        Some(poise::ContextMenuCommandAction::Message(_)) => "message",
        Some(poise::ContextMenuCommandAction::__NonExhaustive) => unreachable!(),
        None => return None,
    };
    Some(format!(
        "{} (on {})",
        command
            .context_menu_name
            .as_deref()
            .unwrap_or(&command.name),
        kind
    ))
}

fn preformat_subcommands(
    list: &mut String,
    command: &Command<Data, Error>,
    prefix: &str,
    command_ids: &HashMap<String, CommandId>,
) {
    let prefix = match prefix.find(":") {
        Some(pos) => &prefix[..pos],
        None => prefix,
    };
    let as_context_command = command.slash_action.is_none() && command.prefix_action.is_none();
    for subcommand in &command.subcommands {
        let command = if as_context_command {
            let name = format_context_menu_name(subcommand);
            if name.is_none() {
                continue;
            };
            name.unwrap()
        } else {
            format!(
                "{} {}:{}>",
                prefix,
                subcommand.name,
                command_ids
                    .get(&command.name)
                    .unwrap_or(&CommandId::default())
            )
        };
        *list = format!("{} {}", list, command);
    }
}

fn preformat_command(
    list: &mut String,
    command: &Command<Data, Error>,
    command_ids: &HashMap<String, CommandId>,
) {
    let prefix = if command.slash_action.is_some() {
        String::from("/")
    } else if command.prefix_action.is_some() {
        String::from("s.")
    } else {
        unreachable!();
    };
    let prefix = format!(
        "<{}{}:{}>",
        prefix,
        command.name,
        command_ids
            .get(&command.name)
            .unwrap_or(&CommandId::default())
    );
    *list = format!("{} {}", list, prefix);
    preformat_subcommands(list, command, &prefix, command_ids);
}

async fn help_single_command(ctx: Context<'_>, command_name: &str) -> Result<CreateEmbed, Error> {
    let commands = &ctx.framework().options().commands;
    let mut command = commands.iter().find(|command| {
        if let Some(context_menu_name) = &command.context_menu_name
            && context_menu_name.eq_ignore_ascii_case(command_name)
        {
            return true;
        }
        false
    });
    let mut all_subcommands = HashMap::<String, String>::new();
    for command in commands {
        for sub in &command.subcommands {
            all_subcommands.insert(sub.name.clone(), format!("{} ", command.name));
        }
    }

    if command.is_none()
        && let Some((c, _, _)) = poise::find_command(commands, command_name, true, &mut vec![])
    {
        command = Some(c);
    }
    let embed = if let Some(command) = command {
        let mut embed = CreateEmbed::new()
            .color(HELP_EMBED_COLOR)
            .title(command_name);

        embed = if command.description.is_some() {
            embed.field(
                "",
                translation::get(ctx, &command.name, Some("description"), None),
                false,
            )
        } else {
            embed.field("", tr!(ctx, "help-no-description"), false)
        };

        let mut params = HashMap::<String, String>::new();
        for param in &command.parameters {
            let name = format!(
                "`{}{}`",
                translation::get(ctx, &command.name, Some(&param.name), None),
                if !param.required {
                    tr!(ctx, "help-optional")
                } else {
                    String::new()
                }
            );
            let desc = translation::get(
                ctx,
                &command.name,
                Some(format!("{}-description", param.name).as_str()),
                None,
            );
            params.insert(name, desc);
        }
        let param_names = params
            .keys()
            .map(|k| k.clone())
            .collect::<Vec<_>>()
            .join(" ");

        let mut invocations = Vec::new();
        if command.slash_action.is_some() {
            invocations.push(format!(
                "`/{}{}` {}",
                all_subcommands.get(&command.name).unwrap_or(&String::new()),
                command.name,
                param_names
            ));
        }
        if command.prefix_action.is_some() {
            let prefix = String::from("s.");
            invocations.push(format!("`{}{}` {}", prefix, command.name, param_names));
        }
        let invocations = invocations.join("\n");
        embed = embed.field(tr!(ctx, "help-usage"), invocations, false);

        if !params.is_empty() {
            let params = params
                .iter()
                .map(|(k, v)| format!("{} — {}", k, v))
                .collect::<Vec<_>>()
                .join("\n");
            embed = embed.field(tr!(ctx, "help-parameters"), params, false);
        }

        if !command.aliases.is_empty() {
            let aliases = command.aliases.join(" ");
            embed = embed.field(tr!(ctx, "help-aliases"), aliases, false);
        }

        embed
    } else {
        CreateEmbed::new()
            .color(ERROR_EMBED_COLOR)
            .title(tr!(ctx, "help-title"))
            .field(
                tr!(ctx, "help-not-found"),
                tr!(ctx, "help-not-found-description", command: command_name),
                false,
            )
    };

    Ok(embed)
}

async fn help_all_commands(ctx: Context<'_>) -> Result<CreateEmbed, Error> {
    let mut categories = HashMap::<Option<&str>, Vec<&Command<_, _>>>::new();

    for cmd in &ctx.framework().options().commands {
        if let Some(category) = cmd.category.as_deref() {
            categories.entry(Some(category)).or_default().push(cmd);
        }
    }

    let command_ids: HashMap<_, _> = ctx
        .serenity_context()
        .http
        .get_global_commands()
        .await?
        .into_iter()
        .map(|cmd| (cmd.name.clone(), cmd.id))
        .collect();

    let mut embed = CreateEmbed::new()
        .color(HELP_EMBED_COLOR)
        .title(tr!(ctx, "help-title"));

    for (category_name, commands) in categories {
        let commands = commands
            .into_iter()
            .filter(|cmd| {
                !cmd.hide_in_help && (cmd.prefix_action.is_some() || cmd.slash_action.is_some())
            })
            .collect::<Vec<_>>();
        if commands.is_empty() {
            continue;
        }
        let mut all_subcommands = Vec::new();
        for command in &commands {
            all_subcommands.extend(command.subcommands.iter().map(|c| c.name.clone()))
        }

        let mut cmd_list = String::new();
        for command in commands {
            if !(all_subcommands.contains(&command.name)) {
                preformat_command(&mut cmd_list, command, &command_ids);
            }
        }
        embed = embed.field(
            format!("─ {}", category_name.unwrap_or("Commands")),
            cmd_list,
            false,
        );
    }
    Ok(embed)
}

#[poise::command(slash_command, prefix_command, category = "Info")]
pub async fn help(ctx: Context<'_>, command: Option<String>) -> Result<(), Error> {
    ctx.defer().await?;

    let mut embed = match command {
        Some(cmd) => help_single_command(ctx, &cmd).await.unwrap_or_default(),
        None => help_all_commands(ctx).await.unwrap_or_default(),
    };
    embed = embed.field("", tr!(ctx, "help-footer"), false);

    ctx.send(
        CreateReply::default()
            .embed(embed)
            .reply(true)
            .allowed_mentions(CreateAllowedMentions::new()),
    )
    .await?;

    Ok(())
}
