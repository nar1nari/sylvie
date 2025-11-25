use crate::Data;
use crate::utils::*;

type FluentBundle = fluent::bundle::FluentBundle<
    fluent::FluentResource,
    intl_memoizer::concurrent::IntlLangMemoizer,
>;

pub struct Translations {
    main: FluentBundle,
    other: std::collections::HashMap<String, FluentBundle>,
}

macro_rules! tr {
    ( $ctx:ident, $id:expr $(, $argname:ident: $argvalue:expr )* $(,)? ) => {{
        #[allow(unused_mut)]
        let mut args = fluent::FluentArgs::new();
        $( args.set(stringify!($argname), $argvalue); )*

        $crate::translation::get($ctx, $id, None, Some(&args))
    }};
}
pub(crate) use tr;

pub fn format(
    bundle: &FluentBundle,
    id: &str,
    attr: Option<&str>,
    args: Option<&fluent::FluentArgs<'_>>,
) -> Option<String> {
    let message = bundle.get_message(id)?;
    let pattern = match attr {
        Some(attribute) => message.get_attribute(attribute)?.value(),
        None => message.value()?,
    };
    let formatted = bundle.format_pattern(pattern, args, &mut vec![]);
    Some(formatted.into_owned())
}

pub fn get<E>(
    ctx: poise::Context<'_, crate::Data, E>,
    id: &str,
    attr: Option<&str>,
    args: Option<&fluent::FluentArgs<'_>>,
) -> String {
    let translations = &ctx.data().translations;
    ctx.locale()
        .and_then(|locale| format(translations.other.get(locale)?, id, attr, args))
        .or_else(|| format(&translations.main, id, attr, args))
        .unwrap_or_else(|| {
            tracing::warn!("unknown fluent message identifier `{}`", id);
            id.to_string()
        })
}

pub fn read_ftl() -> Result<Translations, Error> {
    fn read_single_ftl(path: &std::path::Path) -> Result<(String, FluentBundle), Error> {
        let locale = path.file_stem().ok_or("invalid .ftl filename")?;
        let locale = locale.to_str().ok_or("invalid filename UTF-8")?;

        let file_contents = std::fs::read_to_string(path)?;
        let resource = fluent::FluentResource::try_new(file_contents)
            .map_err(|(_, e)| format!("failed to parse {:?}: {:?}", path, e))?;

        let mut bundle = FluentBundle::new_concurrent(vec![
            locale
                .parse()
                .map_err(|e| format!("invalid locale `{}`: {}", locale, e))?,
        ]);
        bundle
            .add_resource(resource)
            .map_err(|e| format!("failed to add resource to bundle: {:?}", e))?;

        Ok((locale.to_string(), bundle))
    }

    Ok(Translations {
        main: read_single_ftl("translations/en-US.ftl".as_ref())?.1,
        other: std::fs::read_dir("translations")?
            .map(|file| read_single_ftl(&file?.path()))
            .collect::<Result<_, _>>()?,
    })
}

pub fn apply_translations(
    translations: &Translations,
    commands: &mut [poise::Command<Data, Error>],
) {
    fn apply_to_command(translations: &Translations, command: &mut poise::Command<Data, Error>) {
        for (locale, bundle) in &translations.other {
            if let Some(localized_command_name) = format(bundle, &command.name, None, None) {
                command
                    .name_localizations
                    .insert(locale.clone(), localized_command_name);
                if let Some(desc) = format(bundle, &command.name, Some("description"), None) {
                    command
                        .description_localizations
                        .insert(locale.clone(), desc);
                }
            }

            for parameter in &mut command.parameters {
                if let Some(param_name) = format(bundle, &command.name, Some(&parameter.name), None)
                {
                    parameter
                        .name_localizations
                        .insert(locale.clone(), param_name);
                }
                if let Some(param_desc) = format(
                    bundle,
                    &command.name,
                    Some(&format!("{}-description", parameter.name)),
                    None,
                ) {
                    parameter
                        .description_localizations
                        .insert(locale.clone(), param_desc);
                }

                for choice in &mut parameter.choices {
                    if let Some(choice_name) = format(bundle, &choice.name, None, None) {
                        choice.localizations.insert(locale.clone(), choice_name);
                    }
                }
            }
        }

        let bundle = &translations.main;
        if let Some(name) = format(bundle, &command.name, None, None) {
            command.name = name;
        }
        if let Some(desc) = format(bundle, &command.name, Some("description"), None) {
            command.description = Some(desc);
        }

        for parameter in &mut command.parameters {
            if let Some(name) = format(bundle, &command.name, Some(&parameter.name), None) {
                parameter.name = name;
            }
            parameter.description = format(
                bundle,
                &command.name,
                Some(&format!("{}-description", parameter.name)),
                None,
            );
        }

        for choice in &mut command.parameters.iter_mut().flat_map(|p| &mut p.choices) {
            if let Some(name) = format(bundle, &choice.name, None, None) {
                choice.name = name;
            }
        }

        for sub in &mut command.subcommands {
            apply_to_command(translations, sub);
        }
    }

    for command in commands {
        apply_to_command(translations, command);
    }
}
