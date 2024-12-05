use clap::Parser;
use std::process::exit;

#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
    #[arg(id = "arch", help = "Archive?", short = 'a', long = "archive", required_unless_present = "unarch")]
    do_archive: bool,
    #[arg(id = "unarch", help = "Unarchive?", short = 'u', long = "unarchive", required_unless_present = "arch")]
    do_unarchive: bool,
    #[arg(id = "val", help = "Validate the archive's structure, this overrides other arguments", short = 'c', long = "check", required = false)]
    validate: bool,
    #[arg(id = "verb", help = "Increase verbosity", short = 'v', long = "verbose", required = false)]
    verbose: bool,
    #[arg(id = "file", help = "File path (absolute or relative) to archive", required = true)]
    file: String
}

fn main() {
    let args = CliArgs::parse();
    if args.validate == true {
        // Future
    } else if args.do_archive && args.do_unarchive == true {
        eprintln!("Cannot archive and unarchive at the same time!");
        exit(1);
    } else if args.do_archive == true {
        // Future
    } else if args.do_unarchive == true {
        // Future
    }
}