mod config;
mod session;

use anyhow::Result;
use clap::Parser;
use config::Config;
use session::initialize_session;
use std::path::PathBuf;

/// MATW - AI-powered coding assistant in Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Working directory (defaults to current directory)
    #[arg(short, long)]
    dir: Option<PathBuf>,

    /// AI provider to use
    #[arg(long)]
    provider: Option<String>,

    /// Model to use
    #[arg(long)]
    model: Option<String>,

    /// API key (overrides config file)
    #[arg(long)]
    api_key: Option<String>,

    /// Configuration file path
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Run in simple mode (without TUI)
    #[arg(long)]
    simple: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Load configuration
    let mut config = Config::load()?;

    // Override with CLI arguments
    if let Some(provider) = args.provider {
        config.provider = provider;
    }
    if let Some(model) = args.model {
        config.model = model;
    }
    if let Some(api_key) = args.api_key {
        config.api_key = Some(api_key);
    }

    // Determine working directory
    let working_dir = args.dir.unwrap_or_else(|| std::env::current_dir().unwrap());

    if args.simple {
        // Simple mode: just print session info
        run_simple_mode(working_dir)?;
    } else {
        // TUI mode: run terminal UI
        matw_tui::run().await?;
    }

    Ok(())
}

fn run_simple_mode(working_dir: PathBuf) -> Result<()> {
    let session = initialize_session(working_dir)?;

    println!("MATW v{} - AI-powered coding assistant", env!("CARGO_PKG_VERSION"));
    println!();

    if let Some(git_info) = session.context().git_info() {
        println!("Git repository detected:");
        println!("  Branch: {}", git_info.branch);
        println!("  Commit: {}", git_info.commit);
        println!("  Root: {}", git_info.root.display());
        println!();
    }

    if session.context().claude_md().is_some() {
        println!("CLAUDE.md loaded");
    }

    println!("Session ID: {}", session.id());
    println!();
    println!("Use TUI mode for interactive session (omit --simple flag)");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parsing() {
        let args = Args::try_parse_from(["matw", "--provider", "glm"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.provider, Some("glm".to_string()));
    }

    #[test]
    fn test_args_with_dir() {
        let args = Args::try_parse_from(["matw", "--dir", "/tmp"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.dir, Some(PathBuf::from("/tmp")));
    }

    #[test]
    fn test_args_simple_mode() {
        let args = Args::try_parse_from(["matw", "--simple"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert!(args.simple);
    }
}
