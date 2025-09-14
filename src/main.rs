use std::env;
use clap::{Parser, Subcommand};
mod project;
mod venv;
use project::create_project;
use venv::create_venv;


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
	Venv {
		path: String
	},
}



fn main() {
	let cli = Cli::parse();
	match cli.command {
	Commands::Create { name } => {
            create_project(&name);
        }
        Commands::Install => {
            println!("Installing dependencies...");
            // do install logic
        }
        Commands::Add { package } => {
            println!("Adding dependency: {}", package);
            // do dependency add logic
        }
	Commands::Venv { path } => {
	    match create_venv(&path, "/usr/bin/python3"){
		Ok(_) => println!("Virtual environment created"),
        	Err(e) => eprintln!("Error: {}", e),
		}
	}				 
}

}
