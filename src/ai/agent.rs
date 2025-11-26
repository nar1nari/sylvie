use std::time::{SystemTime, UNIX_EPOCH};

use gem_rs::{
    client::GemSession,
    errors::GemError,
    types::{Context, FileData, HarmBlockThreshold, Role, Settings},
};

pub struct Agent {
    session: GemSession,
    settings: Settings,
    last_interaction: u64,
}

impl Agent {
    pub fn new(system_message: &str) -> Self {
        let session = GemSession::Builder()
            .custom_model("gemini-2.5-flash".to_string())
            .context(Context::new())
            .build();
        let mut settings = Settings::new();
        settings.set_all_safety_settings(HarmBlockThreshold::BlockNone);
        settings.set_system_instruction(system_message);
        settings.set_thinking_budget(0);
        let last_interaction = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            session,
            settings,
            last_interaction,
        }
    }

    pub async fn chat(
        &mut self,
        input: &str,
        attachment: Option<FileData>,
    ) -> Result<String, GemError> {
        self.update_last_interaction();

        let msg = match attachment {
            Some(data) => {
                self.session
                    .send_message_with_file(input, data, Role::User, &self.settings)
                    .await?
            }
            None => {
                self.session
                    .send_message(input, Role::User, &self.settings)
                    .await?
            }
        };

        Ok(msg
            .get_results()
            .first()
            .unwrap_or(&"".to_string())
            .to_owned())
    }

    pub fn set_last_interaction(&mut self, time: u64) {
        self.last_interaction = time;
    }

    pub fn last_interaction(&self) -> u64 {
        self.last_interaction
    }

    pub fn time_since_last_interaction(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(self.last_interaction())
    }

    pub fn update_last_interaction(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.set_last_interaction(now);
    }
}
