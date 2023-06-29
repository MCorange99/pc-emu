use std::fmt::Write;

use crate::{runner::shell::Shell, screen::Screen};
use crate::runner::fs::HgPath;

pub fn exec(s: &mut Screen, sh: &mut Shell, _: Vec<String>) -> color_eyre::Result<usize> {
    
    writeln!(s, "{}", sh.cwd.get_host_path().to_str().unwrap())?;

    Ok(0)
}