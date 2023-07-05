use std::{fmt::Write, path::PathBuf};

use crate::{runner::{shell::Shell, fs::HgPath}, screen::Screen};
use crate::runner::hasm::parser;
use clap::Parser;
use bincode;

#[derive(Debug, Parser)]
struct Args {
    pub inf: PathBuf,

    #[arg(short='o', long)]
    pub outf: Option<PathBuf>,

}

pub fn exec(s: &mut Screen, sh: &mut Shell, argv: Vec<String>) -> color_eyre::Result<usize> {
    match hsc(s, sh, argv) {
        Ok(r) => Ok(r),
        Err(e) => {
            writeln!(s, "{} ", e.to_string())?;
            Ok(0)
        }
    }
}


pub fn hsc(s: &mut Screen, sh: &mut Shell, argv: Vec<String>) -> color_eyre::Result<usize> {

    let args = Args::try_parse_from(&argv);

    match args {
        Ok(args) => {
            // let i_f = args.path.unwrap_or_else(|| PathBuf::from("./a.out"));
            let mut i_f = sh.cwd.join(args.inf);
            let mut o_f = sh.cwd.join(args.outf.unwrap_or_else(|| PathBuf::from("./a.out")));


            let code = std::fs::read_to_string(i_f.get_host_path())?;
            
            let mut compiler = parser::Parser::new(code, i_f.to_str().unwrap().to_string());

            compiler.parse()?;
            // compiler.run_program(code, i_f.to_str().unwrap().to_string())?;
            
            let prog = compiler.get_program();
            let mut bin: Vec<u8> = vec![b'.', b'H', b'A', b'S', b'M'];
            bin.append(&mut bincode::serialize(&prog)?);
            std::fs::write(o_f.get_host_path(), bin)?;
            std::fs::write(o_f.with_extension("dbg").get_host_path(),  format!("{:#?}", compiler.get_program()))?;
        }
        Err(_) => {
            writeln!(s, "Error: invalid arguments,\ntry: ls --help")?;
        }
    }
    
    Ok(0)
}