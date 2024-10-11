use clap::Subcommand;

#[derive(Subcommand)]
pub enum BashCommands {
    Create {
        project_name: String,
    },
    Execute,
}

pub fn handle_bash_commands(command: BashCommands) {
    match command {
        BashCommands::Create { project_name } => {
            println!("Creating Bash script project: {}", project_name);
            // Add Bash-specific project creation logic here
        }
        BashCommands::Execute => {
            println!("Executing Bash script");
            // Add Bash execute logic here
        }
    }
}