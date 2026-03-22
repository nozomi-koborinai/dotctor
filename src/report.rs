use crate::checks::{Diagnostic, Level};

/// チェック結果を整形して表示する
pub fn print_report(check_name: &str, diagnostics: &[Diagnostic]) {
    println!("[{check_name}]");

    for d in diagnostics {
        let icon = match d.level {
            Level::Ok => "  OK",
            Level::Warn => "WARN",
            Level::Error => " ERR",
        };
        println!("  {icon}  {}", d.message);
    }

    println!();
}

/// 全チェック結果にエラーが含まれているかを返す
pub fn has_errors(diagnostics: &[Diagnostic]) -> bool {
    diagnostics.iter().any(|d| matches!(d.level, Level::Error))
}
