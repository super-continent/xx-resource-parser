use xx_resource_parser::character_files;

use std::fs::File;
use std::path::PathBuf;

use anyhow::anyhow;
use binread::BinRead;
use simplelog::{LevelFilter, TermLogger};
use structopt::StructOpt;

fn main() {
    if let Err(e) = run() {
        println!("error: {}", e)
    };
}

fn run() -> anyhow::Result<()> {
    let opts = Opt::from_args();

    let level_filter = match opts.verbose {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    TermLogger::init(
        level_filter,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stdout,
    )?;

    match opts.cmd {
        SubCmd::Parse {
            character,
            file: file_path,
        } => {
            let mut file = File::open(file_path)?;

            match character.to_lowercase().as_str() {
                "sol" => {
                    let character_specific__file = character_files::SolResource::read(&mut file)?;
                },
                i => return Err(anyhow!("Unknown character `{}`. Please input a known character name", i)),
            }
        }
        SubCmd::Rebuild => return Err(anyhow!("Feature not implemented yet!"))
    }
    Ok(())
}

#[derive(StructOpt)]
#[structopt(name = "Guilty Gear AC +R Resource Parser", author = "Made by Pangaea")]
/// Parses Guilty Gear +R resource files into an easily modified format and rebuilds them into their original format
///
/// To parse a characters files, you must specify a character name
///
/// List of valid names:
/// sol, ky
struct Opt {
    /// Specifies how verbose the log output should be
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
    #[structopt(subcommand)]
    cmd: SubCmd,
}

#[derive(StructOpt)]
enum SubCmd {
    Parse {
        #[structopt(rename_all = "upper")]
        /// Specifies which characters resource file is being parsed
        character: String,
        #[structopt(parse(from_os_str), rename_all = "upper")]
        file: PathBuf,
    },
    Rebuild,
}