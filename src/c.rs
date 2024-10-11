use clap::Subcommand;

#[derive(Subcommand)]
pub enum CCommands {
    Create {
        project_name: String,
    },
    Compile,
    Run,
}

pub fn handle_c_commands(command: CCommands) {
    match command {
        CCommands::Create { project_name } => {
            println!("Creating C project: {}", project_name);
            // Add C-specific project creation logic here
        }
        CCommands::Compile => {
            println!("Compiling C project");
            // Add C compile logic here
        }
        CCommands::Run => {
            println!("Running C project");
            // Add C run logic here
        }
    }
}