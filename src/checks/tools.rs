use std::process::Command;

use super::{Check, Diagnostic, Level};

pub struct ToolsCheck {
    required: Vec<String>,
}

impl ToolsCheck {
    pub fn new(required: Vec<String>) -> Self {
        Self { required }
    }
}

impl Check for ToolsCheck {
    fn name(&self) -> &str {
        "tools"
    }

    fn run(&self) -> Vec<Diagnostic> {
        let mut results = Vec::new();

        for tool in &self.required {
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
