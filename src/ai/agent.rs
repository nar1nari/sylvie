use gem_rs::{
    client::GemSession,
    errors::GemError,
    types::{Context, FileData, HarmBlockThreshold, Role, Settings},
};

pub struct Agent {
    session: GemSession,
    settings: Settings,
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
        Self { session, settings }
    }

    pub async fn chat(
        &mut self,
        input: &str,
        attachment: Option<FileData>,
    ) -> Result<String, GemError> {
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
}
