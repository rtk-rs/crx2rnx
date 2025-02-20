use clap::{Arg, ArgAction, ArgMatches, ColorChoice, Command};
use std::path::{Path, PathBuf};

pub struct Cli {
    /// arguments passed by user
    pub matches: ArgMatches,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            matches: {
                Command::new("crx2rnx")
                    .author("Guillaume W. Bres <guillaume.bressaix@gmail.com>")
                    .version(env!("CARGO_PKG_VERSION"))
                    .about("CRINEX files decompression tool")
                    .arg_required_else_help(true)
                    .color(ColorChoice::Always)
                    .arg(
                        Arg::new("filepath")
                            .help("Input RINEX file")
                            .value_name("filepath")
                            .required(true),
                    )
                    .arg(
                        Arg::new("quiet")
                            .short('q')
                            .action(ArgAction::SetTrue)
                            .help("Make the tool quiet"),
                    )
                    .arg(
                        Arg::new("short")
                            .short('s')
                            .action(ArgAction::SetTrue)
                            .conflicts_with("output")
                            .help("Prefer V2 (short) filename")
                            .long_help(
                                "Synthesize a short RINEX file (V2 convention)
in case your input is V3 like. Otherwise, this has no effect.
This tool will preserve the input format by default.",
                            ),
                    )
                    .arg(
                        Arg::new("output")
                            .short('o')
                            .long("output")
                            .required(false)
                            .value_name("filename")
                            .action(ArgAction::Set)
                            .conflicts_with_all(["short"])
                            .help("Define custom output name instead of standard convention."),
                    )
                    .arg(
                        Arg::new("prefix")
                            .long("prefix")
                            .required(false)
                            .value_name("directory")
                            .help("Define custom output location (folder) that must exist."),
                    )
                    .get_matches()
            },
        }
    }
    pub fn quiet(&self) -> bool {
        self.matches.get_flag("quiet")
    }
    pub fn forced_short_v2(&self) -> bool {
        self.matches.get_flag("short")
    }
    pub fn input_path(&self) -> PathBuf {
        Path::new(self.matches.get_one::<String>("filepath").unwrap()).to_path_buf()
    }
    pub fn custom_name(&self) -> Option<&String> {
        self.matches.get_one::<String>("output")
    }
    pub fn custom_prefix(&self) -> Option<&String> {
        self.matches.get_one::<String>("prefix")
    }
}
