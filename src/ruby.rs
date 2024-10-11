use clap::Subcommand;

#[derive(Subcommand)]
pub enum RubyCommands {
    Create {
        project_name: String,
    },
    Run,
    Test,
}

pub fn handle_ruby_commands(command: RubyCommands) {
    match command {
        RubyCommands::Create { project_name } => {
            println!("Creating Ruby project: {}", project_name);
            // Add Ruby-specific project creation logic here
        }
        RubyCommands::Run => {
            println!("Running Ruby project");
            // Add Ruby run logic here
        }
        RubyCommands::Test => {
            println!("Testing Ruby project");
            // Add Ruby test logic here
        }
    }
}