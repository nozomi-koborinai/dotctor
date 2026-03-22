pub mod symlink;
pub mod tools;
pub mod version;

/// チェック結果1件
pub struct Diagnostic {
    pub level: Level,
    pub message: String,
}

pub enum Level {
    Ok,
    Warn,
    Error,
}

/// 全チェッカーが実装する trait
pub trait Check {
    fn name(&self) -> &str;
    fn run(&self) -> Vec<Diagnostic>;
}
