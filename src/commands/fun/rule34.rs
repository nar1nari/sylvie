use rand::seq::IndexedRandom;
use serde_json::Value;

use crate::translation::tr;
use crate::utils::*;

async fn get_random_image(ctx: Context<'_>, tags: &str) -> Result<String, String> {
    let api_key = dotenvy::var("R34_API_KEY").unwrap_or_default();
    let client = reqwest::Client::new();

    let params = [
        ("user_id", "5304462"),
        ("api_key", &api_key),
        ("page", "dapi"),
        ("s", "post"),
        ("q", "index"),
        ("json", "1"),
        ("tags", &format!("sort:random {}", tags)),
        ("limit", "1000"),
    ];

    let response = client
        .get("https://api.rule34.xxx/index.php")
        .query(&params)
        .send()
        .await
        .map_err(|_| tr!(ctx, "went-wrong"))?;

    if !response.status().is_success() {
        return Err(tr!(ctx, "went-wrong"));
    }

    let posts: Value = response.json().await.map_err(|_| tr!(ctx, "went-wrong"))?;

    let items = posts
        .as_array()
        .ok_or_else(|| tr!(ctx, "booru-not-found"))?;
    if items.is_empty() {
        return Err(tr!(ctx, "booru-not-found"));
    }
    let mut rng = rand::rng();
    let random_post = items
        .choose(&mut rng)
        .ok_or_else(|| tr!(ctx, "booru-not-found"))?;
    let url = random_post
        .get("sample_url")
        .and_then(Value::as_str)
        .ok_or_else(|| tr!(ctx, "booru-not-found"))?;

    Ok(url.to_string())
}

#[poise::command(
    prefix_command,
    slash_command,
    nsfw_only,
    aliases("r34"),
    category = "Fun"
)]
pub async fn rule34(ctx: Context<'_>, #[rest] tags: Option<String>) -> Result<(), Error> {
    ctx.defer().await?;

    let tags = tags.unwrap_or_default();

    match get_random_image(ctx, &tags).await {
        Ok(url) => {
            reply_without_ping(ctx, url).await?;
        }
        Err(msg) => {
            reply_without_ping(ctx, msg).await?;
        }
    }

    Ok(())
}
