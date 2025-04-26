mod add;
mod ai;
mod branding;
mod commit;
mod config;
mod filters;
mod git_runner;
mod hooks;
mod hunk;
mod interact;
mod llms;
mod prompts;
mod push;
mod staging;
mod status;
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
        #[arg(long, default_value = "false")]
        ai: bool,
    },
    Add {
        #[arg(short, long)]
        all: bool,
        files: Vec<String>,
    },
    Commit {
        #[arg(long, default_value = "false")]
        amend: bool,
        #[arg(long, default_value = "false")]
        reword: bool,
        #[arg(long, default_value = "false")]
        ai: bool,
    },
    Web {},
    InstallHook {},
    UninstallHook {},
    Precommit {}, // ➡️ New command added here
    Push {},
    Status {},
}

#[tokio::main]
async fn main() {
    ai::init_llm_backend(); // 💥 Initialize backend early

    branding::show_banner();

    let cli = Cli::parse();

    match cli.command {
        Commands::Diff { prompt, profile } => {
            git_runner::run_diff(prompt, profile);
        }
        Commands::Stage { interactive, ai } => {
            staging::run_staging(interactive, ai);
        }
        Commands::Add { all, files } => {
            add::add_files(all, files);
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
        Commands::Status {} => {
            status::show_git_status();
        }
        Commands::Commit { amend, reword, ai } => {
            commit::commit_changes(amend, reword, ai).await;
        }
    }
}
