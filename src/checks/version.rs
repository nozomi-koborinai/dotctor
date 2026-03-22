use std::process::Command;

use super::{Check, Diagnostic, Level};

/// バージョン要件の定義
struct VersionRequirement {
    command: &'static str,
    args: &'static [&'static str],
    minimum: &'static str,
}

/// 最低バージョン要件
const REQUIREMENTS: &[VersionRequirement] = &[
    VersionRequirement {
        command: "node",
        args: &["--version"],
        minimum: "22",
    },
    VersionRequirement {
        command: "git",
        args: &["--version"],
        minimum: "2",
    },
    VersionRequirement {
        command: "nvim",
        args: &["--version"],
        minimum: "0.10",
    },
];

pub struct VersionCheck;

impl Check for VersionCheck {
    fn name(&self) -> &str {
        "version"
    }

    fn run(&self) -> Vec<Diagnostic> {
        let mut results = Vec::new();

        for req in REQUIREMENTS {
            let version = get_version(req.command, req.args);

            match version {
                Some(v) => {
                    if version_satisfies(&v, req.minimum) {
                        results.push(Diagnostic {
                            level: Level::Ok,
                            message: format!("{} {} (>= {})", req.command, v, req.minimum),
                        });
                    } else {
                        results.push(Diagnostic {
                            level: Level::Error,
                            message: format!(
                                "{} {} is below minimum {}",
                                req.command, v, req.minimum
                            ),
                        });
                    }
                }
                None => {
                    results.push(Diagnostic {
                        level: Level::Warn,
                        message: format!("{}: could not determine version", req.command),
                    });
                }
            }
        }

        results
    }
}

/// コマンドを実行してバージョン文字列を取得する
fn get_version(command: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(command).args(args).output().ok()?;
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
        } else if c != '.' {
            if let Some(s) = start {
                return Some(text[s..i].to_string());
            }
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
