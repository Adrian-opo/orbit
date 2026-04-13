use super::{Provider, ProviderSpawnConfig};
use crate::journal::JournalState;
use crate::models::SlashCommand;
use crate::services::spawn_manager::{spawn_opencode, OpenCodeConfig, SpawnHandle};

pub struct OpenCodeProvider;

impl Provider for OpenCodeProvider {
    fn id(&self) -> &str {
        "opencode"
    }

    fn display_name(&self) -> &str {
        "agent"
    }

    fn spawn(&self, config: ProviderSpawnConfig) -> Result<SpawnHandle, String> {
        spawn_opencode(OpenCodeConfig {
            session_id: config.session_id,
            cwd: config.cwd,
            model: config.model,
            prompt: config.prompt,
            opencode_session_id: config.resume_id,
            extra_env: config.extra_env,
        })
    }

    fn process_line(&self, state: &mut JournalState, line: &str) {
        crate::journal::process_line_opencode(state, line);
    }

    fn context_window(&self, model: &str) -> Option<u64> {
        // model is "provider/model_id" — split to get provider and model parts
        // e.g. "openrouter/minimax/minimax-m2.5:free" → provider="openrouter",
        //       model="minimax/minimax-m2.5:free"
        let (provider, model_id) = model.split_once('/').unwrap_or(("", model));
        crate::commands::providers::lookup_context_window(provider, model_id)
    }

    fn slash_commands(&self) -> Vec<SlashCommand> {
        crate::commands::plugins::get_opencode_commands()
    }

    fn supports_effort(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TestCase;

    #[test]
    fn should_return_opencode_as_id() {
        let mut t = TestCase::new("should_return_opencode_as_id");
        let provider = OpenCodeProvider;

        t.phase("Assert");
        t.eq("id", provider.id(), "opencode");
    }

    #[test]
    fn should_not_support_effort() {
        let mut t = TestCase::new("should_not_support_effort");
        let provider = OpenCodeProvider;

        t.phase("Assert");
        t.ok("supports_effort is false", !provider.supports_effort());
    }

    #[test]
    fn should_return_agent_as_display_name() {
        let mut t = TestCase::new("should_return_agent_as_display_name");
        let provider = OpenCodeProvider;

        t.phase("Assert");
        t.eq("display_name", provider.display_name(), "agent");
    }

    #[test]
    fn should_parse_opencode_text_line() {
        let mut t = TestCase::new("should_parse_opencode_text_line");

        t.phase("Seed");
        let mut state = JournalState::default();
        let line = r#"{"type":"text","part":{"type":"text","text":"hello"}}"#;

        t.phase("Act");
        let provider = OpenCodeProvider;
        provider.process_line(&mut state, line);

        t.phase("Assert");
        t.len("1 entry produced", &state.entries, 1);
        t.eq(
            "entry type is Assistant",
            state.entries[0].entry_type,
            crate::models::JournalEntryType::Assistant,
        );
    }
}
