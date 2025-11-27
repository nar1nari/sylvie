use poise::{
    CreateReply,
    serenity_prelude::{CreateAllowedMentions, CreateAttachment},
};

use crate::translation::tr;
use crate::utils::*;
use std::path::{Path, PathBuf};
use tokio::process;

const MAX_FILE_SIZE: usize = 10;
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

    if let Some(file_path) = file_path {
        let attachment = CreateAttachment::path(&file_path).await?;
        tokio::fs::remove_file(file_path).await.ok();
        ctx.send(
            CreateReply::default()
                .attachment(attachment)
                .reply(true)
                .allowed_mentions(CreateAllowedMentions::new()),
        )
        .await?;
    } else {
        reply_with_error(ctx, tr!(ctx, "ytdlp-failed")).await?;
    }

    Ok(())
}
