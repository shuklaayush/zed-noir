use zed_extension_api::{self as zed, Result};
use zed::{lsp::{Completion, CompletionKind, Symbol, SymbolKind}, serde_json};

fn merge_json(base: serde_json::Value, override_value: Option<serde_json::Value>) -> serde_json::Value {
    match (base, override_value) {
        (serde_json::Value::Object(mut base), Some(serde_json::Value::Object(override_object))) => {
            for (key, value) in override_object {
                base.insert(key, value);
            }
            serde_json::Value::Object(base)
        }
        (_, Some(override_value)) => override_value,
        (base, None) => base,
    }
}

fn trim_completion_name(label: &str) -> &str {
    label.find(['(', '!']).map_or(label, |index| &label[..index])
}

fn completion_display_parts(completion: &Completion) -> (String, Option<String>) {
    let primary = completion
        .label_details
        .as_ref()
        .and_then(|details| details.description.clone())
        .or_else(|| completion.detail.clone());
    let secondary = completion
        .label_details
        .as_ref()
        .and_then(|details| details.detail.clone());

    let display = match completion.kind {
        Some(CompletionKind::Method)
        | Some(CompletionKind::Function)
        | Some(CompletionKind::Constructor) => {
            let name = trim_completion_name(&completion.label);
            match primary {
                Some(signature) if signature.starts_with("unconstrained fn(") => {
                    format!("unconstrained fn {name}{}", &signature["unconstrained fn".len()..])
                }
                Some(signature) if signature.starts_with("fn(") => {
                    format!("fn {name}{}", &signature["fn".len()..])
                }
                Some(signature) => format!("{name} {signature}"),
                None => completion.label.clone(),
            }
        }
        Some(CompletionKind::Struct) => format!("struct {}", completion.label),
        Some(CompletionKind::Interface) => format!("trait {}", completion.label),
        Some(CompletionKind::Enum) => format!("enum {}", completion.label),
        Some(CompletionKind::Module) => format!("mod {}", completion.label),
        Some(CompletionKind::Constant) => match primary {
            Some(typ) => format!("let {}: {typ}", completion.label),
            None => completion.label.clone(),
        },
        Some(CompletionKind::Field)
        | Some(CompletionKind::Variable)
        | Some(CompletionKind::Value)
        | Some(CompletionKind::Property) => match primary {
            Some(typ) => format!("{}: {typ}", completion.label),
            None => completion.label.clone(),
        },
        _ => match primary {
            Some(detail) => format!("{} {detail}", completion.label),
            None => completion.label.clone(),
        },
    };

    (display, secondary)
}

fn symbol_display(symbol: &Symbol) -> (String, std::ops::Range<usize>) {
    let prefix = match symbol.kind {
        SymbolKind::Module | SymbolKind::Namespace | SymbolKind::Package => "mod ",
        SymbolKind::Struct => "struct ",
        SymbolKind::Enum => "enum ",
        SymbolKind::Interface => "trait ",
        SymbolKind::Function | SymbolKind::Method => "fn ",
        SymbolKind::Constant => "const ",
        SymbolKind::Field => "field ",
        SymbolKind::Variable => "let ",
        SymbolKind::EnumMember => "variant ",
        SymbolKind::TypeParameter => "type ",
        _ => "",
    };

    let display = format!("{prefix}{}", symbol.name);
    (display, prefix.len()..prefix.len() + symbol.name.len())
}

fn literal_label(
    display: String,
    filter_range: std::ops::Range<usize>,
    trailing_detail: Option<String>,
) -> zed::CodeLabel {
    let mut spans = vec![zed::CodeLabelSpan::literal(display.clone(), None)];

    if let Some(detail) = trailing_detail {
        spans.push(zed::CodeLabelSpan::literal(format!(" {detail}"), Some("comment".to_string())));
    }

    zed::CodeLabel { code: display, spans, filter_range: filter_range.into() }
}

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

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let lsp_settings =
            zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        let defaults = serde_json::json!({
            "enableCodeActions": true,
            "enableCompletions": true,
            "enableLightweightMode": false,
            "enableSemanticTokens": true,
            "enableSignatureHelp": true
        });

        Ok(Some(merge_json(defaults, lsp_settings.initialization_options)))
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let lsp_settings =
            zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        Ok(lsp_settings.settings)
    }

    fn label_for_completion(
        &self,
        _language_server_id: &zed::LanguageServerId,
        completion: Completion,
    ) -> Option<zed::CodeLabel> {
        let filter_end = trim_completion_name(&completion.label).len();
        let (display, trailing_detail) = completion_display_parts(&completion);
        Some(literal_label(display, 0..filter_end, trailing_detail))
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &zed::LanguageServerId,
        symbol: Symbol,
    ) -> Option<zed::CodeLabel> {
        let (display, filter_range) = symbol_display(&symbol);
        Some(literal_label(display, filter_range, None))
    }
}

zed::register_extension!(NoirExtension);
