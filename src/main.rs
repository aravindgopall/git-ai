mod ai;
mod branding;
mod config;
mod filters;
mod git_runner;
mod hooks;
mod hunk;
mod interact;
mod prompts;
mod push;
mod staging;
mod utils;
mod web;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "git-ai",
    version,
    about = "Clean diffs. Smart commits. AI magic."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Diff {
        #[arg(short, long)]
        prompt: Option<String>,
        #[arg(long)]
        profile: Option<String>,
    },
    Stage {
        #[arg(short, long, default_value = "false")]
        interactive: bool,
    },
    Web {},
    InstallHook {},
    UninstallHook {},
    Precommit {}, // ➡️ New command added here
    Push {},
}

fn main() {
    branding::show_banner();

    let cli = Cli::parse();

    match cli.command {
        Commands::Diff { prompt, profile } => {
            git_runner::run_diff(prompt, profile);
        }
        Commands::Stage { interactive } => {
            staging::run_staging(interactive);
        }
        Commands::Web {} => {
            web::start_server();
        }
        Commands::InstallHook {} => {
            hooks::install_hook();
        }
        Commands::UninstallHook {} => {
            hooks::uninstall_hook();
        }
        Commands::Precommit {} => {
            hooks::run_precommit(); // ➡️ Run precommit checks
        }
        Commands::Push {} => {
            push::push_changes();
        }
    }
}
