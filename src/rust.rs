use clap::Subcommand;

#[derive(Subcommand)]
pub enum RustCommands {
    Create {
        project_name: String,
    },
    Build,
    Run
}

pub fn handle_rust_commands(command: RustCommands) {
    match command {
        RustCommands::Create { project_name } => {
            println!("Creating Rust project: {}", project_name);
            // Add Rust-specific project creation logic here
        }
        RustCommands::Build => {
            println!("Building Rust project");
            // Add Rust build logic here
        }
        RustCommands::Run => {
            println!("Running Rust project");
            // Add Rust run logic here
        }
    }
}