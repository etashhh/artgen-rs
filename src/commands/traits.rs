use clap::ArgMatches;

pub trait GenericCommand {
    fn run(&self, matches: &ArgMatches) -> anyhow::Result<()>;
}
