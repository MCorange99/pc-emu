use std::{fmt::Write, path::PathBuf};

use crate::{runner::{shell::Shell, fs::HgPath}, screen::Screen};

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    pub path: Option<PathBuf>,

    #[arg(short='a', long)]
    pub all: bool,

    #[arg(short='A', long)]
    pub almost_all: bool,

    #[arg(short='l', long)]
    pub long: bool,


}

pub fn exec(s: &mut Screen, sh: &mut Shell, argv: Vec<String>) -> color_eyre::Result<usize> {

    let args = Args::try_parse_from(&argv);

    match args {
        Ok(mut args) => {
            let path = if let Some(p) = &mut args.path {p} else {&mut sh.cwd};
            let host_path = path.get_host_path();

            if !host_path.exists() {
                writeln!(s, "Path {path:?} doesnt exist")?;
                return Ok(0);
            }

            if host_path.is_dir() {
                
                let mut max_len = 0;
                if args.long {
                    for f in std::fs::read_dir(host_path.clone())? {
                        let f = f?;
                        let name = f.file_name();
                        if name.len() > max_len {
                            max_len = name.len();
                        }
                    }
                }


                for f in std::fs::read_dir(host_path)? {
                    let f = f?;
                    let extra = if f.file_type()?.is_dir() {"/"} else {" "};

                    let mut buf = format!("{}{extra} ",f.file_name().to_str().unwrap());

                    if args.long {
                        let nl = f.file_name().len();
                        for _ in 0..max_len-nl {
                            buf.push(' ');
                        }
                        if f.file_type()?.is_file() {
                            let size = std::fs::read(f.path())?.len();
                            buf.push_str(format!("{} ", get_short_size(size)).as_str());
                        } else {
                            buf.push_str(format!("0 ").as_str());
                        }
                    }


                    writeln!(s, "{}", buf)?;
                }
            } else {
                // single file
            }

        }
        Err(_) => {
            writeln!(s, "Error: invalid arguments,\ntry: ls --help")?;
        }
    }
    
    Ok(0)
}



fn get_short_size(size: usize) -> String {
    
    match size {
        s if s < 1025 => {
            return format!("{s}B")
        }
        s if s < 1024 * 1024 + 1 => {
            return format!("{}K", s/1024)
        }
        s if s < 1024 * 1024 * 1024 + 1 => {
            return format!("{}M", s/1024/1024)
        }
        s => {
            return format!("{}G", s/1024/1024/1024)
        }
    }
}