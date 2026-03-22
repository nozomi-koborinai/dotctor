use std::process::Command;

use super::{Check, Diagnostic, Level};

/// PATH に存在すべきコマンド一覧
const REQUIRED_TOOLS: &[&str] = &[
    "git", "nvim", "gh", "fzf", "fd", "lazygit", "deno", "node", "cargo", "wezterm",
];

pub struct ToolsCheck;

impl Check for ToolsCheck {
    fn name(&self) -> &str {
        "tools"
    }

    fn run(&self) -> Vec<Diagnostic> {
        let mut results = Vec::new();

        for tool in REQUIRED_TOOLS {
            let found = Command::new("which")
                .arg(tool)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);

            if found {
                results.push(Diagnostic {
                    level: Level::Ok,
                    message: format!("{tool} is installed"),
                });
            } else {
                results.push(Diagnostic {
                    level: Level::Error,
                    message: format!("{tool} is not installed"),
                });
            }
        }

        results
    }
}
