use std::collections::BTreeMap;
use std::path::Path;

use super::{Check, Diagnostic, Level};

pub struct SymlinkCheck {
    /// target -> source のマッピング
    symlinks: BTreeMap<String, String>,
}

impl SymlinkCheck {
    pub fn new(symlinks: BTreeMap<String, String>) -> Self {
        Self { symlinks }
    }
}

impl Check for SymlinkCheck {
    fn name(&self) -> &str {
        "symlink"
    }

    fn run(&self) -> Vec<Diagnostic> {
        let mut results = Vec::new();

        for (target, source) in &self.symlinks {
            let expanded = expand_tilde(target);
            let path = Path::new(&expanded);

            if !path.exists() {
                results.push(Diagnostic {
                    level: Level::Error,
                    message: format!("{target} does not exist"),
                });
            } else if !path.is_symlink() {
                results.push(Diagnostic {
                    level: Level::Warn,
                    message: format!("{target} exists but is not a symlink"),
                });
            } else {
                results.push(Diagnostic {
                    level: Level::Ok,
                    message: format!("{target} -> {source}"),
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
