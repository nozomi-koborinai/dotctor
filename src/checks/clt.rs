use std::process::Command;

use super::{Check, Diagnostic, Level};

/// Homebrew のビルド環境が Command Line Tools (CLT) の問題で壊れていないかを
/// `brew doctor` の個別チェックで診断する。
///
/// macOS をアップデートすると CLT が古いまま取り残され、ソースビルドを伴う
/// `brew install` が失敗することがある。実際にビルドが失敗する状態を Error、
/// 更新推奨レベルを Warn として報告する。macOS 以外や brew がない環境では
/// チェック対象外として何も報告しない。
pub struct CltCheck;

/// これらが失敗していると brew のソースビルドが実際に失敗する
const CRITICAL_CHECKS: &[&str] = &[
    "check_clt_minimum_version",
    "check_if_supported_sdk_available",
];

/// 新しい CLT が出ているだけで、直ちにビルドが失敗するわけではない
const ADVISORY_CHECKS: &[&str] = &["check_clt_up_to_date"];

impl Check for CltCheck {
    fn name(&self) -> &str {
        "clt"
    }

    fn run(&self) -> Vec<Diagnostic> {
        if !cfg!(target_os = "macos") || !brew_exists() {
            return Vec::new();
        }

        match run_brew_doctor(CRITICAL_CHECKS) {
            Ok(None) => {}
            Ok(Some(warning)) => {
                return vec![Diagnostic {
                    level: Level::Error,
                    message: format!("{warning} (run `brew doctor` for how to fix)"),
                }];
            }
            Err(e) => {
                return vec![Diagnostic {
                    level: Level::Warn,
                    message: format!("could not run brew doctor: {e}"),
                }];
            }
        }

        match run_brew_doctor(ADVISORY_CHECKS) {
            Ok(None) => vec![Diagnostic {
                level: Level::Ok,
                message: "Command Line Tools support this macOS".to_string(),
            }],
            Ok(Some(warning)) => vec![Diagnostic {
                level: Level::Warn,
                message: warning,
            }],
            Err(e) => vec![Diagnostic {
                level: Level::Warn,
                message: format!("could not run brew doctor: {e}"),
            }],
        }
    }
}

fn brew_exists() -> bool {
    Command::new("which")
        .arg("brew")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// `brew doctor` の個別チェックを実行し、問題があれば警告の要約を返す
fn run_brew_doctor(checks: &[&str]) -> Result<Option<String>, String> {
    let output = Command::new("brew")
        .arg("doctor")
        .args(checks)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        return Ok(None);
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    Ok(Some(summarize_warning(&stderr)))
}

/// brew doctor の stderr から最初の Warning 行を取り出して 1 行に要約する
fn summarize_warning(stderr: &str) -> String {
    stderr
        .lines()
        .find_map(|line| line.trim().strip_prefix("Warning: "))
        .unwrap_or("Command Line Tools are outdated for this macOS")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarize_picks_first_warning_line() {
        let stderr = "Please note that these warnings are just used to help.\n\
                      \n\
                      Warning: Your Command Line Tools (CLT) does not support macOS 26.\n\
                      It is either outdated or was modified.\n";
        assert_eq!(
            summarize_warning(stderr),
            "Your Command Line Tools (CLT) does not support macOS 26."
        );
    }

    #[test]
    fn summarize_falls_back_without_warning_line() {
        assert_eq!(
            summarize_warning("something unexpected"),
            "Command Line Tools are outdated for this macOS"
        );
    }
}
