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
                            .value_name("[filepath]")
                            .required(true),
                    )
                    .arg(
                        Arg::new("short")
                            .short('s')
                            .action(ArgAction::SetTrue)
                            .conflicts_with("output")
                            .help(
                                "Synthesize a short RINEX file (V2 convention).
Otherwise, this tool will prefer modern (long filenames) when that may apply."
                            ),
                    )
                    .arg(
                        Arg::new("output")
                            .short('o')
                            .long("output")
                            .value_name("[filename]")
                            .action(ArgAction::Set)
                            .conflicts_with_all(["short"])
                            .help("Define custom output name instead of standard convention."),
                    )
                    .arg(
                        Arg::new("prefix")
                            .long("prefix")
                            .help("Define custom output location (folder) that must exist.")
                    )
                    .get_matches()
            },
        }
    }
    pub fn input_path(&self) -> PathBuf {
        Path::new(self.matches.get_one::<String>("filepath").unwrap()).to_path_buf()
    }
    pub fn output_name(&self) -> Option<&String> {
        self.matches.get_one::<String>("output")
    }
    pub fn workspace(&self) -> Option<&String> {
        self.matches.get_one::<String>("workspace")
    }
}
