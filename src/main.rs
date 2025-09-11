use std::env;
use clap::{Parser, Subcommand};
mod project;
use project::create_project



#[derive(Parser, Debug)]
#[command(name = "pymanager")]
#[command(version = "0.1")]
#[command(about = "Manage Python projects and dependencies", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}



#[derive(Subcommand, Debug)]
enum Commands {
	Create {
		name: String,
	},
	Install,
	Add {
		package: String
	},
}



fn main() {
	let cli = Cli::parse();
	match cli.command {
	Commands::Create { &name } => {
            create_project(name);
        }
        Commands::Install => {
            println!("Installing dependencies...");
            // do install logic
        }
        Commands::Add { package } => {
            println!("Adding dependency: {}", package);
            // do dependency add logic
        }
	}				 
}

