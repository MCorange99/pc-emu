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


    for dir in args.paths {
        let mut dir = sh.cwd.join(dir);
        if args.parent {
            if let Err(e) = std::fs::create_dir_all(&dir.get_host_path()) {
                writeln!(s, "{}", e.to_string())?;
                return Ok(1);
            }
        } else {
            if let Err(e) = std::fs::create_dir(&dir.get_host_path()) {
                writeln!(s, "{}", e.to_string())?;
                return Ok(1);
            }
        }
        writeln!(s, "created folder {dir:?}")?;

    }


    Ok(0)
}