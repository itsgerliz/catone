use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
    #[arg(id = "arch", help = "Archive?", short = 'a', long = "archive", required_unless_present = "unarch")]
    do_archive: bool,
    #[arg(id = "unarch", help = "Unarchive?", short = 'u', long = "unarchive", required_unless_present = "arch")]
    do_unarchive: bool,
    #[arg(id = "verb", help = "Increase verbosity", short = 'v', long = "verbose", required = false)]
    verbose: bool
}

fn main() {
    CliArgs::parse();
}