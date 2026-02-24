use clap::{Parser, Subcommand};
use std::process::{Command, exit};
use anyhow::Result;
use clap::CommandFactory;

use theseus::{commands, init, run};

#[derive(Parser, Debug)]
#[command(
    name = "tsus",
    about = "A student-friendly, Docker-powered container runner",
    disable_version_flag = true,
)]
struct Cli {
    #[command(subcommand)]
    command: Option<TsusCommand>,

    /// Passthrough arguments for docker when no tsus command matches
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    passthrough: Vec<String>,
}

#[derive(Subcommand, Debug)]
enum TsusCommand {
    /// Initialize a new Theseus project with a minimal Dockerfile
    Init {
        /// Project type (python, node, rust, go)
        language: String,
    },
    /// Run the current project in a development container
    Run,
    /// Stop and remove the project container
    Stop,
    /// Show the status of the project container
    Status,
    /// Clean up project images (use --all for system prune)
    Clean {
        #[arg(long)]
        all: bool,
    },
    /// Tail logs of the project container
    Logs,
    /// Open a shell in the running container
    Exec,
    /// Run diagnostics to verify Docker dependencies
    Doctor,
}

fn handle_command_error(res: Result<()>) {
    if let Err(e) = res {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(TsusCommand::Init { language }) => {
            handle_command_error(init::execute(language));
        }
        Some(TsusCommand::Run) => {
            handle_command_error(run::execute());
        }
        Some(TsusCommand::Stop) => {
            handle_command_error(commands::stop());
        }
        Some(TsusCommand::Status) => {
            handle_command_error(commands::status());
        }
        Some(TsusCommand::Clean { all }) => {
            handle_command_error(commands::clean(*all));
        }
        Some(TsusCommand::Logs) => {
            handle_command_error(commands::logs());
        }
        Some(TsusCommand::Exec) => {
            handle_command_error(commands::exec());
        }
        Some(TsusCommand::Doctor) => {
            handle_command_error(commands::doctor());
        }
        None => {
            // Forward everything to docker
            if !cli.passthrough.is_empty() {
                let status = Command::new("docker")
                    .args(&cli.passthrough)
                    .status()
                    .unwrap_or_else(|e| {
                        eprintln!(
                            "\nError: Could not execute 'docker'. Is Docker installed and on your PATH?\n\nDetails: {}\n",
                            e
                        );
                        exit(1);
                    });
                
                if let Some(code) = status.code() {
                    exit(code);
                } else {
                    // Process terminated by a signal
                    exit(1);
                }
            } else {
                // If no command and no passthrough, just show help
                let mut cmd = Cli::command();
                cmd.print_help()?;
            }
        }
    }

    Ok(())
}
