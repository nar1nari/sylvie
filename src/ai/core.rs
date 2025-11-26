use std::{fs, io, path::Path, time::Duration};

use gem_rs::types::{FileData, FileManager};
use serde_json::{Value, json};

use crate::ai::{agent::Agent, agents, request::AiRequest};

pub async fn ask(request: &AiRequest) -> Option<String> {
    let mut agents = agents().await;
    let agent = agents
        .entry(request.guild.0)
        .or_insert_with(|| Agent::new(&generate_prompt(request.guild.clone())));

    let client = reqwest::Client::new();

    let file_manager = FileManager::new();

    let mut file_data: Option<FileData> = None;

    if let Some(attachment) = request.attachments.first()
        && let Ok(resp) = client.get(&attachment.url).send().await
        && let Ok(bytes) = resp.bytes().await
    {
        let mut mime = attachment
            .content_type
            .clone()
            .unwrap_or_else(|| "application/octet-stream".to_string());

        if let Some((m, _)) = mime.split_once(';') {
            mime = m.trim().to_string();
        }

        if mime.starts_with("text/") || mime == "application/octet-stream" {
            mime = "text/plain".to_string();
        }

        match file_manager
            .add_file_from_bytes("upload", bytes.to_vec(), &mime)
            .await
        {
            Ok(f) => file_data = Some(f),
            Err(e) => tracing::error!("add_file_from_bytes error: {}", e),
        }
    }

    let prompt = format!(
        "[{}]<{}:{}>: {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        request.author.name,
        request.author.id,
        request.content
    );

    let response = agent.chat(&prompt, file_data).await;

    match response {
        Ok(mut res) => {
            if res.contains("[REM]") {
                res = res.replace("[REM]", "");
                if let Err(e) = save_prompt_line(request.guild.0, &prompt) {
                    tracing::error!("Error saving prompt: {}", e.kind());
                }
            }
            Some(res)
        }
        Err(_) => None,
    }
}

pub fn get_guild_history(guild_id: u64) -> Option<Value> {
    let path_str = format!("database/prompts/{}.json", guild_id);
    let path = Path::new(&path_str);

    if !path.exists() {
        return None;
    }

    let content = fs::read_to_string(path).ok()?;
    let json: Value = serde_json::from_str(&content).ok()?;
    Some(json)
}

pub fn generate_prompt(guild: (u64, String)) -> String {
    let prompt = fs::read_to_string("prompt.txt").unwrap_or_default();
    let mut result = prompt.replace("{guild_name}", &guild.1);

    let json = match get_guild_history(guild.0) {
        Some(j) => j,
        None => return result,
    };

    let history = match json.get("history").and_then(|v| v.as_array()) {
        Some(h) => h,
        None => return result,
    };

    let history_text = history
        .iter()
        .filter_map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    if !history_text.is_empty() {
        result.push_str(&history_text);
    }

    result
}

pub fn save_prompt_line(guild_id: u64, line: &str) -> io::Result<()> {
    let dir_path = Path::new("database/prompts");
    let file_path = dir_path.join(format!("{}.json", guild_id));

    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?;
    }

    let mut data: Value = if file_path.exists() {
        let content = fs::read_to_string(&file_path)?;
        serde_json::from_str(&content).unwrap_or_else(|_| json!({ "history": [] }))
    } else {
        json!({ "history": [] })
    };

    if let Some(history) = data.get_mut("history").and_then(|v| v.as_array_mut()) {
        history.push(Value::String(line.to_string()));
    }

    fs::write(&file_path, serde_json::to_string_pretty(&data)?)?;

    Ok(())
}

pub fn remove_prompt_lines(guild_id: u64, indices: &[usize]) -> io::Result<()> {
    let dir_path = Path::new("database/prompts");
    let file_path = dir_path.join(format!("{}.json", guild_id));

    if !file_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "history file not found",
        ));
    }

    let content = fs::read_to_string(&file_path)?;
    let mut data: Value =
        serde_json::from_str(&content).unwrap_or_else(|_| json!({ "history": [] }));

    let history = match data.get_mut("history").and_then(|v| v.as_array_mut()) {
        Some(h) => h,
        None => return Ok(()),
    };

    let max_index = if history.is_empty() {
        0
    } else {
        history.len() - 1
    };
    if indices.iter().any(|&i| i > max_index) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("index out of range (max index: {})", max_index),
        ));
    }

    let updated_history: Vec<Value> = history
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if !indices.contains(&i) {
                Some(v.clone())
            } else {
                None
            }
        })
        .collect();

    *history = updated_history;

    fs::write(&file_path, serde_json::to_string_pretty(&data)?)?;
    Ok(())
}

pub fn remove_prompt_file(guild_id: u64) -> io::Result<()> {
    let dir_path = Path::new("database/prompts");
    let file_path = dir_path.join(format!("{}.json", guild_id));

    fs::remove_file(file_path)
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "history file not found"))
}

pub async fn remove_agent(guild_id: u64) -> io::Result<Agent> {
    let mut agents = agents().await;
    agents
        .remove(&guild_id)
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "agent not found"))
}

async fn clear_old_agents() {
    let mut agents = agents().await;
    agents.retain(|_, agent| agent.time_since_last_interaction() <= 3600);
}

pub async fn clear_agents_task() {
    loop {
        clear_old_agents().await;
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
