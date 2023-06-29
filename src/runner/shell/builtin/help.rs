use std::fmt::Write;

use crate::{runner::shell::Shell, screen::Screen};


pub fn exec(s: &mut Screen, _: &mut Shell, _: Vec<String>) -> color_eyre::Result<usize> {
    
    writeln!(s, "Help:")?;
    writeln!(s, "    help:  Shows help")?;
    writeln!(s, "    pwd:   Prints working directory")?;
    writeln!(s, "    hpwd:  Prints working directory for host")?;
    writeln!(s, "    echo:  Prints text")?;
    writeln!(s, "    exit:  Exits the program")?;
    writeln!(s, "    cd:    Changes directory")?;
    writeln!(s, "    ls:    List direcrory items")?;
    writeln!(s, "    mkdir: Create directories")?;
    writeln!(s, "    rm: Remove files and directories")?;

    Ok(0)
}