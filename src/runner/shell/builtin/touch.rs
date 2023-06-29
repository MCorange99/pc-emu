use std::{fmt::Write, path::PathBuf};

use clap::Parser;

use crate::{runner::{shell::Shell, fs::HgPath}, screen::Screen};

#[derive(Debug, Parser)]
struct Args {
    pub paths: Vec<PathBuf>,

    #[arg(short='p', long)]
    pub parent: bool,


}

pub fn exec(s: &mut Screen, sh: &mut Shell, argv: Vec<String>) -> color_eyre::Result<usize> {

    let args = Args::try_parse_from(&argv);

    if args.is_err() {
        writeln!(s, "Error: invalid arguments,\ntry: {} --help", argv[0])?;
    }
    let args = args?;


    for file in args.paths {
        let mut file = sh.cwd.join(file);
        if file.get_host_path().exists() {
            continue;
        }

        if let Err(e) = std::fs::File::create(&file.get_host_path()) {
            writeln!(s, "{}", e.to_string())?;
            return Ok(1);
        }
        writeln!(s, "created file {file:?}")?;

    }


    Ok(0)
}