use zed_extension_api::{self as zed, Result};

struct NoirExtension;

impl zed::Extension for NoirExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let nargo_binary = worktree
            .which("nargo")
            .ok_or_else(|| "nargo must be installed and available on PATH".to_string())?;

        Ok(zed::Command {
            command: nargo_binary,
            args: vec!["lsp".to_string()],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(NoirExtension);
