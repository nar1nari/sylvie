use crate::translation::tr;
use crate::utils::*;

#[poise::command(slash_command, prefix_command, category = "Fun")]
pub async fn cat(ctx: Context<'_>) -> Result<(), Error> {
    let error_msg = tr!(ctx, "cat-not-found");

    let client = reqwest::Client::new();
    let response = match client
        .get("https://api.thecatapi.com/v1/images/search")
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(_) => {
            reply_without_ping(ctx, error_msg).await?;
            return Ok(());
        }
    };

    if !response.status().is_success() {
        reply_without_ping(ctx, error_msg).await?;
        return Ok(());
    }

    let data: serde_json::Value = response.json().await?;

    if let Some(url) = data[0]["url"].as_str() {
        reply_without_ping(ctx, url).await?;
    } else {
        reply_with_error(ctx, error_msg).await?;
    }

    Ok(())
}
