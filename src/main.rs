mod rust;
mod c;
mod flutter;
mod ruby;
mod bash;
mod java;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "medic")]
#[command(about = "CLI tool. Project management and speeding up the workflow!\nCreator: Tomek Ondřej, 2024", long_about = None)]
struct Args {
    //Rust (default), Flutter, C, Java, Ruby, Bash
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Update,
    /// Commands for Rust projects
    Rust {
        #[command(subcommand)]
        command: rust::RustCommands,
    },

    /// Commands for Flutter projects
    Flutter {
        #[command(subcommand)]
        command: flutter::FlutterCommands,
    },

    /// Commands for C projects
    C {
        #[command(subcommand)]
        command: c::CCommands,
    },

    /// Commands for Java projects
    Java {
        #[command(subcommand)]
        command: java::JavaCommands,
    },

    /// Commands for Ruby projects
    Ruby {
        #[command(subcommand)]
        command: ruby::RubyCommands,
    },

    /// Commands for Bash projects
    Bash {
        #[command(subcommand)]
        command: bash::BashCommands,
    },
}


fn main() {
    let args = Args::parse();

    match args.commands {
        Commands::Update => {
            println!("Updating the Rust project...");

            // Step 1: Compile the newest version of the app
            let build_status = std::process::Command::new("cargo")
                .arg("build")
                .arg("--release")
                .status()
                .expect("Failed to build the project");

            if !build_status.success() {
                eprintln!("Build failed. Please check for errors.");
                return;
            }

            // Step 2: Find the path to the compiled binary
            let current_dir = std::env::current_dir().expect("Failed to get current directory");
            let binary_name = current_dir.file_name()
                .expect("Failed to get project name")
                .to_string_lossy()
                .to_string();

            let source_path = format!("{}/target/release/{}", current_dir.display(), binary_name);
            let destination_path = format!("/usr/local/bin/{}", binary_name);

            // Step 3: Copy the binary to /usr/local/bin
            std::fs::copy(&source_path, &destination_path)
                .expect("Failed to copy binary to /usr/local/bin");

            println!("Successfully updated the app to the latest version at /usr/local/bin/{}", binary_name);
        }
        Commands::Rust { command } => rust::handle_rust_commands(command),
        Commands::Flutter { command } => flutter::handle_flutter_commands(command),
        Commands::C { command } => c::handle_c_commands(command),
        Commands::Java { command } => java::handle_java_commands(command),
        Commands::Ruby { command } => ruby::handle_ruby_commands(command),
        Commands::Bash { command } => bash::handle_bash_commands(command),
    }
}