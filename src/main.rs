mod api;
mod commands;
mod config;
mod git;
mod prompt;

use clap::{Parser, Subcommand};
use config::Config;
use std::process;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "claude-git", version = VERSION, about = "AI-powered git helpers using Claude")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Propose a commit message
    Msg,
    /// Propose + commit with confirmation
    Commit {
        /// Skip confirmation
        #[arg(short, long)]
        yes: bool,
    },
    /// Review changes for bugs
    Review,
    /// Generate PR description
    Pr {
        /// Base branch
        #[arg(default_value = "main")]
        base: String,
    },
    /// Explain what the changes do
    Explain,
    /// Show or set config
    Config {
        /// Config key
        key: Option<String>,
        /// Config value to set
        value: Option<String>,
    },
    /// Print shell aliases
    Aliases,
}

fn main() {
    let cli = Cli::parse();
    let mut config = Config::load();

    let result = match cli.command {
        Some(Commands::Msg) => commands::msg::run(&config),
        Some(Commands::Commit { yes }) => commands::commit::run(&config, yes),
        Some(Commands::Review) => commands::review::run(&config),
        Some(Commands::Pr { ref base }) => commands::pr::run(&config, base),
        Some(Commands::Explain) => commands::explain::run(&config),
        Some(Commands::Config { ref key, ref value }) => {
            match (key, value) {
                (None, _) => {
                    config.display();
                    Ok(())
                }
                (Some(k), None) => match config.get(k) {
                    Ok(v) => {
                        println!("{v}");
                        Ok(())
                    }
                    Err(e) => Err(e),
                },
                (Some(k), Some(v)) => {
                    config.set(k, v).map(|_| println!("{k} = {v}"))
                }
            }
        }
        Some(Commands::Aliases) => {
            print!(
                "# claude-git aliases\n\
                 alias gpm='claude-git msg'\n\
                 alias gac='claude-git commit'\n\
                 alias gacf='claude-git commit --yes'\n\
                 alias grev='claude-git review'\n\
                 alias gpr='claude-git pr'\n\
                 alias gex='claude-git explain'\n"
            );
            Ok(())
        }
        None => {
            Cli::parse_from(["claude-git", "--help"]);
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("{e}");
        process::exit(1);
    }
}
