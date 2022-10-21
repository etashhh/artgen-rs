use console::style;

use artgen::{app::build_app, commands::Command, constants::*};

use anyhow::Result;

type ExitCode = i32;

// TODO: Error handling if asset folders don't follow required convention
// TODO: Error handling for .DS_STORE files
// TODO: Update piece of metadata without regenerating assets
// TODO: Wipe assets and regenerate everything
// TODO: Add Config

fn main() {
    let result = run();
    match result {
        Ok(exit_code) => {
            std::process::exit(exit_code);
        }
        // TODO: Provide better error reporting
        Err(err) => {
            eprintln!(
                "\n{} {} {}\n",
                style("There was an error during the run:").red().bold(),
                err,
                ERROR_EMOJI
            );
            std::process::exit(1);
        }
    }
}

fn run() -> Result<ExitCode> {
    let app = build_app();
    let matches = app.get_matches();

    if let Some((subcommand, matches)) = matches.subcommand() {
        let command = Command::read(subcommand);
        command.execute(matches);
    } else {
        panic!("Subcommand is required");
    }

    Ok(0)
}
