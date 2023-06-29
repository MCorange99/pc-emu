use std::fmt::Write;

use crate::{runner::{shell::Shell, fs::HgPath}, screen::Screen};


pub fn exec(s: &mut Screen, sh: &mut Shell, argv: Vec<String>) -> color_eyre::Result<usize> {
    let mut p = sh.cwd.join(&argv[1]).simplify();

    if 
        !p.get_host_path().exists() ||
        !p.get_host_path().is_dir() {
        writeln!(s, "Folder {:?} doesnt exist", p)?;
    }

    sh.cwd = sh.cwd.join(&argv[1]).simplify();
    Ok(0)
}