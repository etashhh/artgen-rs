use clap::{crate_description, crate_name, crate_version, Arg, Command};

pub fn build_app() -> Command<'static> {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .color(clap::ColorChoice::Auto)
        .dont_collapse_args_in_usage(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_negative_numbers(false)
        .subcommand(
            Command::new("generate")
                .alias("gen")
                .alias("create")
                .about("Generate asset(s) based on given input layers")
                .arg(
                    Arg::new("root")
                        .help("Root directory where the layers are stored")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("number")
                        .long("number")
                        .short('n')
                        .help("Number of assets to generate in the current run")
                        .takes_value(true)
                        .default_value("5")
                        .value_name("count"),
                )
                .arg(
                    Arg::new("unique")
                        .long("unique")
                        .short('u')
                        .help(
                            "Boolean flag to indicate whether the generated set \
                                should be all unique assets",
                        )
                        .takes_value(true)
                        .possible_values(&["yes", "y", "no", "n"])
                        .value_name("unique flag")
                        .default_value("yes"),
                )
                .arg(
                    Arg::new("fresh")
                        .long("fresh")
                        .short('f')
                        .help(
                            "Boolean flag to indicate whether the generated set \
                                should wipe existing assets and start fresh",
                        )
                        .takes_value(true)
                        .possible_values(&["yes", "no"])
                        .value_name("fresh flag")
                        .default_value("no"),
                ),
        )
}

#[test]
fn verify_app() {
    build_app().debug_assert()
}
