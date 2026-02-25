use zed_extension_api::{self as zed, CodeLabel, CodeLabelSpan, Result};

const SERVER_PATH: &str = "fe";
const SERVER_PATH_ENV: &str = "FE_PATH";

struct FeAnalyzerExtension;
impl FeAnalyzerExtension {
    fn server_path_from_env(worktree: &zed::Worktree) -> Option<String> {
        worktree.shell_env().into_iter().find_map(|(name, value)| {
            if name == SERVER_PATH_ENV && !value.is_empty() {
                Some(value)
            } else {
                None
            }
        })
    }

    fn server_script_path(&mut self, worktree: &zed::Worktree) -> Result<String> {
        if let Some(path) = Self::server_path_from_env(worktree) {
            return Ok(path);
        }

        if let Some(path) = worktree.which(SERVER_PATH) {
            return Ok(path);
        }

        if let Some(home) = worktree.shell_env().into_iter().find_map(|(k, v)| {
            if k == "HOME" { Some(v) } else { None }
        }) {
            // Check ~/.fe/bin (installed by feup)
            let feup_path = format!("{home}/.fe/bin/{SERVER_PATH}");
            if std::fs::metadata(&feup_path).is_ok() {
                return Ok(feup_path);
            }

            // Check ~/.cargo/bin (installed from source)
            let cargo_path = format!("{home}/.cargo/bin/{SERVER_PATH}");
            if std::fs::metadata(&cargo_path).is_ok() {
                return Ok(cargo_path);
            }
        }

        Err("fe not found. Set FE_PATH or install with: curl -L https://raw.githubusercontent.com/argotorg/fe/master/feup/feup.sh | bash".into())
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
            env: worktree.shell_env(),
            args: vec!["lsp".into()],
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
