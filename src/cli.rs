use clap::{Arg, Command};

pub struct CliArgs {
    pub auto_commit: bool,
}

pub fn parse_args() -> CliArgs {
    let matches = Command::new("Git Commit CLI")
        .version("1.0")
        .about("Generates conventional commits using OpenAI")
        .arg(
            Arg::new("auto-commit")
                .short('a')
                .long("auto-commit")
                .help("Automatically create the commit after generating the message")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    CliArgs {
        auto_commit: matches.get_flag("auto-commit"),
    }
}
