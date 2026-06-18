use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(name = "asura")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Check {
        #[arg(long)]
        nginx: bool,
    },
}

pub fn run() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { nginx } if nginx => nginx(),
        _ => {}
    }
}

fn nginx() {
    match Command::new("nginx").arg("-v").output() {
        Ok(_) => println!("✅ Nginx is installed"),
        Err(_) => println!("❌ Nginx is not installed"),
    }
}