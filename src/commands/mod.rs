use anyhow;
use clap::ArgMatches;

mod generate;
mod prelude;
mod select;
mod traits;

use traits::GenericCommand;

use generate::Generate;
use select::Select;

// Leaving in case I want to add different values in the future
pub enum Command {
    Generic(Box<dyn GenericCommand>),
}

impl Command {
    pub fn read(command: &str) -> Command {
        match command {
            "generate" => Command::Generic(Box::new(Generate)),
            "select" => Command::Generic(Box::new(Select)),
            _ => panic!("Unknown subcommand"),
        }
    }

    pub fn execute(&self, matches: &ArgMatches) -> anyhow::Result<()> {
        match self {
            Command::Generic(command) => command.run(matches),
            // _ => unreachable!("Not available"),
        }
    }
}
