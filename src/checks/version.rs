use std::collections::BTreeMap;
use std::process::Command;

use super::{Check, Diagnostic, Level};

pub struct VersionCheck {
    /// command -> ">= X.Y" のマッピング
    versions: BTreeMap<String, String>,
}

impl VersionCheck {
    pub fn new(versions: BTreeMap<String, String>) -> Self {
        Self { versions }
    }
}

impl Check for VersionCheck {
    fn name(&self) -> &str {
        "version"
    }

    fn run(&self) -> Vec<Diagnostic> {
        let mut results = Vec::new();

        for (command, requirement) in &self.versions {
            let minimum = requirement
                .strip_prefix(">= ")
                .unwrap_or(requirement.as_str());

            let version = get_version(command);

            match version {
                Some(v) => {
                    if version_satisfies(&v, minimum) {
                        results.push(Diagnostic {
                            level: Level::Ok,
                            message: format!("{command} {v} (>= {minimum})"),
                        });
                    } else {
                        results.push(Diagnostic {
                            level: Level::Error,
                            message: format!("{command} {v} is below minimum {minimum}"),
                        });
                    }
                }
                None => {
                    results.push(Diagnostic {
                        level: Level::Warn,
                        message: format!("{command}: could not determine version"),
                    });
                }
            }
        }

        results
    }
}

/// コマンドを実行してバージョン文字列を取得する
fn get_version(command: &str) -> Option<String> {
    let output = Command::new(command).arg("--version").output().ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    extract_version(&stdout)
}

/// 文字列からバージョン番号（数字.数字...）を抽出する
fn extract_version(text: &str) -> Option<String> {
    let mut start = None;

    for (i, c) in text.char_indices() {
        if c.is_ascii_digit() {
            if start.is_none() {
                start = Some(i);
            }
        } else if c != '.'
            && let Some(s) = start
        {
            return Some(text[s..i].to_string());
        }
    }

    start.map(|s| text[s..].trim().to_string())
}

/// 簡易的なバージョン比較（メジャー.マイナー レベル）
fn version_satisfies(actual: &str, minimum: &str) -> bool {
    let actual_parts: Vec<u32> = actual.split('.').filter_map(|s| s.parse().ok()).collect();
    let min_parts: Vec<u32> = minimum.split('.').filter_map(|s| s.parse().ok()).collect();

    for (a, m) in actual_parts.iter().zip(min_parts.iter()) {
        if a > m {
            return true;
        }
        if a < m {
            return false;
        }
    }

    actual_parts.len() >= min_parts.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_version_from_node() {
        assert_eq!(extract_version("v22.12.0"), Some("22.12.0".to_string()));
    }

    #[test]
    fn extract_version_from_git() {
        assert_eq!(
            extract_version("git version 2.53.0"),
            Some("2.53.0".to_string())
        );
    }

    #[test]
    fn extract_version_from_nvim() {
        assert_eq!(extract_version("NVIM v0.10.4"), Some("0.10.4".to_string()));
    }

    #[test]
    fn extract_version_empty() {
        assert_eq!(extract_version("no version here!"), None);
    }

    #[test]
    fn satisfies_equal() {
        assert!(version_satisfies("22.12.0", "22"));
    }

    #[test]
    fn satisfies_higher_major() {
        assert!(version_satisfies("23.0.0", "22"));
    }

    #[test]
    fn satisfies_lower_major() {
        assert!(!version_satisfies("21.0.0", "22"));
    }

    #[test]
    fn satisfies_minor_comparison() {
        assert!(version_satisfies("0.11.0", "0.10"));
        assert!(!version_satisfies("0.9.0", "0.10"));
    }

    #[test]
    fn satisfies_exact_match() {
        assert!(version_satisfies("0.10", "0.10"));
    }
}
