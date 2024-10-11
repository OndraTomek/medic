use clap::Subcommand;
use std::process::Command;

#[derive(Subcommand)]
pub enum FlutterCommands {
    Create {
        project_name: String,
    },
    Build,
    Test,
    FixIOSEmulator
}

pub fn handle_flutter_commands(command: FlutterCommands) {
    match command {
        FlutterCommands::Create { project_name } => {
            println!("Creating Flutter project: {}", project_name);
            // Add Flutter-specific project creation logic here
        }
        FlutterCommands::Build => {
            println!("Building Flutter project");
            // Add Flutter build logic here
        }
        FlutterCommands::Test => {
            println!("Testing Flutter project");
            // Add Flutter test logic here
        }
        FlutterCommands::FixIOSEmulator => {
            let path = "~/Library/Developer/CoreSimulator/Caches";

            let expanded_path = shellexpand::tilde(path).to_string();

            // Execute the `rm -r` command
            let status = Command::new("rm")
                .arg("-r")
                .arg(&expanded_path)    
                .status()
                .expect("Failed to execute command");

            // Check the result of the command
            if status.success() {
                println!("Successfully removed: {}", expanded_path);
            } else {
                eprintln!("Failed to remove: {}", expanded_path);
            }
                }
        }
}