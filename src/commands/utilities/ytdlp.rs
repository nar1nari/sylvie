use poise::serenity_prelude as serenity;

use crate::translation::tr;
use crate::utils::*;
use std::path::{Path, PathBuf};
use tokio::process;

const MAX_FILE_SIZE: usize = 10;
const MAX_BYTES: u64 = 10485760;
const DOWNLOAD_PATH: &str = "/tmp";

async fn get_filename(url: &str) -> Option<String> {
    let output = process::Command::new("yt-dlp")
        .arg("--max-filesize")
        .arg(format!("{}M", MAX_FILE_SIZE))
        .arg("--print")
        .arg("filename")
        .arg(url)
        .output()
        .await
        .ok()?;

    if !output.status.success() {
        return None;
    }

    String::from_utf8(output.stdout).ok()
}

async fn download_video(url: &str, output_dir: &Path) -> Option<PathBuf> {
    let filename = get_filename(url).await?;
    let filename = filename.trim();
    if filename.is_empty() {
        return None;
    }

    let final_path = output_dir.join(filename);

    let output = process::Command::new("yt-dlp")
        .arg("--max-filesize")
        .arg(format!("{}M", MAX_FILE_SIZE))
        .arg("-o")
        .arg(final_path.to_string_lossy().to_string())
        .arg(url)
        .output()
        .await
        .ok()?;

    if !output.status.success() {
        return None;
    }

    if !final_path.exists() {
        return None;
    }

    Some(output_dir.join(filename))
}

#[poise::command(slash_command, prefix_command, category = "Utilities")]
pub async fn ytdlp(ctx: Context<'_>, url: String) -> Result<(), Error> {
    ctx.defer().await?;

    let download_path = Path::new(DOWNLOAD_PATH);
    let file_path = download_video(&url, download_path).await;

    let Some(file_path) = file_path else {
        reply_with_error(ctx, tr!(ctx, "ytdlp-failed")).await?;
        return Ok(());
    };

    let size = tokio::fs::metadata(&file_path).await?.len();

    if size >= MAX_BYTES {
        reply_with_error(ctx, tr!(ctx, "ytdlp-failed")).await?;
        tokio::fs::remove_file(&file_path).await.ok();
        return Ok(());
    }

    let attachment = serenity::CreateAttachment::path(&file_path).await?;
    reply_without_ping(ctx, poise::CreateReply::default().attachment(attachment)).await?;

    tokio::fs::remove_file(file_path).await.ok();

    Ok(())
}
