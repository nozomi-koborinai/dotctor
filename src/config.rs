use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub symlinks: BTreeMap<String, String>,
    pub tools: ToolsConfig,
    pub versions: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct ToolsConfig {
    pub required: Vec<String>,
}

/// 設定ファイルの検索順序:
/// 1. 引数で指定されたパス
/// 2. カレントディレクトリの dotctor.toml
/// 3. ~/.dotctor.toml
pub fn load(path: Option<&str>) -> Result<Config, String> {
    let config_path = match path {
        Some(p) => {
            let p = Path::new(p);
            if !p.exists() {
                return Err(format!("Config file not found: {}", p.display()));
            }
            p.to_path_buf()
        }
        None => find_config()?,
    };

    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read {}: {e}", config_path.display()))?;

    toml::from_str(&content).map_err(|e| format!("Failed to parse {}: {e}", config_path.display()))
}

fn find_config() -> Result<std::path::PathBuf, String> {
    let candidates = vec![
        std::path::PathBuf::from("dotctor.toml"),
        dirs_home().join(".dotctor.toml"),
    ];

    for path in candidates {
        if path.exists() {
            return Ok(path);
        }
    }

    Err("No config file found. Create dotctor.toml or ~/.dotctor.toml".to_string())
}

fn dirs_home() -> std::path::PathBuf {
    std::env::var("HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_toml() {
        let toml_str = r#"
[symlinks]
"~/.zshrc" = "configs/zshrc"

[tools]
required = ["git", "nvim"]

[versions]
node = ">= 22"
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.symlinks.len(), 1);
        assert_eq!(config.symlinks["~/.zshrc"], "configs/zshrc");
        assert_eq!(config.tools.required, vec!["git", "nvim"]);
        assert_eq!(config.versions["node"], ">= 22");
    }

    #[test]
    fn load_nonexistent_file() {
        let result = load(Some("/nonexistent/path.toml"));
        assert!(result.is_err());
    }
}
