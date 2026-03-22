use std::path::Path;

use super::{Check, Diagnostic, Level};

/// チェック対象の symlink 定義
struct SymlinkEntry {
    source: &'static str,
    target: &'static str,
}

/// nozomi-koborinai/dotfiles 用の symlink 一覧
const SYMLINKS: &[SymlinkEntry] = &[
    SymlinkEntry {
        source: "configs/zshrc",
        target: "~/.zshrc",
    },
    SymlinkEntry {
        source: "configs/gitconfig",
        target: "~/.gitconfig",
    },
    SymlinkEntry {
        source: "configs/nvim",
        target: "~/.config/nvim",
    },
    SymlinkEntry {
        source: "configs/wezterm",
        target: "~/.config/wezterm",
    },
    SymlinkEntry {
        source: "configs/gh/config.yml",
        target: "~/.config/gh/config.yml",
    },
    SymlinkEntry {
        source: "configs/zeno/config.yml",
        target: "~/.config/zeno/config.yml",
    },
    SymlinkEntry {
        source: "configs/aerospace/aerospace.toml",
        target: "~/.config/aerospace/aerospace.toml",
    },
    SymlinkEntry {
        source: "configs/lazygit/config.yml",
        target: "~/Library/Application Support/lazygit/config.yml",
    },
];

pub struct SymlinkCheck;

impl Check for SymlinkCheck {
    fn name(&self) -> &str {
        "symlink"
    }

    fn run(&self) -> Vec<Diagnostic> {
        let mut results = Vec::new();

        for entry in SYMLINKS {
            let target = expand_tilde(entry.target);
            let path = Path::new(&target);

            if !path.exists() {
                results.push(Diagnostic {
                    level: Level::Error,
                    message: format!("{} does not exist", entry.target),
                });
            } else if !path.is_symlink() {
                results.push(Diagnostic {
                    level: Level::Warn,
                    message: format!("{} exists but is not a symlink", entry.target),
                });
            } else {
                results.push(Diagnostic {
                    level: Level::Ok,
                    message: format!("{} -> {}", entry.target, entry.source),
                });
            }
        }

        results
    }
}

fn expand_tilde(path: &str) -> String {
    if let Some(rest) = path.strip_prefix("~/") {
        let home = std::env::var("HOME").unwrap_or_default();
        format!("{home}/{rest}")
    } else {
        path.to_string()
    }
}
