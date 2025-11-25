use rand::seq::IndexedRandom;
use serde_json::Value;

use crate::translation::tr;
use crate::utils::*;

async fn get_random_image(ctx: Context<'_>, tags: &str) -> Result<String, String> {
    let client = reqwest::Client::new();

    let params = [
        ("page", "dapi"),
        ("s", "post"),
        ("q", "index"),
        ("json", "1"),
        ("tags", tags),
    ];

    let response = client
        .get("https://safebooru.org/index.php")
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

    let directory = random_post.get("directory").and_then(Value::as_i64);
    let image = random_post.get("image").and_then(Value::as_str);

    let url = match (directory, image) {
        (Some(dir), Some(img)) => format!("https://safebooru.org/images/{}/{}", dir, img),
        _ => return Err(tr!(ctx, "booru-not-found")),
    };

    Ok(url.to_string())
}

#[poise::command(prefix_command, slash_command, aliases("sb"), category = "Fun")]
pub async fn safebooru(ctx: Context<'_>, #[rest] tags: Option<String>) -> Result<(), Error> {
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
