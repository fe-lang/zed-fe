use zed_extension_api::{self as zed, CodeLabel, CodeLabelSpan, Result};

const SERVER_PATH: &str = "fe-language-server";

struct FeAnalyzerExtension;
impl FeAnalyzerExtension {
    fn server_script_path(&mut self, worktree: &zed::Worktree) -> Result<String> {
        worktree
            .which(SERVER_PATH)
            .ok_or_else(|| "fe-language-server not found in PATH".into())
    }
}

impl zed::Extension for FeAnalyzerExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.server_script_path(worktree)?;
        Ok(zed::Command {
            command: server_path,
            env: Default::default(),
            args: Default::default(),
        })
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &zed::LanguageServerId,
        symbol: zed::lsp::Symbol,
    ) -> Option<CodeLabel> {
        use zed::lsp::SymbolKind;

        let (keyword, suffix) = match symbol.kind {
            SymbolKind::Function | SymbolKind::Method => ("fn ", "() {}"),
            SymbolKind::Struct => ("struct ", " {}"),
            SymbolKind::Enum => ("enum ", " {}"),
            SymbolKind::Interface => ("trait ", " {}"),
            SymbolKind::Constant => ("const ", ": ()"),
            SymbolKind::Module => ("mod ", " {}"),
            SymbolKind::Class => ("contract ", " {}"),
            _ => return None,
        };

        let code = format!("{keyword}{}{suffix}", symbol.name);
        let name_start = keyword.len() as u32;
        let name_end = name_start + symbol.name.len() as u32;

        Some(CodeLabel {
            spans: vec![CodeLabelSpan::code_range(0..code.len() as u32)],
            filter_range: (name_start..name_end).into(),
            code,
        })
    }
}

zed::register_extension!(FeAnalyzerExtension);
