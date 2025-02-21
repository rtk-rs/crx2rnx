mod cli;

use cli::Cli;
use rinex::prelude::Rinex;

//use std::path::PathBuf;

// fn input_name(path: &PathBuf) -> String {
//     let stem = path
//         .file_stem()
//         .expect("failed to determine input file name")
//         .to_str()
//         .expect("failed to determine input file name");
//
//     if stem.ends_with(".crx") {
//         stem.strip_suffix(".crx")
//             .expect("failed to determine input file name")
//             .to_string()
//     } else {
//         stem.to_string()
//     }
// }

fn main() {
    let cli = Cli::new();

    let quiet = cli.quiet();
    let input_path = cli.input_path();
    let input_path_str = input_path.to_string_lossy();

    // let manual_gzip = cli.gzip();
    // let manual_unzip = cli.unzip();
    let forced_short_v2 = cli.forced_short_v2();

    let gzip_input = input_path_str.ends_with(".gz");

    let rinex = if gzip_input {
        Rinex::from_gzip_file(&input_path)
    } else {
        Rinex::from_file(&input_path)
    };

    let mut rinex = rinex
        .unwrap_or_else(|e| panic!("RINEX parsing error: {}", e));

    rinex.crnx2rnx_mut();

    let version_major = rinex.header.version.major;

    let short_file_name = forced_short_v2 || version_major < 3;

    let output_name = match cli.custom_name() {
        Some(name) => name.clone(),
        _ => rinex.standard_filename(short_file_name, None, None),
    };

    let output_path = match cli.custom_prefix() {
        Some(prefix) => format!("{}/{}", prefix, output_name),
        None => output_name.to_string(),
    };

    rinex.to_file(&output_path)
        .unwrap_or_else(|e| panic!("RINEX formatting error: {}", e));

    if !quiet {
        println!("Decompressed {}", output_path);
    }
}
