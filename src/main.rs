use clap::Parser;
use dotctor::checks::symlink::SymlinkCheck;
use dotctor::checks::tools::ToolsCheck;
use dotctor::checks::version::VersionCheck;
use dotctor::checks::Check;
use dotctor::report;

/// A CLI tool to diagnose your dotfiles health
#[derive(Parser)]
#[command(name = "dotctor", version)]
struct Cli {
    /// Run only the specified check (symlink, tools, version)
    #[arg(short, long)]
    check: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let checkers: Vec<Box<dyn Check>> = match cli.check.as_deref() {
        Some("symlink") => vec![Box::new(SymlinkCheck)],
        Some("tools") => vec![Box::new(ToolsCheck)],
        Some("version") => vec![Box::new(VersionCheck)],
        Some(name) => {
            eprintln!("Unknown check: {name}");
            eprintln!("Available checks: symlink, tools, version");
            std::process::exit(1);
        }
        None => vec![
            Box::new(SymlinkCheck),
            Box::new(ToolsCheck),
            Box::new(VersionCheck),
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
