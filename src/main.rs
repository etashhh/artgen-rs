use console::style;
// use std::path::Path;
// use std::{env, process};

use artgen::{
    app::build_app,
    constants::*,
    // run::{run, Config},
    utils::*,
};

use anyhow;

type ExitCode = i32;

fn main() {
    println!(
        "\n{} {}\n",
        style("Time to overhaul the codebase!").cyan().bold(),
        PALETTE_EMOJI
    );

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

    // let config = Config::new(env::args()).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });
    // if let Err(e) = run(Path::new(&config.dir)) {
    //     eprintln!("Application error: {}", e);
    //     process::exit(1);
    // }
    // TODO: Error handling if asset folders don't follow required convention
    // TODO: Error handling for .DS_STORE files
    // TODO: Update piece of metadata without regenerating assets
    // TODO: Wipe assets and regenerate everything
    // TODO: Add Config
}

fn run() -> anyhow::Result<ExitCode> {
    let app = build_app();
    let matches = app.get_matches();

    println!(
        "\n{} {}\n",
        style("We're generating some digital art!").yellow().bold(),
        PALETTE_EMOJI
    );

    if let Some((subcommand, matches)) = matches.subcommand() {
    } else {
        unreachable!("Subcommand is required");
    }

    Ok(0)
}
