use clap::Parser;
use dotctor::checks::Check;
use dotctor::checks::symlink::SymlinkCheck;
use dotctor::checks::tools::ToolsCheck;
use dotctor::checks::version::VersionCheck;
use dotctor::{config, report};

/// A CLI tool to diagnose your dotfiles health
#[derive(Parser)]
#[command(name = "dotctor", version)]
struct Cli {
    /// Run only the specified check (symlink, tools, version)
    #[arg(short, long)]
    check: Option<String>,

    /// Path to config file (default: dotctor.toml or ~/.dotctor.toml)
    #[arg(long)]
    config: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let cfg = match config::load(cli.config.as_deref()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    let checkers: Vec<Box<dyn Check>> = match cli.check.as_deref() {
        Some("symlink") => vec![Box::new(SymlinkCheck::new(cfg.symlinks))],
        Some("tools") => vec![Box::new(ToolsCheck::new(cfg.tools.required))],
        Some("version") => vec![Box::new(VersionCheck::new(cfg.versions))],
        Some(name) => {
            eprintln!("Unknown check: {name}");
            eprintln!("Available checks: symlink, tools, version");
            std::process::exit(1);
        }
        None => vec![
            Box::new(SymlinkCheck::new(cfg.symlinks)),
            Box::new(ToolsCheck::new(cfg.tools.required)),
            Box::new(VersionCheck::new(cfg.versions)),
        ],
    };

    println!("dotctor - diagnosing your dotfiles...\n");

    let mut has_any_error = false;

    for checker in &checkers {
        let diagnostics = checker.run();
        report::print_report(checker.name(), &diagnostics);
        if report::has_errors(&diagnostics) {
            has_any_error = true;
        }
    }

    if has_any_error {
        std::process::exit(1);
    }
}
